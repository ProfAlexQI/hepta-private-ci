//! HMAC-authenticated native HTTP preference ingress.
//!
//! Challenge planning and commit use separate HMAC domains. The gateway
//! is admitted only on strict loopback Host/Origin/CSRF transport. Clear HTTP
//! still makes no confidentiality claim beyond the local host boundary.

use std::env;
use std::ffi::OsString;
use std::fmt;
use std::future::Future;
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Context;
use anyhow::Result;
use hepta_contracts::CapabilityId;
use hepta_contracts::CapabilityManifestRef;
use hepta_contracts::ContentHash;
use hepta_contracts::PreferenceEvidenceId;
use hepta_contracts::PreferenceId;
use hepta_contracts::PreferenceState;
use hepta_contracts::PreferenceTransitionId;
use hepta_contracts::PrincipalId;
use hepta_contracts::ReceiptId;
use hepta_contracts::ReceiptRef;
use hepta_contracts::Revision;
use hepta_contracts::RevisionStamp;
use hepta_intelligence::DurableHmacTrustedPreferenceIngress;
use hepta_intelligence::ExplicitPreferenceFeedbackChallengeInputParts;
use hepta_intelligence::ExplicitPreferenceFeedbackInput;
use hepta_intelligence::ExplicitPreferenceFeedbackInputParts;
use hepta_intelligence::ExplicitPreferenceSignal;
use hepta_intelligence::ExplicitPreferenceTarget;
use hepta_intelligence::PreferenceIngressAuthenticationKey;
use hepta_intelligence::PreferenceIngressCommitError;
use hepta_intelligence::PreferenceIngressProof;
#[cfg(test)]
use hepta_intelligence::TrustedExplicitPreferenceReducer;
use hepta_intelligence::explicit_preference_feedback_challenge_hash;
use hepta_intelligence::sign_preference_ingress_challenge;
use hepta_memory::DurableIntegrityKey;
use hepta_memory::PreferenceAuthorityError;
use hepta_memory::PreferenceCasError;
use hepta_memory::PreferenceFeedbackSourceRef;
use serde::Deserialize;
use serde::Serialize;
use sha2::Digest;
use sha2::Sha256;

use crate::preference_attachment::PreferenceAttachmentCandidate;
use crate::preference_attachment::PreferenceAttachmentStore;
use crate::secure_key_file::read_private_key;

#[cfg(all(test, unix))]
use crate::secure_key_file::PRIVATE_FILE_MODE;
#[cfg(all(test, unix))]
use std::path::Path;

pub(crate) const PREFERENCE_CHALLENGE_ENDPOINT: &str = "/api/v2/preferences/challenge";
pub(crate) const PREFERENCE_COMMIT_ENDPOINT: &str = "/api/v2/preferences/commit";

const PREFERENCE_DATABASE_ENV: &str = "HEPTA_PREFERENCE_DATABASE";
const PREFERENCE_INTEGRITY_KEY_FILE_ENV: &str = "HEPTA_PREFERENCE_INTEGRITY_KEY_FILE";
const PREFERENCE_AUTH_KEY_FILE_ENV: &str = "HEPTA_PREFERENCE_INGRESS_AUTH_KEY_FILE";
const PREFERENCE_STORE_MODE_ENV: &str = "HEPTA_PREFERENCE_STORE_MODE";
const OPEN_EXISTING_MODE: &str = "open-existing";
const BOOTSTRAP_NEW_MODE: &str = "bootstrap-new";
const SOURCE_IDENTITY: &str = "source:hepta-native-http-preference-ingress";
const SOURCE_REVISION: u64 = 2;
const SOURCE_DESCRIPTOR: &[u8] =
    b"hepta.native-http.preference-ingress.v2|challenge=/api/v2/preferences/challenge|commit=/api/v2/preferences/commit|challenge-plan-proof=hmac-sha256-domain-v1|commit-proof=memory-challenge-hmac-sha256-v1|transport=strict-loopback-host-origin-csrf-json|transport-confidentiality=local-host-only";
const RESPONSE_SCHEMA: &str = "hepta.native-http.preference-ingress.v2";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PreferenceStoreMode {
    OpenExisting,
    BootstrapNew,
}

