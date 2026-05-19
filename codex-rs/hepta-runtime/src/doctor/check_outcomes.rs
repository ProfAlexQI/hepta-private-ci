use hepta_core::DoctorCheckOutcome;

use super::{
    DoctorCheck, DoctorProviderProbe, DoctorStatus, integrity_catalog, provider_probe,
    status_rollup,
};

pub(super) struct DoctorOutcomeBundle {
    pub overall_status: DoctorStatus,
    pub v2_checks: Vec<DoctorCheckOutcome>,
}

pub(super) fn assemble_doctor_outcomes(
    provider_probes: &[DoctorProviderProbe],
    integrity_checks: &[DoctorCheck],
) -> DoctorOutcomeBundle {
    DoctorOutcomeBundle {
        overall_status: status_rollup::roll_up_statuses(provider_probes, integrity_checks)
            .legacy_overall_status(),
        v2_checks: adapt_v2_checks(provider_probes, integrity_checks),
    }
}

fn adapt_v2_checks(
    provider_probes: &[DoctorProviderProbe],
    integrity_checks: &[DoctorCheck],
) -> Vec<DoctorCheckOutcome> {
    provider_probes
        .iter()
        .cloned()
        .map(provider_probe::adapt_provider_probe)
        .chain(
            integrity_checks
                .iter()
                .cloned()
                .map(integrity_catalog::adapt_integrity_check),
        )
        .collect()
}

#[cfg(test)]
mod tests {
    use hepta_core::{DoctorArea, DoctorStatus as CoreDoctorStatus, ModelRef};

    use super::*;

    #[test]
    fn assembles_overall_status_and_v2_checks_from_mixed_doctor_inputs() {
        let outcomes = assemble_doctor_outcomes(
            &[DoctorProviderProbe {
                provider_name: "demo".into(),
                model: Some(ModelRef {
                    provider: "demo".into(),
                    model: "demo-chat".into(),
                }),
                status: DoctorStatus::Fail,
                detail: "connection refused".into(),
            }],
            &[DoctorCheck {
                name: "runtime snapshot roundtrip".into(),
                status: DoctorStatus::Warn,
                detail: "serde drift detected".into(),
            }],
        );

        assert_eq!(outcomes.overall_status, DoctorStatus::Fail);
        assert_eq!(outcomes.v2_checks.len(), 2);
        assert_eq!(outcomes.v2_checks[0].id, "provider_probe.demo.demo-chat");
        assert_eq!(outcomes.v2_checks[0].area, DoctorArea::ProviderProbe);
        assert_eq!(outcomes.v2_checks[0].status, CoreDoctorStatus::Fail);
        assert_eq!(outcomes.v2_checks[1].id, "runtime_snapshot.roundtrip");
        assert_eq!(outcomes.v2_checks[1].area, DoctorArea::RuntimeSnapshot);
        assert_eq!(outcomes.v2_checks[1].status, CoreDoctorStatus::Warn);
    }

    #[test]
    fn keeps_ok_overall_status_when_every_doctor_outcome_is_healthy() {
        let outcomes = assemble_doctor_outcomes(
            &[DoctorProviderProbe {
                provider_name: "demo".into(),
                model: None,
                status: DoctorStatus::Ok,
                detail: "pong".into(),
            }],
            &[DoctorCheck {
                name: "active session export roundtrip".into(),
                status: DoctorStatus::Ok,
                detail: "session-main export serializable".into(),
            }],
        );

        assert_eq!(outcomes.overall_status, DoctorStatus::Ok);
        assert_eq!(outcomes.v2_checks.len(), 2);
        assert!(
            outcomes
                .v2_checks
                .iter()
                .all(|check| check.status == CoreDoctorStatus::Ok)
        );
    }
}
