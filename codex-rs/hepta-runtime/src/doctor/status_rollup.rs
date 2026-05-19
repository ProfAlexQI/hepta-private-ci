use hepta_core::DoctorSummaryCounts;

use super::{DoctorCheck, DoctorProviderProbe, DoctorStatus};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct DoctorStatusRollup {
    counts: DoctorSummaryCounts,
}

impl DoctorStatusRollup {
    pub(super) fn legacy_overall_status(&self) -> DoctorStatus {
        if self.counts.fail > 0 {
            DoctorStatus::Fail
        } else if self.counts.warn > 0 {
            DoctorStatus::Warn
        } else {
            DoctorStatus::Ok
        }
    }
}

pub(super) fn roll_up_statuses(
    provider_probes: &[DoctorProviderProbe],
    integrity_checks: &[DoctorCheck],
) -> DoctorStatusRollup {
    let mut counts = DoctorSummaryCounts::default();

    for status in provider_probes
        .iter()
        .map(|probe| probe.status)
        .chain(integrity_checks.iter().map(|check| check.status))
    {
        match status {
            DoctorStatus::Ok => counts.ok += 1,
            DoctorStatus::Warn => counts.warn += 1,
            DoctorStatus::Fail => counts.fail += 1,
        }
    }

    DoctorStatusRollup { counts }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legacy_rollup_keeps_warn_when_no_failures_exist() {
        let rollup = roll_up_statuses(
            &[DoctorProviderProbe {
                provider_name: "demo".into(),
                model: None,
                status: DoctorStatus::Ok,
                detail: "pong".into(),
            }],
            &[DoctorCheck {
                name: "runtime snapshot roundtrip".into(),
                status: DoctorStatus::Warn,
                detail: "serde drift detected".into(),
            }],
        );

        assert_eq!(rollup.legacy_overall_status(), DoctorStatus::Warn);
    }

    #[test]
    fn legacy_rollup_keeps_fail_as_the_highest_severity() {
        let rollup = roll_up_statuses(
            &[DoctorProviderProbe {
                provider_name: "demo".into(),
                model: None,
                status: DoctorStatus::Warn,
                detail: "slow response".into(),
            }],
            &[DoctorCheck {
                name: "active session exists".into(),
                status: DoctorStatus::Fail,
                detail: "missing session-main".into(),
            }],
        );

        assert_eq!(rollup.legacy_overall_status(), DoctorStatus::Fail);
    }
}