impl PreferenceStoreMode {
    fn parse(value: &str) -> Result<Self> {
        match value {
            OPEN_EXISTING_MODE => Ok(Self::OpenExisting),
            BOOTSTRAP_NEW_MODE => Ok(Self::BootstrapNew),
            _ => anyhow::bail!(
                "{PREFERENCE_STORE_MODE_ENV} must be {OPEN_EXISTING_MODE} or {BOOTSTRAP_NEW_MODE}"
            ),
        }
    }

    const fn as_str(self) -> &'static str {
        match self {
            Self::OpenExisting => OPEN_EXISTING_MODE,
            Self::BootstrapNew => BOOTSTRAP_NEW_MODE,
        }
    }
}

#[derive(Debug)]
pub(crate) struct NativePreferenceIngressConfig {
    pub(crate) database: PathBuf,
    pub(crate) integrity_key_file: PathBuf,
    pub(crate) authentication_key_file: PathBuf,
    pub(crate) mode: PreferenceStoreMode,
}

impl NativePreferenceIngressConfig {
    pub(crate) fn from_env() -> Result<Self> {
        Self::from_lookup(|name| env::var_os(name))
    }

    fn from_lookup(mut lookup: impl FnMut(&str) -> Option<OsString>) -> Result<Self> {
        Ok(Self {
            database: required_absolute_value(
                PREFERENCE_DATABASE_ENV,
                lookup(PREFERENCE_DATABASE_ENV),
            )?,
            integrity_key_file: required_absolute_value(
                PREFERENCE_INTEGRITY_KEY_FILE_ENV,
                lookup(PREFERENCE_INTEGRITY_KEY_FILE_ENV),
            )?,
            authentication_key_file: required_absolute_value(
                PREFERENCE_AUTH_KEY_FILE_ENV,
                lookup(PREFERENCE_AUTH_KEY_FILE_ENV),
            )?,
            mode: lookup(PREFERENCE_STORE_MODE_ENV)
                .map(|value| value.to_string_lossy().into_owned())
                .map(|value| PreferenceStoreMode::parse(value.trim()))
                .transpose()?
                .unwrap_or(PreferenceStoreMode::OpenExisting),
        })
    }

    #[cfg(all(test, unix))]
    pub(crate) fn bootstrap_for_test(root: &Path) -> Result<Self> {
        use std::fs;
        use std::os::unix::fs::PermissionsExt;

        let integrity_key_file = root.join("preference-integrity.key");
        let authentication_key_file = root.join("preference-authentication.key");
        fs::write(
            &integrity_key_file,
            b"202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c3d3e3f",
        )?;
        fs::write(
            &authentication_key_file,
            b"404142434445464748494a4b4c4d4e4f505152535455565758595a5b5c5d5e5f",
        )?;
        fs::set_permissions(
            &integrity_key_file,
            fs::Permissions::from_mode(PRIVATE_FILE_MODE),
        )?;
        fs::set_permissions(
            &authentication_key_file,
            fs::Permissions::from_mode(PRIVATE_FILE_MODE),
        )?;
        Ok(Self {
            database: root.join("preferences.sqlite3"),
            integrity_key_file,
            authentication_key_file,
            mode: PreferenceStoreMode::BootstrapNew,
        })
    }
}

pub(crate) struct NativePreferenceIngress {
    authority: DurableHmacTrustedPreferenceIngress,
    prevalidation_key: PreferenceIngressAuthenticationKey,
    attachment: PreferenceAttachmentStore,
    executor: PreferenceAsyncExecutor,
    mode: PreferenceStoreMode,
}

pub(crate) struct PreparedNativePreferenceIngress {
    config: NativePreferenceIngressConfig,
    integrity_key: DurableIntegrityKey,
    authentication_key: PreferenceIngressAuthenticationKey,
    prevalidation_key: PreferenceIngressAuthenticationKey,
    attachment: PreferenceAttachmentStore,
    source: PreferenceFeedbackSourceRef,
    executor: PreferenceAsyncExecutor,
}

