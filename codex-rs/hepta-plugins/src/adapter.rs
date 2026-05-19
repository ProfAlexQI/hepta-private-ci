use crate::GatewayPluginBindingLookupResolution;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GatewayPluginExecutableHandoff {
    pub plugin_id: String,
    pub lookup_key: String,
    pub specificity_score: usize,
    pub payload_preview: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GatewayPluginExecutionPlan {
    pub payload_present: bool,
    pub ready: bool,
    pub handoffs: Vec<GatewayPluginExecutableHandoff>,
    pub blockers: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GatewayPluginAdapterResult {
    pub plugin_id: String,
    pub accepted: bool,
    pub output: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GatewayPluginExecutionAttempt {
    pub ready: bool,
    pub selected_plugin_id: Option<String>,
    pub result: Option<GatewayPluginAdapterResult>,
    pub blockers: Vec<String>,
    pub telemetry: GatewayPluginExecutionTelemetry,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GatewayPluginExecutionPolicy {
    pub max_attempts_per_plugin: usize,
    pub timeout_ms: u64,
    pub allow_fallback: bool,
}

impl Default for GatewayPluginExecutionPolicy {
    fn default() -> Self {
        Self {
            max_attempts_per_plugin: 1,
            timeout_ms: 30_000,
            allow_fallback: true,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct GatewayPluginExecutionTelemetry {
    pub timeout_ms: u64,
    pub allow_fallback: bool,
    pub total_attempts: usize,
    pub fallback_count: usize,
    pub trace: Vec<GatewayPluginExecutionTraceStep>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GatewayPluginExecutionTraceStep {
    pub plugin_id: String,
    pub lookup_key: String,
    pub attempt: usize,
    pub outcome: String,
}

pub trait GatewayPluginAdapter {
    fn plugin_id(&self) -> &str;

    fn execute(
        &self,
        handoff: &GatewayPluginExecutableHandoff,
        normalized_text: &str,
    ) -> GatewayPluginAdapterResult;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EchoGatewayPluginAdapter {
    plugin_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FailingGatewayPluginAdapter {
    plugin_id: String,
    error: String,
}

impl EchoGatewayPluginAdapter {
    pub fn new(plugin_id: impl Into<String>) -> Self {
        Self {
            plugin_id: plugin_id.into().trim().to_ascii_lowercase(),
        }
    }
}

impl FailingGatewayPluginAdapter {
    pub fn new(plugin_id: impl Into<String>, error: impl Into<String>) -> Self {
        Self {
            plugin_id: plugin_id.into().trim().to_ascii_lowercase(),
            error: error.into().trim().to_string(),
        }
    }
}

impl GatewayPluginAdapter for EchoGatewayPluginAdapter {
    fn plugin_id(&self) -> &str {
        &self.plugin_id
    }

    fn execute(
        &self,
        handoff: &GatewayPluginExecutableHandoff,
        normalized_text: &str,
    ) -> GatewayPluginAdapterResult {
        GatewayPluginAdapterResult {
            plugin_id: self.plugin_id.clone(),
            accepted: true,
            output: Some(format!(
                "echo adapter accepted {} via {}: {}",
                handoff.plugin_id,
                handoff.lookup_key,
                normalized_text.trim()
            )),
            error: None,
        }
    }
}

impl GatewayPluginAdapter for FailingGatewayPluginAdapter {
    fn plugin_id(&self) -> &str {
        &self.plugin_id
    }

    fn execute(
        &self,
        _handoff: &GatewayPluginExecutableHandoff,
        _normalized_text: &str,
    ) -> GatewayPluginAdapterResult {
        GatewayPluginAdapterResult {
            plugin_id: self.plugin_id.clone(),
            accepted: false,
            output: None,
            error: Some(self.error.clone()),
        }
    }
}

impl GatewayPluginExecutionPlan {
    pub fn from_lookup_resolution(
        resolution: &GatewayPluginBindingLookupResolution,
        normalized_text: &str,
    ) -> Self {
        let payload = normalized_text.trim();
        let payload_present = !payload.is_empty();
        let payload_preview = preview(payload, 96);
        let handoffs = resolution
            .matches
            .iter()
            .map(|binding| GatewayPluginExecutableHandoff {
                plugin_id: binding.plugin_id.clone(),
                lookup_key: binding.lookup_key.clone(),
                specificity_score: binding.specificity_score,
                payload_preview: payload_preview.clone(),
            })
            .collect::<Vec<_>>();

        let mut blockers = Vec::new();
        if !payload_present {
            blockers.push("empty normalized payload".into());
        }
        if handoffs.is_empty() {
            blockers.push("no plugin binding match available for execution".into());
        }

        Self {
            payload_present,
            ready: blockers.is_empty(),
            handoffs,
            blockers,
        }
    }

    pub fn preferred_handoff(&self) -> Option<&GatewayPluginExecutableHandoff> {
        self.handoffs.first()
    }

    pub fn execute_first_matching_adapter(
        &self,
        normalized_text: &str,
        adapters: &[&dyn GatewayPluginAdapter],
    ) -> GatewayPluginExecutionAttempt {
        self.execute_with_policy(
            normalized_text,
            adapters,
            GatewayPluginExecutionPolicy::default(),
        )
    }

    pub fn execute_with_policy(
        &self,
        normalized_text: &str,
        adapters: &[&dyn GatewayPluginAdapter],
        policy: GatewayPluginExecutionPolicy,
    ) -> GatewayPluginExecutionAttempt {
        let mut telemetry = GatewayPluginExecutionTelemetry {
            timeout_ms: policy.timeout_ms,
            allow_fallback: policy.allow_fallback,
            ..GatewayPluginExecutionTelemetry::default()
        };

        if !self.ready {
            return GatewayPluginExecutionAttempt {
                ready: false,
                selected_plugin_id: None,
                result: None,
                blockers: self.blockers.clone(),
                telemetry,
            };
        }

        let max_attempts = policy.max_attempts_per_plugin.max(1);
        let mut last_result = None;
        let mut attempted_plugin_count = 0usize;

        for handoff in &self.handoffs {
            if let Some(adapter) = adapters
                .iter()
                .find(|adapter| adapter.plugin_id() == handoff.plugin_id)
            {
                if attempted_plugin_count > 0 {
                    telemetry.fallback_count += 1;
                }
                attempted_plugin_count += 1;

                for attempt_index in 1..=max_attempts {
                    telemetry.total_attempts += 1;
                    let result = adapter.execute(handoff, normalized_text);
                    let outcome = if result.accepted {
                        "accepted".to_string()
                    } else {
                        format!(
                            "failed{}",
                            result
                                .error
                                .as_deref()
                                .map(|error| format!(": {}", error))
                                .unwrap_or_default()
                        )
                    };
                    telemetry.trace.push(GatewayPluginExecutionTraceStep {
                        plugin_id: handoff.plugin_id.clone(),
                        lookup_key: handoff.lookup_key.clone(),
                        attempt: attempt_index,
                        outcome,
                    });

                    if result.accepted {
                        return GatewayPluginExecutionAttempt {
                            ready: true,
                            selected_plugin_id: Some(handoff.plugin_id.clone()),
                            result: Some(result),
                            blockers: Vec::new(),
                            telemetry,
                        };
                    }

                    last_result = Some(result);
                }

                if !policy.allow_fallback {
                    return GatewayPluginExecutionAttempt {
                        ready: false,
                        selected_plugin_id: Some(handoff.plugin_id.clone()),
                        result: last_result,
                        blockers: vec![format!(
                            "adapter {} failed and fallback is disabled",
                            handoff.plugin_id
                        )],
                        telemetry,
                    };
                }
            }
        }

        GatewayPluginExecutionAttempt {
            ready: false,
            selected_plugin_id: None,
            result: last_result,
            blockers: if telemetry.total_attempts == 0 {
                vec!["no registered adapter matched resolved plugin candidates".into()]
            } else {
                vec!["all registered adapters rejected the resolved plugin handoff".into()]
            },
            telemetry,
        }
    }
}

fn preview(value: &str, limit: usize) -> String {
    let mut chars = value.chars().take(limit).collect::<String>();
    if value.chars().count() > limit {
        chars.push('…');
    }
    chars
}

#[cfg(test)]
mod tests {
    use super::{
        EchoGatewayPluginAdapter, FailingGatewayPluginAdapter, GatewayPluginExecutionPlan,
        GatewayPluginExecutionPolicy,
    };
    use crate::{GatewayPluginBinding, GatewayPluginBindingCatalog};

    #[test]
    fn execution_plan_selects_preferred_handoff_and_echo_adapter() {
        let mut catalog = GatewayPluginBindingCatalog::new();
        catalog.register(
            GatewayPluginBinding::new("status-plugin", "hepta", "webhook", "status commands")
                .with_command_selector("/status"),
        );
        catalog.register(GatewayPluginBinding::for_surface(
            "surface-plugin",
            "hepta",
            "fallback",
        ));

        let resolution = catalog.resolve_lookup_keys([
            "surface=hepta|transport=webhook|command=/status",
            "surface=hepta",
        ]);
        let plan =
            GatewayPluginExecutionPlan::from_lookup_resolution(&resolution, "/status --json");

        assert!(plan.ready);
        assert_eq!(plan.handoffs.len(), 2);
        assert_eq!(
            plan.preferred_handoff()
                .map(|handoff| handoff.plugin_id.as_str()),
            Some("status-plugin")
        );

        let status_adapter = EchoGatewayPluginAdapter::new("status-plugin");
        let attempt = plan.execute_first_matching_adapter("/status --json", &[&status_adapter]);

        assert!(attempt.ready);
        assert_eq!(attempt.selected_plugin_id.as_deref(), Some("status-plugin"));
        assert_eq!(attempt.telemetry.total_attempts, 1);
        assert!(
            attempt
                .result
                .as_ref()
                .and_then(|result| result.output.as_deref())
                .unwrap_or_default()
                .contains("/status --json")
        );
    }

    #[test]
    fn execution_plan_fails_closed_without_payload_or_adapter() {
        let resolution = GatewayPluginBindingCatalog::new().resolve_lookup_keys(["surface=hepta"]);
        let empty_plan = GatewayPluginExecutionPlan::from_lookup_resolution(&resolution, " ");

        assert!(!empty_plan.ready);
        assert!(
            empty_plan
                .blockers
                .iter()
                .any(|item| item.contains("empty"))
        );
        assert!(
            empty_plan
                .blockers
                .iter()
                .any(|item| item.contains("no plugin"))
        );

        let mut catalog = GatewayPluginBindingCatalog::new();
        catalog.register(GatewayPluginBinding::for_surface(
            "surface-plugin",
            "hepta",
            "fallback",
        ));
        let resolution = catalog.resolve_lookup_keys(["surface=hepta"]);
        let plan = GatewayPluginExecutionPlan::from_lookup_resolution(&resolution, "hello");
        let attempt = plan.execute_first_matching_adapter("hello", &[]);

        assert!(!attempt.ready);
        assert!(attempt.blockers[0].contains("no registered adapter"));
    }

    #[test]
    fn execution_policy_records_retry_and_fallback_telemetry() {
        let mut catalog = GatewayPluginBindingCatalog::new();
        catalog.register(
            GatewayPluginBinding::new("status-plugin", "hepta", "webhook", "status commands")
                .with_command_selector("/status"),
        );
        catalog.register(GatewayPluginBinding::for_surface(
            "surface-plugin",
            "hepta",
            "fallback",
        ));
        let resolution = catalog.resolve_lookup_keys([
            "surface=hepta|transport=webhook|command=/status",
            "surface=hepta",
        ]);
        let plan = GatewayPluginExecutionPlan::from_lookup_resolution(&resolution, "/status");
        let failing = FailingGatewayPluginAdapter::new("status-plugin", "timeout after 25ms");
        let fallback = EchoGatewayPluginAdapter::new("surface-plugin");

        let attempt = plan.execute_with_policy(
            "/status",
            &[&failing, &fallback],
            GatewayPluginExecutionPolicy {
                max_attempts_per_plugin: 2,
                timeout_ms: 25,
                allow_fallback: true,
            },
        );

        assert!(attempt.ready);
        assert_eq!(
            attempt.selected_plugin_id.as_deref(),
            Some("surface-plugin")
        );
        assert_eq!(attempt.telemetry.timeout_ms, 25);
        assert_eq!(attempt.telemetry.total_attempts, 3);
        assert_eq!(attempt.telemetry.fallback_count, 1);
        assert_eq!(attempt.telemetry.trace[0].attempt, 1);
        assert!(attempt.telemetry.trace[0].outcome.contains("timeout"));
        assert_eq!(attempt.telemetry.trace[2].outcome, "accepted");
    }
}
