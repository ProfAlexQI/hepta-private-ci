use crate::HeptaError;
use crate::ModelRef;
use crate::PolicyDecision;
use crate::RuntimeKernel;
use crate::ToolDescriptor;
use crate::runtime_kernel::execution_attempt::ExecutorBinding;
use hepta_contracts::CapabilityDescriptor;
use hepta_contracts::CapabilityId;
use hepta_contracts::CapabilityManifestRef;
use hepta_contracts::ContentHash;
use hepta_contracts::FrozenTurnContext;
use hepta_contracts::ObservationId;
use hepta_contracts::ObservationRef;
use hepta_contracts::PrincipalId;
use hepta_contracts::Revision;
use hepta_contracts::RevisionStamp;
use serde_json::Map;
use serde_json::Value;
use serde_json::json;
use sha2::Digest;
use sha2::Sha256;
use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

#[derive(Debug, Default)]
pub(crate) struct ContextRevisionState {
    entries: Vec<ContextRevisionEntry>,
}

#[derive(Debug)]
struct ContextRevisionEntry {
    session_id: String,
    domain: String,
    content_hash: ContentHash,
    revision: u64,
}

impl ContextRevisionState {
    fn stamp(
        &mut self,
        session_id: &str,
        domain: &str,
        content_hash: ContentHash,
    ) -> Result<RevisionStamp, HeptaError> {
        if let Some(entry) = self
            .entries
            .iter_mut()
            .find(|entry| entry.session_id == session_id && entry.domain == domain)
        {
            if entry.content_hash != content_hash {
                entry.revision = entry.revision.checked_add(1).ok_or_else(|| {
                    HeptaError(format!(
                        "safety context revision overflow for {session_id}/{domain}"
                    ))
                })?;
                entry.content_hash = content_hash;
            }
            return Ok(RevisionStamp::new(
                Revision::new(entry.revision),
                entry.content_hash.clone(),
            ));
        }
        self.entries.push(ContextRevisionEntry {
            session_id: session_id.to_string(),
            domain: domain.to_string(),
            content_hash: content_hash.clone(),
            revision: 1,
        });
        Ok(RevisionStamp::new(Revision::new(1), content_hash))
    }

    pub(crate) fn remove_session(&mut self, session_id: &str) {
        self.entries.retain(|entry| entry.session_id != session_id);
    }

    pub(crate) fn clear(&mut self) {
        self.entries.clear();
    }
}

#[derive(Debug, Clone)]
pub(crate) struct FrozenToolInputs {
    pub(crate) canonical_arguments: String,
    pub(crate) payload_hash: ContentHash,
    pub(crate) context: FrozenTurnContext,
    pub(crate) capability: CapabilityDescriptor,
    pub(crate) metacontrol_hash: ContentHash,
}

struct FrozenCapabilityManifest {
    descriptor: ToolDescriptor,
    catalog_hash: ContentHash,
    manifest_hash: ContentHash,
}