impl fmt::Debug for NativePreferenceIngress {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("NativePreferenceIngress")
            .field("source", self.authority.source_binding())
            .field("mode", &self.mode.as_str())
            .field("authentication_key", &"[REDACTED]")
            .field("integrity_key", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}

impl NativePreferenceIngress {
    pub(crate) fn prepare(
        config: NativePreferenceIngressConfig,
    ) -> Result<PreparedNativePreferenceIngress> {
        let integrity_key_bytes = read_private_key(
            &config.integrity_key_file,
            PREFERENCE_INTEGRITY_KEY_FILE_ENV,
            "preference integrity",
        )?;
        let integrity_key = DurableIntegrityKey::from_bytes(*integrity_key_bytes);
        let authentication_key_bytes = read_private_key(
            &config.authentication_key_file,
            PREFERENCE_AUTH_KEY_FILE_ENV,
            "preference ingress authentication",
        )?;
        let attachment =
            PreferenceAttachmentStore::for_database(&config.database, *authentication_key_bytes)?;
        let authentication_key =
            PreferenceIngressAuthenticationKey::from_bytes(*authentication_key_bytes);
        let prevalidation_key =
            PreferenceIngressAuthenticationKey::from_bytes(*authentication_key_bytes);
        let source = native_http_preference_source()?;
        let executor = PreferenceAsyncExecutor::new()?;
        Ok(PreparedNativePreferenceIngress {
            config,
            integrity_key,
            authentication_key,
            prevalidation_key,
            attachment,
            source,
            executor,
        })
    }

    pub(crate) fn open(prepared: PreparedNativePreferenceIngress) -> Result<Self> {
        let PreparedNativePreferenceIngress {
            config,
            integrity_key,
            authentication_key,
            prevalidation_key,
            attachment,
            source,
            executor,
        } = prepared;
        let authority = executor
            .block_on(async {
                match config.mode {
                    PreferenceStoreMode::OpenExisting => {
                        DurableHmacTrustedPreferenceIngress::open_existing(
                            &config.database,
                            integrity_key,
                            authentication_key,
                            source,
                        )
                        .await
                    }
                    PreferenceStoreMode::BootstrapNew => {
                        DurableHmacTrustedPreferenceIngress::bootstrap_new(
                            &config.database,
                            integrity_key,
                            authentication_key,
                            source,
                        )
                        .await
                    }
                }
            })?
            .with_context(|| {
                format!(
                    "initialize {} keyed trusted preference ingress",
                    config.mode.as_str()
                )
            })?;
        Ok(Self {
            authority,
            prevalidation_key,
            attachment,
            executor,
            mode: config.mode,
        })
    }

    pub(crate) const fn mode(&self) -> &'static str {
        self.mode.as_str()
    }

    pub(crate) fn validate_readiness(&self) -> Result<()> {
        self.authority
            .reducer_binding()
            .map(|_| ())
            .context("validate trusted preference ingress reducer binding")
    }

    pub(crate) fn hydrate_runtime_context(
        &self,
        expected_session_binding_hash: &str,
    ) -> Result<Option<RevisionStamp>> {
        self.executor.block_on(
            self.attachment
                .hydrate(&self.authority, expected_session_binding_hash),
        )?
    }

    pub(crate) fn monotonic_state(&self) -> Result<hepta_memory::DurableMonotonicState> {
        self.executor
            .block_on(self.authority.monotonic_state())?
            .context("read keyed preference monotonic state")
    }

    pub(crate) fn route_http(
        &self,
        method: &str,
        path: &str,
        body: Option<&str>,
        request_binding_hash: &str,
        expected_session_binding_hash: &str,
    ) -> Option<PreferenceHttpResponse> {
        let result = match (method, path) {
            ("POST", PREFERENCE_CHALLENGE_ENDPOINT) => Some(self.handle_challenge(
                body,
                request_binding_hash,
                expected_session_binding_hash,
            )),
            ("POST", PREFERENCE_COMMIT_ENDPOINT) => {
                Some(self.handle_commit(body, request_binding_hash, expected_session_binding_hash))
            }
            ("GET", PREFERENCE_CHALLENGE_ENDPOINT | PREFERENCE_COMMIT_ENDPOINT) => {
                Some(PreferenceHttpResponse::error(
                    "405 Method Not Allowed",
                    "trusted_preference_ingress.method_not_allowed",
                ))
            }
            _ => None,
        };
        result
    }

