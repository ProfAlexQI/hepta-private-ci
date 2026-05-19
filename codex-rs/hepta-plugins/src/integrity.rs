use std::collections::BTreeSet;

use hepta_core::{DoctorArea, DoctorCheckOutcome, DoctorOwner, DoctorStatus};

use crate::{GatewayPluginBindingCatalog, PluginCatalog};

pub const PLUGIN_MANIFESTS_VALID: &str = "plugin.manifests_valid";
pub const PLUGIN_REGISTRATIONS_RESOLVED: &str = "plugin.registrations_resolved";
pub const PLUGIN_OPERATIONAL_READY: &str = "plugin.operational_ready";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PluginIntegritySnapshot {
    pub manifest_ids: Vec<String>,
    pub normalized_manifest_ids: Vec<String>,
    pub invalid_manifest_ids: Vec<String>,
    pub duplicate_normalized_manifest_ids: Vec<String>,
    pub binding_plugin_ids: Vec<String>,
    pub bindings_missing_manifest: Vec<String>,
    pub manifests_without_bindings: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PluginOperationalReadinessReport {
    pub manifest_count: usize,
    pub binding_plugin_count: usize,
    pub manifests_valid: bool,
    pub registrations_resolved: bool,
    pub ready: bool,
    pub blockers: Vec<String>,
    pub warnings: Vec<String>,
}

impl PluginIntegritySnapshot {
    pub fn from_catalogs(
        manifests: &PluginCatalog,
        bindings: &GatewayPluginBindingCatalog,
    ) -> Self {
        let manifest_ids = manifests
            .manifests()
            .iter()
            .map(|manifest| manifest.id.clone())
            .collect::<Vec<_>>();

        let mut normalized_manifest_ids = Vec::new();
        let mut invalid_manifest_ids = Vec::new();
        let mut duplicate_normalized_manifest_ids = Vec::new();
        let mut seen_normalized_manifest_ids = BTreeSet::new();

        for manifest_id in &manifest_ids {
            match normalize_manifest_id(manifest_id) {
                Some(normalized_manifest_id) => {
                    if !seen_normalized_manifest_ids.insert(normalized_manifest_id.clone()) {
                        duplicate_normalized_manifest_ids.push(normalized_manifest_id);
                    } else {
                        normalized_manifest_ids.push(normalized_manifest_id);
                    }
                }
                None => invalid_manifest_ids.push(manifest_id.clone()),
            }
        }

        let binding_plugin_ids = bindings
            .bindings()
            .iter()
            .map(|binding| binding.plugin_id.clone())
            .fold(Vec::new(), |mut ids, plugin_id| {
                if !ids.contains(&plugin_id) {
                    ids.push(plugin_id);
                }
                ids
            });

        let bindings_missing_manifest = binding_plugin_ids
            .iter()
            .filter(|plugin_id| !normalized_manifest_ids.contains(plugin_id))
            .cloned()
            .collect::<Vec<_>>();

        let manifests_without_bindings = normalized_manifest_ids
            .iter()
            .filter(|manifest_id| !binding_plugin_ids.contains(manifest_id))
            .cloned()
            .collect::<Vec<_>>();

        Self {
            manifest_ids,
            normalized_manifest_ids,
            invalid_manifest_ids,
            duplicate_normalized_manifest_ids,
            binding_plugin_ids,
            bindings_missing_manifest,
            manifests_without_bindings,
        }
    }

    pub fn manifests_are_valid(&self) -> bool {
        self.invalid_manifest_ids.is_empty() && self.duplicate_normalized_manifest_ids.is_empty()
    }

    pub fn registrations_are_resolved(&self) -> bool {
        self.bindings_missing_manifest.is_empty()
    }

    pub fn doctor_checks(&self) -> Vec<DoctorCheckOutcome> {
        vec![self.manifest_check(), self.registration_check()]
    }

    pub fn operational_doctor_checks(&self) -> Vec<DoctorCheckOutcome> {
        let mut checks = self.doctor_checks();
        checks.push(self.operational_readiness_report().doctor_check());
        checks
    }

    pub fn operational_readiness_report(&self) -> PluginOperationalReadinessReport {
        let manifests_valid = self.manifests_are_valid();
        let registrations_resolved = self.registrations_are_resolved();
        let mut blockers = Vec::new();
        if !manifests_valid {
            blockers.push("plugin manifest ids are invalid or collide after normalization".into());
        }
        if !registrations_resolved {
            blockers.push(format!(
                "bindings missing manifests: {}",
                self.bindings_missing_manifest.join(", ")
            ));
        }
        if self.binding_plugin_ids.is_empty() {
            blockers.push("no gateway plugin bindings registered".into());
        }

        let mut warnings = Vec::new();
        if !self.manifests_without_bindings.is_empty() {
            warnings.push(format!(
                "manifests without gateway bindings: {}",
                self.manifests_without_bindings.join(", ")
            ));
        }

        PluginOperationalReadinessReport {
            manifest_count: self.normalized_manifest_ids.len(),
            binding_plugin_count: self.binding_plugin_ids.len(),
            manifests_valid,
            registrations_resolved,
            ready: blockers.is_empty(),
            blockers,
            warnings,
        }
    }

    fn manifest_check(&self) -> DoctorCheckOutcome {
        let status = if self.manifests_are_valid() {
            DoctorStatus::Ok
        } else {
            DoctorStatus::Fail
        };

        let detail = if self.manifests_are_valid() {
            format!(
                "{} manifests normalize into {} stable plugin ids",
                self.manifest_ids.len(),
                self.normalized_manifest_ids.len()
            )
        } else {
            let mut issues = Vec::new();
            if !self.invalid_manifest_ids.is_empty() {
                issues.push(format!(
                    "invalid manifest ids: {}",
                    self.invalid_manifest_ids.join(", ")
                ));
            }
            if !self.duplicate_normalized_manifest_ids.is_empty() {
                issues.push(format!(
                    "normalized id collisions: {}",
                    self.duplicate_normalized_manifest_ids.join(", ")
                ));
            }
            issues.join("; ")
        };

        DoctorCheckOutcome {
            id: PLUGIN_MANIFESTS_VALID.into(),
            area: DoctorArea::Plugin,
            owner: owner("manifest catalog"),
            status,
            summary: "plugin manifests normalize into stable ids".into(),
            detail,
            remediation: (status != DoctorStatus::Ok).then(|| {
                "trim plugin manifest ids and remove normalized collisions before registering bindings"
                    .into()
            }),
        }
    }

    fn registration_check(&self) -> DoctorCheckOutcome {
        let status = if self.registrations_are_resolved() {
            DoctorStatus::Ok
        } else {
            DoctorStatus::Warn
        };

        let mut detail = if self.registrations_are_resolved() {
            format!(
                "{} binding plugin ids resolve against {} normalized manifests",
                self.binding_plugin_ids.len(),
                self.normalized_manifest_ids.len()
            )
        } else {
            format!(
                "bindings missing manifests: {}",
                self.bindings_missing_manifest.join(", ")
            )
        };

        if !self.manifests_without_bindings.is_empty() {
            detail.push_str("; manifests without bindings: ");
            detail.push_str(&self.manifests_without_bindings.join(", "));
        }

        DoctorCheckOutcome {
            id: PLUGIN_REGISTRATIONS_RESOLVED.into(),
            area: DoctorArea::Plugin,
            owner: owner("plugin registrations"),
            status,
            summary: "plugin bindings resolve against registered manifests".into(),
            detail,
            remediation: (status != DoctorStatus::Ok).then(|| {
                "register manifests for every bound plugin id or remove orphaned bindings".into()
            }),
        }
    }
}

impl PluginOperationalReadinessReport {
    pub fn doctor_check(&self) -> DoctorCheckOutcome {
        let status = if !self.ready {
            DoctorStatus::Fail
        } else if !self.warnings.is_empty() {
            DoctorStatus::Warn
        } else {
            DoctorStatus::Ok
        };

        let detail = if !self.blockers.is_empty() {
            format!("plugin readiness blockers: {}", self.blockers.join("; "))
        } else if !self.warnings.is_empty() {
            format!("plugin readiness warnings: {}", self.warnings.join("; "))
        } else {
            format!(
                "{} manifests and {} gateway-bound plugin ids are operationally ready",
                self.manifest_count, self.binding_plugin_count
            )
        };

        DoctorCheckOutcome {
            id: PLUGIN_OPERATIONAL_READY.into(),
            area: DoctorArea::Plugin,
            owner: owner("plugin operational readiness"),
            status,
            summary: "plugin manifests and gateway bindings are operationally ready".into(),
            detail,
            remediation: (status != DoctorStatus::Ok).then(|| {
                if self.ready {
                    "add gateway bindings for unbound manifests or accept the explicit unused-manifest warning"
                        .into()
                } else {
                    "fix manifest ids, register missing manifests for bindings, and ensure at least one gateway binding exists"
                        .into()
                }
            }),
        }
    }
}

fn normalize_manifest_id(value: &str) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_ascii_lowercase())
    }
}

