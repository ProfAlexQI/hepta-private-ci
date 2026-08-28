use crate::ContractError;
use crate::Digest32;
use crate::P1_1B_CALLERS_RATCHET;
use crate::P1_1B_CONTEXT_ATTACHMENT;
use crate::P1_1B_DEFAULT_RECALL_CHANGED;
use crate::P1_1B_EXTERNAL_EFFECTS;
use crate::P1_1B_FEDERATION_RECALL_CHANGED;
use crate::P1_1B_OPERATOR_ACCEPTANCE;
use crate::P1_1B_PHYSICAL_SEND;
use crate::P1_1B_PRODUCTION_AUTHORITY;
use crate::P1_1B_PROMOTION;
use crate::digest::framed_digest;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum SemanticFallbackReason {
    TokenizerUnavailable,
    EmbeddingProviderUnavailable,
    IndexUnavailable,
    BindingMismatch,
    DependencyUnqualified,
}

impl SemanticFallbackReason {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TokenizerUnavailable => "tokenizer_unavailable",
            Self::EmbeddingProviderUnavailable => "embedding_provider_unavailable",
            Self::IndexUnavailable => "index_unavailable",
            Self::BindingMismatch => "binding_mismatch",
            Self::DependencyUnqualified => "dependency_unqualified",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum LocalSemanticRoute {
    ShadowSemanticReady,
    LexicalOnly,
}

impl LocalSemanticRoute {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ShadowSemanticReady => "shadow_semantic_ready",
            Self::LexicalOnly => "lexical_only",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalSemanticReadiness {
    pub tokenizer_registered: bool,
    pub embedding_provider_registered: bool,
    pub index_opened_and_verified: bool,
    pub model_tokenizer_index_bindings_match: bool,
    pub dependencies_executable_qualified: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalSemanticRouteReceipt {
    pub route: LocalSemanticRoute,
    pub fallback_reason: Option<SemanticFallbackReason>,
    pub tokenizer_registered: bool,
    pub embedding_provider_registered: bool,
    pub index_opened_and_verified: bool,
    pub model_tokenizer_index_bindings_match: bool,
    pub dependencies_executable_qualified: bool,
    pub deterministic: bool,
    pub runtime_wired: bool,
    pub default_recall_changed: bool,
    pub federation_recall_changed: bool,
    pub context_attachment: bool,
    pub physical_send: bool,
    pub external_effects: bool,
    pub production_authority: bool,
    pub operator_acceptance: bool,
    pub promotion: bool,
    pub callers_ratchet: bool,
    pub receipt_sha256: Digest32,
}

impl LocalSemanticRouteReceipt {
    pub fn validate(&self) -> Result<(), ContractError> {
        if !self.deterministic
            || self.runtime_wired
            || self.default_recall_changed
            || self.federation_recall_changed
            || self.context_attachment
            || self.physical_send
            || self.external_effects
            || self.production_authority
            || self.operator_acceptance
            || self.promotion
            || self.callers_ratchet
        {
            return Err(ContractError::Corrupt(
                "semantic route receipt crosses the source-only boundary".to_string(),
            ));
        }
        match self.route {
            LocalSemanticRoute::ShadowSemanticReady if self.fallback_reason.is_some() => {
                return Err(ContractError::Corrupt(
                    "ready semantic route must not contain a fallback reason".to_string(),
                ));
            }
            LocalSemanticRoute::LexicalOnly if self.fallback_reason.is_none() => {
                return Err(ContractError::Corrupt(
                    "lexical-only route requires a fallback reason".to_string(),
                ));
            }
            _ => {}
        }
        if self.receipt_sha256 != route_receipt_digest(self) {
            return Err(ContractError::Corrupt(
                "semantic route receipt digest mismatch".to_string(),
            ));
        }
        Ok(())
    }
}

pub fn decide_local_semantic_route(
    readiness: &LocalSemanticReadiness,
) -> Result<LocalSemanticRouteReceipt, ContractError> {
    let fallback_reason = if !readiness.dependencies_executable_qualified {
        Some(SemanticFallbackReason::DependencyUnqualified)
    } else if !readiness.tokenizer_registered {
        Some(SemanticFallbackReason::TokenizerUnavailable)
    } else if !readiness.embedding_provider_registered {
        Some(SemanticFallbackReason::EmbeddingProviderUnavailable)
    } else if !readiness.index_opened_and_verified {
        Some(SemanticFallbackReason::IndexUnavailable)
    } else if !readiness.model_tokenizer_index_bindings_match {
        Some(SemanticFallbackReason::BindingMismatch)
    } else {
        None
    };
    let route = if fallback_reason.is_some() {
        LocalSemanticRoute::LexicalOnly
    } else {
        LocalSemanticRoute::ShadowSemanticReady
    };
    let mut receipt = LocalSemanticRouteReceipt {
        route,
        fallback_reason,
        tokenizer_registered: readiness.tokenizer_registered,
        embedding_provider_registered: readiness.embedding_provider_registered,
        index_opened_and_verified: readiness.index_opened_and_verified,
        model_tokenizer_index_bindings_match: readiness.model_tokenizer_index_bindings_match,
        dependencies_executable_qualified: readiness.dependencies_executable_qualified,
        deterministic: true,
        runtime_wired: false,
        default_recall_changed: P1_1B_DEFAULT_RECALL_CHANGED,
        federation_recall_changed: P1_1B_FEDERATION_RECALL_CHANGED,
        context_attachment: P1_1B_CONTEXT_ATTACHMENT,
        physical_send: P1_1B_PHYSICAL_SEND,
        external_effects: P1_1B_EXTERNAL_EFFECTS,
        production_authority: P1_1B_PRODUCTION_AUTHORITY,
        operator_acceptance: P1_1B_OPERATOR_ACCEPTANCE,
        promotion: P1_1B_PROMOTION,
        callers_ratchet: P1_1B_CALLERS_RATCHET,
        receipt_sha256: Digest32::for_bytes(b"uncomputed"),
    };
    receipt.receipt_sha256 = route_receipt_digest(&receipt);
    receipt.validate()?;
    Ok(receipt)
}

fn route_receipt_digest(receipt: &LocalSemanticRouteReceipt) -> Digest32 {
    let fallback = receipt
        .fallback_reason
        .map_or("", SemanticFallbackReason::as_str);
    framed_digest(
        b"hepta:intelligence:p1.1b:semantic-route-receipt:v1",
        &[
            receipt.route.as_str().as_bytes(),
            fallback.as_bytes(),
            &[u8::from(receipt.tokenizer_registered)],
            &[u8::from(receipt.embedding_provider_registered)],
            &[u8::from(receipt.index_opened_and_verified)],
            &[u8::from(receipt.model_tokenizer_index_bindings_match)],
            &[u8::from(receipt.dependencies_executable_qualified)],
            &[u8::from(receipt.deterministic)],
            &[u8::from(receipt.runtime_wired)],
            &[u8::from(receipt.default_recall_changed)],
            &[u8::from(receipt.federation_recall_changed)],
            &[u8::from(receipt.context_attachment)],
            &[u8::from(receipt.physical_send)],
            &[u8::from(receipt.external_effects)],
            &[u8::from(receipt.production_authority)],
            &[u8::from(receipt.operator_acceptance)],
            &[u8::from(receipt.promotion)],
            &[u8::from(receipt.callers_ratchet)],
        ],
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dependency_gate_forces_lexical_fallback_first() {
        let receipt = decide_local_semantic_route(&LocalSemanticReadiness {
            tokenizer_registered: true,
            embedding_provider_registered: true,
            index_opened_and_verified: true,
            model_tokenizer_index_bindings_match: true,
            dependencies_executable_qualified: false,
        })
        .expect("route");
        assert_eq!(receipt.route, LocalSemanticRoute::LexicalOnly);
        assert_eq!(
            receipt.fallback_reason,
            Some(SemanticFallbackReason::DependencyUnqualified)
        );
    }

    #[test]
    fn all_local_bindings_allow_shadow_only_semantic_route() {
        let receipt = decide_local_semantic_route(&LocalSemanticReadiness {
            tokenizer_registered: true,
            embedding_provider_registered: true,
            index_opened_and_verified: true,
            model_tokenizer_index_bindings_match: true,
            dependencies_executable_qualified: true,
        })
        .expect("route");
        assert_eq!(receipt.route, LocalSemanticRoute::ShadowSemanticReady);
        receipt.validate().expect("receipt");
    }
}
