use crate::CandidateId;
use crate::CapabilityId;
use crate::CapabilityRequestId;
use crate::ContentHash;
use crate::ContractError;
use crate::FrozenTurnContext;
use crate::PrincipalId;
use crate::Revision;
use crate::RevisionStamp;

/// Exact reference to one capability manifest in a catalog snapshot.
///
/// The capability identity is never used alone across a decision boundary.
/// Revision, manifest hash, and catalog stamp make definition replacement or
/// catalog drift observable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityManifestRef {
    id: CapabilityId,
    revision: Revision,
    manifest_hash: ContentHash,
    catalog: RevisionStamp,
}

impl CapabilityManifestRef {
    /// Creates an exact capability-manifest reference.
    pub fn new(
        id: CapabilityId,
        revision: Revision,
        manifest_hash: ContentHash,
        catalog: RevisionStamp,
    ) -> Self {
        Self {
            id,
            revision,
            manifest_hash,
            catalog,
        }
    }

    /// Returns the capability identity.
    pub fn id(&self) -> &CapabilityId {
        &self.id
    }

    /// Returns the capability-manifest revision.
    pub const fn revision(&self) -> Revision {
        self.revision
    }

    /// Returns the digest of the canonical capability manifest.
    pub fn manifest_hash(&self) -> &ContentHash {
        &self.manifest_hash
    }

    /// Returns the catalog snapshot containing this manifest.
    pub fn catalog(&self) -> &RevisionStamp {
        &self.catalog
    }
}

/// Versioned declaration of a capability offered by a principal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityDescriptor {
    id: CapabilityId,
    revision: Revision,
    manifest_hash: ContentHash,
    catalog: RevisionStamp,
    provider: PrincipalId,
    operation: String,
}

impl CapabilityDescriptor {
    /// Creates a capability manifest descriptor.
    pub fn new(
        id: CapabilityId,
        revision: Revision,
        manifest_hash: ContentHash,
        catalog: RevisionStamp,
        provider: PrincipalId,
        operation: impl Into<String>,
    ) -> Self {
        Self {
            id,
            revision,
            manifest_hash,
            catalog,
            provider,
            operation: operation.into(),
        }
    }

    /// Returns the capability identity.
    pub fn id(&self) -> &CapabilityId {
        &self.id
    }

    /// Returns the capability-manifest revision.
    pub const fn revision(&self) -> Revision {
        self.revision
    }

    /// Returns the digest of the canonical capability manifest.
    pub fn manifest_hash(&self) -> &ContentHash {
        &self.manifest_hash
    }

    /// Returns the catalog snapshot containing this manifest.
    pub fn catalog(&self) -> &RevisionStamp {
        &self.catalog
    }

    /// Returns the principal offering the capability.
    pub fn provider(&self) -> &PrincipalId {
        &self.provider
    }

    /// Returns the provider-defined operation name.
    pub fn operation(&self) -> &str {
        &self.operation
    }

    /// Returns an exact reference to this capability manifest.
    pub fn reference(&self) -> CapabilityManifestRef {
        CapabilityManifestRef::new(
            self.id.clone(),
            self.revision,
            self.manifest_hash.clone(),
            self.catalog.clone(),
        )
    }
}

/// Exact reference to a capability request and its safety-relevant inputs.
///
/// `request_hash` covers the complete canonical request envelope. The manifest,
/// frozen context, and payload hash are also carried directly so safety layers
/// never need to trust a reusable request identity or an indirect lookup.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityRequestRef {
    id: CapabilityRequestId,
    request_hash: ContentHash,
    capability: CapabilityManifestRef,
    requester: PrincipalId,
    context: FrozenTurnContext,
    payload_hash: ContentHash,
}

impl CapabilityRequestRef {
    /// Creates an exact capability-request reference.
    pub fn new(
        id: CapabilityRequestId,
        request_hash: ContentHash,
        capability: CapabilityManifestRef,
        requester: PrincipalId,
        context: FrozenTurnContext,
        payload_hash: ContentHash,
    ) -> Self {
        Self {
            id,
            request_hash,
            capability,
            requester,
            context,
            payload_hash,
        }
    }