fn owner(responsibility: &str) -> DoctorOwner {
    DoctorOwner {
        component: "hepta-plugins".into(),
        responsibility: responsibility.into(),
    }
}

#[cfg(test)]
mod tests {
    use hepta_core::{DoctorArea, DoctorStatus, PluginManifest};

    use crate::{GatewayPluginBinding, GatewayPluginBindingCatalog, PluginCatalog};

    use super::{
        PLUGIN_MANIFESTS_VALID, PLUGIN_OPERATIONAL_READY, PLUGIN_REGISTRATIONS_RESOLVED,
        PluginIntegritySnapshot,
    };

    #[test]
    fn integrity_snapshot_tracks_manifest_and_binding_alignment() {
        let mut manifests = PluginCatalog::new();
        manifests.register_manifest(PluginManifest {
            id: "Status-Plugin".into(),
            version: "0.1.0".into(),
            description: "status helpers".into(),
        });
        manifests.register_manifest(PluginManifest {
            id: "surface-plugin".into(),
            version: "0.1.0".into(),
            description: "surface helpers".into(),
        });

        let mut bindings = GatewayPluginBindingCatalog::new();
        bindings.register(
            GatewayPluginBinding::new("status-plugin", "hepta", "webhook", "status")
                .with_command_selector("/status"),
        );

        let snapshot = PluginIntegritySnapshot::from_catalogs(&manifests, &bindings);
        let checks = snapshot.doctor_checks();

        assert!(snapshot.manifests_are_valid());
        assert!(snapshot.registrations_are_resolved());
        assert_eq!(
            snapshot.normalized_manifest_ids,
            vec!["status-plugin", "surface-plugin"]
        );
        assert_eq!(snapshot.binding_plugin_ids, vec!["status-plugin"]);
        assert_eq!(snapshot.manifests_without_bindings, vec!["surface-plugin"]);
        assert_eq!(checks.len(), 2);
        assert_eq!(checks[0].id, PLUGIN_MANIFESTS_VALID);
        assert_eq!(checks[0].area, DoctorArea::Plugin);
        assert_eq!(checks[0].status, DoctorStatus::Ok);
        assert_eq!(checks[1].id, PLUGIN_REGISTRATIONS_RESOLVED);
        assert_eq!(checks[1].status, DoctorStatus::Ok);
        assert!(
            checks[1]
                .detail
                .contains("manifests without bindings: surface-plugin")
        );

        let readiness = snapshot.operational_readiness_report();
        assert!(readiness.ready);
        assert_eq!(readiness.manifest_count, 2);
        assert_eq!(readiness.binding_plugin_count, 1);
        assert!(readiness.blockers.is_empty());
        assert_eq!(readiness.warnings.len(), 1);
        assert!(readiness.warnings[0].contains("surface-plugin"));
        let operational_checks = snapshot.operational_doctor_checks();
        assert_eq!(operational_checks.len(), 3);
        assert_eq!(operational_checks[2].id, PLUGIN_OPERATIONAL_READY);
        assert_eq!(operational_checks[2].status, DoctorStatus::Warn);
    }

