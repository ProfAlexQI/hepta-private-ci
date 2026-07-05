use hepta_core::DoctorArea;
use hepta_core::DoctorCheckOutcome;
use hepta_core::DoctorOwner;
use hepta_core::MessageRole;
use hepta_core::ModelMessage;
use hepta_core::ModelRef;
use hepta_core::ModelRequest;
use hepta_core::ModelResponse;
use hepta_core::ProviderDescriptor;
use hepta_core::ThinkingLevel;

use super::DoctorProviderProbe;
use super::DoctorStatus;

const DOCTOR_PING_CONTENT: &str = "doctor:ping";
const NO_ASSISTANT_MESSAGE: &str = "no assistant message";
const NO_MODEL_REGISTERED: &str = "no model registered";
const PROVIDER_PROBE_OWNER: &str = "provider probing";
const PREVIEW_LIMIT: usize = 72;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ProviderProbePlan {
    pub provider_name: String,
    pub model: Option<ModelRef>,
}

pub(super) fn plan_provider_probes(descriptors: &[ProviderDescriptor]) -> Vec<ProviderProbePlan> {
    descriptors
        .iter()
        .map(|descriptor| ProviderProbePlan {
            provider_name: descriptor.id.clone(),
            model: descriptor.available_models.first().cloned(),
        })
        .collect()
}

pub(super) fn should_skip_live_provider_probe(provider_name: &str) -> bool {
    matches!(provider_name, "openai-codex" | "codex")
        && std::env::var("HEPTA_DOCTOR_LIVE_CODEX_PROBE")
            .map(|value| value != "1")
            .unwrap_or(true)
}

pub(super) fn probe_request(model: &ModelRef) -> ModelRequest {
    ModelRequest {
        model: model.clone(),
        messages: vec![ModelMessage {
            role: MessageRole::User,
            content: DOCTOR_PING_CONTENT.into(),
        }],
        thinking: ThinkingLevel::Low,
        tools: vec![],
        timeout_ms: Some(15_000),
    }
}

pub(super) fn probe_success(
    plan: ProviderProbePlan,
    response: ModelResponse,
) -> DoctorProviderProbe {
    DoctorProviderProbe {
        provider_name: plan.provider_name,
        model: plan.model,
        status: DoctorStatus::Ok,
        detail: response_preview(response),
    }
}

pub(super) fn probe_failure(plan: ProviderProbePlan, detail: String) -> DoctorProviderProbe {
    let status = if is_external_or_optional_runtime_unavailable(&plan.provider_name, &detail) {
        DoctorStatus::Warn
    } else {
        DoctorStatus::Fail
    };

    DoctorProviderProbe {
        provider_name: plan.provider_name,
        model: plan.model,
        status,
        detail,
    }
}

fn is_external_or_optional_runtime_unavailable(provider_name: &str, detail: &str) -> bool {
    detail.contains("is imported but has no Hepta-native HTTP runtime config")
        || detail.contains("Hepta native provider currently allows plain HTTP only for local providers; unsupported scheme: https")
        || detail.contains("live provider probe skipped")
        || (provider_name == "mlx-local" && detail.contains("provider read timeout"))
        || (provider_name == "ollama"
            && detail.contains("provider returned non-200 status: HTTP/1.1 404 Not Found"))
}

pub(super) fn probe_unconfigured(plan: ProviderProbePlan) -> DoctorProviderProbe {
    DoctorProviderProbe {
        provider_name: plan.provider_name,
        model: None,
        status: DoctorStatus::Fail,
        detail: NO_MODEL_REGISTERED.into(),
    }
}

pub(super) fn adapt_provider_probe(probe: DoctorProviderProbe) -> DoctorCheckOutcome {
    let remediation = remediation(&probe);

    DoctorCheckOutcome {
        id: stable_id(&probe),
        area: DoctorArea::ProviderProbe,
        owner: owner(PROVIDER_PROBE_OWNER),
        status: probe.status.into(),
        summary: summary(&probe),
        detail: probe.detail,
        remediation,
    }
}

fn response_preview(response: ModelResponse) -> String {
    response
        .message
        .map(|message| message.content)
        .unwrap_or_else(|| NO_ASSISTANT_MESSAGE.into())
        .chars()
        .take(PREVIEW_LIMIT)
        .collect()
}

fn stable_id(probe: &DoctorProviderProbe) -> String {
    match probe.model.as_ref() {
        Some(model) => format!(
            "provider_probe.{}.{}",
            sanitize_segment(&probe.provider_name),
            sanitize_segment(&model.model)
        ),
        None => format!("provider_probe.{}", sanitize_segment(&probe.provider_name)),
    }
}

fn summary(probe: &DoctorProviderProbe) -> String {
    match probe.model.as_ref() {
        Some(model) => format!(
            "provider probe {} via {}/{}",
            probe.provider_name, model.provider, model.model
        ),
        None => format!("provider probe {}", probe.provider_name),
    }
}

