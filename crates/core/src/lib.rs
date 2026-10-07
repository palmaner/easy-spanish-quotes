//! Shared product state and recovery primitives. No platform mutations here.
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    fs::{self, OpenOptions},
    io::{self, Write},
    path::Path,
};

pub const PROFILE: &str = "es-ES-altgr-zx-v1";
pub mod recovery;

pub fn allocate_windows_ids(
    names: &std::collections::HashSet<String>,
    ids: &std::collections::HashSet<String>,
) -> Result<(String, String), &'static str> {
    let klid = (1..=0xfff)
        .map(|i| format!("A{i:03X}040A"))
        .find(|x| !names.contains(x))
        .ok_or("No free custom KLID")?;
    let id = (1..=0xffff)
        .map(|i| format!("{i:04X}"))
        .find(|x| !ids.contains(x))
        .ok_or("No free Layout Id")?;
    Ok((klid, id))
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Mapping {
    pub open: char,
    pub close: char,
}
impl Mapping {
    pub fn validate(&self) -> Result<(), &'static str> {
        if !self.open.is_ascii_uppercase() || !self.close.is_ascii_uppercase() {
            return Err("Las teclas deben ser letras A–Z.");
        }
        if self.open == self.close {
            return Err("Las teclas deben ser distintas.");
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum State {
    Absent,
    InstalledDisabled,
    Enabled,
    PendingReboot,
    RepairRequired,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Receipt {
    pub schema: u32,
    pub profile: String,
    pub owner: String,
    pub original_klid: String,
    pub klid: String,
    pub layout_id: String,
    pub language_id: String,
    pub filename: String,
    pub sha256: String,
    pub state: State,
    pub pending: Option<String>,
    pub completed: Vec<String>,
    #[serde(default)]
    pub baseline_user_state: serde_json::Value,
}

impl Receipt {
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.schema != 1 || self.profile != PROFILE {
            return Err("Unknown receipt version/profile");
        }
        if self.original_klid != "0000040A" {
            return Err("Unsupported original layout");
        }
        if self.klid.len() != 8
            || !self.klid.starts_with('A')
            || !self.klid.ends_with("040A")
            || !self.klid.bytes().all(|b| b.is_ascii_hexdigit())
        {
            return Err("Invalid owned KLID");
        }
        if self.layout_id.len() != 4
            || self.layout_id == "0000"
            || !self.layout_id.bytes().all(|b| b.is_ascii_hexdigit())
        {
            return Err("Invalid layout ID");
        }
        if !matches!(self.language_id.as_str(), "040A" | "0C0A") {
            return Err("Unsupported enrollment language");
        }
        if self.sha256.len() != 64 || !self.sha256.bytes().all(|b| b.is_ascii_hexdigit()) {
            return Err("Invalid payload digest");
        }
        if self.filename != format!("esq-{}.dll", &self.sha256[..16]) {
            return Err("Invalid payload filename");
        }
        if self.owner.is_empty() {
            return Err("Missing owner");
        }
        Ok(())
    }
}

pub fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

/// Restore only if the value still equals our last write.
pub fn restore_owned<T: PartialEq + Clone>(baseline: &T, applied: &T, current: &T) -> T {
    if applied == current {
        baseline.clone()
    } else {
        current.clone()
    }
}

pub fn load_receipt(path: &Path) -> io::Result<Receipt> {
    let data = fs::read(path)?;
    if data.len() > 64 * 1024 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Receipt too large",
        ));
    }
    let receipt: Receipt = serde_json::from_slice(&data).map_err(invalid)?;
    receipt.validate().map_err(invalid)?;
    Ok(receipt)
}

fn invalid(e: impl std::fmt::Display) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, e.to_string())
}

/// Same-directory durable temporary write. Caller owns directory and commit primitive.
pub fn prepare_receipt(path: &Path, receipt: &Receipt) -> io::Result<std::path::PathBuf> {
    receipt.validate().map_err(invalid)?;
    let temporary = path.with_extension("pending");
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temporary)?;
    file.write_all(&serde_json::to_vec_pretty(receipt).map_err(invalid)?)?;
    file.sync_all()?;
    Ok(temporary)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn native_id_collision_is_avoided_independently() {
        let names = std::collections::HashSet::from(["A001040A".into()]);
        let ids = std::collections::HashSet::from(["0001".into(), "0002".into()]);
        assert_eq!(
            allocate_windows_ids(&names, &ids).unwrap(),
            ("A002040A".into(), "0003".into())
        );
    }
    #[test]
    fn allocation_exhaustion_is_reported() {
        let names = (1..=0xfff).map(|i| format!("A{i:03X}040A")).collect();
        assert!(allocate_windows_ids(&names, &std::collections::HashSet::new()).is_err());
    }
    #[test]
    fn invalid_pairs() {
        assert!(Mapping {
            open: 'Z',
            close: 'X'
        }
        .validate()
        .is_ok());
        for pair in [('Z', 'Z'), ('é', 'X'), ('z', 'X'), ('Z', '1')] {
            assert!(Mapping {
                open: pair.0,
                close: pair.1
            }
            .validate()
            .is_err());
        }
    }
    #[test]
    fn preserve_external_edits() {
        assert_eq!(restore_owned(&"old", &"ours", &"ours"), "old");
        assert_eq!(
            restore_owned(&"old", &"ours", &"new-user-value"),
            "new-user-value"
        );
    }
    #[test]
    fn digest_known_vector() {
        assert_eq!(
            digest(b"abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }
    #[test]
    fn reject_privileged_path_in_receipt() {
        let mut r = Receipt {
            schema: 1,
            profile: PROFILE.into(),
            owner: "S-1-5-test".into(),
            original_klid: "0000040A".into(),
            klid: "A001040A".into(),
            layout_id: "0001".into(),
            language_id: "0C0A".into(),
            filename: format!("esq-{}.dll", &digest(b"abc")[..16]),
            sha256: digest(b"abc"),
            state: State::Absent,
            pending: None,
            completed: vec![],
            baseline_user_state: serde_json::Value::Null,
        };
        assert!(r.validate().is_ok());
        r.filename = "../../KBDSP.DLL".into();
        assert!(r.validate().is_err());
    }
}