    pub(crate) fn prevalidate_commit_http(
        &self,
        body: Option<&str>,
        expected_session_binding_hash: &str,
    ) -> Option<PreferenceHttpResponse> {
        let request = match parse_body::<PreferenceCommitHttpRequest>(body) {
            Ok(request) => request,
            Err(code) => {
                return Some(PreferenceHttpResponse::error("400 Bad Request", code));
            }
        };
        if request.commit.request.session_binding_hash != expected_session_binding_hash {
            return Some(PreferenceHttpResponse::error(
                "403 Forbidden",
                "trusted_preference_ingress.runtime_session_binding_mismatch",
            ));
        }
        if request.commit.source != SourceBinding::from_ref(self.authority.source_binding()) {
            return Some(PreferenceHttpResponse::error(
                "403 Forbidden",
                "trusted_preference_ingress.source_binding_mismatch",
            ));
        }
        let reducer = match self.authority.reducer_binding() {
            Ok(reducer) => reducer,
            Err(error) => return Some(authority_error_response(&error)),
        };
        if request.commit.reducer != ReducerBinding::from_ref(&reducer) {
            return Some(PreferenceHttpResponse::error(
                "403 Forbidden",
                "trusted_preference_ingress.reducer_binding_mismatch",
            ));
        }
        let proof = match PreferenceIngressProof::from_hex(&request.proof) {
            Ok(proof) => proof,
            Err(_) => {
                return Some(PreferenceHttpResponse::error(
                    "403 Forbidden",
                    "trusted_preference_ingress.proof_encoding_invalid",
                ));
            }
        };
        let input = match request.commit.clone().try_into_input() {
            Ok(input) => input,
            Err(code) => {
                return Some(PreferenceHttpResponse::error(
                    "422 Unprocessable Entity",
                    code,
                ));
            }
        };
        let challenge_hash = match explicit_preference_feedback_challenge_hash(
            &input,
            self.authority.source_binding().clone(),
        ) {
            Ok(hash) => hash,
            Err(error) => return Some(authority_error_response(&error)),
        };
        if challenge_hash.as_str() != request.commit.challenge_hash {
            return Some(PreferenceHttpResponse::error(
                "403 Forbidden",
                "trusted_preference_ingress.challenge_binding_mismatch",
            ));
        }
        let expected_proof =
            match sign_preference_ingress_challenge(&self.prevalidation_key, &challenge_hash) {
                Ok(proof) => proof,
                Err(_) => {
                    return Some(PreferenceHttpResponse::error(
                        "503 Service Unavailable",
                        "trusted_preference_ingress.authority_unavailable",
                    ));
                }
            };
        if proof != expected_proof {
            return Some(PreferenceHttpResponse::error(
                "403 Forbidden",
                "trusted_preference_ingress.authentication_denied",
            ));
        }
        None
    }

    fn handle_challenge(
        &self,
        body: Option<&str>,
        request_binding_hash: &str,
        expected_session_binding_hash: &str,
    ) -> PreferenceHttpResponse {
        let envelope = match parse_body::<PreferenceChallengeHttpEnvelope>(body) {
            Ok(envelope) => envelope,
            Err(code) => return PreferenceHttpResponse::error("400 Bad Request", code),
        };
        let proof = match PreferenceIngressProof::from_hex(&envelope.proof) {
            Ok(proof) => proof,
            Err(_) => {
                return PreferenceHttpResponse::error(
                    "403 Forbidden",
                    "trusted_preference_ingress.plan_proof_encoding_invalid",
                );
            }
        };
        let request = envelope.request;
        if request.session_binding_hash != expected_session_binding_hash {
            return PreferenceHttpResponse::error(
                "403 Forbidden",
                "trusted_preference_ingress.runtime_session_binding_mismatch",
            );
        }
        let parts = match request.clone().try_into_parts() {
            Ok(parts) => parts,
            Err(code) => return PreferenceHttpResponse::error("422 Unprocessable Entity", code),
        };
        let plan = match self
            .executor
            .block_on(self.authority.plan_challenge(parts, proof))
        {
            Ok(Ok(plan)) => plan,
            Ok(Err(error)) => return authority_error_response(&error),
            Err(_) => {
                return PreferenceHttpResponse::error(
                    "503 Service Unavailable",
                    "trusted_preference_ingress.executor_unavailable",
                );
            }
        };
        let reducer = match self.authority.reducer_binding() {
            Ok(reducer) => ReducerBinding::from_ref(&reducer),
            Err(error) => return authority_error_response(&error),
        };
        let source = SourceBinding::from_ref(self.authority.source_binding());
        let expected_previous =
            StateBinding::from_state(plan.input().request().expected_previous());
        let prepared = PreferencePreparedCommit {
            request,
            source,
            reducer,
            expected_previous,
            challenge_hash: plan.challenge_hash().as_str().to_owned(),
        };
        PreferenceHttpResponse::json(
            "200 OK",
            &PreferenceChallengeHttpResponse {
                schema: RESPONSE_SCHEMA,
                authority: "hepta.intelligence.authenticated-preference-plan",
                commit_authority: "hepta.memory.authenticated-preference-cas",
                challenge_authenticated: true,
                network_binding_policy: "strict_loopback_host_origin_csrf_json",
                transport_confidentiality_claimed: false,
                runtime_preflight: "plan_only_quarantine",
                runtime_effect_authority_claimed: false,
                request_binding_hash,
                commit: prepared,
            },
        )
    }

