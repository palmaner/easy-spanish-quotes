//! Read-only reconciliation: a journal records intent, not proof of native state.
use crate::State;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Observation {
    Absent,
    Verified,
    Modified,
    Unknown,
}

#[derive(Debug, Serialize)]
pub struct Diagnosis {
    pub state: State,
    pub findings: Vec<&'static str>,
}

pub fn diagnose(
    journal: &State,
    pending: Option<&str>,
    artifact: Observation,
    registry: Observation,
    enrolled: Option<bool>,
) -> Diagnosis {
    use Observation::*;
    let mut findings = vec![];
    for (observation, modified, unknown) in [
        (artifact, "artifact-modified", "artifact-unreadable"),
        (registry, "registration-modified", "registration-unreadable"),
    ] {
        match observation {
            Modified => findings.push(modified),
            Unknown => findings.push(unknown),
            _ => (),
        }
    }
    if enrolled.is_none() {
        findings.push("owner-enrollment-unverified");
    }
    let consistent = match journal {
        State::Enabled => artifact == Verified && registry == Verified && enrolled == Some(true),
        State::InstalledDisabled => {
            artifact == Verified && registry == Verified && enrolled == Some(false)
        }
        State::PendingReboot => {
            // Missing DLL means cleanup still needs to close the durable receipt.
            pending == Some("remove-file")
                && artifact == Verified
                && registry == Absent
                && enrolled == Some(false)
        }
        State::Absent | State::RepairRequired => false, // A receipt still exists.
    };
    if !consistent {
        findings.push("journal-observation-mismatch");
    }
    if pending.is_some() && *journal != State::PendingReboot {
        findings.push("interrupted-operation");
    }
    if *journal == State::PendingReboot && artifact == Absent {
        findings.push("receipt-cleanup-required");
    }
    Diagnosis {
        state: if findings.is_empty() {
            journal.clone()
        } else {
            State::RepairRequired
        },
        findings,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Observation::*;

    #[test]
    fn enrollment_is_not_activation() {
        let d = diagnose(&State::Enabled, None, Verified, Verified, Some(true));
        assert_eq!(d.state, State::Enabled);
        assert!(d.findings.is_empty());
        assert_eq!(
            diagnose(
                &State::InstalledDisabled,
                None,
                Verified,
                Verified,
                Some(false)
            )
            .state,
            State::InstalledDisabled
        );
    }
    #[test]
    fn stale_enabled_receipt_cannot_hide_missing_or_foreign_resources() {
        for resource in [Absent, Modified, Unknown] {
            for (artifact, registry) in [(resource, Verified), (Verified, resource)] {
                assert_eq!(
                    diagnose(&State::Enabled, None, artifact, registry, Some(true)).state,
                    State::RepairRequired
                );
            }
        }
    }
    #[test]
    fn enrollment_for_another_user_is_not_owner_evidence() {
        for enrollment in [None, Some(false)] {
            assert_eq!(
                diagnose(&State::Enabled, None, Verified, Verified, enrollment).state,
                State::RepairRequired
            );
        }
    }
    #[test]
    fn interrupted_operation_requires_review_even_after_matching_readback() {
        let d = diagnose(
            &State::Enabled,
            Some("enroll"),
            Verified,
            Verified,
            Some(true),
        );
        assert_eq!(d.state, State::RepairRequired);
        assert!(d.findings.contains(&"interrupted-operation"));
    }
    #[test]
    fn reboot_wait_requires_verified_remaining_payload() {
        assert_eq!(
            diagnose(
                &State::PendingReboot,
                Some("remove-file"),
                Verified,
                Absent,
                Some(false)
            )
            .state,
            State::PendingReboot
        );
        let d = diagnose(
            &State::PendingReboot,
            Some("remove-file"),
            Absent,
            Absent,
            Some(false),
        );
        assert_eq!(d.state, State::RepairRequired);
        assert!(d.findings.contains(&"receipt-cleanup-required"));
        assert_eq!(
            diagnose(
                &State::PendingReboot,
                Some("remove-file"),
                Modified,
                Absent,
                Some(false)
            )
            .state,
            State::RepairRequired
        );
    }
    #[test]
    fn empty_resources_do_not_prove_receipt_cleanup() {
        assert_eq!(
            diagnose(&State::Absent, None, Absent, Absent, Some(false)).state,
            State::RepairRequired
        );
    }
}
