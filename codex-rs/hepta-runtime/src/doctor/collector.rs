use hepta_core::{DoctorReportV2, HeptaError};

use super::{DoctorReport, report_inputs::DoctorReportInputs};
use crate::{RuntimeKernel, current_unix_ms};

pub(super) async fn collect_doctor_report(
    runtime: &RuntimeKernel,
) -> Result<DoctorReport, HeptaError> {
    Ok(DoctorReportInputs::gather(runtime)
        .await?
        .into_report_bundle(runtime)
        .into_report())
}

pub(super) async fn collect_doctor_report_v2(
    runtime: &RuntimeKernel,
) -> Result<DoctorReportV2, HeptaError> {
    Ok(DoctorReportInputs::gather(runtime)
        .await?
        .into_report_bundle(runtime)
        .into_v2_report(current_unix_ms()?))
}