    #[test]
    fn integrity_snapshot_reports_manifest_collisions_and_orphaned_bindings() {
        let mut manifests = PluginCatalog::new();
        manifests.register_manifest(PluginManifest {
            id: "Lint".into(),
            version: "0.1.0".into(),
            description: "lint helpers".into(),
        });
        manifests.register_manifest(PluginManifest {
            id: " lint ".into(),
            version: "0.1.1".into(),
            description: "lint helpers patched".into(),
        });
        manifests.register_manifest(PluginManifest {
            id: "   ".into(),
            version: "0.2.0".into(),
            description: "broken manifest".into(),
        });

        let mut bindings = GatewayPluginBindingCatalog::new();
        bindings.register(GatewayPluginBinding::new(
            "doctor-plugin",
            "hepta",
            "webhook",
            "doctor helpers",
        ));

        let snapshot = PluginIntegritySnapshot::from_catalogs(&manifests, &bindings);
        let checks = snapshot.doctor_checks();

        assert!(!snapshot.manifests_are_valid());
        assert!(!snapshot.registrations_are_resolved());
        assert_eq!(snapshot.invalid_manifest_ids, vec!["   "]);
        assert_eq!(snapshot.duplicate_normalized_manifest_ids, vec!["lint"]);
        assert_eq!(snapshot.bindings_missing_manifest, vec!["doctor-plugin"]);
        assert_eq!(checks[0].id, PLUGIN_MANIFESTS_VALID);
        assert_eq!(checks[0].status, DoctorStatus::Fail);
        assert!(checks[0].detail.contains("invalid manifest ids"));
        assert!(checks[0].detail.contains("normalized id collisions: lint"));
        assert!(checks[0].remediation.is_some());
        assert_eq!(checks[1].id, PLUGIN_REGISTRATIONS_RESOLVED);
        assert_eq!(checks[1].status, DoctorStatus::Warn);
        assert!(
            checks[1]
                .detail
                .contains("bindings missing manifests: doctor-plugin")
        );
        assert!(checks[1].remediation.is_some());

        let readiness = snapshot.operational_readiness_report();
        assert!(!readiness.ready);
        assert!(
            readiness
                .blockers
                .iter()
                .any(|blocker| blocker.contains("manifest ids"))
        );
        assert!(
            readiness
                .blockers
                .iter()
                .any(|blocker| blocker.contains("doctor-plugin"))
        );
        let operational_checks = snapshot.operational_doctor_checks();
        assert_eq!(operational_checks[2].status, DoctorStatus::Fail);
        assert!(operational_checks[2].detail.contains("readiness blockers"));
    }
}
