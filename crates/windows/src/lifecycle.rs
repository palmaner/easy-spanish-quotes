//! Bounded, unsigned proof installer for disposable, single-user x64 VMs only.
//! This is deliberately gated until W1-W5 and security/recovery tests pass.
use esq_core::{digest, Receipt, State, PROFILE};
use serde_json::{json, Value};
use std::{
    collections::HashSet,
    fs,
    io::{self, Write},
    os::windows::fs::MetadataExt,
    path::{Path, PathBuf},
    ptr,
};
use windows_sys::Win32::{
    Foundation::{
        CloseHandle, FreeLibrary, LocalFree, HANDLE, HMODULE, WAIT_ABANDONED, WAIT_OBJECT_0,
    },
    Security::Authorization::ConvertSidToStringSidW,
    Security::{
        GetTokenInformation, TokenElevation, TokenUser, TOKEN_ELEVATION, TOKEN_QUERY, TOKEN_USER,
    },
    Storage::FileSystem::{MoveFileExW, MOVEFILE_DELAY_UNTIL_REBOOT},
    System::{
        LibraryLoader::{GetProcAddress, LoadLibraryExW, LOAD_LIBRARY_SEARCH_SYSTEM32},
        Registry::RegFlushKey,
        SystemInformation::GetSystemDirectoryW,
        Threading::{
            CreateMutexW, GetCurrentProcess, OpenProcessToken, ReleaseMutex, WaitForSingleObject,
        },
    },
};
use winreg::{enums::*, RegKey};

const ROOT: &str = r"SOFTWARE\easy-spanish-quotes";
const LAYOUTS: &str = r"SYSTEM\CurrentControlSet\Control\Keyboard Layouts";
const PAYLOAD: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/layout.dll"));
struct InstallationLock(HANDLE);
impl Drop for InstallationLock {
    fn drop(&mut self) {
        unsafe {
            ReleaseMutex(self.0);
            CloseHandle(self.0);
        }
    }
}
fn lock_installation() -> Result<InstallationLock, String> {
    let name = wide(r"Global\easy-spanish-quotes-lifecycle-v1");
    let handle = unsafe { CreateMutexW(ptr::null(), 0, name.as_ptr()) };
    if handle.is_null() {
        return Err(err(io::Error::last_os_error()));
    }
    let waited = unsafe { WaitForSingleObject(handle, 0) };
    if waited != WAIT_OBJECT_0 && waited != WAIT_ABANDONED {
        unsafe {
            CloseHandle(handle);
        }
        return Err("Another lifecycle operation is running".into());
    }
    Ok(InstallationLock(handle))
}
#[link(name = "user32")]
extern "system" {
    fn LoadKeyboardLayoutW(name: *const u16, flags: u32) -> *mut std::ffi::c_void;
}

struct InputModule(HMODULE);
impl Drop for InputModule {
    fn drop(&mut self) {
        unsafe {
            FreeLibrary(self.0);
        }
    }
}
fn input_module() -> Result<InputModule, String> {
    let path = system_directory()?.join("input.dll");
    let path = wide(path.to_str().ok_or("Invalid system path")?);
    let module =
        unsafe { LoadLibraryExW(path.as_ptr(), ptr::null_mut(), LOAD_LIBRARY_SEARCH_SYSTEM32) };
    if module.is_null() {
        return Err(err(io::Error::last_os_error()));
    }
    Ok(InputModule(module))
}

