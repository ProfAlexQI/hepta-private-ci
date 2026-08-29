use std::fmt;

use crate::InferError;
use crate::Result;

const MAX_ID_BYTES: usize = 128;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct BoundedId(String);

impl BoundedId {
    fn parse(value: &str, label: &'static str) -> Result<Self> {
        let bytes = value.as_bytes();
        if bytes.is_empty()
            || bytes.len() > MAX_ID_BYTES
            || value != value.trim()
            || !bytes
                .iter()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'))
        {
            return Err(InferError::InvalidIdentity(label));
        }
        Ok(Self(value.to_owned()))
    }

    fn as_str(&self) -> &str {
        &self.0
    }
}

macro_rules! define_id {
    ($name:ident, $label:literal) => {
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(BoundedId);

        impl $name {
            pub fn parse(value: &str) -> Result<Self> {
                BoundedId::parse(value, $label).map(Self)
            }

            pub fn as_str(&self) -> &str {
                self.0.as_str()
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str(self.as_str())
            }
        }
    };
}

define_id!(TenantId, "tenant_id");
define_id!(WorkspaceId, "workspace_id");
define_id!(AgentId, "agent_id");
define_id!(TaskId, "task_id");
define_id!(RequestId, "request_id");
define_id!(ResourceBudgetId, "resource_budget_id");

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Digest(String);

impl Digest {
    pub fn parse(value: &str) -> Result<Self> {
        let digest = value
            .strip_prefix("sha256:")
            .ok_or(InferError::InvalidDigest)?;
        if digest.len() != 64
            || !digest
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        {
            return Err(InferError::InvalidDigest);
        }
        Ok(Self(value.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Digest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RequestIdentity {
    pub tenant_id: TenantId,
    pub workspace_id: WorkspaceId,
    pub agent_id: AgentId,
    pub task_id: TaskId,
    pub request_id: RequestId,
}