    fn handle_commit(
        &self,
        body: Option<&str>,
        request_binding_hash: &str,
        expected_session_binding_hash: &str,
    ) -> PreferenceHttpResponse {
        let request = match parse_body::<PreferenceCommitHttpRequest>(body) {
            Ok(request) => request,
            Err(code) => return PreferenceHttpResponse::error("400 Bad Request", code),
        };
        if request.commit.request.session_binding_hash != expected_session_binding_hash {
            return PreferenceHttpResponse::error(
                "403 Forbidden",
                "trusted_preference_ingress.runtime_session_binding_mismatch",
            );
        }
        let attachment_subject = request.commit.request.subject.clone();
        let attachment_preference = request.commit.request.preference.clone();
        let attachment_session_binding_hash = request.commit.request.session_binding_hash.clone();
        if request.commit.source != SourceBinding::from_ref(self.authority.source_binding()) {
            return PreferenceHttpResponse::error(
                "403 Forbidden",
                "trusted_preference_ingress.source_binding_mismatch",
            );
        }
        let reducer = match self.authority.reducer_binding() {
            Ok(reducer) => reducer,
            Err(error) => return authority_error_response(&error),
        };
        if request.commit.reducer != ReducerBinding::from_ref(&reducer) {
            return PreferenceHttpResponse::error(
                "403 Forbidden",
                "trusted_preference_ingress.reducer_binding_mismatch",
            );
        }
        let proof = match PreferenceIngressProof::from_hex(&request.proof) {
            Ok(proof) => proof,
            Err(_) => {
                return PreferenceHttpResponse::error(
                    "403 Forbidden",
                    "trusted_preference_ingress.proof_encoding_invalid",
                );
            }
        };
        let input = match request.commit.clone().try_into_input() {
            Ok(input) => input,
            Err(code) => return PreferenceHttpResponse::error("422 Unprocessable Entity", code),
        };
        let challenge_hash = match explicit_preference_feedback_challenge_hash(
            &input,
            self.authority.source_binding().clone(),
        ) {
            Ok(hash) => hash,
            Err(error) => return authority_error_response(&error),
        };
        if challenge_hash.as_str() != request.commit.challenge_hash {
            return PreferenceHttpResponse::error(
                "403 Forbidden",
                "trusted_preference_ingress.challenge_binding_mismatch",
            );
        }
        let outcome = match self.executor.block_on(self.authority.commit(input, proof)) {
            Ok(Ok(outcome)) => outcome,
            Ok(Err(error)) => return commit_error_response(&error),
            Err(_) => {
                return PreferenceHttpResponse::error(
                    "503 Service Unavailable",
                    "trusted_preference_ingress.executor_unavailable",
                );
            }
        };
        let committed_state = outcome.commit().document().state();
        if let Err(error) = self.attachment.persist(&PreferenceAttachmentCandidate {
            session_binding_hash: attachment_session_binding_hash,
            subject: attachment_subject,
            preference: attachment_preference,
            stamp: RevisionStamp::new(
                committed_state.revision(),
                committed_state.content_hash().clone(),
            ),
        }) {
            eprintln!("trusted preference attachment persistence failed: {error:#}");
            return PreferenceHttpResponse::error(
                "503 Service Unavailable",
                "trusted_preference_ingress.attachment_persistence_failed",
            );
        }
        let mut response = PreferenceHttpResponse::json(
            "200 OK",
            &PreferenceCommitHttpResponse {
                schema: RESPONSE_SCHEMA,
                authority: "hepta.memory.authenticated-preference-cas",
                challenge_authenticated: true,
                network_binding_policy: "strict_loopback_host_origin_csrf_json",
                transport_confidentiality_claimed: false,
                runtime_preflight: "plan_only_quarantine",
                runtime_effect_authority_claimed: false,
                request_binding_hash,
                committed_now: outcome.committed_now(),
                transition_id: outcome.transition_id().as_str(),
                evidence_hash: outcome.evidence().evidence_hash().as_str(),
                committed_next: StateBinding::from_state(committed_state),
            },
        );
        response.preference_context = Some(RevisionStamp::new(
            committed_state.revision(),
            committed_state.content_hash().clone(),
        ));
        response
    }
}

