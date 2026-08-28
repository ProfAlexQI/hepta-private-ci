#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub(crate) struct IntelligenceMutationBinding {
    pub(crate) operation_id: Sha256Digest,
    pub(crate) lease_id: String,
    pub(crate) lease_epoch: u64,
    pub(crate) expected_revision: Option<u64>,
    pub(crate) starting_projection_generation: u64,
    pub(crate) causal_root_sha256: Sha256Digest,
}

impl IntelligenceMutationBinding {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn derive(
        owner_agent_id: &str,
        scope_key: &str,
        operation_kind: &str,
        idempotency_key: &str,
        lease_id: &str,
        lease_epoch: u64,
        expected_revision: Option<u64>,
        starting_projection_generation: u64,
    ) -> Result<Self, IntelligenceMutationStateError> {
        validate_id(owner_agent_id, "owner agent id")?;
        validate_id(scope_key, "scope key")?;
        validate_id(operation_kind, "operation kind")?;
        validate_id(idempotency_key, "idempotency key")?;
        validate_id(lease_id, "lease id")?;
        validate_lease_and_revision(lease_epoch, expected_revision)?;

        let mut operation_hasher = Sha256::new();
        frame_part(&mut operation_hasher, OPERATION_ID_DOMAIN);
        frame_part(&mut operation_hasher, owner_agent_id.as_bytes());
        frame_part(&mut operation_hasher, scope_key.as_bytes());
        frame_part(&mut operation_hasher, operation_kind.as_bytes());
        frame_part(&mut operation_hasher, idempotency_key.as_bytes());
        let operation_id = Sha256Digest::from_sha256_output(operation_hasher.finalize());

        let mut causal_hasher = Sha256::new();
        frame_part(&mut causal_hasher, CAUSAL_ROOT_DOMAIN);
        frame_part(&mut causal_hasher, operation_id.as_str().as_bytes());
        frame_part(&mut causal_hasher, lease_id.as_bytes());
        frame_part(&mut causal_hasher, &lease_epoch.to_be_bytes());
        frame_optional_u64(&mut causal_hasher, expected_revision);
        frame_part(
            &mut causal_hasher,
            &starting_projection_generation.to_be_bytes(),
        );
        let causal_root_sha256 =
            Sha256Digest::from_sha256_output(causal_hasher.finalize());

        let binding = Self {
            operation_id,
            lease_id: lease_id.to_string(),
            lease_epoch,
            expected_revision,
            starting_projection_generation,
            causal_root_sha256,
        };
        binding.validate()?;
        Ok(binding)
    }

    pub(crate) fn validate(&self) -> Result<(), IntelligenceMutationStateError> {
        validate_digest(&self.operation_id, "operation id")?;
        validate_id(&self.lease_id, "lease id")?;
        validate_lease_and_revision(self.lease_epoch, self.expected_revision)?;
        validate_digest(&self.causal_root_sha256, "causal root digest")
    }
}