pub(crate) fn freeze_tool_inputs(
    runtime: &RuntimeKernel,
    session_id: &str,
    active_model: &ModelRef,
    tool_name: &str,
    arguments_json: &str,
    decision: &PolicyDecision,
) -> Result<FrozenToolInputs, HeptaError> {
    let canonical_arguments = canonical_json(arguments_json)?;
    let payload_hash = framed_hash(
        "hepta.runtime.tool-payload.v1",
        &[
            ("tool_name", tool_name.as_bytes()),
            ("arguments", canonical_arguments.as_bytes()),
        ],
    );
    let observation_hash = framed_hash(
        "hepta.runtime.tool-observation.v1",
        &[
            ("session_id", session_id.as_bytes()),
            ("tool_name", tool_name.as_bytes()),
            ("payload_hash", payload_hash.as_str().as_bytes()),
        ],
    );

    let execution_profile = runtime.execution_profile_for_session(session_id)?;
    let filesystem_scope = runtime.filesystem_scope_for_session(session_id)?;
    let write_path_scope = runtime.write_path_scope_for_session(session_id)?;
    let mut path_gates = runtime.path_capability_gates_for_session(session_id)?;
    path_gates.sort_by(|left, right| {
        (
            left.tool_name.as_str(),
            left.argument_name.as_str(),
            left.id.as_str(),
        )
            .cmp(&(
                right.tool_name.as_str(),
                right.argument_name.as_str(),
                right.id.as_str(),
            ))
    });
    let state_hash = hash_json(
        "hepta.runtime.safety-state.v1",
        json!({
            "active_model": active_model,
            "execution_profile": execution_profile,
            "filesystem_scope": filesystem_scope,
            "path_capability_gates": path_gates,
            "session_id": session_id,
            "write_path_scope": write_path_scope,
        }),
    )?;

    let default_rules = runtime.policy.default_rules();
    let custom_rules = runtime
        .policy
        .custom_rules()
        .map_err(|error| HeptaError(error.0))?;
    let policy_hash = hash_json(
        "hepta.runtime.policy-snapshot.v1",
        json!({
            "custom_rules": custom_rules,
            "default_rules": default_rules,
        }),
    )?;

    let frozen_manifest = freeze_capability_manifest(runtime, tool_name)?;
    let preference_hash = framed_hash(
        "hepta.runtime.preference-context.v1",
        &[("attachment", b"unattached")],
    );
    let metacontrol_hash = hash_json(
        "hepta.runtime.tool-metacontrol.v1",
        serde_json::to_value(decision)
            .map_err(|error| HeptaError(format!("failed to freeze policy decision: {error}")))?,
    )?;

    let mut revisions = runtime
        .context_revision_state
        .lock()
        .map_err(|_| HeptaError("context revision state mutex poisoned".into()))?;
    let observation_stamp = revisions.stamp(
        session_id,
        &format!("observation:{tool_name}"),
        observation_hash.clone(),
    )?;
    let state_stamp = revisions.stamp(session_id, "state", state_hash)?;
    let policy_stamp = revisions.stamp(session_id, "policy", policy_hash)?;
    let catalog_stamp =
        revisions.stamp(session_id, "catalog", frozen_manifest.catalog_hash.clone())?;
    let preference_stamp = revisions.stamp(session_id, "preference:unattached", preference_hash)?;
    drop(revisions);

    let observation = ObservationRef::new(
        ObservationId::new(format!("tool-observation:{session_id}:{tool_name}")),
        observation_stamp.revision(),
        observation_stamp.content_hash().clone(),
    );
    let context = FrozenTurnContext::new(
        observation,
        state_stamp,
        policy_stamp,
        catalog_stamp.clone(),
        preference_stamp,
    );
    let capability = CapabilityDescriptor::new(
        CapabilityId::new(format!("tool:{tool_name}")),
        catalog_stamp.revision(),
        frozen_manifest.manifest_hash,
        catalog_stamp,
        PrincipalId::new(frozen_manifest.descriptor.executor_provider),
        frozen_manifest.descriptor.operation,
    );

    Ok(FrozenToolInputs {
        canonical_arguments,
        payload_hash,
        context,
        capability,
        metacontrol_hash,
    })
}

pub(crate) fn current_capability_descriptor(
    runtime: &RuntimeKernel,
    tool_name: &str,
    expected_catalog: &RevisionStamp,
) -> Result<CapabilityDescriptor, HeptaError> {
    let frozen = freeze_capability_manifest(runtime, tool_name)?;
    if frozen.catalog_hash != *expected_catalog.content_hash() {
        return Err(HeptaError(
            "execution capability catalog changed after candidate freeze".into(),
        ));
    }
    Ok(CapabilityDescriptor::new(
        CapabilityId::new(format!("tool:{tool_name}")),
        expected_catalog.revision(),
        frozen.manifest_hash,
        expected_catalog.clone(),
        PrincipalId::new(frozen.descriptor.executor_provider),
        frozen.descriptor.operation,
    ))
}

fn freeze_capability_manifest(
    runtime: &RuntimeKernel,
    tool_name: &str,
) -> Result<FrozenCapabilityManifest, HeptaError> {
    let mut descriptors = runtime.tools.descriptors();
    descriptors.sort_by(|left, right| left.name.cmp(&right.name));
    let descriptor = descriptors
        .iter()
        .find(|descriptor| descriptor.name == tool_name)
        .cloned()
        .ok_or_else(|| HeptaError(format!("unknown tool: {tool_name}")))?;
    let catalog_hash = hash_json(
        "hepta.runtime.capability-catalog.v1",
        serde_json::to_value(&descriptors)
            .map_err(|error| HeptaError(format!("failed to freeze tool catalog: {error}")))?,
    )?;
    let manifest_hash = hash_json(
        "hepta.runtime.capability-manifest.v1",
        serde_json::to_value(&descriptor)
            .map_err(|error| HeptaError(format!("failed to freeze tool manifest: {error}")))?,
    )?;
    Ok(FrozenCapabilityManifest {
        descriptor,
        catalog_hash,
        manifest_hash,
    })
}

