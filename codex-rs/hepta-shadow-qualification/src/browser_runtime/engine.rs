use std::fmt;

use crate::browser_contracts::BrowserAction;
use crate::browser_contracts::BrowserChallengeCode;
use crate::browser_contracts::BrowserDenialCode;
use crate::browser_contracts::BrowserIndeterminateCode;
use crate::browser_contracts::BrowserWaitCondition;

use super::MAX_TYPED_TEXT_BYTES;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BrowserEngineError {
    Denied(BrowserDenialCode),
    Challenge(BrowserChallengeCode),
    Indeterminate(BrowserIndeterminateCode),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BrowserEngineNode {
    pub key: String,
    pub role: String,
    pub name: String,
    pub value: String,
    pub interactive: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BrowserEngineSnapshot {
    pub url: String,
    pub title: String,
    pub nodes: Vec<BrowserEngineNode>,
    pub raw_secret_bytes_present: bool,
    pub cross_tenant_data_present: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BrowserEngineExtract {
    pub value: String,
    pub raw_secret_bytes_present: bool,
    pub cross_tenant_data_present: bool,
}

pub trait BrowserEngine: fmt::Debug {
    fn navigate(&mut self, url: &str) -> Result<(), BrowserEngineError>;

    fn snapshot(&self) -> Result<BrowserEngineSnapshot, BrowserEngineError>;

    fn act(
        &mut self,
        target_key: &str,
        action: &BrowserAction,
    ) -> Result<(), BrowserEngineError>;

    fn wait(&self, condition: &BrowserWaitCondition) -> Result<(), BrowserEngineError>;

    fn extract(&self, query: &str) -> Result<BrowserEngineExtract, BrowserEngineError>;
}

#[derive(Clone, Debug)]
pub struct FixtureBrowserEngine {
    url: String,
    title: String,
    input_value: String,
    click_count: u64,
    cookie_revision: u64,
    history: Vec<String>,
}

impl Default for FixtureBrowserEngine {
    fn default() -> Self {
        Self {
            url: "fixture://blank".to_string(),
            title: "Blank fixture".to_string(),
            input_value: String::new(),
            click_count: 0,
            cookie_revision: 0,
            history: vec!["fixture://blank".to_string()],
        }
    }
}

impl BrowserEngine for FixtureBrowserEngine {
    fn navigate(&mut self, url: &str) -> Result<(), BrowserEngineError> {
        match url {
            "fixture://blank" => {
                self.url = url.to_string();
                self.title = "Blank fixture".to_string();
                self.input_value.clear();
                self.click_count = 0;
                self.cookie_revision = 0;
                self.history.push(url.to_string());
                Ok(())
            }
            "fixture://shared-form" => {
                self.url = url.to_string();
                self.title = "Shared human-agent fixture".to_string();
                self.input_value.clear();
                self.click_count = 0;
                self.cookie_revision = 0;
                self.history.push(url.to_string());
                Ok(())
            }
            value if value.starts_with("https://") || value.starts_with("http://") => Err(
                BrowserEngineError::Denied(BrowserDenialCode::ExternalNavigationDisabled),
            ),
            _ => Err(BrowserEngineError::Denied(
                BrowserDenialCode::InvalidCommand,
            )),
        }
    }

    fn snapshot(&self) -> Result<BrowserEngineSnapshot, BrowserEngineError> {
        let mut nodes = Vec::new();
        if self.url == "fixture://shared-form" {
            nodes.push(BrowserEngineNode {
                key: "name-input".to_string(),
                role: "textbox".to_string(),
                name: "Name".to_string(),
                value: self.input_value.clone(),
                interactive: true,
            });
            nodes.push(BrowserEngineNode {
                key: "submit-button".to_string(),
                role: "button".to_string(),
                name: "Submit".to_string(),
                value: self.click_count.to_string(),
                interactive: true,
            });
            nodes.push(BrowserEngineNode {
                key: "status".to_string(),
                role: "status".to_string(),
                name: "Status".to_string(),
                value: format!("submitted:{}", self.click_count),
                interactive: false,
            });
        }
        Ok(BrowserEngineSnapshot {
            url: self.url.clone(),
            title: self.title.clone(),
            nodes,
            raw_secret_bytes_present: false,
            cross_tenant_data_present: false,
        })
    }

    fn act(
        &mut self,
        target_key: &str,
        action: &BrowserAction,
    ) -> Result<(), BrowserEngineError> {
        match (target_key, action) {
            ("name-input", BrowserAction::TypeText { text })
                if text.len() <= MAX_TYPED_TEXT_BYTES =>
            {
                self.input_value.push_str(text);
                Ok(())
            }
            ("name-input", BrowserAction::Clear) => {
                self.input_value.clear();
                Ok(())
            }
            ("submit-button", BrowserAction::Click) => {
                self.click_count = self.click_count.checked_add(1).ok_or_else(|| {
                    BrowserEngineError::Denied(BrowserDenialCode::ResourceLimit)
                })?;
                self.cookie_revision = self.cookie_revision.checked_add(1).ok_or_else(|| {
                    BrowserEngineError::Denied(BrowserDenialCode::ResourceLimit)
                })?;
                Ok(())
            }
            (_, BrowserAction::TypeText { text }) if text.len() > MAX_TYPED_TEXT_BYTES => Err(
                BrowserEngineError::Denied(BrowserDenialCode::ResourceLimit),
            ),
            _ => Err(BrowserEngineError::Denied(
                BrowserDenialCode::InvalidCommand,
            )),
        }
    }

    fn wait(&self, condition: &BrowserWaitCondition) -> Result<(), BrowserEngineError> {
        let satisfied = match condition {
            BrowserWaitCondition::DocumentReady => true,
            BrowserWaitCondition::TextContains { text } => {
                self.title.contains(text)
                    || self.input_value.contains(text)
                    || format!("submitted:{}", self.click_count).contains(text)
            }
            BrowserWaitCondition::HistoryLengthAtLeast { length } => {
                self.history.len() >= *length as usize
            }
        };
        if satisfied {
            Ok(())
        } else {
            Err(BrowserEngineError::Indeterminate(
                BrowserIndeterminateCode::EngineTimeout,
            ))
        }
    }

    fn extract(&self, query: &str) -> Result<BrowserEngineExtract, BrowserEngineError> {
        let value = match query {
            "document.title" => self.title.clone(),
            "document.url" => self.url.clone(),
            "input.value" => self.input_value.clone(),
            "button.click_count" => self.click_count.to_string(),
            "storage.cookie_revision" => self.cookie_revision.to_string(),
            "history.length" => self.history.len().to_string(),
            "document.cookie" => {
                return Ok(BrowserEngineExtract {
                    value: format!("fixture_session={}", self.cookie_revision),
                    raw_secret_bytes_present: true,
                    cross_tenant_data_present: false,
                });
            }
            _ => {
                return Err(BrowserEngineError::Denied(
                    BrowserDenialCode::InvalidCommand,
                ));
            }
        };
        Ok(BrowserEngineExtract {
            value,
            raw_secret_bytes_present: false,
            cross_tenant_data_present: false,
        })
    }
}
