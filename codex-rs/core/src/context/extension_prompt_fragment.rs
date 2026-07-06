use super::ContextualUserFragment;

pub(crate) const EXTENSION_DEVELOPER_POLICY_OPEN_TAG: &str = "<extension_developer_policy>";
pub(crate) const EXTENSION_DEVELOPER_POLICY_CLOSE_TAG: &str = "</extension_developer_policy>";
pub(crate) const EXTENSION_DEVELOPER_CAPABILITIES_OPEN_TAG: &str =
    "<extension_developer_capabilities>";
pub(crate) const EXTENSION_DEVELOPER_CAPABILITIES_CLOSE_TAG: &str =
    "</extension_developer_capabilities>";
pub(crate) const EXTENSION_SEPARATE_DEVELOPER_OPEN_TAG: &str = "<extension_separate_developer>";
pub(crate) const EXTENSION_SEPARATE_DEVELOPER_CLOSE_TAG: &str = "</extension_separate_developer>";
pub(crate) const EXTENSION_CONTEXTUAL_USER_OPEN_TAG: &str = "<extension_contextual_user>";
pub(crate) const EXTENSION_CONTEXTUAL_USER_CLOSE_TAG: &str = "</extension_contextual_user>";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ExtensionPromptSlot {
    DeveloperPolicy,
    DeveloperCapabilities,
    SeparateDeveloper,
    ContextualUser,
}

impl ExtensionPromptSlot {
    pub(crate) fn source_id(self) -> &'static str {
        match self {
            Self::DeveloperPolicy => "extension_developer_policy",
            Self::DeveloperCapabilities => "extension_developer_capabilities",
            Self::SeparateDeveloper => "extension_separate_developer",
            Self::ContextualUser => "extension_contextual_user",
        }
    }

    pub(crate) fn open_tag(self) -> &'static str {
        match self {
            Self::DeveloperPolicy => EXTENSION_DEVELOPER_POLICY_OPEN_TAG,
            Self::DeveloperCapabilities => EXTENSION_DEVELOPER_CAPABILITIES_OPEN_TAG,
            Self::SeparateDeveloper => EXTENSION_SEPARATE_DEVELOPER_OPEN_TAG,
            Self::ContextualUser => EXTENSION_CONTEXTUAL_USER_OPEN_TAG,
        }
    }

    pub(crate) fn close_tag(self) -> &'static str {
        match self {
            Self::DeveloperPolicy => EXTENSION_DEVELOPER_POLICY_CLOSE_TAG,
            Self::DeveloperCapabilities => EXTENSION_DEVELOPER_CAPABILITIES_CLOSE_TAG,
            Self::SeparateDeveloper => EXTENSION_SEPARATE_DEVELOPER_CLOSE_TAG,
            Self::ContextualUser => EXTENSION_CONTEXTUAL_USER_CLOSE_TAG,
        }
    }

    pub(crate) fn clear_label(self) -> &'static str {
        match self {
            Self::DeveloperPolicy => "extension developer policy",
            Self::DeveloperCapabilities => "extension developer capabilities",
            Self::SeparateDeveloper => "extension separate developer",
            Self::ContextualUser => "extension contextual-user",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ExtensionPromptFragment {
    slot: ExtensionPromptSlot,
    text: String,
}

impl ExtensionPromptFragment {
    pub(crate) fn new(slot: ExtensionPromptSlot, text: impl Into<String>) -> Self {
        Self {
            slot,
            text: text.into(),
        }
    }

    pub(crate) fn cleared(slot: ExtensionPromptSlot) -> Self {
        Self::new(
            slot,
            format!(
                "{} extension prompt fragments were cleared. Do not continue applying previously injected {} extension prompt fragments.",
                slot.clear_label(),
                slot.clear_label()
            ),
        )
    }

    pub(crate) fn render(&self) -> String {
        format!(
            "{}\n{}\n{}",
            self.slot.open_tag(),
            self.text,
            self.slot.close_tag()
        )
    }
}

pub(crate) struct ExtensionContextualUser;

impl ContextualUserFragment for ExtensionContextualUser {
    const ROLE: &'static str = "user";
    const START_MARKER: &'static str = EXTENSION_CONTEXTUAL_USER_OPEN_TAG;
    const END_MARKER: &'static str = EXTENSION_CONTEXTUAL_USER_CLOSE_TAG;

    fn body(&self) -> String {
        String::new()
    }
}