#[repr(C)]
struct InputProfile {
    kind: u32,
    language: u16,
    clsid: windows_sys::core::GUID,
    profile: windows_sys::core::GUID,
    category: windows_sys::core::GUID,
    substitute: u32,
    flags: u32,
    id: [u16; 260],
}
pub fn enabled_profiles() -> Result<Vec<String>, String> {
    let module = input_module()?;
    unsafe {
        let function = GetProcAddress(module.0, c"EnumEnabledLayoutOrTip".as_ptr().cast())
            .ok_or("Missing enumeration API")?;
        let function: unsafe extern "system" fn(
            *const u16,
            *const u16,
            *const u16,
            *mut InputProfile,
            u32,
        ) -> u32 = std::mem::transmute(function);
        let count = function(ptr::null(), ptr::null(), ptr::null(), ptr::null_mut(), 0);
        if count > 1024 {
            return Err("Unexpected input profile count".into());
        }
        let mut profiles: Vec<InputProfile> = (0..count).map(|_| std::mem::zeroed()).collect();
        let copied = function(
            ptr::null(),
            ptr::null(),
            ptr::null(),
            profiles.as_mut_ptr(),
            count,
        );
        if copied > count {
            return Err("Input profile list changed; retry".into());
        }
        Ok(profiles[..copied as usize]
            .iter()
            .filter(|p| p.kind == 2)
            .map(|p| {
                let n = p.id.iter().position(|x| *x == 0).unwrap_or(p.id.len());
                String::from_utf16_lossy(&p.id[..n])
            })
            .collect())
    }
}
fn is_enrolled(r: &Receipt) -> Result<bool, String> {
    Ok(enabled_profiles()?.iter().any(|id| {
        id.rsplit(':')
            .next()
            .unwrap_or(id)
            .trim_start_matches("0x")
            .eq_ignore_ascii_case(&r.klid)
    }))
}

pub fn select_owned() -> Result<(), String> {
    let r = read_receipt()?.ok_or("No owned layout to select")?;
    if r.owner != identity()?.0 || !is_enrolled(&r)? {
        return Err("Owned layout is not enrolled for this user".into());
    }
    let name = wide(&r.klid);
    if unsafe { LoadKeyboardLayoutW(name.as_ptr(), 1) }.is_null() {
        return Err("Native layout selection failed".into());
    }
    let status = crate::inspect()?;
    if status["calling_thread_klid"] != r.klid
        || status["conflicts"][0]["output"] != "«"
        || status["conflicts"][1]["output"] != "»"
    {
        return Err(
            "Windows returned a fallback or unexpected translation; selection unverified".into(),
        );
    }
    Ok(())
}

fn err(e: impl std::fmt::Display) -> String {
    e.to_string()
}
fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(Some(0)).collect()
}
fn system_directory() -> Result<PathBuf, String> {
    let mut buf = [0u16; 32768];
    let n = unsafe { GetSystemDirectoryW(buf.as_mut_ptr(), buf.len() as u32) } as usize;
    if n == 0 || n >= buf.len() {
        return Err("Cannot resolve trusted System32".into());
    }
    Ok(PathBuf::from(String::from_utf16_lossy(&buf[..n])))
}
fn owned_bytes(path: &Path) -> Result<Vec<u8>, String> {
    let metadata = fs::symlink_metadata(path).map_err(err)?;
    if metadata.file_attributes() & 0x400 != 0 || !metadata.is_file() {
        return Err("Refusing a reparse point or non-file native payload".into());
    }
    fs::read(path).map_err(err)
}
fn reject_other_loaded_users(r: &Receipt) -> Result<(), String> {
    let users = RegKey::predef(HKEY_USERS);
    for sid in users.enum_keys() {
        let sid = sid.map_err(err)?;
        if sid == r.owner || sid.ends_with("_Classes") {
            continue;
        }
        for path in [r"Keyboard Layout\Preload", r"Keyboard Layout\Substitutes"] {
            match users.open_subkey(format!(r"{sid}\{path}")) {
                Ok(key) => {
                    for value in key.enum_values() {
                        let (_, raw) = value.map_err(err)?;
                        if raw.vtype == REG_SZ {
                            let units: Vec<u16> = raw
                                .bytes
                                .as_chunks::<2>()
                                .0
                                .iter()
                                .map(|b| u16::from_le_bytes([b[0], b[1]]))
                                .collect();
                            let value = String::from_utf16_lossy(&units)
                                .trim_end_matches('\0')
                                .to_uppercase();
                            if value == r.klid {
                                return Err("Another loaded user references this layout; retaining machine resources".into());
                            }
                        }
                    }
                }
                Err(e) if e.kind() == io::ErrorKind::NotFound => (),
                Err(e) => return Err(err(e)),
            }
        }
    }
    Ok(())
}