fn remediation(probe: &DoctorProviderProbe) -> Option<String> {
    match (probe.status, probe.model.as_ref()) {
        (DoctorStatus::Ok, _) => None,
        (_, Some(model)) => Some(format!(
            "check connectivity and credentials for {}/{} on provider {}",
            model.provider, model.model, probe.provider_name
        )),
        (_, None) => Some(format!(
            "register at least one model for provider {}",
            probe.provider_name
        )),
    }
}

fn owner(responsibility: &str) -> DoctorOwner {
    DoctorOwner {
        component: "hepta-runtime".into(),
        responsibility: responsibility.into(),
    }
}

fn sanitize_segment(value: &str) -> String {
    let mut segment = value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '-' {
                ch.to_ascii_lowercase()
            } else {
                '_'
            }
        })
        .collect::<String>();
    while segment.contains("__") {
        segment = segment.replace("__", "_");
    }
    segment.trim_matches('_').to_string()
}

#[cfg(test)]
mod tests {
    use hepta_core::DoctorArea;
    use hepta_core::DoctorStatus as CoreDoctorStatus;
    use hepta_core::FinishReason;
    use hepta_core::ModelRef;
    use hepta_core::ProviderTransportKind;
    use hepta_core::Usage;

    use super::*;

    #[test]
    fn plans_provider_probes_from_registered_catalog_entries() {
        let plans = plan_provider_probes(&[
            ProviderDescriptor {
                id: "demo".into(),
                display_name: "Demo".into(),
                transport_kind: ProviderTransportKind::InProcess,
                default_model: ModelRef {
                    provider: "demo".into(),
                    model: "demo-chat".into(),
                },
                available_models: vec![ModelRef {
                    provider: "demo".into(),
                    model: "demo-chat".into(),
                }],
                requires_auth: false,
                supports_tool_calls: true,
            },
            ProviderDescriptor {
                id: "empty".into(),
                display_name: "Empty".into(),
                transport_kind: ProviderTransportKind::OpenAiCompatibleHttp,
                default_model: ModelRef {
                    provider: "empty".into(),
                    model: "unused".into(),
                },
                available_models: vec![],
                requires_auth: true,
                supports_tool_calls: false,
            },
        ]);

        assert_eq!(plans.len(), 2);
        assert_eq!(plans[0].provider_name, "demo");
        assert_eq!(
            plans[0].model,
            Some(ModelRef {
                provider: "demo".into(),
                model: "demo-chat".into(),
            })
        );
        assert_eq!(plans[1].provider_name, "empty");
        assert_eq!(plans[1].model, None);
    }

    #[test]
    fn successful_probe_truncates_preview_to_stable_limit() {
        let plan = ProviderProbePlan {
            provider_name: "demo".into(),
            model: Some(ModelRef {
                provider: "demo".into(),
                model: "demo-chat".into(),
            }),
        };

        let probe = probe_success(
            plan,
            ModelResponse {
                message: Some(ModelMessage {
                    role: MessageRole::Assistant,
                    content: "x".repeat(PREVIEW_LIMIT + 10),
                }),
                tool_calls: vec![],
                finish_reason: FinishReason::Stop,
                usage: Usage::default(),
            },
        );

        assert_eq!(probe.status, DoctorStatus::Ok);
        assert_eq!(probe.detail.len(), PREVIEW_LIMIT);
    }

    #[test]
    fn adapted_provider_probe_keeps_stable_ids_and_remediation() {
        let outcome = adapt_provider_probe(DoctorProviderProbe {
            provider_name: "mock ollama".into(),
            model: Some(ModelRef {
                provider: "mock ollama".into(),
                model: "local/chat".into(),
            }),
            status: DoctorStatus::Fail,
            detail: "connection refused".into(),
        });

        assert_eq!(outcome.id, "provider_probe.mock_ollama.local_chat");
        assert_eq!(outcome.area, DoctorArea::ProviderProbe);
        assert_eq!(outcome.status, CoreDoctorStatus::Fail);
        assert!(
            outcome
                .remediation
                .as_deref()
                .expect("failing probes should include remediation")
                .contains("mock ollama")
        );
    }

    #[test]
    fn imported_or_optional_runtime_probe_failures_are_warnings() {
        let plan = ProviderProbePlan {
            provider_name: "openai".into(),
            model: Some(ModelRef {
                provider: "openai".into(),
                model: "gpt-5.5".into(),
            }),
        };

        let probe = probe_failure(
            plan,
            "provider openai is imported but has no Hepta-native HTTP runtime config".into(),
        );

        assert_eq!(probe.status, DoctorStatus::Warn);

        let local_optional_timeout = probe_failure(
            ProviderProbePlan {
                provider_name: "mlx-local".into(),
                model: Some(ModelRef {
                    provider: "mlx-local".into(),
                    model: "local-heavy-model".into(),
                }),
            },
            "provider read timeout after 15000 ms".into(),
        );

        assert_eq!(local_optional_timeout.status, DoctorStatus::Warn);
    }
}
