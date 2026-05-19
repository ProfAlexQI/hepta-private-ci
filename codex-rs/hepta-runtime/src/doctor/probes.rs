use super::{DoctorProviderProbe, provider_probe};
use crate::RuntimeKernel;

impl RuntimeKernel {
    pub(super) async fn probe_providers(&self) -> Vec<DoctorProviderProbe> {
        let plans = provider_probe::plan_provider_probes(&self.provider_catalog().providers);
        let mut probes = Vec::with_capacity(plans.len());

        for plan in plans {
            if provider_probe::should_skip_live_provider_probe(&plan.provider_name) {
                probes.push(provider_probe::probe_failure(
                    plan,
                    "openai-codex live provider probe skipped; set HEPTA_DOCTOR_LIVE_CODEX_PROBE=1 to opt in"
                        .into(),
                ));
                continue;
            }
            match plan.model.clone() {
                Some(model) => match self
                    .providers
                    .chat(provider_probe::probe_request(&model))
                    .await
                {
                    Ok(response) => probes.push(provider_probe::probe_success(plan, response)),
                    Err(err) => probes.push(provider_probe::probe_failure(plan, err.0)),
                },
                None => probes.push(provider_probe::probe_unconfigured(plan)),
            }
        }

        probes
    }
}
