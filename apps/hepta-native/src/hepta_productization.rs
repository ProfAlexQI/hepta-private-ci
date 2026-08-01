//! Local-only Hepta Native productization status model.
//!
//! This module is intentionally side-effect free. It does not inspect the live
//! filesystem, call Matrix APIs, launch packaging tools, start simulators,
//! call OpenClaw Gateway, or mutate runtime/action/approval state. The snapshot is a
//! bounded operator-facing status surface for the current Robrix-to-Hepta
//! productization line.

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HeptaProductizationItem {
    pub key: &'static str,
    pub label: &'static str,
    pub status: HeptaProductizationStatus,
    pub detail: &'static str,
    pub blocking: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HeptaProductizationStatus {
    Complete,
    InProgress,
    Gated,
}

impl HeptaProductizationStatus {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::InProgress => "in progress",
            Self::Gated => "gated",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HeptaProductizationSnapshot {
    pub title: &'static str,
    pub summary: &'static str,
    pub android_package: &'static str,
    pub desktop_bundle_id: &'static str,
    pub items: Vec<HeptaProductizationItem>,
}

impl HeptaProductizationSnapshot {
    pub fn completed_count(&self) -> usize {
        self.items
            .iter()
            .filter(|item| item.status == HeptaProductizationStatus::Complete)
            .count()
    }

    pub fn gated_count(&self) -> usize {
        self.items
            .iter()
            .filter(|item| item.status == HeptaProductizationStatus::Gated)
            .count()
    }

    pub fn item(&self, key: &str) -> Option<&HeptaProductizationItem> {
        self.items.iter().find(|item| item.key == key)
    }
}

pub fn sample_productization_snapshot() -> HeptaProductizationSnapshot {
    HeptaProductizationSnapshot {
        title: "Hepta Native productization",
        summary: "Matrix-heart baseline is fully transplanted; the product shell exposes Hepta-owned OpenClaw-parity semantics. All capability domains have at least M2 local adapters, and 19/22 domains are M4 product-ready, including all 6 Hepta-unique domains and all 13 OpenClaw required absorb domains. External live execution remains separately gated.",
        android_package: "ai.hepta.nativeapp",
        desktop_bundle_id: "ai.hepta.nativeapp",
        items: vec![
            HeptaProductizationItem {
                key: "matrix_heart",
                label: "Matrix-heart baseline absorption",
                status: HeptaProductizationStatus::Complete,
                detail: "All Robrix non-target baseline files are present under apps/hepta-native; Matrix SDK, room list, timeline, composer, and mobile/desktop shells remain intact.",
                blocking: false,
            },
            HeptaProductizationItem {
                key: "hepta_cockpit",
                label: "Hepta cockpit overlays",
                status: HeptaProductizationStatus::Complete,
                detail: "Runtime, task, tool, approval, action outbox, payload inspection, context, command templates, and packaging gates are visible on desktop/mobile surfaces.",
                blocking: false,
            },
            HeptaProductizationItem {
                key: "branding_metadata",
                label: "Hepta packaging and bundle probe",
                status: HeptaProductizationStatus::Complete,
                detail: "Bundle IDs, desktop metadata, macOS plist text, app icons, DMG background, operator docs, and the local unsigned .app bundle probe target Hepta Native; internal Robrix-derived widget identifiers remain only where preserving the Matrix-heart substrate is safer.",
                blocking: false,
            },
            HeptaProductizationItem {
                key: "native_runtime_parity",
                label: "Hepta native OpenClaw-parity runtime",
                status: HeptaProductizationStatus::Complete,
                detail: "All 6 Hepta-unique domains and all 13 OpenClaw required absorb domains are M4 product-ready in the runtime capability matrix, with Hepta-owned durable stores, ledgers, handoffs, CLI/API/UI evidence, tests, installed-runtime acceptance, and docs. The live-adapter activation discipline gate remains the final preflight for provider/channel/node/process effects; external live execution is still gated by exact payload preview/hash, policy decision, operator confirmation, idempotency, and readback evidence. This is Hepta-native capability replication, not a Gateway-backed integration path.",
                blocking: false,
            },
            HeptaProductizationItem {
                key: "mobile_release",
                label: "Mobile release packaging",
                status: HeptaProductizationStatus::Complete,
                detail: "Android APK smoke is validated with ai.hepta.nativeapp; iOS 26.5 Simulator runtime is installed and release simulator build smoke completed from apps/hepta-native.",
                blocking: false,
            },
            HeptaProductizationItem {
                key: "release_candidate",
                label: "Release candidate posture",
                status: HeptaProductizationStatus::Gated,
                detail: "The product shell and native parity runtime are runnable, but Hepta Native is not a public release candidate until operator-approved external provider/channel/auth confirmations are complete and release-channel ownership is explicit.",
                blocking: true,
            },
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn productization_snapshot_tracks_absorbed_matrix_heart_and_gates() {
        let snapshot = sample_productization_snapshot();
        assert_eq!(snapshot.android_package, "ai.hepta.nativeapp");
        assert_eq!(snapshot.desktop_bundle_id, "ai.hepta.nativeapp");
        assert!(snapshot.completed_count() >= 2);
        assert!(snapshot.gated_count() >= 1);
        assert_eq!(
            snapshot.item("matrix_heart").map(|item| item.status),
            Some(HeptaProductizationStatus::Complete),
        );
    }

    #[test]
    fn native_runtime_parity_is_complete_while_external_release_remains_gated() {
        let snapshot = sample_productization_snapshot();
        let native_runtime = snapshot.item("native_runtime_parity").unwrap();
        assert_eq!(native_runtime.status, HeptaProductizationStatus::Complete);
        assert!(!native_runtime.blocking);
        assert!(native_runtime.detail.contains("M4 product-ready"));
        assert!(native_runtime.detail.contains("All 6 Hepta-unique domains"));
        assert!(
            native_runtime
                .detail
                .contains("all 13 OpenClaw required absorb domains")
        );
        assert!(
            native_runtime
                .detail
                .contains("live-adapter activation discipline")
        );
        assert!(
            native_runtime
                .detail
                .contains("external live execution is still gated")
        );
        assert!(
            native_runtime
                .detail
                .contains("Hepta-native capability replication")
        );
        assert!(
            native_runtime
                .detail
                .contains("not a Gateway-backed integration path")
        );
        assert!(native_runtime.detail.contains("readback evidence"));

        let release = snapshot.item("release_candidate").unwrap();
        assert_eq!(release.status, HeptaProductizationStatus::Gated);
        assert!(release.blocking);
        assert!(release.detail.contains("public release candidate"));
    }
}