    /// Returns the request identity.
    pub fn id(&self) -> &CapabilityRequestId {
        &self.id
    }

    /// Returns the digest of the canonical request envelope.
    pub fn request_hash(&self) -> &ContentHash {
        &self.request_hash
    }

    /// Returns the exact requested capability manifest.
    pub fn capability(&self) -> &CapabilityManifestRef {
        &self.capability
    }

    /// Returns the accountable requesting principal.
    pub fn requester(&self) -> &PrincipalId {
        &self.requester
    }

    /// Returns the frozen turn context of the request.
    pub fn context(&self) -> &FrozenTurnContext {
        &self.context
    }

    /// Returns the digest of the exact request payload.
    pub fn payload_hash(&self) -> &ContentHash {
        &self.payload_hash
    }
}

/// Request to exercise one exact capability manifest with an exact payload.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityRequest {
    id: CapabilityRequestId,
    request_hash: ContentHash,
    capability: CapabilityManifestRef,
    requester: PrincipalId,
    context: FrozenTurnContext,
    payload_hash: ContentHash,
}

impl CapabilityRequest {
    /// Creates a capability request with an opaque, externally stored payload.
    ///
    /// The manifest must belong to the capability catalog frozen in `context`.
    pub fn try_new(
        id: CapabilityRequestId,
        request_hash: ContentHash,
        capability: CapabilityManifestRef,
        requester: PrincipalId,
        context: FrozenTurnContext,
        payload_hash: ContentHash,
    ) -> Result<Self, ContractError> {
        if capability.catalog() != context.capability_catalog() {
            return Err(ContractError::CapabilityCatalogMismatch);
        }

        Ok(Self {
            id,
            request_hash,
            capability,
            requester,
            context,
            payload_hash,
        })
    }

    /// Returns the request identity.
    pub fn id(&self) -> &CapabilityRequestId {
        &self.id
    }

    /// Returns the digest of the complete canonical request envelope.
    pub fn request_hash(&self) -> &ContentHash {
        &self.request_hash
    }

    /// Returns the exact requested capability manifest.
    pub fn capability(&self) -> &CapabilityManifestRef {
        &self.capability
    }

    /// Returns the accountable requesting principal.
    pub fn requester(&self) -> &PrincipalId {
        &self.requester
    }

    /// Returns the complete frozen context consumed by the request.
    pub fn context(&self) -> &FrozenTurnContext {
        &self.context
    }

    /// Returns the digest of the exact request payload.
    pub fn payload_hash(&self) -> &ContentHash {
        &self.payload_hash
    }

    /// Returns an exact reference to this request.
    pub fn reference(&self) -> CapabilityRequestRef {
        CapabilityRequestRef::new(
            self.id.clone(),
            self.request_hash.clone(),
            self.capability.clone(),
            self.requester.clone(),
            self.context.clone(),
            self.payload_hash.clone(),
        )
    }
}

/// Exact reference to one joint action/metacontrol candidate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CandidateRef {
    id: CandidateId,
    revision: Revision,
    content_hash: ContentHash,
    context: FrozenTurnContext,
    action_hash: ContentHash,
    metacontrol_hash: ContentHash,
    payload_set_hash: ContentHash,
}

impl CandidateRef {
    /// Creates an exact candidate reference.
    pub fn new(
        id: CandidateId,
        revision: Revision,
        content_hash: ContentHash,
        context: FrozenTurnContext,
        action_hash: ContentHash,
        metacontrol_hash: ContentHash,
        payload_set_hash: ContentHash,
    ) -> Self {
        Self {
            id,
            revision,
            content_hash,
            context,
            action_hash,
            metacontrol_hash,
            payload_set_hash,
        }
    }

    /// Returns the candidate identity.
    pub fn id(&self) -> &CandidateId {
        &self.id
    }

    /// Returns the candidate revision.
    pub const fn revision(&self) -> Revision {
        self.revision
    }

    /// Returns the digest of the complete canonical candidate.
    pub fn content_hash(&self) -> &ContentHash {
        &self.content_hash
    }

