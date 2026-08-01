//! Local Phase 6 packaging status model for Hepta Native.
//!
//! This is a UI/reporting model only. It records the packaging gates the native
//! client should display while Android/iOS builds are being made reproducible;
//! it never shells out, installs toolchains, or touches devices by itself.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeptaPackagingGateStatus {
    Ready,
    Pending,
    Blocked,
}

impl HeptaPackagingGateStatus {
    pub fn label(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::Pending => "pending",
            Self::Blocked => "blocked",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeptaPackagingGate {
    pub id: &'static str,
    pub label: &'static str,
    pub status: HeptaPackagingGateStatus,
    pub evidence: &'static str,
    pub command: &'static str,
}

impl HeptaPackagingGate {
    pub fn operator_line(&self) -> String {
        format!(
            "{} · {} · {}",
            self.status.label(),
            self.evidence,
            self.command,
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeptaPackagingStatusSnapshot {
    pub title: &'static str,
    pub subtitle: &'static str,
    pub gates: Vec<HeptaPackagingGate>,
}

impl HeptaPackagingStatusSnapshot {
    pub fn ready_count(&self) -> usize {
        self.gates
            .iter()
            .filter(|gate| gate.status == HeptaPackagingGateStatus::Ready)
            .count()
    }

    pub fn pending_count(&self) -> usize {
        self.gates
            .iter()
            .filter(|gate| gate.status == HeptaPackagingGateStatus::Pending)
            .count()
    }

    pub fn blocked_count(&self) -> usize {
        self.gates
            .iter()
            .filter(|gate| gate.status == HeptaPackagingGateStatus::Blocked)
            .count()
    }

    pub fn summary_line(&self) -> String {
        format!(
            "{} ready · {} pending · {} blocked",
            self.ready_count(),
            self.pending_count(),
            self.blocked_count(),
        )
    }
}

pub fn sample_mobile_packaging_status() -> HeptaPackagingStatusSnapshot {
    HeptaPackagingStatusSnapshot {
        title: "Mobile packaging gates",
        subtitle: "Phase 6 status shown inside the desktop/mobile client; no install or device action is triggered from UI.",
        gates: vec![
            HeptaPackagingGate {
                id: "cargo-makepad",
                label: "cargo-makepad CLI",
                status: HeptaPackagingGateStatus::Ready,
                evidence: "installed from Makepad dev branch as cargo-makepad v1.0.0",
                command: "cargo makepad --help",
            },
            HeptaPackagingGate {
                id: "apple-ios-toolchain",
                label: "Apple iOS toolchain",
                status: HeptaPackagingGateStatus::Ready,
                evidence: "nightly aarch64-apple-ios-sim and aarch64-apple-ios std components installed",
                command: "cargo makepad apple ios install-toolchain",
            },
            HeptaPackagingGate {
                id: "android-toolchain",
                label: "Android toolchain",
                status: HeptaPackagingGateStatus::Ready,
                evidence: "android_33_sdk platform-tools/build-tools/NDK/JDK materialized locally after Makepad stripped-NDK installer bug",
                command: "cargo makepad android --abi=aarch64 --package-name=ai.hepta.nativeapp --app-label='Hepta Native' --sdk-path=./android_33_sdk install-toolchain",
            },
            HeptaPackagingGate {
                id: "ios-sim-build",
                label: "iOS simulator build smoke",
                status: HeptaPackagingGateStatus::Ready,
                evidence: "iOS 26.5 Simulator runtime installed and release simulator build completed from apps/hepta-native",
                command: "cargo makepad apple ios --org=ai.hepta --app=hepta-native build -p hepta-native --release",
            },
            HeptaPackagingGate {
                id: "android-build",
                label: "Android build smoke",
                status: HeptaPackagingGateStatus::Ready,
                evidence: "APK build completed with Java-safe package ai.hepta.nativeapp; no adb run is started by UI",
                command: "cargo makepad android --abi=aarch64 --package-name=ai.hepta.nativeapp --app-label='Hepta Native' --sdk-path=./android_33_sdk build -p hepta-native --release",
            },
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mobile_packaging_status_tracks_phase_six_without_side_effects() {
        let snapshot = sample_mobile_packaging_status();
        assert_eq!(snapshot.ready_count(), 5);
        assert_eq!(snapshot.pending_count(), 0);
        assert_eq!(snapshot.blocked_count(), 0);
        assert!(
            snapshot
                .summary_line()
                .contains("5 ready · 0 pending · 0 blocked")
        );
        assert!(
            snapshot
                .gates
                .iter()
                .any(|gate| gate.id == "cargo-makepad" && gate.evidence.contains("v1.0.0"))
        );
    }

    #[test]
    fn packaging_commands_preserve_exact_mobile_gate_invocations() {
        let snapshot = sample_mobile_packaging_status();
        let android = snapshot
            .gates
            .iter()
            .find(|gate| gate.id == "android-build")
            .expect("android build gate should be present");
        assert!(
            android
                .command
                .contains("--package-name=ai.hepta.nativeapp")
        );
        assert!(android.command.contains("--sdk-path=./android_33_sdk"));
        assert!(android.evidence.contains("APK build completed"));
        let ios = snapshot
            .gates
            .iter()
            .find(|gate| gate.id == "ios-sim-build")
            .expect("ios sim gate should be present");
        assert!(ios.command.contains("apple ios"));
        assert!(ios.command.contains("build -p hepta-native --release"));
        assert!(ios.evidence.contains("release simulator build completed"));
    }
}