pub fn identity() -> Result<(String, bool), String> {
    unsafe {
        let mut token = ptr::null_mut();
        if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token) == 0 {
            return Err(err(io::Error::last_os_error()));
        }
        let result = (|| {
            let mut size = 0;
            GetTokenInformation(token, TokenUser, ptr::null_mut(), 0, &mut size);
            if size == 0 {
                return Err("Cannot inspect owner SID".into());
            }
            let mut storage = vec![0usize; (size as usize).div_ceil(std::mem::size_of::<usize>())];
            if GetTokenInformation(
                token,
                TokenUser,
                storage.as_mut_ptr().cast(),
                size,
                &mut size,
            ) == 0
            {
                return Err(err(io::Error::last_os_error()));
            }
            let user = &*(storage.as_ptr() as *const TOKEN_USER);
            let mut sid = ptr::null_mut();
            if ConvertSidToStringSidW(user.User.Sid, &mut sid) == 0 {
                return Err(err(io::Error::last_os_error()));
            }
            let mut len = 0;
            while *sid.add(len) != 0 {
                len += 1;
            }
            let owner = String::from_utf16_lossy(std::slice::from_raw_parts(sid, len));
            LocalFree(sid.cast());
            let mut elevation: TOKEN_ELEVATION = std::mem::zeroed();
            if GetTokenInformation(
                token,
                TokenElevation,
                (&mut elevation as *mut TOKEN_ELEVATION).cast(),
                std::mem::size_of::<TOKEN_ELEVATION>() as u32,
                &mut size,
            ) == 0
            {
                return Err(err(io::Error::last_os_error()));
            }
            Ok((owner, elevation.TokenIsElevated != 0))
        })();
        CloseHandle(token);
        result
    }
}

fn receipt_key(write: bool) -> Result<RegKey, String> {
    RegKey::predef(HKEY_LOCAL_MACHINE)
        .open_subkey_with_flags(
            ROOT,
            (if write {
                KEY_READ | KEY_WRITE
            } else {
                KEY_READ
            }) | KEY_WOW64_64KEY,
        )
        .map_err(err)
}
fn read_receipt() -> Result<Option<Receipt>, String> {
    let key = match receipt_key(false) {
        Ok(key) => key,
        Err(_) => {
            // Distinguish missing from inaccessible instead of claiming absence.
            match RegKey::predef(HKEY_LOCAL_MACHINE)
                .open_subkey_with_flags(ROOT, KEY_READ | KEY_WOW64_64KEY)
            {
                Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(None),
                Err(e) => return Err(err(e)),
                Ok(_) => return Err("Receipt access changed during inspection".into()),
            }
        }
    };
    let data: String = key.get_value("Receipt").map_err(err)?;
    if data.len() > 65536 {
        return Err("Oversized receipt".into());
    }
    let receipt: Receipt = serde_json::from_str(&data).map_err(err)?;
    receipt.validate().map_err(err)?;
    Ok(Some(receipt))
}
fn save(receipt: &Receipt) -> Result<(), String> {
    receipt.validate().map_err(err)?;
    let key = receipt_key(true)?;
    key.set_value("Receipt", &serde_json::to_string(receipt).map_err(err)?)
        .map_err(err)?;
    let code = unsafe { RegFlushKey(key.raw_handle().cast()) };
    if code != 0 {
        return Err(format!("Receipt flush failed: {code}"));
    }
    Ok(())
}
fn begin(r: &mut Receipt, step: &str) -> Result<(), String> {
    r.pending = Some(step.into());
    save(r)
}
fn finish(r: &mut Receipt, step: &str) -> Result<(), String> {
    r.pending = None;
    if !r.completed.iter().any(|s| s == step) {
        r.completed.push(step.into());
    }
    save(r)
}

