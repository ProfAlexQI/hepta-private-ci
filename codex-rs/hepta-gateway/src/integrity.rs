use hepta_core::{DoctorArea, DoctorCheckOutcome, DoctorOwner, DoctorStatus};

use crate::{GatewayEnvelope, GatewayPluginResolutionSnapshot, GatewaySurface};

pub const GATEWAY_TRANSPORT_SUPPORTED: &str = "gateway.transport_supported";
pub const GATEWAY_RESOLUTION_SNAPSHOT_CONSISTENT: &str = "gateway.resolution_snapshot_consistent";
pub const GATEWAY_DISPATCH_READY: &str = "gateway.dispatch_ready";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GatewayRouteIntegritySnapshot {
    pub surface_id: String,
    pub transport_key: String,
    pub supported_transport: bool,
    pub requested_lookup_keys: Vec<String>,
    pub candidate_plugin_ids: Vec<String>,
    pub candidate_lookup_keys: Vec<String>,
    pub candidate_lookup_indices: Vec<Option<usize>>,
    pub unmatched_lookup_keys: Vec<String>,
    pub out_of_contract_candidate_lookup_keys: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GatewayDispatchReadinessReport {
    pub supported_transport: bool,
    pub has_session_key: bool,
    pub has_normalized_text: bool,
    pub has_plugin_candidate: bool,
    pub resolution_snapshot_consistent: bool,
    pub full_lookup_coverage: bool,
    pub ready: bool,
    pub blockers: Vec<String>,
    pub warnings: Vec<String>,
}

impl GatewayRouteIntegritySnapshot {
    pub fn from_resolution(
        surface: &GatewaySurface,
        envelope: &GatewayEnvelope,
        resolution: &GatewayPluginResolutionSnapshot,
    ) -> Self {
        let candidate_plugin_ids = resolution
            .plugin_ids()
            .into_iter()
            .map(str::to_string)
            .collect::<Vec<_>>();
        let candidate_lookup_keys = resolution
            .matched_lookup_keys()
            .into_iter()
            .map(str::to_string)
            .collect::<Vec<_>>();
        let candidate_lookup_indices = candidate_lookup_keys
            .iter()
            .map(|lookup_key| {
                resolution
                    .binding_lookup_keys
                    .iter()
                    .position(|candidate_lookup_key| candidate_lookup_key == lookup_key)
            })
            .collect::<Vec<_>>();
        let out_of_contract_candidate_lookup_keys = candidate_lookup_keys
            .iter()
            .filter(|lookup_key| !resolution.binding_lookup_keys.contains(lookup_key))
            .cloned()
            .collect::<Vec<_>>();

        Self {
            surface_id: resolution.surface_id.clone(),
            transport_key: resolution.transport_key.clone(),
            supported_transport: surface.supports_transport(envelope.transport),
            requested_lookup_keys: resolution.binding_lookup_keys.clone(),
            candidate_plugin_ids,
            candidate_lookup_keys,
            candidate_lookup_indices,
            unmatched_lookup_keys: resolution
                .unmatched_lookup_keys()
                .into_iter()
                .map(str::to_string)
                .collect(),
            out_of_contract_candidate_lookup_keys,
        }
    }

    pub fn resolution_snapshot_is_consistent(&self) -> bool {
        self.out_of_contract_candidate_lookup_keys.is_empty()
            && self.candidate_lookup_indices.iter().all(Option::is_some)
            && self
                .candidate_lookup_indices
                .iter()
                .flatten()
                .zip(self.candidate_lookup_indices.iter().flatten().skip(1))
                .all(|(left, right)| left <= right)
    }

    pub fn has_full_lookup_coverage(&self) -> bool {
        self.unmatched_lookup_keys.is_empty()
    }

    pub fn doctor_checks(&self) -> Vec<DoctorCheckOutcome> {
        vec![self.transport_check(), self.resolution_check()]
    }

    pub fn dispatch_doctor_checks(
        &self,
        resolution: &GatewayPluginResolutionSnapshot,
    ) -> Vec<DoctorCheckOutcome> {
        let mut checks = self.doctor_checks();
        checks.push(self.dispatch_readiness_report(resolution).doctor_check());
        checks
    }

    pub fn dispatch_readiness_report(
        &self,
        resolution: &GatewayPluginResolutionSnapshot,
    ) -> GatewayDispatchReadinessReport {
        let has_session_key = !resolution.session_key.trim().is_empty();
        let has_normalized_text = !resolution.normalized_text.trim().is_empty();
        let has_plugin_candidate = !self.candidate_plugin_ids.is_empty();
        let resolution_snapshot_consistent = self.resolution_snapshot_is_consistent();
        let full_lookup_coverage = self.has_full_lookup_coverage();

        let mut blockers = Vec::new();
        if !self.supported_transport {
            blockers.push(format!(
                "unsupported transport {} for surface {}",
                self.transport_key, self.surface_id
            ));
        }
        if !has_session_key {
            blockers.push("missing session key".into());
        }
        if !has_normalized_text {
            blockers.push("empty normalized payload".into());
        }
        if !has_plugin_candidate {
            blockers.push("no plugin candidate resolved".into());
        }
        if !resolution_snapshot_consistent {
            blockers.push("plugin resolution snapshot drifted from lookup contract".into());
        }

        let mut warnings = Vec::new();
        if !full_lookup_coverage {
            warnings.push(format!(
                "partial lookup coverage: {}",
                self.unmatched_lookup_keys.join(", ")
            ));
        }

        GatewayDispatchReadinessReport {
            supported_transport: self.supported_transport,
            has_session_key,
            has_normalized_text,
            has_plugin_candidate,
            resolution_snapshot_consistent,
            full_lookup_coverage,
            ready: blockers.is_empty(),
            blockers,
            warnings,
        }
    }

    fn transport_check(&self) -> DoctorCheckOutcome {
        let status = if self.supported_transport {
            DoctorStatus::Ok
        } else {
            DoctorStatus::Fail
        };

        DoctorCheckOutcome {
            id: GATEWAY_TRANSPORT_SUPPORTED.into(),
            area: DoctorArea::Gateway,
            owner: owner("route transport support"),
            status,
            summary: "gateway surface supports the routed transport".into(),
            detail: if self.supported_transport {
                format!(
                    "surface {} accepts transport {}",
                    self.surface_id, self.transport_key
                )
            } else {
                format!(
                    "surface {} does not accept transport {}",
                    self.surface_id, self.transport_key
                )
            },
            remediation: (!self.supported_transport).then(|| {
                "route this envelope through a supported transport or add explicit gateway support"
                    .into()
            }),
        }
    }

    fn resolution_check(&self) -> DoctorCheckOutcome {
        let status = if !self.resolution_snapshot_is_consistent() {
            DoctorStatus::Fail
        } else if !self.has_full_lookup_coverage() {
            DoctorStatus::Warn
        } else {
            DoctorStatus::Ok
        };

        let detail = if !self.out_of_contract_candidate_lookup_keys.is_empty() {
            format!(
                "candidate lookup keys outside requested contract: {}",
                self.out_of_contract_candidate_lookup_keys.join(", ")
            )
        } else if !self.resolution_snapshot_is_consistent() {
            format!(
                "candidate lookup ordering drifted from requested contract: {:?}",
                self.candidate_lookup_indices
            )
        } else if !self.has_full_lookup_coverage() {
            format!(
                "matched {} candidates with remaining lookup gaps: {}",
                self.candidate_plugin_ids.len(),
                self.unmatched_lookup_keys.join(", ")
            )
        } else {
            format!(
                "matched {} candidates across {} requested lookup keys without gaps",
                self.candidate_plugin_ids.len(),
                self.requested_lookup_keys.len()
            )
        };

        DoctorCheckOutcome {
            id: GATEWAY_RESOLUTION_SNAPSHOT_CONSISTENT.into(),
            area: DoctorArea::Gateway,
            owner: owner("plugin resolution snapshot"),
            status,
            summary: "gateway plugin resolution snapshot stays aligned with lookup contract"
                .into(),
            detail,
            remediation: (status != DoctorStatus::Ok).then(|| {
                if !self.resolution_snapshot_is_consistent() {
                    "rebuild the gateway resolution snapshot from ordered gateway lookup keys before dispatch"
                        .into()
                } else {
                    "add plugin bindings for the remaining gateway lookup keys or keep the partial coverage explicit"
                        .into()
                }
            }),
        }
    }
}

impl GatewayDispatchReadinessReport {
    pub fn doctor_check(&self) -> DoctorCheckOutcome {
        let status = if !self.ready {
            DoctorStatus::Fail
        } else if !self.warnings.is_empty() {
            DoctorStatus::Warn
        } else {
            DoctorStatus::Ok
        };

        let detail = if !self.blockers.is_empty() {
            format!("dispatch blockers: {}", self.blockers.join("; "))
        } else if !self.warnings.is_empty() {
            format!("dispatch warnings: {}", self.warnings.join("; "))
        } else {
            "gateway dispatch inputs are ready for plugin handoff".into()
        };

        DoctorCheckOutcome {
            id: GATEWAY_DISPATCH_READY.into(),
            area: DoctorArea::Gateway,
            owner: owner("dispatch readiness"),
            status,
            summary: "gateway dispatch readiness is fail-closed before plugin handoff".into(),
            detail,
            remediation: (status != DoctorStatus::Ok).then(|| {
                if self.ready {
                    "add plugin bindings for partial lookup coverage or accept the explicit fallback warning"
                        .into()
                } else {
                    "repair gateway transport, session, payload, plugin candidates, or resolution ordering before dispatch"
                        .into()
                }
            }),
        }
    }
}

fn owner(responsibility: &str) -> DoctorOwner {
    DoctorOwner {
        component: "hepta-gateway".into(),
        responsibility: responsibility.into(),
    }
}

#[cfg(test)]
mod tests {
    use hepta_core::{DoctorArea, DoctorStatus};

    use crate::{
        GatewayEnvelope, GatewayPluginHandoffDraft, GatewayPluginResolutionSnapshot,
        GatewayResolvedPluginCandidate, GatewayRoutePlan, GatewaySurface, GatewayTransport,
    };

    use super::{
        GATEWAY_DISPATCH_READY, GATEWAY_RESOLUTION_SNAPSHOT_CONSISTENT,
        GATEWAY_TRANSPORT_SUPPORTED, GatewayRouteIntegritySnapshot,
    };

    #[test]
    fn integrity_snapshot_reports_supported_transport_and_full_coverage() {
        let surface = GatewaySurface;
        let envelope = GatewayEnvelope::new(
            "hepta",
            "user-7",
            GatewayTransport::Webhook,
            "/status --json",
        );
        let draft = GatewayPluginHandoffDraft::from_route(&surface.route_plan(&envelope));
        let resolution = GatewayPluginResolutionSnapshot::from_handoff_draft(&draft)
            .with_candidates([
                GatewayResolvedPluginCandidate::new(
                    "status-plugin",
                    "surface=hepta|transport=webhook|command=/status",
                    2,
                ),
                GatewayResolvedPluginCandidate::new(
                    "fallback-plugin",
                    "surface=hepta|transport=webhook",
                    1,
                ),
                GatewayResolvedPluginCandidate::new("surface-plugin", "surface=hepta", 0),
            ]);

        let snapshot =
            GatewayRouteIntegritySnapshot::from_resolution(&surface, &envelope, &resolution);
        let checks = snapshot.doctor_checks();

        assert!(snapshot.supported_transport);
        assert!(snapshot.resolution_snapshot_is_consistent());
        assert!(snapshot.has_full_lookup_coverage());
        assert_eq!(
            snapshot.candidate_lookup_indices,
            vec![Some(0), Some(1), Some(2)]
        );
        assert_eq!(checks.len(), 2);
        assert_eq!(checks[0].id, GATEWAY_TRANSPORT_SUPPORTED);
        assert_eq!(checks[0].area, DoctorArea::Gateway);
        assert_eq!(checks[0].status, DoctorStatus::Ok);
        assert_eq!(checks[1].id, GATEWAY_RESOLUTION_SNAPSHOT_CONSISTENT);
        assert_eq!(checks[1].status, DoctorStatus::Ok);

        let readiness = snapshot.dispatch_readiness_report(&resolution);
        assert!(readiness.ready);
        assert!(readiness.blockers.is_empty());
        assert!(readiness.warnings.is_empty());
        let dispatch_checks = snapshot.dispatch_doctor_checks(&resolution);
        assert_eq!(dispatch_checks.len(), 3);
        assert_eq!(dispatch_checks[2].id, GATEWAY_DISPATCH_READY);
        assert_eq!(dispatch_checks[2].status, DoctorStatus::Ok);
    }

    #[test]
    fn integrity_snapshot_reports_partial_coverage_without_contract_drift() {
        let surface = GatewaySurface;
        let envelope = GatewayEnvelope::new("telegram", "user-9", GatewayTransport::Cli, "/status");
        let draft = GatewayPluginHandoffDraft::from_route(&surface.route_plan(&envelope));
        let resolution = GatewayPluginResolutionSnapshot::from_handoff_draft(&draft)
            .with_candidates([GatewayResolvedPluginCandidate::new(
                "fallback-plugin",
                "surface=telegram|transport=cli",
                1,
            )]);

        let snapshot =
            GatewayRouteIntegritySnapshot::from_resolution(&surface, &envelope, &resolution);
        let checks = snapshot.doctor_checks();

        assert!(snapshot.supported_transport);
        assert!(snapshot.resolution_snapshot_is_consistent());
        assert!(!snapshot.has_full_lookup_coverage());
        assert_eq!(
            snapshot.unmatched_lookup_keys,
            vec![
                "surface=telegram|transport=cli|command=/status".to_string(),
                "surface=telegram".to_string(),
            ]
        );
        assert_eq!(checks[0].status, DoctorStatus::Ok);
        assert_eq!(checks[1].status, DoctorStatus::Warn);
        assert!(checks[1].detail.contains("remaining lookup gaps"));

        let readiness = snapshot.dispatch_readiness_report(&resolution);
        assert!(readiness.ready);
        assert!(readiness.blockers.is_empty());
        assert_eq!(readiness.warnings.len(), 1);
        assert!(readiness.warnings[0].contains("partial lookup coverage"));
        let dispatch_checks = snapshot.dispatch_doctor_checks(&resolution);
        assert_eq!(dispatch_checks[2].status, DoctorStatus::Warn);
        assert!(dispatch_checks[2].detail.contains("dispatch warnings"));
    }

    #[test]
    fn integrity_snapshot_reports_queue_transport_contract_drift_without_dispatch() {
        let surface = GatewaySurface;
        let envelope = GatewayEnvelope::new("hepta", "user-7", GatewayTransport::Queue, "/status");
        let resolution = GatewayPluginResolutionSnapshot::from_handoff_draft(
            &GatewayPluginHandoffDraft::from_route(&GatewayRoutePlan::new(
                "hepta",
                "session-42",
                GatewayTransport::Queue,
                "/status",
            )),
        )
        .with_candidates([
            GatewayResolvedPluginCandidate::new("fallback-plugin", "surface=hepta", 0),
            GatewayResolvedPluginCandidate::new(
                "status-plugin",
                "surface=hepta|transport=queue|command=/status",
                2,
            ),
        ]);

        let snapshot =
            GatewayRouteIntegritySnapshot::from_resolution(&surface, &envelope, &resolution);
        let checks = snapshot.doctor_checks();

        assert!(snapshot.supported_transport);
        assert!(!snapshot.resolution_snapshot_is_consistent());
        assert_eq!(snapshot.candidate_lookup_indices, vec![Some(2), Some(0)]);
        assert_eq!(checks[0].status, DoctorStatus::Ok);
        assert_eq!(checks[1].status, DoctorStatus::Fail);
        assert!(checks[0].remediation.is_none());
        assert!(checks[1].detail.contains("ordering drifted"));
        assert!(checks[1].remediation.is_some());

        let readiness = snapshot.dispatch_readiness_report(&resolution);
        assert!(!readiness.ready);
        assert!(
            readiness
                .blockers
                .iter()
                .any(|blocker| blocker.contains("drifted"))
        );
        let dispatch_checks = snapshot.dispatch_doctor_checks(&resolution);
        assert_eq!(dispatch_checks[2].status, DoctorStatus::Fail);
        assert!(dispatch_checks[2].detail.contains("dispatch blockers"));
    }
}
