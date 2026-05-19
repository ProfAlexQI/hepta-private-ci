#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GatewayTransport {
    Cli,
    Webhook,
    Queue,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GatewayEnvelope {
    pub surface_id: String,
    pub user_id: String,
    pub session_hint: Option<String>,
    pub transport: GatewayTransport,
    pub payload_text: String,
}

impl GatewayEnvelope {
    pub fn new(
        surface_id: impl Into<String>,
        user_id: impl Into<String>,
        transport: GatewayTransport,
        payload_text: impl Into<String>,
    ) -> Self {
        Self {
            surface_id: surface_id.into(),
            user_id: user_id.into(),
            session_hint: None,
            transport,
            payload_text: payload_text.into(),
        }
    }

    pub fn with_session_hint(mut self, session_hint: impl Into<String>) -> Self {
        let session_hint = session_hint.into();
        let session_hint = session_hint.trim();
        if !session_hint.is_empty() {
            self.session_hint = Some(session_hint.to_string());
        }
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GatewayRoutePlan {
    pub surface_id: String,
    pub session_key: String,
    pub transport: GatewayTransport,
    pub normalized_text: String,
}

impl GatewayRoutePlan {
    pub fn new(
        surface_id: impl Into<String>,
        session_key: impl Into<String>,
        transport: GatewayTransport,
        normalized_text: impl Into<String>,
    ) -> Self {
        Self {
            surface_id: surface_id.into(),
            session_key: session_key.into(),
            transport,
            normalized_text: normalized_text.into(),
        }
    }
}