enum PreferenceAsyncExecutor {
    Shared(tokio::runtime::Handle),
    Owned(Arc<tokio::runtime::Runtime>),
}

impl PreferenceAsyncExecutor {
    fn new() -> Result<Self> {
        match tokio::runtime::Handle::try_current() {
            Ok(handle) if handle.runtime_flavor() == tokio::runtime::RuntimeFlavor::MultiThread => {
                Ok(Self::Shared(handle))
            }
            Ok(_) => anyhow::bail!(
                "trusted preference ingress requires a multi-thread Tokio composition runtime"
            ),
            Err(_) => Ok(Self::Owned(Arc::new(
                tokio::runtime::Builder::new_multi_thread()
                    .enable_all()
                    .worker_threads(1)
                    .thread_name("hepta-preference-ingress")
                    .build()
                    .context("build trusted preference ingress executor")?,
            ))),
        }
    }

    fn block_on<F>(&self, future: F) -> Result<F::Output>
    where
        F: Future,
    {
        match self {
            Self::Shared(handle) => {
                if tokio::runtime::Handle::try_current().is_ok() {
                    Ok(tokio::task::block_in_place(|| handle.block_on(future)))
                } else {
                    Ok(handle.block_on(future))
                }
            }
            Self::Owned(runtime) => {
                if tokio::runtime::Handle::try_current().is_ok() {
                    anyhow::bail!(
                        "owned trusted preference executor cannot nest inside a Tokio runtime"
                    );
                }
                Ok(runtime.block_on(future))
            }
        }
    }
}

pub(crate) struct PreferenceHttpResponse {
    pub(crate) status: &'static str,
    pub(crate) body: String,
    pub(crate) preference_context: Option<RevisionStamp>,
}

