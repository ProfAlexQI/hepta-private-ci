use crate::trust::{SignedDigest, TrustRole, TrustStore, VerifiedSignatureReceipt, verify_signed_digest};
use crate::{ContractError, Digest32, framed_digest, validate_git_oid, validate_id};

pub const QUALIFICATION_SCHEMA: &str = "hepta.intelligence.p1_1c3.executable_qualification.v1";
pub const MAX_QUALIFICATION_STEPS: u32 = 512;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QualificationGateSet {
    pub source_binding: bool,
    pub source_gate: bool,
    pub rustfmt: bool,
    pub tests: bool,
    pub check: bool,
    pub clippy: bool,
    pub receipt_reproducibility: bool,
    pub receipt_redaction: bool,
    pub clean_tree: bool,
}

impl QualificationGateSet {
    #[must_use]
    pub const fn all_pass(&self) -> bool {
        self.source_binding
            && self.source_gate
            && self.rustfmt
            && self.tests
            && self.check
            && self.clippy
            && self.receipt_reproducibility
            && self.receipt_redaction
            && self.clean_tree
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QualificationEvidence {
    pub schema: String,
    pub repository: String,
    pub source_commit: String,
    pub source_tree: String,
    pub workflow_name: String,
    pub workflow_run_id: u64,
    pub workflow_run_attempt: u32,
    pub job_id: u64,
    pub runner_id: u64,
    pub runner_name: String,
    pub runner_os: String,
    pub runner_arch: String,
    pub step_count: u32,
    pub commands_executed: bool,
    pub conclusion: String,
    pub toolchain: String,
    pub tests_passed: u32,
    pub artifact_id: u64,
    pub artifact_sha256: Digest32,
    pub gates: QualificationGateSet,
    pub signed: SignedDigest,
}

impl QualificationEvidence {
    pub fn validate_shape(&self) -> Result<(), ContractError> {
        if self.schema != QUALIFICATION_SCHEMA {
            return Err(ContractError::Invalid(
                "qualification evidence schema mismatch".to_string(),
            ));
        }
        validate_id(&self.repository, "qualification repository")?;
        validate_git_oid(&self.source_commit, "qualification source commit")?;
        validate_git_oid(&self.source_tree, "qualification source tree")?;
        validate_id(&self.workflow_name, "qualification workflow name")?;
        validate_id(&self.runner_name, "qualification runner name")?;
        validate_id(&self.runner_os, "qualification runner OS")?;
        validate_id(&self.runner_arch, "qualification runner architecture")?;
        validate_id(&self.toolchain, "qualification toolchain")?;
        if self.workflow_run_id == 0
            || self.workflow_run_attempt == 0
            || self.job_id == 0
            || self.runner_id == 0
            || self.artifact_id == 0
            || self.step_count == 0
            || self.step_count > MAX_QUALIFICATION_STEPS
        {
            return Err(ContractError::Invalid(
                "qualification run, job, runner, artifact and step identities must be bounded and non-zero"
                    .to_string(),
            ));
        }
        if self.conclusion != "success" {
            return Err(ContractError::Invalid(
                "qualification conclusion must be success".to_string(),
            ));
        }
        if !self.commands_executed || !self.gates.all_pass() {
            return Err(ContractError::Invalid(
                "qualification evidence does not contain a complete executable gate pass"
                    .to_string(),
            ));
        }
        self.signed.validate()?;
        let payload = self.payload_sha256();
        if self.signed.payload_sha256 != payload {
            return Err(ContractError::Corrupt(
                "qualification signature payload does not match evidence".to_string(),
            ));
        }
        Ok(())
    }

    #[must_use]
    pub fn payload_sha256(&self) -> Digest32 {
        let gate_bytes = [
            u8::from(self.gates.source_binding),
            u8::from(self.gates.source_gate),
            u8::from(self.gates.rustfmt),
            u8::from(self.gates.tests),
            u8::from(self.gates.check),
            u8::from(self.gates.clippy),
            u8::from(self.gates.receipt_reproducibility),
            u8::from(self.gates.receipt_redaction),
            u8::from(self.gates.clean_tree),
        ];
        framed_digest(
            b"hepta:intelligence:p1.1c3:qualification-evidence:v1",
            &[
                self.schema.as_bytes(),
                self.repository.as_bytes(),
                self.source_commit.as_bytes(),
                self.source_tree.as_bytes(),
                self.workflow_name.as_bytes(),
                &self.workflow_run_id.to_be_bytes(),
                &self.workflow_run_attempt.to_be_bytes(),
                &self.job_id.to_be_bytes(),
                &self.runner_id.to_be_bytes(),
                self.runner_name.as_bytes(),
                self.runner_os.as_bytes(),
                self.runner_arch.as_bytes(),
                &self.step_count.to_be_bytes(),
                &[u8::from(self.commands_executed)],
                self.conclusion.as_bytes(),
                self.toolchain.as_bytes(),
                &self.tests_passed.to_be_bytes(),
                &self.artifact_id.to_be_bytes(),
                self.artifact_sha256.as_bytes(),
                &gate_bytes,
            ],
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QualificationPolicy {
    pub policy_id: String,
    pub expected_repository: String,
    pub expected_source_commit: String,
    pub expected_source_tree: String,
    pub expected_workflow_name: String,
    pub expected_toolchain: String,
    pub minimum_tests_passed: u32,
    pub minimum_step_count: u32,
    pub expected_trust_store_sha256: Digest32,
    pub require_external_signer: bool,
    pub policy_sha256: Digest32,
}

impl QualificationPolicy {
    pub fn validate(&self) -> Result<(), ContractError> {
        validate_id(&self.policy_id, "qualification policy ID")?;
        validate_id(&self.expected_repository, "qualification policy repository")?;
        validate_git_oid(&self.expected_source_commit, "qualification policy source commit")?;
        validate_git_oid(&self.expected_source_tree, "qualification policy source tree")?;
        validate_id(&self.expected_workflow_name, "qualification policy workflow")?;
        validate_id(&self.expected_toolchain, "qualification policy toolchain")?;
        if self.minimum_tests_passed == 0
            || self.minimum_step_count == 0
            || self.minimum_step_count > MAX_QUALIFICATION_STEPS
        {
            return Err(ContractError::Invalid(
                "qualification policy test and step minima must be bounded and positive"
                    .to_string(),
            ));
        }
        if self.policy_sha256 != qualification_policy_digest(self) {
            return Err(ContractError::Corrupt(
                "qualification policy digest mismatch".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedQualificationReceipt {
    pub policy_id: String,
    pub repository: String,
    pub source_commit: String,
    pub source_tree: String,
    pub workflow_name: String,
    pub workflow_run_id: u64,
    pub job_id: u64,
    pub runner_id: u64,
    pub artifact_sha256: Digest32,
    pub evidence_payload_sha256: Digest32,
    pub signature_receipt_sha256: Digest32,
    pub trust_store_sha256: Digest32,
    pub external_attested: bool,
    pub receipt_sha256: Digest32,
    verified: bool,
}

impl VerifiedQualificationReceipt {
    pub fn validate(&self) -> Result<(), ContractError> {
        if !self.verified {
            return Err(ContractError::Corrupt(
                "qualification receipt is not verified".to_string(),
            ));
        }
        validate_git_oid(&self.source_commit, "verified source commit")?;
        validate_git_oid(&self.source_tree, "verified source tree")?;
        if self.receipt_sha256 != verified_qualification_receipt_digest(self) {
            return Err(ContractError::Corrupt(
                "verified qualification receipt digest mismatch".to_string(),
            ));
        }
        Ok(())
    }

    #[must_use]
    pub const fn is_verified(&self) -> bool {
        self.verified
    }
}

pub fn verify_qualification(
    evidence: &QualificationEvidence,
    policy: &QualificationPolicy,
    trust_store: &TrustStore,
    now_unix_seconds: u64,
) -> Result<VerifiedQualificationReceipt, ContractError> {
    evidence.validate_shape()?;
    policy.validate()?;
    trust_store.validate()?;
    if trust_store.store_sha256 != policy.expected_trust_store_sha256 {
        return Err(ContractError::Invalid(
            "qualification trust store does not match policy".to_string(),
        ));
    }
    if evidence.repository != policy.expected_repository
        || evidence.source_commit != policy.expected_source_commit
        || evidence.source_tree != policy.expected_source_tree
        || evidence.workflow_name != policy.expected_workflow_name
        || evidence.toolchain != policy.expected_toolchain
    {
        return Err(ContractError::Invalid(
            "qualification evidence does not match the exact policy binding".to_string(),
        ));
    }
    if evidence.tests_passed < policy.minimum_tests_passed
        || evidence.step_count < policy.minimum_step_count
    {
        return Err(ContractError::Invalid(
            "qualification evidence is below the policy test or step floor".to_string(),
        ));
    }
    let signature = verify_signed_digest(
        trust_store,
        &evidence.signed,
        TrustRole::CiQualification,
        None,
        now_unix_seconds,
        policy.require_external_signer,
    )?;
    signature.validate()?;
    let external_attested = signature.domain == crate::TrustDomain::ExternalAttested;
    let mut receipt = VerifiedQualificationReceipt {
        policy_id: policy.policy_id.clone(),
        repository: evidence.repository.clone(),
        source_commit: evidence.source_commit.clone(),
        source_tree: evidence.source_tree.clone(),
        workflow_name: evidence.workflow_name.clone(),
        workflow_run_id: evidence.workflow_run_id,
        job_id: evidence.job_id,
        runner_id: evidence.runner_id,
        artifact_sha256: evidence.artifact_sha256,
        evidence_payload_sha256: evidence.payload_sha256(),
        signature_receipt_sha256: signature.receipt_sha256,
        trust_store_sha256: trust_store.store_sha256,
        external_attested,
        receipt_sha256: Digest32::for_bytes(b"uncomputed"),
        verified: true,
    };
    receipt.receipt_sha256 = verified_qualification_receipt_digest(&receipt);
    receipt.validate()?;
    Ok(receipt)
}

#[must_use]
pub fn qualification_policy_digest(policy: &QualificationPolicy) -> Digest32 {
    framed_digest(
        b"hepta:intelligence:p1.1c3:qualification-policy:v1",
        &[
            policy.policy_id.as_bytes(),
            policy.expected_repository.as_bytes(),
            policy.expected_source_commit.as_bytes(),
            policy.expected_source_tree.as_bytes(),
            policy.expected_workflow_name.as_bytes(),
            policy.expected_toolchain.as_bytes(),
            &policy.minimum_tests_passed.to_be_bytes(),
            &policy.minimum_step_count.to_be_bytes(),
            policy.expected_trust_store_sha256.as_bytes(),
            &[u8::from(policy.require_external_signer)],
        ],
    )
}

fn verified_qualification_receipt_digest(receipt: &VerifiedQualificationReceipt) -> Digest32 {
    framed_digest(
        b"hepta:intelligence:p1.1c3:verified-qualification:v1",
        &[
            receipt.policy_id.as_bytes(),
            receipt.repository.as_bytes(),
            receipt.source_commit.as_bytes(),
            receipt.source_tree.as_bytes(),
            receipt.workflow_name.as_bytes(),
            &receipt.workflow_run_id.to_be_bytes(),
            &receipt.job_id.to_be_bytes(),
            &receipt.runner_id.to_be_bytes(),
            receipt.artifact_sha256.as_bytes(),
            receipt.evidence_payload_sha256.as_bytes(),
            receipt.signature_receipt_sha256.as_bytes(),
            receipt.trust_store_sha256.as_bytes(),
            &[u8::from(receipt.external_attested)],
            &[u8::from(receipt.verified)],
        ],
    )
}
