mod actor;
mod engine;

pub use actor::BrowserActor;
pub use actor::BrowserActorStatus;
pub use engine::BrowserEngine;
pub use engine::BrowserEngineError;
pub use engine::BrowserEngineExtract;
pub use engine::BrowserEngineNode;
pub use engine::BrowserEngineSnapshot;
pub use engine::FixtureBrowserEngine;

const MAX_ENGINE_NODES: usize = 1_024;
const MAX_OBSERVE_NODES: u16 = 256;
const MAX_EXTRACT_BYTES: u32 = 16_384;
const MAX_TYPED_TEXT_BYTES: usize = 4_096;
const MAX_URL_BYTES: usize = 2_048;
const MAX_TITLE_BYTES: usize = 512;
const MAX_NODE_ROLE_BYTES: usize = 128;
const MAX_NODE_NAME_BYTES: usize = 512;
const MAX_NODE_VALUE_BYTES: usize = 4_096;
const MAX_CACHED_RESPONSES: usize = 1_024;
const MAX_HUMAN_LEASE_MS: u64 = 300_000;