    /// Returns the complete frozen context used to assemble the candidate.
    pub fn context(&self) -> &FrozenTurnContext {
        &self.context
    }

    /// Returns the digest of the proposed external action `a_t`.
    pub fn action_hash(&self) -> &ContentHash {
        &self.action_hash
    }

    /// Returns the digest of the proposed metacontrol `theta_t`.
    pub fn metacontrol_hash(&self) -> &ContentHash {
        &self.metacontrol_hash
    }

    /// Returns the digest of the ordered execution-payload set.
    pub fn payload_set_hash(&self) -> &ContentHash {
        &self.payload_set_hash
    }
}

/// Candidate jointly assembling an action `a_t` and metacontrol `theta_t`.
///
/// Contributor and request order is part of the canonical candidate content.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JointCandidate {
    id: CandidateId,
    revision: Revision,
    content_hash: ContentHash,
    context: FrozenTurnContext,
    action_hash: ContentHash,
    metacontrol_hash: ContentHash,
    payload_set_hash: ContentHash,
    contributors: Vec<PrincipalId>,
    capability_requests: Vec<CapabilityRequestRef>,
}

impl JointCandidate {
    /// Creates a jointly assembled action/metacontrol candidate.
    ///
    /// Every request must use the same frozen context and a capability manifest
    /// from that context's catalog. Its requester must also appear in the
    /// ordered contributor set.
    #[allow(clippy::too_many_arguments)]
    pub fn try_new(
        id: CandidateId,
        revision: Revision,
        content_hash: ContentHash,
        context: FrozenTurnContext,
        action_hash: ContentHash,
        metacontrol_hash: ContentHash,
        payload_set_hash: ContentHash,
        contributors: Vec<PrincipalId>,
        capability_requests: Vec<CapabilityRequestRef>,
    ) -> Result<Self, ContractError> {
        for (request_index, request) in capability_requests.iter().enumerate() {
            if request.context() != &context {
                return Err(ContractError::CandidateRequestContextMismatch { request_index });
            }
            if request.capability().catalog() != context.capability_catalog() {
                return Err(ContractError::CandidateRequestCatalogMismatch { request_index });
            }
            if !contributors
                .iter()
                .any(|contributor| contributor == request.requester())
            {
                return Err(ContractError::CandidateRequestRequesterMissing { request_index });
            }
        }

        Ok(Self {
            id,
            revision,
            content_hash,
            context,
            action_hash,
            metacontrol_hash,
            payload_set_hash,
            contributors,
            capability_requests,
        })
    }

    /// Returns the candidate identity.
    pub fn id(&self) -> &CandidateId {
        &self.id
    }

    /// Returns the candidate revision.
    pub const fn revision(&self) -> Revision {
        self.revision
    }

    /// Returns the digest of the complete canonical candidate.
    pub fn content_hash(&self) -> &ContentHash {
        &self.content_hash
    }

    /// Returns the complete frozen context used to assemble the candidate.
    pub fn context(&self) -> &FrozenTurnContext {
        &self.context
    }

    /// Returns the digest of the proposed external action `a_t`.
    pub fn action_hash(&self) -> &ContentHash {
        &self.action_hash
    }

    /// Returns the digest of the proposed metacontrol `theta_t`.
    pub fn metacontrol_hash(&self) -> &ContentHash {
        &self.metacontrol_hash
    }

    /// Returns the digest of the ordered execution-payload set.
    pub fn payload_set_hash(&self) -> &ContentHash {
        &self.payload_set_hash
    }

    /// Returns the ordered contributing principals.
    pub fn contributors(&self) -> &[PrincipalId] {
        &self.contributors
    }

    /// Returns the ordered exact capability requests in the candidate.
    pub fn capability_requests(&self) -> &[CapabilityRequestRef] {
        &self.capability_requests
    }

    /// Returns an exact reference to this candidate.
    pub fn reference(&self) -> CandidateRef {
        CandidateRef::new(
            self.id.clone(),
            self.revision,
            self.content_hash.clone(),
            self.context.clone(),
            self.action_hash.clone(),
            self.metacontrol_hash.clone(),
            self.payload_set_hash.clone(),
        )
    }
}