fn payload_check() -> Result<(), String> {
    if std::env::consts::ARCH != "x86_64" {
        return Err("Only native x64 is supported".into());
    }
    if PAYLOAD.len() < 256 || &PAYLOAD[..2] != b"MZ" {
        return Err("Build the native layout before this binary".into());
    }
    let pe = u32::from_le_bytes(PAYLOAD[60..64].try_into().unwrap()) as usize;
    if pe + 160 > PAYLOAD.len()
        || &PAYLOAD[pe..pe + 4] != b"PE\0\0"
        || PAYLOAD[pe + 4..pe + 6] != [0x64, 0x86]
    {
        return Err("Invalid embedded native payload".into());
    }
    let opt = pe + 24;
    if PAYLOAD[opt + 16..opt + 20] != [0; 4] || PAYLOAD[opt + 120..opt + 128] != [0; 8] {
        return Err("Payload must have no entrypoint/imports".into());
    }
    Ok(())
}

fn layouts(write: bool) -> Result<RegKey, String> {
    RegKey::predef(HKEY_LOCAL_MACHINE)
        .open_subkey_with_flags(
            LAYOUTS,
            (if write {
                KEY_READ | KEY_WRITE
            } else {
                KEY_READ
            }) | KEY_WOW64_64KEY,
        )
        .map_err(err)
}
fn allocate() -> Result<(String, String), String> {
    let root = layouts(false)?;
    let mut names = HashSet::new();
    let mut ids = HashSet::new();
    for name in root.enum_keys() {
        let name = name.map_err(err)?;
        let key = root.open_subkey(&name).map_err(err)?;
        match key.get_value::<String, _>("Layout Id") {
            Ok(id) => {
                ids.insert(id.to_uppercase());
            }
            Err(e) if e.kind() == io::ErrorKind::NotFound => (),
            Err(e) => return Err(err(e)),
        }
        names.insert(name.to_uppercase());
    }
    esq_core::allocate_windows_ids(&names, &ids).map_err(err)
}

fn enroll(r: &Receipt, remove: bool) -> Result<(), String> {
    let module = input_module()?;
    unsafe {
        let function = GetProcAddress(module.0, c"InstallLayoutOrTip".as_ptr().cast())
            .ok_or("Input.dll has no InstallLayoutOrTip")?;
        let function: unsafe extern "system" fn(*const u16, u32) -> i32 =
            std::mem::transmute(function);
        let profile = wide(&format!("0x{}:0x{}", r.language_id, r.klid));
        if function(profile.as_ptr(), u32::from(remove)) == 0 {
            return Err("InstallLayoutOrTip failed; inspect enrollment before retrying".into());
        }
    }
    if is_enrolled(r)? == remove {
        return Err(
            "Enrollment read-back did not match requested state; recovery receipt retained".into(),
        );
    }
    Ok(())
}

fn user_snapshot() -> Result<Value, String> {
    let root = RegKey::predef(HKEY_CURRENT_USER);
    let mut result = serde_json::Map::new();
    for path in [
        r"Keyboard Layout\Preload",
        r"Keyboard Layout\Substitutes",
        r"Control Panel\International\User Profile",
    ] {
        let key = match root.open_subkey(path) {
            Ok(k) => k,
            Err(e) if e.kind() == io::ErrorKind::NotFound => {
                result.insert(path.into(), Value::Null);
                continue;
            }
            Err(e) => return Err(err(e)),
        };
        let mut values = serde_json::Map::new();
        for value in key.enum_values() {
            let (name, value) = value.map_err(err)?;
            values.insert(
                name,
                json!({"kind":format!("{:?}",value.vtype),"bytes":value.bytes}),
            );
        }
        result.insert(path.into(), Value::Object(values));
    }
    Ok(Value::Object(result))
}

