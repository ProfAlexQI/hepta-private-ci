use hepta_core::DoctorCheckOutcome;

use super::DoctorCheck;
use super::DoctorProviderProbe;
use super::DoctorStatus;
use super::integrity_catalog;
use super::provider_probe;
use super::status_rollup;

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
#[path = "../../tests/unit/doctor_check_outcomes.rs"]
mod tests;
