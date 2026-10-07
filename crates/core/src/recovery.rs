//! Testable intent-before-mutation recovery protocol for owned native resources.
//! Platform receipts provide persistence; foreign state always stops recovery.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Observation {
    Absent,
    Owned,
    Foreign,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Goal {
    Present,
    Absent,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Record {
    Empty,
    Intent(Goal),
    Completed(Goal),
}

pub trait Resource {
    fn observe(&mut self) -> Result<Observation, String>;
    fn apply(&mut self, goal: Goal) -> Result<(), String>;
}
pub trait Journal {
    fn read(&self) -> Result<Record, String>;
    /// Must be durable before returning success.
    fn write(&mut self, record: Record) -> Result<(), String>;
}

pub fn converge(
    resource: &mut impl Resource,
    journal: &mut impl Journal,
    goal: Goal,
) -> Result<(), String> {
    if resource.observe()? == Observation::Foreign {
        return Err("Resource ownership changed".into());
    }
    journal.write(Record::Intent(goal))?;
    resume(resource, journal)
}
pub fn resume(resource: &mut impl Resource, journal: &mut impl Journal) -> Result<(), String> {
    let goal = match journal.read()? {
        Record::Empty => return Ok(()),
        Record::Intent(goal) | Record::Completed(goal) => goal,
    };
    let desired = if goal == Goal::Present {
        Observation::Owned
    } else {
        Observation::Absent
    };
    let observed = resource.observe()?;
    if observed == Observation::Foreign {
        return Err("Resource ownership changed".into());
    }
    if observed != desired {
        resource.apply(goal)?;
    }
    if resource.observe()? != desired {
        return Err("Native action did not converge".into());
    }
    journal.write(Record::Completed(goal))
}

#[cfg(test)]
mod tests {
    use super::*;
    struct Fake {
        observed: Observation,
        fail_after_apply: bool,
        writes: usize,
    }
    impl Resource for Fake {
        fn observe(&mut self) -> Result<Observation, String> {
            Ok(self.observed)
        }
        fn apply(&mut self, goal: Goal) -> Result<(), String> {
            self.writes += 1;
            self.observed = if goal == Goal::Present {
                Observation::Owned
            } else {
                Observation::Absent
            };
            if self.fail_after_apply {
                Err("Simulated crash after native mutation".into())
            } else {
                Ok(())
            }
        }
    }
    struct Memory {
        record: Record,
        fail_write: Option<usize>,
        writes: usize,
    }
    impl Journal for Memory {
        fn read(&self) -> Result<Record, String> {
            Ok(self.record)
        }
        fn write(&mut self, r: Record) -> Result<(), String> {
            self.writes += 1;
            if self.fail_write == Some(self.writes) {
                return Err("Disk failure".into());
            }
            self.record = r;
            Ok(())
        }
    }
    #[test]
    fn failure_before_intent_never_mutates() {
        let mut r = Fake {
            observed: Observation::Absent,
            fail_after_apply: false,
            writes: 0,
        };
        let mut j = Memory {
            record: Record::Empty,
            fail_write: Some(1),
            writes: 0,
        };
        assert!(converge(&mut r, &mut j, Goal::Present).is_err());
        assert_eq!(r.observed, Observation::Absent);
        assert_eq!(r.writes, 0);
    }
    #[test]
    fn crash_after_mutation_replays_without_duplicate_write() {
        for goal in [Goal::Present, Goal::Absent] {
            let before = if goal == Goal::Present {
                Observation::Absent
            } else {
                Observation::Owned
            };
            let mut r = Fake {
                observed: before,
                fail_after_apply: true,
                writes: 0,
            };
            let mut j = Memory {
                record: Record::Empty,
                fail_write: None,
                writes: 0,
            };
            assert!(converge(&mut r, &mut j, goal).is_err());
            assert_eq!(j.record, Record::Intent(goal));
            resume(&mut r, &mut j).unwrap();
            assert_eq!(r.writes, 1);
            assert_eq!(j.record, Record::Completed(goal));
        }
    }
    #[test]
    fn completion_write_failure_remains_recoverable() {
        let mut r = Fake {
            observed: Observation::Absent,
            fail_after_apply: false,
            writes: 0,
        };
        let mut j = Memory {
            record: Record::Empty,
            fail_write: Some(2),
            writes: 0,
        };
        assert!(converge(&mut r, &mut j, Goal::Present).is_err());
        assert_eq!(j.record, Record::Intent(Goal::Present));
        resume(&mut r, &mut j).unwrap();
        assert_eq!(r.writes, 1);
    }
    #[test]
    fn independent_replacement_is_never_overwritten() {
        let mut r = Fake {
            observed: Observation::Foreign,
            fail_after_apply: false,
            writes: 0,
        };
        let mut j = Memory {
            record: Record::Intent(Goal::Absent),
            fail_write: None,
            writes: 0,
        };
        assert!(resume(&mut r, &mut j).is_err());
        assert_eq!(r.writes, 0);
        assert_eq!(j.record, Record::Intent(Goal::Absent));
    }
}