pub fn installation_status() -> Result<Value, String> {
    let Some(r) = read_receipt()? else {
        return Ok(json!({"state":"absent"}));
    };
    let path = system_directory()?.join(&r.filename);
    let artifact = match fs::read(&path) {
        Ok(data) => {
            if digest(&data) == r.sha256 {
                "verified"
            } else {
                "modified"
            }
        }
        Err(e) if e.kind() == io::ErrorKind::NotFound => "absent",
        Err(_) => "unknown",
    };
    let enrolled = is_enrolled(&r)?;
    let registry = match layouts(false)?.open_subkey(&r.klid) {
        Ok(key) => match (
            key.get_value::<String, _>("Layout File"),
            key.get_value::<String, _>("Layout Id"),
        ) {
            (Ok(f), Ok(i)) if f == r.filename && i == r.layout_id => "verified",
            _ => "modified",
        },
        Err(e) if e.kind() == io::ErrorKind::NotFound => "absent",
        Err(_) => "unknown",
    };
    Ok(
        json!({"state":r.state,"pending":r.pending,"klid":r.klid,"artifact":artifact,
        "registry":registry,"enrolled":enrolled,"experimental":true,"activation":"unknown"}),
    )
}

/// Must be explicitly requested in a disposable single-user VM. No auto-elevation.
pub fn lifecycle(command: &str, expected_owner: &str) -> Result<Value, String> {
    let result = lifecycle_inner(command, expected_owner);
    if result.is_err() {
        if let Ok(Some(mut r)) = read_receipt() {
            if r.owner == expected_owner
                && identity().is_ok_and(|(sid, elevated)| sid == expected_owner && elevated)
            {
                r.state = State::RepairRequired;
                let _ = save(&r); // Preserve pending intent even when recovery write fails.
            }
        }
    }
    result
}
fn lifecycle_inner(command: &str, expected_owner: &str) -> Result<Value, String> {
    let (owner, elevated) = identity()?;
    if owner != expected_owner {
        return Err("UAC changed the owner account; this prototype cannot continue".into());
    }
    if !elevated {
        return Err("Requires an elevated terminal in a disposable single-user VM".into());
    }
    let _lock = lock_installation()?;
    payload_check()?;
    if command == "install" && read_receipt()?.is_none() {
        let inspection = crate::inspect()?;
        if inspection["calling_thread_klid"] != "0000040A" {
            return Err("Select Spanish Spain before the experiment".into());
        }
        let (klid, layout_id) = allocate()?;
        let profiles = enabled_profiles()?;
        let original: Vec<_> = profiles
            .iter()
            .filter(|id| {
                id.rsplit(':')
                    .next()
                    .is_some_and(|x| x.trim_start_matches("0x").eq_ignore_ascii_case("0000040A"))
            })
            .collect();
        if original.len() != 1 {
            return Err("Original Spanish enrollment is missing or ambiguous; refusing to guess language ID".into());
        }
        let language_id = original[0]
            .split(':')
            .next()
            .unwrap()
            .trim_start_matches("0x")
            .to_uppercase();
        if !matches!(language_id.as_str(), "040A" | "0C0A") {
            return Err("Unsupported Spanish enrollment language".into());
        }
        let sha256 = digest(PAYLOAD);
        let filename = format!("esq-{}.dll", &sha256[..16]);
        if system_directory()?.join(&filename).exists() {
            return Err("Payload path already exists without ownership receipt".into());
        }
        let mut r = Receipt {
            schema: 1,
            profile: PROFILE.into(),
            owner: owner.clone(),
            original_klid: "0000040A".into(),
            klid,
            layout_id,
            language_id,
            filename,
            sha256,
            state: State::Absent,
            pending: None,
            completed: vec![],
            baseline_user_state: user_snapshot()?,
        };
        let (_, disposition) = RegKey::predef(HKEY_LOCAL_MACHINE)
            .create_subkey_with_flags(ROOT, KEY_READ | KEY_WRITE | KEY_WOW64_64KEY)
            .map_err(err)?;
        if disposition != REG_CREATED_NEW_KEY {
            return Err("Unknown existing product registry key; refusing overwrite".into());
        }
        save(&r)?;
        begin(&mut r, "copy")?;
        let path = system_directory()?.join(&r.filename);
        let mut file = fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&path)
            .map_err(err)?;
        file.write_all(PAYLOAD).map_err(err)?;
        file.sync_all().map_err(err)?;
        drop(file);
        if digest(&owned_bytes(&path)?) != r.sha256 {
            return Err("Payload read-back mismatch".into());
        }
        finish(&mut r, "copy")?;
        begin(&mut r, "register")?;
        let (key, disposition) = layouts(true)?.create_subkey(&r.klid).map_err(err)?;
        if disposition != REG_CREATED_NEW_KEY {
            return Err("Concurrent layout registration collision; receipt retained".into());
        }
        key.set_value("Layout File", &r.filename).map_err(err)?;
        key.set_value("Layout Text", &"Español — comillas « » (experimental)")
            .map_err(err)?;
        key.set_value("Layout Id", &r.layout_id).map_err(err)?;
        let code = unsafe { RegFlushKey(key.raw_handle().cast()) };
        if code != 0 {
            return Err(format!("Layout registry flush failed: {code}"));
        }
        r.state = State::InstalledDisabled;
        finish(&mut r, "register")?;
    }
    let mut r = read_receipt()?.ok_or("No owned installation")?;
    if r.owner != owner || r.sha256 != digest(PAYLOAD) {
        return Err("Receipt belongs to another owner/build; refusing mutation".into());
    }
    if command == "install" || command == "enable" {
        if r.pending.is_some() {
            return Err("Interrupted transaction: use repair/uninstall in the test VM".into());
        }
        begin(&mut r, "enroll")?;
        enroll(&r, false)?;
        r.state = State::Enabled;
        finish(&mut r, "enroll")?;
    } else if command == "disable" || command == "uninstall" || command == "repair" {
        let partial_copy =
            r.pending.as_deref() == Some("copy") && !r.completed.iter().any(|x| x == "copy");
        begin(&mut r, "unenroll")?;
        if is_enrolled(&r)? {
            enroll(&r, true)?;
        }
        r.state = State::InstalledDisabled;
        finish(&mut r, "unenroll")?;
        if command != "disable" {
            reject_other_loaded_users(&r)?;
            let root = layouts(true)?;
            if let Ok(key) = root.open_subkey(&r.klid) {
                let filename: String = key.get_value("Layout File").map_err(err)?;
                let id: String = key.get_value("Layout Id").map_err(err)?;
                if filename != r.filename || id != r.layout_id {
                    return Err("Registry ownership changed; refusing deletion".into());
                }
                begin(&mut r, "unregister")?;
                root.delete_subkey(&r.klid).map_err(err)?;
                finish(&mut r, "unregister")?;
            }
            let path = system_directory()?.join(&r.filename);
            if path.exists() {
                let data = owned_bytes(&path)?;
                if digest(&data) != r.sha256
                    && !(partial_copy && data.len() < PAYLOAD.len() && PAYLOAD.starts_with(&data))
                {
                    return Err("Artifact modified; refusing deletion".into());
                }
                begin(&mut r, "remove-file")?;
                if let Err(e) = fs::remove_file(&path) {
                    if matches!(e.raw_os_error(), Some(5 | 32)) {
                        let name = wide(path.to_str().ok_or("Invalid native path")?);
                        if unsafe {
                            MoveFileExW(name.as_ptr(), ptr::null(), MOVEFILE_DELAY_UNTIL_REBOOT)
                        } == 0
                        {
                            return Err(err(io::Error::last_os_error()));
                        }
                        r.state = State::PendingReboot;
                        save(&r)?;
                        return installation_status();
                    }
                    return Err(err(e));
                }
                finish(&mut r, "remove-file")?;
            }
            if path.exists() {
                return Err("Removal not verified".into());
            }
            if user_snapshot()? != r.baseline_user_state {
                return Err("Native resources removed, but user settings differ from the baseline. Receipt retained for W3 review; independent changes are not overwritten.".into());
            }
            RegKey::predef(HKEY_LOCAL_MACHINE)
                .open_subkey_with_flags("SOFTWARE", KEY_WRITE | KEY_WOW64_64KEY)
                .map_err(err)?
                .delete_subkey("easy-spanish-quotes")
                .map_err(err)?;
        }
    } else {
        return Err("Unknown lifecycle operation".into());
    }
    installation_status()
}
