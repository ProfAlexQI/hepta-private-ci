use std::collections::BTreeSet;

use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct SupplementalPayloadSpec {
    pub(super) path: String,
    pub(super) sha256: String,
    pub(super) classification: String,
    pub(super) mutation_allowed: bool,
    pub(super) owner: String,
}

pub(super) fn validate_supplemental_payloads(payloads: &[SupplementalPayloadSpec]) -> Result<()> {
    let mut paths = BTreeSet::new();
    for payload in payloads {
        if !payload
            .path
            .starts_with("scripts/lib/hepta-gate-pair-compat-v1/")
            || !matches!(
                payload
                    .path
                    .rsplit_once('.')
                    .map(|(_, extension)| extension),
                Some("gate" | "report")
            )
        {
            anyhow::bail!(
                "invalid supplemental gate-pair payload path: {}",
                payload.path
            );
        }
        if !paths.insert(payload.path.as_str()) {
            anyhow::bail!(
                "duplicate supplemental gate-pair payload path: {}",
                payload.path
            );
        }
        if payload.sha256.len() != 64
            || !payload
                .sha256
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        {
            anyhow::bail!(
                "invalid supplemental gate-pair payload SHA-256: {}",
                payload.path
            );
        }
        if payload.classification != "explicit_non_pair_compatibility_surface"
            || payload.mutation_allowed
            || payload.owner != "hepta-backend-maintainers"
        {
            anyhow::bail!(
                "invalid supplemental gate-pair payload policy: {}",
                payload.path
            );
        }
    }
    Ok(())
}
