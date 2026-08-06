use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;

use crate::Sha256Digest;
use crate::channel::digest_parts;

pub const MIGRATION_SNAPSHOT_SCHEMA_VERSION: u32 = 1;
pub const MAX_MIGRATION_FAMILY_ID_BYTES: usize = 128;

const MIGRATION_SNAPSHOT_DOMAIN: &str = "hepta.migration-family-snapshot.v1";
const MIGRATION_SNAPSHOT_ID_PREFIX: &str = "migration-snapshot:v1:";

/// A bounded, human-readable family label for migration evidence.
///
/// This label is metadata. Constructing one does not grant authority over the
/// referenced implementation family.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(transparent)]
pub struct MigrationFamilyId(String);

impl MigrationFamilyId {
    pub fn new(value: impl Into<String>) -> Result<Self, String> {
        let value = value.into();
        let first = value.bytes().next();
        if value.is_empty()
            || value.len() > MAX_MIGRATION_FAMILY_ID_BYTES
            || !first.is_some_and(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit())
            || !value.bytes().all(|byte| {
                byte.is_ascii_lowercase()
                    || byte.is_ascii_digit()
                    || matches!(byte, b'-' | b'_' | b'.' | b'/')
            })
        {
            return Err("migration family ID is invalid".to_string());
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<'de> Deserialize<'de> for MigrationFamilyId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::new(value).map_err(serde::de::Error::custom)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MigrationDisposition {
    Keep,
    Rewrite,
    Project,
    HistoricalOnly,
    Drop,
}

impl MigrationDisposition {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Keep => "keep",
            Self::Rewrite => "rewrite",
            Self::Project => "project",
            Self::HistoricalOnly => "historical_only",
            Self::Drop => "drop",
        }
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(transparent)]
pub struct MigrationSnapshotId(String);

impl MigrationSnapshotId {
    pub fn parse(value: impl Into<String>) -> Result<Self, String> {
        let value = value.into();
        let Some(digest) = value.strip_prefix(MIGRATION_SNAPSHOT_ID_PREFIX) else {
            return Err("migration snapshot ID has the wrong prefix".to_string());
        };
        Sha256Digest::parse(digest.to_string())?;
        Ok(Self(value))
    }

    fn for_snapshot(snapshot: &MigrationFamilySnapshot) -> Self {
        let schema_version = snapshot.schema_version.to_string();
        let digest = digest_parts([
            MIGRATION_SNAPSHOT_DOMAIN,
            schema_version.as_str(),
            snapshot.family_id.as_str(),
            snapshot.disposition.as_str(),
            snapshot.old_implementation_sha256.as_str(),
            snapshot.vnext_implementation_sha256.as_str(),
            snapshot.candidate_sha256.as_str(),
        ]);
        Self(format!("{MIGRATION_SNAPSHOT_ID_PREFIX}{}", digest.as_str()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<'de> Deserialize<'de> for MigrationSnapshotId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::parse(value).map_err(serde::de::Error::custom)
    }
}

/// Canonical metadata that names one proposed migration candidate.
///
/// Every digest is supplied by the caller. Validation proves only canonical
/// shape and exact internal binding; it does not prove that the digests name
/// authoritative source trees, build artifacts, or operator acceptance.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct MigrationFamilySnapshot {
    schema_version: u32,
    snapshot_id: MigrationSnapshotId,
    family_id: MigrationFamilyId,
    disposition: MigrationDisposition,
    old_implementation_sha256: Sha256Digest,
    vnext_implementation_sha256: Sha256Digest,
    candidate_sha256: Sha256Digest,
}

impl MigrationFamilySnapshot {
    pub fn new(
        family_id: MigrationFamilyId,
        disposition: MigrationDisposition,
        old_implementation_sha256: Sha256Digest,
        vnext_implementation_sha256: Sha256Digest,
        candidate_sha256: Sha256Digest,
    ) -> Self {
        let mut snapshot = Self {
            schema_version: MIGRATION_SNAPSHOT_SCHEMA_VERSION,
            snapshot_id: MigrationSnapshotId(String::new()),
            family_id,
            disposition,
            old_implementation_sha256,
            vnext_implementation_sha256,
            candidate_sha256,
        };
        snapshot.snapshot_id = MigrationSnapshotId::for_snapshot(&snapshot);
        snapshot
    }

    pub const fn schema_version(&self) -> u32 {
        self.schema_version
    }

    pub const fn snapshot_id(&self) -> &MigrationSnapshotId {
        &self.snapshot_id
    }

    pub const fn family_id(&self) -> &MigrationFamilyId {
        &self.family_id
    }

    pub const fn disposition(&self) -> MigrationDisposition {
        self.disposition
    }

    pub const fn old_implementation_sha256(&self) -> &Sha256Digest {
        &self.old_implementation_sha256
    }

    pub const fn vnext_implementation_sha256(&self) -> &Sha256Digest {
        &self.vnext_implementation_sha256
    }

    pub const fn candidate_sha256(&self) -> &Sha256Digest {
        &self.candidate_sha256
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.schema_version != MIGRATION_SNAPSHOT_SCHEMA_VERSION {
            return Err("unsupported migration snapshot schema version".to_string());
        }
        MigrationFamilyId::new(self.family_id.as_str())?;
        Sha256Digest::parse(self.old_implementation_sha256.as_str())?;
        Sha256Digest::parse(self.vnext_implementation_sha256.as_str())?;
        Sha256Digest::parse(self.candidate_sha256.as_str())?;
        MigrationSnapshotId::parse(self.snapshot_id.as_str())?;
        if self.snapshot_id != MigrationSnapshotId::for_snapshot(self) {
            return Err("migration snapshot ID does not match its bindings".to_string());
        }
        Ok(())
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct MigrationFamilySnapshotWire {
    schema_version: u32,
    snapshot_id: MigrationSnapshotId,
    family_id: MigrationFamilyId,
    disposition: MigrationDisposition,
    old_implementation_sha256: Sha256Digest,
    vnext_implementation_sha256: Sha256Digest,
    candidate_sha256: Sha256Digest,
}

impl<'de> Deserialize<'de> for MigrationFamilySnapshot {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = MigrationFamilySnapshotWire::deserialize(deserializer)?;
        let snapshot = Self {
            schema_version: wire.schema_version,
            snapshot_id: wire.snapshot_id,
            family_id: wire.family_id,
            disposition: wire.disposition,
            old_implementation_sha256: wire.old_implementation_sha256,
            vnext_implementation_sha256: wire.vnext_implementation_sha256,
            candidate_sha256: wire.candidate_sha256,
        };
        snapshot.validate().map_err(serde::de::Error::custom)?;
        Ok(snapshot)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;
    use serde_json::json;

    use super::*;

    fn digest(label: &str) -> Sha256Digest {
        Sha256Digest::for_bytes(label.as_bytes())
    }

    fn snapshot() -> MigrationFamilySnapshot {
        MigrationFamilySnapshot::new(
            MigrationFamilyId::new("proof/exact-lineage").expect("family"),
            MigrationDisposition::Rewrite,
            digest("old-proof-implementation"),
            digest("vnext-proof-implementation"),
            digest("candidate"),
        )
    }

    #[test]
    fn snapshot_has_a_stable_domain_separated_identity() {
        let snapshot = snapshot();
        assert_eq!(snapshot.schema_version(), 1);
        assert_eq!(
            snapshot.snapshot_id().as_str(),
            "migration-snapshot:v1:01bcd09631743f33557083156d7f7ac5adb6b0f467754e72cb484f4eb0c884e5"
        );
        snapshot.validate().expect("valid snapshot");
        let encoded = serde_json::to_vec(&snapshot).expect("serialize snapshot");
        let decoded: MigrationFamilySnapshot =
            serde_json::from_slice(&encoded).expect("deserialize snapshot");
        assert_eq!(decoded, snapshot);
    }

    #[test]
    fn snapshot_deserialization_rejects_every_bound_field_substitution() {
        let canonical = serde_json::to_value(snapshot()).expect("snapshot JSON");
        let substitutions: [(&str, Value); 7] = [
            ("schema_version", json!(2)),
            (
                "snapshot_id",
                json!(format!(
                    "migration-snapshot:v1:{}",
                    digest("other-snapshot").as_str()
                )),
            ),
            ("family_id", json!("proof/other-lineage")),
            ("disposition", json!("project")),
            (
                "old_implementation_sha256",
                json!(digest("other-old").as_str()),
            ),
            (
                "vnext_implementation_sha256",
                json!(digest("other-vnext").as_str()),
            ),
            (
                "candidate_sha256",
                json!(digest("other-candidate").as_str()),
            ),
        ];

        for (field, replacement) in substitutions {
            let mut substituted = canonical.clone();
            substituted[field] = replacement;
            assert!(
                serde_json::from_value::<MigrationFamilySnapshot>(substituted).is_err(),
                "{field} substitution must fail closed"
            );
        }
        let mut unknown = canonical;
        unknown["unknown"] = json!(true);
        assert!(serde_json::from_value::<MigrationFamilySnapshot>(unknown).is_err());
    }

    #[test]
    fn family_and_snapshot_ids_are_strictly_bounded() {
        for invalid in ["", "Upper", "-leading", "contains space", "../escape"] {
            assert!(
                MigrationFamilyId::new(invalid).is_err(),
                "accepted {invalid}"
            );
        }
        assert!(MigrationFamilyId::new("a".repeat(MAX_MIGRATION_FAMILY_ID_BYTES + 1)).is_err());

        for invalid in [
            "migration-snapshot:v1:short".to_string(),
            format!("migration-snapshot:v1:{}", "A".repeat(64)),
            format!("other:v1:{}", "a".repeat(64)),
            format!("migration-snapshot:v1:{}/..", "a".repeat(64)),
        ] {
            assert!(MigrationSnapshotId::parse(invalid).is_err());
        }
    }
}