impl PreferenceHttpResponse {
    fn json<T>(status: &'static str, value: &T) -> Self
    where
        T: Serialize,
    {
        match serde_json::to_string(value) {
            Ok(body) => Self {
                status,
                body,
                preference_context: None,
            },
            Err(_) => Self::error(
                "500 Internal Server Error",
                "trusted_preference_ingress.response_encoding_failed",
            ),
        }
    }

    fn error(status: &'static str, code: &'static str) -> Self {
        Self::json(status, &PreferenceErrorHttpResponse { error: code })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct PreferenceChallengeHttpRequest {
    transition_id: String,
    evidence_id: String,
    signal: PreferenceSignal,
    receipt: ReceiptBinding,
    session_binding_hash: String,
    subject: String,
    preference: String,
    target: CapabilityTargetBinding,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct PreferenceChallengeHttpEnvelope {
    request: PreferenceChallengeHttpRequest,
    proof: String,
}

impl PreferenceChallengeHttpRequest {
    fn try_into_parts(
        self,
    ) -> std::result::Result<ExplicitPreferenceFeedbackChallengeInputParts, &'static str> {
        self.validate()?;
        Ok(ExplicitPreferenceFeedbackChallengeInputParts {
            transition_id: PreferenceTransitionId::new(self.transition_id),
            evidence_id: PreferenceEvidenceId::new(self.evidence_id),
            signal: self.signal.into_contract(),
            receipt: self.receipt.into_ref(),
            session_binding_hash: ContentHash::new(self.session_binding_hash),
            subject: PrincipalId::new(self.subject),
            preference: PreferenceId::new(self.preference),
            target: self.target.into_target(),
        })
    }

    fn validate(&self) -> std::result::Result<(), &'static str> {
        for value in [
            self.transition_id.as_str(),
            self.evidence_id.as_str(),
            self.receipt.id.as_str(),
            self.receipt.hash.as_str(),
            self.session_binding_hash.as_str(),
            self.subject.as_str(),
            self.preference.as_str(),
            self.target.capability_id.as_str(),
            self.target.manifest_hash.as_str(),
            self.target.catalog_hash.as_str(),
        ] {
            if value.is_empty() {
                return Err("trusted_preference_ingress.empty_binding");
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct PreferencePreparedCommit {
    request: PreferenceChallengeHttpRequest,
    source: SourceBinding,
    reducer: ReducerBinding,
    expected_previous: StateBinding,
    challenge_hash: String,
}

impl PreferencePreparedCommit {
    fn try_into_input(self) -> std::result::Result<ExplicitPreferenceFeedbackInput, &'static str> {
        self.request.validate()?;
        if self.expected_previous.content_hash.is_empty() || self.challenge_hash.is_empty() {
            return Err("trusted_preference_ingress.empty_binding");
        }
        ExplicitPreferenceFeedbackInput::try_new(ExplicitPreferenceFeedbackInputParts {
            transition_id: PreferenceTransitionId::new(self.request.transition_id),
            evidence_id: PreferenceEvidenceId::new(self.request.evidence_id),
            signal: self.request.signal.into_contract(),
            receipt: self.request.receipt.into_ref(),
            session_binding_hash: ContentHash::new(self.request.session_binding_hash),
            subject: PrincipalId::new(self.request.subject),
            preference: PreferenceId::new(self.request.preference),
            target: self.request.target.into_target(),
            expected_previous: self.expected_previous.into_state(),
        })
        .map_err(|_| "trusted_preference_ingress.contract_invalid")
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct PreferenceCommitHttpRequest {
    commit: PreferencePreparedCommit,
    proof: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum PreferenceSignal {
    Accepted,
    Rejected,
}

impl PreferenceSignal {
    const fn into_contract(self) -> ExplicitPreferenceSignal {
        match self {
            Self::Accepted => ExplicitPreferenceSignal::Accepted,
            Self::Rejected => ExplicitPreferenceSignal::Rejected,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct ReceiptBinding {
    id: String,
    hash: String,
}

impl ReceiptBinding {
    fn into_ref(self) -> ReceiptRef {
        ReceiptRef::new(ReceiptId::new(self.id), ContentHash::new(self.hash))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct CapabilityTargetBinding {
    kind: CapabilityTargetKind,
    capability_id: String,
    capability_revision: u64,
    manifest_hash: String,
    catalog_revision: u64,
    catalog_hash: String,
}

impl CapabilityTargetBinding {
    fn into_target(self) -> ExplicitPreferenceTarget {
        match self.kind {
            CapabilityTargetKind::Capability => {
                ExplicitPreferenceTarget::Capability(CapabilityManifestRef::new(
                    CapabilityId::new(self.capability_id),
                    Revision::new(self.capability_revision),
                    ContentHash::new(self.manifest_hash),
                    RevisionStamp::new(
                        Revision::new(self.catalog_revision),
                        ContentHash::new(self.catalog_hash),
                    ),
                ))
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum CapabilityTargetKind {
    Capability,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct SourceBinding {
    identity: String,
    revision: u64,
    content_hash: String,
}

impl SourceBinding {
    fn from_ref(source: &PreferenceFeedbackSourceRef) -> Self {
        Self {
            identity: source.identity().as_str().to_owned(),
            revision: source.revision().get(),
            content_hash: source.content_hash().as_str().to_owned(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct ReducerBinding {
    identity: String,
    version: String,
}

impl ReducerBinding {
    fn from_ref(reducer: &hepta_memory::PreferenceReducerRef) -> Self {
        Self {
            identity: reducer.identity().to_owned(),
            version: reducer.version().to_owned(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct StateBinding {
    revision: u64,
    content_hash: String,
}

impl StateBinding {
    fn from_state(state: &PreferenceState) -> Self {
        Self {
            revision: state.revision().get(),
            content_hash: state.content_hash().as_str().to_owned(),
        }
    }

    fn into_state(self) -> PreferenceState {
        PreferenceState::new(
            Revision::new(self.revision),
            ContentHash::new(self.content_hash),
        )
    }
}

#[derive(Serialize)]
struct PreferenceChallengeHttpResponse<'a> {
    schema: &'static str,
    authority: &'static str,
    commit_authority: &'static str,
    challenge_authenticated: bool,
    network_binding_policy: &'static str,
    transport_confidentiality_claimed: bool,
    runtime_preflight: &'static str,
    runtime_effect_authority_claimed: bool,
    request_binding_hash: &'a str,
    commit: PreferencePreparedCommit,
}

#[derive(Serialize)]
struct PreferenceCommitHttpResponse<'a> {
    schema: &'static str,
    authority: &'static str,
    challenge_authenticated: bool,
    network_binding_policy: &'static str,
    transport_confidentiality_claimed: bool,
    runtime_preflight: &'static str,
    runtime_effect_authority_claimed: bool,
    request_binding_hash: &'a str,
    committed_now: bool,
    transition_id: &'a str,
    evidence_hash: &'a str,
    committed_next: StateBinding,
}

#[derive(Serialize)]
struct PreferenceErrorHttpResponse {
    error: &'static str,
}

fn parse_body<T>(body: Option<&str>) -> std::result::Result<T, &'static str>
where
    T: for<'de> Deserialize<'de>,
{
    let Some(body) = body else {
        return Err("trusted_preference_ingress.body_required");
    };
    serde_json::from_str(body).map_err(|_| "trusted_preference_ingress.body_invalid")
}

fn native_http_preference_source() -> Result<PreferenceFeedbackSourceRef> {
    let hash = Sha256::digest(SOURCE_DESCRIPTOR);
    PreferenceFeedbackSourceRef::try_new(
        PrincipalId::new(SOURCE_IDENTITY),
        Revision::new(SOURCE_REVISION),
        ContentHash::new(format!("sha256:{hash:x}")),
    )
    .map_err(Into::into)
}

#[cfg(test)]
pub(crate) fn authenticated_challenge_envelope_for_test(
    request: &serde_json::Value,
    authentication_key: [u8; 32],
) -> Result<serde_json::Value> {
    let request = serde_json::from_value::<PreferenceChallengeHttpRequest>(request.clone())
        .context("parse test challenge request")?;
    let parts = request
        .clone()
        .try_into_parts()
        .map_err(anyhow::Error::msg)?;
    let source = native_http_preference_source()?;
    let reducer = TrustedExplicitPreferenceReducer::try_new()?;
    let proof = hepta_intelligence::sign_preference_ingress_challenge_plan(
        &PreferenceIngressAuthenticationKey::from_bytes(authentication_key),
        &parts,
        &source,
        reducer.binding(),
    )?;
    Ok(serde_json::json!({
        "request": request,
        "proof": proof.to_hex(),
    }))
}

fn authority_error_response(error: &PreferenceAuthorityError) -> PreferenceHttpResponse {
    let (status, code) = match error {
        PreferenceAuthorityError::Authentication(_) => (
            "403 Forbidden",
            "trusted_preference_ingress.authentication_denied",
        ),
        PreferenceAuthorityError::Cas(
            PreferenceCasError::StateConflict { .. }
            | PreferenceCasError::TransitionReuseConflict { .. }
            | PreferenceCasError::EvidenceReuseConflict { .. }
            | PreferenceCasError::ReceiptReuseConflict { .. }
            | PreferenceCasError::GenesisConflict { .. },
        ) => ("409 Conflict", "trusted_preference_ingress.state_conflict"),
        PreferenceAuthorityError::Cas(
            PreferenceCasError::Persistence { .. }
            | PreferenceCasError::Corrupt { .. }
            | PreferenceCasError::StorePoisoned,
        ) => (
            "503 Service Unavailable",
            "trusted_preference_ingress.store_unavailable",
        ),
        _ => (
            "422 Unprocessable Entity",
            "trusted_preference_ingress.authority_denied",
        ),
    };
    PreferenceHttpResponse::error(status, code)
}

fn commit_error_response(error: &PreferenceIngressCommitError) -> PreferenceHttpResponse {
    match error {
        PreferenceIngressCommitError::ReplayDenied => PreferenceHttpResponse::error(
            "409 Conflict",
            "trusted_preference_ingress.replay_denied",
        ),
        PreferenceIngressCommitError::Authority(error) => authority_error_response(error),
        _ => PreferenceHttpResponse::error(
            "422 Unprocessable Entity",
            "trusted_preference_ingress.commit_denied",
        ),
    }
}

fn required_absolute_value(environment_name: &str, value: Option<OsString>) -> Result<PathBuf> {
    let path = value
        .filter(|value| !value.is_empty())
        .map(PathBuf::from)
        .with_context(|| format!("{environment_name} is required for --serve-ui"))?;
    if !path.is_absolute() {
        anyhow::bail!("{environment_name} must be an absolute path");
    }
    Ok(path)
}

#[cfg(test)]
#[path = "../tests/unit/preference_ingress.rs"]
mod tests;
