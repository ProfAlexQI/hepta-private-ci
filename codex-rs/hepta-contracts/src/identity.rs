use serde::Deserialize;
use serde::Serialize;

use crate::canonical::length_delimited_sha256;
use crate::stable_id::parse_prefixed_sha256_id;

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(transparent)]
pub struct ActionId(String);

impl ActionId {
    pub fn parse(value: impl Into<String>) -> Result<Self, String> {
        parse_prefixed_sha256_id(value, "tool:v1:", "tool action").map(Self)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn for_tool_call(thread_id: &str, turn_id: &str, call_id: &str) -> Self {
        Self(format!(
            "tool:v1:{}",
            length_delimited_sha256([thread_id, turn_id, call_id]).as_str()
        ))
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(transparent)]
pub struct DecisionId(String);

impl DecisionId {
    pub fn for_action(action_id: &ActionId, phase: &str) -> Self {
        Self(format!(
            "decision:v1:{}",
            length_delimited_sha256([action_id.as_str(), phase]).as_str()
        ))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(transparent)]
pub struct ReceiptId(String);

impl ReceiptId {
    pub fn for_action(action_id: &ActionId) -> Self {
        Self(format!(
            "receipt:v1:{}",
            length_delimited_sha256([action_id.as_str()]).as_str()
        ))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::ActionId;
    use super::DecisionId;
    use super::ReceiptId;

    #[test]
    fn tool_identity_is_versioned_and_length_delimited() {
        let left = ActionId::for_tool_call("ab", "c", "d");
        let right = ActionId::for_tool_call("a", "bc", "d");

        assert!(left.as_str().starts_with("tool:v1:"));
        assert_ne!(left, right);
        assert_eq!(left, ActionId::for_tool_call("ab", "c", "d"));
    }

    #[test]
    fn governance_ids_have_fixed_canonical_oracles() {
        let action = ActionId::for_tool_call("thread-1", "turn-1", "call-1");
        let decision = DecisionId::for_action(&action, "authorization");
        let receipt = ReceiptId::for_action(&action);

        assert_eq!(
            action.as_str(),
            "tool:v1:96bf3e5017e6063cdcd767179f5badf6fb1357541119939ac4db94db0e482400"
        );
        assert_eq!(
            decision.as_str(),
            "decision:v1:7b3e5fa2f59800b0cf435efa21f4f8faa1377708a46b1422dc047245c8a0206f"
        );
        assert_eq!(
            receipt.as_str(),
            "receipt:v1:678f7e1ff3d20f7257d677c910ec0d41c2f4e9fa128cfdc6e2e70662f2e1583c"
        );
    }
}
