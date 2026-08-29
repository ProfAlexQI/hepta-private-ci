use crate::protocol::ClientMessage;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MessageRole {
    PublicClient,
    Worker,
    Operator,
}

impl MessageRole {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PublicClient => "public_client",
            Self::Worker => "worker",
            Self::Operator => "operator",
        }
    }
}

impl ClientMessage {
    pub const fn required_role(&self) -> MessageRole {
        match self {
            Self::Ping { .. }
            | Self::Admit(_)
            | Self::Cancel { .. }
            | Self::GetReceipt { .. }
            | Self::Snapshot => MessageRole::PublicClient,
            Self::Start { .. } | Self::Token { .. } | Self::Complete { .. } => {
                MessageRole::Worker
            }
            Self::RestartBackend { .. } => MessageRole::Operator,
        }
    }

    pub const fn is_public_client_operation(&self) -> bool {
        matches!(self.required_role(), MessageRole::PublicClient)
    }
}