fn canonical_json(input: &str) -> Result<String, HeptaError> {
    let value = serde_json::from_str::<Value>(input)
        .map_err(|error| HeptaError(format!("tool arguments are not valid JSON: {error}")))?;
    serde_json::to_string(&canonical_value(value))
        .map_err(|error| HeptaError(format!("failed to canonicalize tool arguments: {error}")))
}

fn canonical_value(value: Value) -> Value {
    match value {
        Value::Array(values) => Value::Array(values.into_iter().map(canonical_value).collect()),
        Value::Object(values) => {
            let mut entries = values.into_iter().collect::<Vec<_>>();
            entries.sort_by(|left, right| left.0.cmp(&right.0));
            let mut canonical = Map::new();
            for (key, value) in entries {
                canonical.insert(key, canonical_value(value));
            }
            Value::Object(canonical)
        }
        scalar => scalar,
    }
}

fn hash_json(domain: &str, value: Value) -> Result<ContentHash, HeptaError> {
    let canonical = serde_json::to_string(&canonical_value(value))
        .map_err(|error| HeptaError(format!("failed to hash safety context: {error}")))?;
    Ok(framed_hash(
        domain,
        &[("canonical_json", canonical.as_bytes())],
    ))
}

pub(crate) fn framed_hash(domain: &str, fields: &[(&str, &[u8])]) -> ContentHash {
    let mut hash = Sha256::new();
    update_frame(&mut hash, "domain", domain.as_bytes());
    for (name, value) in fields {
        update_frame(&mut hash, name, value);
    }
    ContentHash::new(format!("sha256:{:x}", hash.finalize()))
}

pub(crate) fn authorization_scope_hash(
    attempt_id: &str,
    tool_name: &str,
    payload_hash: &ContentHash,
    capability: &CapabilityManifestRef,
    executor: &ExecutorBinding,
    context: &FrozenTurnContext,
) -> ContentHash {
    let capability_revision = capability.revision().get().to_string();
    let catalog_revision = capability.catalog().revision().get().to_string();
    framed_hash(
        "hepta.runtime.authorization-scope.v2",
        &[
            ("attempt_id", attempt_id.as_bytes()),
            ("tool_name", tool_name.as_bytes()),
            ("payload_hash", payload_hash.as_str().as_bytes()),
            ("capability_id", capability.id().as_str().as_bytes()),
            ("capability_revision", capability_revision.as_bytes()),
            (
                "capability_manifest_hash",
                capability.manifest_hash().as_str().as_bytes(),
            ),
            ("catalog_revision", catalog_revision.as_bytes()),
            (
                "executor_principal",
                executor.principal().as_str().as_bytes(),
            ),
            ("executor_provider", executor.provider().as_bytes()),
            ("executor_operation", executor.operation().as_bytes()),
            (
                "executor_manifest_hash",
                executor.manifest_hash().as_str().as_bytes(),
            ),
            ("state", context.state().content_hash().as_str().as_bytes()),
            (
                "policy",
                context.policy().content_hash().as_str().as_bytes(),
            ),
            (
                "catalog",
                context
                    .capability_catalog()
                    .content_hash()
                    .as_str()
                    .as_bytes(),
            ),
        ],
    )
}

pub(crate) fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
        .min(u128::from(u64::MAX)) as u64
}

pub(crate) fn add_ttl(now_ms: u64, ttl: Duration) -> u64 {
    now_ms.saturating_add(ttl.as_millis().min(u128::from(u64::MAX)) as u64)
}

fn update_frame(hash: &mut Sha256, name: &str, value: &[u8]) {
    hash.update((name.len() as u64).to_be_bytes());
    hash.update(name.as_bytes());
    hash.update((value.len() as u64).to_be_bytes());
    hash.update(value);
}
