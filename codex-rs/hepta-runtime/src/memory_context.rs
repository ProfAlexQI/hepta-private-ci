use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

use hepta_core::HeptaError;
use serde::Deserialize;
use serde::Serialize;

use crate::current_unix_ms;
use crate::delivery_queue::ReadbackEvidenceLedger;

pub const DEFAULT_MEMORY_CONTEXT_LEDGER_PATH: &str = ".hepta/memory-context-ledger-v0.json";
pub const DEFAULT_MEMORY_CONTEXT_LEDGER_ID: &str = "hepta-native-memory-context-ledger";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryContextLedgerFile {
    pub version: u32,
    pub ledger_id: String,
    pub created_at_unix_ms: u64,
    pub updated_at_unix_ms: u64,
    #[serde(default)]
    pub context_packs: Vec<MemoryContextPack>,
    #[serde(default)]
    pub retrieval_handoffs: Vec<MemoryContextRetrievalHandoffRecord>,
    #[serde(default)]
    pub local_retrievals: Vec<MemoryContextLocalRetrievalRecord>,
    #[serde(default)]
    pub events: Vec<MemoryContextEvent>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryContextPack {
    pub context_id: String,
    pub requester: String,
    pub query_preview: String,
    pub citation_count: usize,
    pub redaction_policy: String,
    #[serde(default)]
    pub citations: Vec<MemoryCitation>,
    pub created_at_unix_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryCitation {
    pub source_path: String,
    pub line_start: u32,
    pub line_end: u32,
    pub excerpt_preview: String,
    pub source_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryContextEvent {
    pub event_id: String,
    pub event_type: String,
    pub context_id: String,
    pub occurred_at_unix_ms: u64,
    pub summary: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryContextRetrievalHandoffRecord {
    pub handoff_id: String,
    pub context_id: String,
    pub requester: String,
    pub citation_count: usize,
    pub query_preview: String,
    pub policy_decision: String,
    pub operator_confirmed: bool,
    pub idempotency_key: String,
    pub readback_evidence_id: String,
    pub created_at_unix_ms: u64,
    pub context_pack_mutated_by_gate: bool,
    pub private_memory_read_by_gate: bool,
    pub external_network_read_by_gate: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryContextLocalRetrievalRecord {
    pub retrieval_id: String,
    pub context_id: String,
    pub requester: String,
    pub citation_count: usize,
    pub scanned_source_count: usize,
    pub query_preview: String,
    pub policy_decision: String,
    pub operator_confirmed: bool,
    pub idempotency_key: String,
    pub matched_source_hashes: Vec<String>,
    pub readback_evidence_id: String,
    pub created_at_unix_ms: u64,
    pub private_memory_read_by_adapter: bool,
    pub index_mutated_by_adapter: bool,
    pub external_network_read_by_adapter: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MemoryContextLedgerReport {
    pub ledger_path: String,
    pub ledger: MemoryContextLedgerFile,
    pub context_pack_count: usize,
    pub citation_count: usize,
    pub persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MemoryContextBuildReport {
    pub ledger_path: String,
    pub context_pack: MemoryContextPack,
    pub duplicate_context_id: bool,
    pub persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MemoryContextRetrievalHandoffInput {
    pub context_id: String,
    pub requester: String,
    pub query: String,
    pub citations: Vec<MemoryCitationInput>,
    pub policy_decision: String,
    pub operator_confirmed: bool,
    pub idempotency_key: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MemoryContextLocalRetrievalInput {
    pub context_id: String,
    pub requester: String,
    pub query: String,
    pub allowed_source_paths: Vec<String>,
    pub max_citations: usize,
    pub policy_decision: String,
    pub operator_confirmed: bool,
    pub idempotency_key: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MemoryContextRetrievalHandoffReport {
    pub ledger_path: String,
    pub evidence_ledger_path: String,
    pub handoff: MemoryContextRetrievalHandoffRecord,
    pub context_pack: MemoryContextBuildReport,
    pub duplicate_idempotency_key: bool,
    pub context_pack_mutated_by_gate: bool,
    pub private_memory_read_by_gate: bool,
    pub external_network_read_by_gate: bool,
    pub persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MemoryContextLocalRetrievalReport {
    pub ledger_path: String,
    pub evidence_ledger_path: String,
    pub retrieval: MemoryContextLocalRetrievalRecord,
    pub context_pack: MemoryContextBuildReport,
    pub duplicate_idempotency_key: bool,
    pub private_memory_read_by_adapter: bool,
    pub index_mutated_by_adapter: bool,
    pub external_network_read_by_adapter: bool,
    pub persisted: bool,
}

pub struct MemoryContextLedger {
    path: PathBuf,
}

impl MemoryContextLedger {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn default_in_current_dir() -> Result<Self, HeptaError> {
        let cwd = std::env::current_dir().map_err(|err| {
            HeptaError(format!(
                "failed to resolve cwd for memory-context-ledger: {err}"
            ))
        })?;
        Ok(Self::new(cwd.join(DEFAULT_MEMORY_CONTEXT_LEDGER_PATH)))
    }

    pub fn path_display(&self) -> String {
        self.path.display().to_string()
    }

    pub fn report(
        &self,
        now_unix_ms: Option<u64>,
    ) -> Result<MemoryContextLedgerReport, HeptaError> {
        let now = now_unix_ms.unwrap_or(current_unix_ms()?);
        let ledger = self.load_or_default(now)?;
        let citation_count = ledger
            .context_packs
            .iter()
            .map(|pack| pack.citations.len())
            .sum();
        Ok(MemoryContextLedgerReport {
            ledger_path: self.path_display(),
            context_pack_count: ledger.context_packs.len(),
            citation_count,
            persisted: self.path.exists(),
            ledger,
        })
    }

    pub fn build_context_pack(
        &self,
        context_id: &str,
        requester: &str,
        query: &str,
        citations: Vec<MemoryCitationInput>,
    ) -> Result<MemoryContextBuildReport, HeptaError> {
        let now = current_unix_ms()?;
        let mut ledger = self.load_or_default(now)?;
        let context_id = normalize_scoped_id(context_id, "context id")?;
        let requester = normalize_scoped_id(requester, "requester")?;
        let query_preview = redact_preview(&normalize_non_empty(query, "query")?);
        if citations.is_empty() {
            return Err(HeptaError(
                "memory context pack requires at least one citation".into(),
            ));
        }
        if let Some(existing) = ledger
            .context_packs
            .iter()
            .find(|pack| pack.context_id == context_id)
            .cloned()
        {
            return Ok(MemoryContextBuildReport {
                ledger_path: self.path_display(),
                context_pack: existing,
                duplicate_context_id: true,
                persisted: self.path.exists(),
            });
        }
        let citations = normalize_citations(citations)?;
        let pack = MemoryContextPack {
            context_id: context_id.clone(),
            requester,
            query_preview,
            citation_count: citations.len(),
            redaction_policy: "preview-only-no-secret-raw-transcript".into(),
            citations,
            created_at_unix_ms: now,
        };
        ledger.context_packs.push(pack.clone());
        push_event(
            &mut ledger,
            "context_pack_built",
            &context_id,
            now,
            "memory context pack built with source citations",
        );
        self.save(&mut ledger, now)?;
        Ok(MemoryContextBuildReport {
            ledger_path: self.path_display(),
            context_pack: pack,
            duplicate_context_id: false,
            persisted: true,
        })
    }

    pub fn gated_retrieval_handoff(
        &self,
        evidence_ledger: &ReadbackEvidenceLedger,
        input: MemoryContextRetrievalHandoffInput,
    ) -> Result<MemoryContextRetrievalHandoffReport, HeptaError> {
        let now = current_unix_ms()?;
        let context_id = normalize_scoped_id(&input.context_id, "context id")?;
        let requester = normalize_scoped_id(&input.requester, "requester")?;
        let query = normalize_non_empty(&input.query, "query")?;
        let policy_decision = normalize_non_empty(&input.policy_decision, "policy decision")?;
        let idempotency_key = normalize_non_empty(&input.idempotency_key, "idempotency key")?;
        if !input.operator_confirmed {
            return Err(HeptaError(format!(
                "memory retrieval handoff for {context_id} requires explicit operator confirmation"
            )));
        }
        if !policy_allows_handoff(&policy_decision) {
            return Err(HeptaError(format!(
                "memory retrieval handoff for {context_id} requires allow/approved policy decision"
            )));
        }
        let preflight = self.load_or_default(now)?;
        if let Some(existing) = preflight
            .retrieval_handoffs
            .iter()
            .find(|handoff| handoff.idempotency_key == idempotency_key)
            .cloned()
        {
            let context_pack =
                self.build_context_pack(&existing.context_id, &requester, &query, input.citations)?;
            return Ok(MemoryContextRetrievalHandoffReport {
                ledger_path: self.path_display(),
                evidence_ledger_path: evidence_ledger.path_display(),
                handoff: existing,
                context_pack,
                duplicate_idempotency_key: true,
                context_pack_mutated_by_gate: false,
                private_memory_read_by_gate: false,
                external_network_read_by_gate: false,
                persisted: self.path.exists(),
            });
        }
        let context_pack =
            self.build_context_pack(&context_id, &requester, &query, input.citations)?;
        let mut ledger = self.load_or_default(now)?;
        let handoff_id = format!(
            "memctxhandoff-{}-{}",
            now,
            ledger.retrieval_handoffs.len() + 1
        );
        let evidence = evidence_ledger.append(
            "memory_context_retrieval_handoff",
            &handoff_id,
            "context_pack_ready",
            &format!(
                "memory context retrieval handoff recorded for {context_id}; private memory read and external network read not performed by this gate"
            ),
        )?;
        let handoff = MemoryContextRetrievalHandoffRecord {
            handoff_id: handoff_id.clone(),
            context_id: context_id.clone(),
            requester,
            citation_count: context_pack.context_pack.citation_count,
            query_preview: context_pack.context_pack.query_preview.clone(),
            policy_decision,
            operator_confirmed: input.operator_confirmed,
            idempotency_key,
            readback_evidence_id: evidence.entry.evidence_id,
            created_at_unix_ms: now,
            context_pack_mutated_by_gate: !context_pack.duplicate_context_id,
            private_memory_read_by_gate: false,
            external_network_read_by_gate: false,
        };
        ledger.retrieval_handoffs.push(handoff.clone());
        ledger.retrieval_handoffs.truncate(1024);
        push_event(
            &mut ledger,
            "retrieval_handoff_recorded",
            &context_id,
            now,
            "memory context retrieval handoff recorded with readback evidence",
        );
        self.save(&mut ledger, now)?;
        let context_pack_mutated_by_gate = !context_pack.duplicate_context_id;
        Ok(MemoryContextRetrievalHandoffReport {
            ledger_path: self.path_display(),
            evidence_ledger_path: evidence_ledger.path_display(),
            handoff,
            context_pack,
            duplicate_idempotency_key: false,
            context_pack_mutated_by_gate,
            private_memory_read_by_gate: false,
            external_network_read_by_gate: false,
            persisted: true,
        })
    }

    pub fn gated_local_memory_retrieval(
        &self,
        evidence_ledger: &ReadbackEvidenceLedger,
        input: MemoryContextLocalRetrievalInput,
    ) -> Result<MemoryContextLocalRetrievalReport, HeptaError> {
        let now = current_unix_ms()?;
        let context_id = normalize_scoped_id(&input.context_id, "context id")?;
        let requester = normalize_scoped_id(&input.requester, "requester")?;
        let query = normalize_non_empty(&input.query, "query")?;
        let policy_decision = normalize_non_empty(&input.policy_decision, "policy decision")?;
        let idempotency_key = normalize_non_empty(&input.idempotency_key, "idempotency key")?;
        if !input.operator_confirmed {
            return Err(HeptaError(format!(
                "local memory retrieval for {context_id} requires explicit operator confirmation"
            )));
        }
        if !policy_allows_handoff(&policy_decision) {
            return Err(HeptaError(format!(
                "local memory retrieval for {context_id} requires allow/approved policy decision"
            )));
        }
        let max_citations = normalize_max_citations(input.max_citations)?;
        let allowed_source_paths = normalize_allowed_source_paths(input.allowed_source_paths)?;
        let preflight = self.load_or_default(now)?;
        if let Some(existing) = preflight
            .local_retrievals
            .iter()
            .find(|retrieval| retrieval.idempotency_key == idempotency_key)
            .cloned()
        {
            let context_pack = preflight
                .context_packs
                .iter()
                .find(|pack| pack.context_id == existing.context_id)
                .cloned()
                .ok_or_else(|| {
                    HeptaError(format!(
                        "local memory retrieval {} references missing context pack {}",
                        existing.retrieval_id, existing.context_id
                    ))
                })?;
            return Ok(MemoryContextLocalRetrievalReport {
                ledger_path: self.path_display(),
                evidence_ledger_path: evidence_ledger.path_display(),
                retrieval: existing,
                context_pack: MemoryContextBuildReport {
                    ledger_path: self.path_display(),
                    context_pack,
                    duplicate_context_id: true,
                    persisted: self.path.exists(),
                },
                duplicate_idempotency_key: true,
                private_memory_read_by_adapter: false,
                index_mutated_by_adapter: false,
                external_network_read_by_adapter: false,
                persisted: self.path.exists(),
            });
        }
        let citations =
            collect_matching_memory_citations(&query, &allowed_source_paths, max_citations)?;
        let mut matched_source_hashes = citations
            .iter()
            .map(|citation| stable_preview_hash(&citation.source_path))
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        matched_source_hashes.sort();
        let context_pack = self.build_context_pack(&context_id, &requester, &query, citations)?;
        let mut ledger = self.load_or_default(now)?;
        let retrieval_id = format!(
            "memctxretrieve-{}-{}",
            now,
            ledger.local_retrievals.len() + 1
        );
        let evidence = evidence_ledger.append(
            "memory_context_local_retrieval",
            &retrieval_id,
            "retrieved",
            &format!(
                "local memory retrieval recorded for {context_id}; read scoped allowed sources with no index mutation or network read"
            ),
        )?;
        let retrieval = MemoryContextLocalRetrievalRecord {
            retrieval_id: retrieval_id.clone(),
            context_id: context_id.clone(),
            requester,
            citation_count: context_pack.context_pack.citation_count,
            scanned_source_count: allowed_source_paths.len(),
            query_preview: context_pack.context_pack.query_preview.clone(),
            policy_decision,
            operator_confirmed: input.operator_confirmed,
            idempotency_key,
            matched_source_hashes,
            readback_evidence_id: evidence.entry.evidence_id,
            created_at_unix_ms: now,
            private_memory_read_by_adapter: true,
            index_mutated_by_adapter: false,
            external_network_read_by_adapter: false,
        };
        ledger.local_retrievals.push(retrieval.clone());
        ledger.local_retrievals.truncate(1024);
        push_event(
            &mut ledger,
            "local_retrieval_recorded",
            &context_id,
            now,
            "local memory retrieval recorded with scoped source citations and readback evidence",
        );
        self.save(&mut ledger, now)?;
        Ok(MemoryContextLocalRetrievalReport {
            ledger_path: self.path_display(),
            evidence_ledger_path: evidence_ledger.path_display(),
            retrieval,
            context_pack,
            duplicate_idempotency_key: false,
            private_memory_read_by_adapter: true,
            index_mutated_by_adapter: false,
            external_network_read_by_adapter: false,
            persisted: true,
        })
    }

    fn load_or_default(&self, now_unix_ms: u64) -> Result<MemoryContextLedgerFile, HeptaError> {
        if !self.path.exists() {
            return Ok(MemoryContextLedgerFile {
                version: 1,
                ledger_id: DEFAULT_MEMORY_CONTEXT_LEDGER_ID.into(),
                created_at_unix_ms: now_unix_ms,
                updated_at_unix_ms: now_unix_ms,
                context_packs: Vec::new(),
                retrieval_handoffs: Vec::new(),
                local_retrievals: Vec::new(),
                events: Vec::new(),
            });
        }
        let text = fs::read_to_string(&self.path).map_err(|err| {
            HeptaError(format!(
                "failed to read memory-context ledger {}: {err}",
                self.path.display()
            ))
        })?;
        let mut ledger: MemoryContextLedgerFile = serde_json::from_str(&text).map_err(|err| {
            HeptaError(format!(
                "failed to parse memory-context ledger {}: {err}",
                self.path.display()
            ))
        })?;
        if ledger.version != 1 {
            return Err(HeptaError(format!(
                "unsupported memory-context ledger version {} in {}",
                ledger.version,
                self.path.display()
            )));
        }
        ledger.events.truncate(1024);
        Ok(ledger)
    }

    fn save(
        &self,
        ledger: &mut MemoryContextLedgerFile,
        now_unix_ms: u64,
    ) -> Result<(), HeptaError> {
        ledger.updated_at_unix_ms = now_unix_ms;
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent).map_err(|err| {
                HeptaError(format!(
                    "failed to create memory-context ledger directory {}: {err}",
                    parent.display()
                ))
            })?;
        }
        let text = serde_json::to_string_pretty(ledger).map_err(|err| {
            HeptaError(format!("failed to serialize memory-context ledger: {err}"))
        })?;
        fs::write(&self.path, text).map_err(|err| {
            HeptaError(format!(
                "failed to write memory-context ledger {}: {err}",
                self.path.display()
            ))
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MemoryCitationInput {
    pub source_path: String,
    pub line_start: u32,
    pub line_end: u32,
    pub excerpt: String,
}

fn normalize_citations(
    inputs: Vec<MemoryCitationInput>,
) -> Result<Vec<MemoryCitation>, HeptaError> {
    let mut seen = HashSet::new();
    let mut citations = Vec::new();
    for input in inputs {
        let source_path = normalize_source_path(&input.source_path)?;
        if input.line_start == 0 || input.line_end < input.line_start {
            return Err(HeptaError(format!(
                "invalid citation line range for {source_path}: {}-{}",
                input.line_start, input.line_end
            )));
        }
        let excerpt = normalize_non_empty(&input.excerpt, "excerpt")?;
        let key = format!("{source_path}:{}:{}", input.line_start, input.line_end);
        if !seen.insert(key.clone()) {
            continue;
        }
        citations.push(MemoryCitation {
            source_path,
            line_start: input.line_start,
            line_end: input.line_end,
            excerpt_preview: redact_preview(&excerpt),
            source_hash: stable_preview_hash(&key),
        });
    }
    if citations.is_empty() {
        return Err(HeptaError(
            "memory context pack citations collapsed to empty".into(),
        ));
    }
    Ok(citations)
}

fn collect_matching_memory_citations(
    query: &str,
    allowed_source_paths: &[String],
    max_citations: usize,
) -> Result<Vec<MemoryCitationInput>, HeptaError> {
    let terms = query_terms(query);
    if terms.is_empty() {
        return Err(HeptaError(
            "local memory retrieval query did not contain searchable terms".into(),
        ));
    }
    let mut matches = Vec::<(usize, usize, MemoryCitationInput)>::new();
    for (source_index, source_path) in allowed_source_paths.iter().enumerate() {
        let text = fs::read_to_string(source_path).map_err(|err| {
            HeptaError(format!(
                "failed to read allowed memory source {source_path}: {err}"
            ))
        })?;
        for (line_index, line) in text.lines().enumerate() {
            let score = line_match_score(line, &terms);
            if score == 0 {
                continue;
            }
            let line_number = u32::try_from(line_index + 1).map_err(|_| {
                HeptaError(format!("memory source {source_path} has too many lines"))
            })?;
            matches.push((
                score,
                source_index.saturating_mul(1_000_000) + line_index,
                MemoryCitationInput {
                    source_path: source_path.clone(),
                    line_start: line_number,
                    line_end: line_number,
                    excerpt: line.to_string(),
                },
            ));
        }
    }
    matches.sort_by(|left, right| right.0.cmp(&left.0).then(left.1.cmp(&right.1)));
    let citations = matches
        .into_iter()
        .take(max_citations)
        .map(|(_, _, citation)| citation)
        .collect::<Vec<_>>();
    if citations.is_empty() {
        return Err(HeptaError(
            "local memory retrieval found no matching citations".into(),
        ));
    }
    Ok(citations)
}

fn normalize_allowed_source_paths(inputs: Vec<String>) -> Result<Vec<String>, HeptaError> {
    if inputs.is_empty() {
        return Err(HeptaError(
            "local memory retrieval requires at least one allowed source path".into(),
        ));
    }
    let mut seen = HashSet::new();
    let mut paths = Vec::new();
    for input in inputs {
        let path = normalize_source_path(&input)?;
        if seen.insert(path.clone()) {
            paths.push(path);
        }
    }
    if paths.is_empty() {
        return Err(HeptaError(
            "local memory retrieval allowed source paths collapsed to empty".into(),
        ));
    }
    Ok(paths)
}

fn normalize_max_citations(value: usize) -> Result<usize, HeptaError> {
    if value == 0 {
        return Err(HeptaError(
            "local memory retrieval max_citations must be greater than zero".into(),
        ));
    }
    Ok(value.min(20))
}

fn query_terms(query: &str) -> Vec<String> {
    query
        .split(|ch: char| !ch.is_alphanumeric())
        .map(|term| term.trim().to_ascii_lowercase())
        .filter(|term| term.chars().count() >= 2)
        .take(16)
        .collect()
}

fn line_match_score(line: &str, terms: &[String]) -> usize {
    let line = line.to_ascii_lowercase();
    terms.iter().filter(|term| line.contains(*term)).count()
}

fn normalize_source_path(value: &str) -> Result<String, HeptaError> {
    let value = normalize_non_empty(value, "source path")?;
    if value.contains('\n') || value.contains('\r') || value.contains("..") {
        return Err(HeptaError(
            "memory source path must be single-line and scoped".into(),
        ));
    }
    Ok(value)
}

fn normalize_scoped_id(value: &str, label: &str) -> Result<String, HeptaError> {
    let value = normalize_non_empty(value, label)?;
    if value.contains('\n') || value.contains('\r') || value.contains("..") {
        return Err(HeptaError(format!(
            "memory context {label} must be single-line and scoped"
        )));
    }
    Ok(value)
}

fn normalize_non_empty(value: &str, label: &str) -> Result<String, HeptaError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(HeptaError(format!(
            "memory context {label} must not be empty"
        )));
    }
    Ok(trimmed.to_string())
}

fn policy_allows_handoff(policy_decision: &str) -> bool {
    let policy = policy_decision.to_ascii_lowercase();
    policy.contains("allow") || policy.contains("approved")
}

fn redact_preview(value: &str) -> String {
    let bounded = if value.chars().count() > 240 {
        let mut text: String = value.chars().take(240).collect();
        text.push('…');
        text
    } else {
        value.to_string()
    };
    bounded
        .split_whitespace()
        .map(|part| {
            if part.len() > 56 || part.contains("token=") || part.contains("secret") {
                "<redacted>"
            } else {
                part
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn stable_preview_hash(value: &str) -> String {
    let mut hash: u64 = 14_695_981_039_346_656_037;
    for byte in value.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(1_099_511_628_211);
    }
    format!("{hash:016x}")
}

fn push_event(
    ledger: &mut MemoryContextLedgerFile,
    event_type: &str,
    context_id: &str,
    now_unix_ms: u64,
    summary: &str,
) {
    ledger.events.push(MemoryContextEvent {
        event_id: format!("memctxevt-{}-{}", now_unix_ms, ledger.events.len() + 1),
        event_type: event_type.into(),
        context_id: context_id.into(),
        occurred_at_unix_ms: now_unix_ms,
        summary: summary.into(),
    });
    ledger.events.truncate(1024);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_file(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "hepta-memory-context-test-{}-{}-{name}.json",
            std::process::id(),
            current_unix_ms().unwrap_or(0)
        ))
    }

    #[test]
    fn memory_context_pack_persists_citations_with_redaction() {
        let path = temp_file("pack");
        let ledger = MemoryContextLedger::new(&path);
        let report = ledger
            .build_context_pack(
                "ctx-main",
                "session-main",
                "what did we decide token=secret",
                vec![
                    MemoryCitationInput {
                        source_path: "memory/2026-05-14.md".into(),
                        line_start: 10,
                        line_end: 12,
                        excerpt: "Hepta native runtime secret".into(),
                    },
                    MemoryCitationInput {
                        source_path: "MEMORY.md".into(),
                        line_start: 1,
                        line_end: 1,
                        excerpt: "long-term project note".into(),
                    },
                ],
            )
            .unwrap();
        assert_eq!(report.context_pack.citation_count, 2);
        assert!(report.context_pack.query_preview.contains("<redacted>"));
        assert!(
            report.context_pack.citations[0]
                .excerpt_preview
                .contains("<redacted>")
        );
        let duplicate = ledger
            .build_context_pack(
                "ctx-main",
                "session-main",
                "dupe",
                vec![MemoryCitationInput {
                    source_path: "MEMORY.md".into(),
                    line_start: 1,
                    line_end: 1,
                    excerpt: "dupe".into(),
                }],
            )
            .unwrap();
        assert!(duplicate.duplicate_context_id);
        let ledger_report = ledger.report(None).unwrap();
        assert_eq!(ledger_report.context_pack_count, 1);
        assert_eq!(ledger_report.citation_count, 2);
        let _ = fs::remove_file(path);
    }

    #[test]
    fn memory_context_rejects_empty_or_traversing_sources() {
        let path = temp_file("reject");
        let ledger = MemoryContextLedger::new(&path);
        assert!(
            ledger
                .build_context_pack("ctx", "session", "query", vec![])
                .is_err()
        );
        assert!(
            ledger
                .build_context_pack(
                    "ctx2",
                    "session",
                    "query",
                    vec![MemoryCitationInput {
                        source_path: "../MEMORY.md".into(),
                        line_start: 1,
                        line_end: 2,
                        excerpt: "bad".into(),
                    }],
                )
                .is_err()
        );
        let _ = fs::remove_file(path);
    }

    #[test]
    fn memory_context_gated_retrieval_handoff_records_citation_readback_without_reads() {
        use crate::ReadbackEvidenceLedger;

        let path = temp_file("handoff");
        let ledger_path = temp_file("handoff-ledger");
        let ledger = MemoryContextLedger::new(&path);
        let evidence = ReadbackEvidenceLedger::new(&ledger_path);
        let input = MemoryContextRetrievalHandoffInput {
            context_id: "ctx-handoff".into(),
            requester: "session-main".into(),
            query: "find previous decision token=secret".into(),
            citations: vec![MemoryCitationInput {
                source_path: "memory/2026-05-15.md".into(),
                line_start: 1,
                line_end: 3,
                excerpt: "remember secret".into(),
            }],
            policy_decision: "approved-memory-context".into(),
            operator_confirmed: true,
            idempotency_key: "memory-context-handoff-1".into(),
        };
        let unconfirmed = MemoryContextRetrievalHandoffInput {
            operator_confirmed: false,
            ..input.clone()
        };
        assert!(
            ledger
                .gated_retrieval_handoff(&evidence, unconfirmed)
                .is_err()
        );
        let handoff = ledger
            .gated_retrieval_handoff(&evidence, input.clone())
            .unwrap();
        assert!(handoff.context_pack_mutated_by_gate);
        assert!(!handoff.private_memory_read_by_gate);
        assert!(!handoff.external_network_read_by_gate);
        assert_eq!(handoff.handoff.citation_count, 1);
        assert!(handoff.handoff.query_preview.contains("<redacted>"));
        let duplicate = ledger.gated_retrieval_handoff(&evidence, input).unwrap();
        assert!(duplicate.duplicate_idempotency_key);
        assert!(!duplicate.context_pack_mutated_by_gate);
        let report = ledger.report(None).unwrap();
        assert_eq!(report.context_pack_count, 1);
        assert_eq!(report.ledger.retrieval_handoffs.len(), 1);
        let readback = evidence.report(None).unwrap();
        assert_eq!(readback.evidence_count, 1);
        let _ = fs::remove_file(path);
        let _ = fs::remove_file(ledger_path);
    }

    #[test]
    fn memory_context_gated_local_retrieval_reads_allowed_sources_with_readback() {
        use crate::ReadbackEvidenceLedger;

        let source_path = temp_file("source.md");
        fs::write(
            &source_path,
            "unrelated note\nlaunch blocker resolved with operator evidence\nsecret token=abc\nsecond launch blocker citation\n",
        )
        .unwrap();
        let path = temp_file("local-retrieval");
        let ledger_path = temp_file("local-retrieval-ledger");
        let ledger = MemoryContextLedger::new(&path);
        let evidence = ReadbackEvidenceLedger::new(&ledger_path);
        let input = MemoryContextLocalRetrievalInput {
            context_id: "ctx-local-search".into(),
            requester: "session-main".into(),
            query: "launch blocker".into(),
            allowed_source_paths: vec![source_path.display().to_string()],
            max_citations: 2,
            policy_decision: "allow-local-memory-search".into(),
            operator_confirmed: true,
            idempotency_key: "memory-local-search-1".into(),
        };
        let unconfirmed = MemoryContextLocalRetrievalInput {
            operator_confirmed: false,
            ..input.clone()
        };
        assert!(
            ledger
                .gated_local_memory_retrieval(&evidence, unconfirmed)
                .is_err()
        );
        let retrieval = ledger
            .gated_local_memory_retrieval(&evidence, input.clone())
            .unwrap();
        assert!(retrieval.private_memory_read_by_adapter);
        assert!(!retrieval.index_mutated_by_adapter);
        assert!(!retrieval.external_network_read_by_adapter);
        assert_eq!(retrieval.retrieval.scanned_source_count, 1);
        assert_eq!(retrieval.retrieval.citation_count, 2);
        assert_eq!(retrieval.context_pack.context_pack.citation_count, 2);
        assert!(
            retrieval
                .context_pack
                .context_pack
                .citations
                .iter()
                .all(|citation| citation.source_path == source_path.display().to_string())
        );
        let duplicate = ledger
            .gated_local_memory_retrieval(&evidence, input)
            .unwrap();
        assert!(duplicate.duplicate_idempotency_key);
        assert!(!duplicate.private_memory_read_by_adapter);
        let report = ledger.report(None).unwrap();
        assert_eq!(report.context_pack_count, 1);
        assert_eq!(report.ledger.local_retrievals.len(), 1);
        let readback = evidence.report(None).unwrap();
        assert_eq!(readback.evidence_count, 1);
        let _ = fs::remove_file(source_path);
        let _ = fs::remove_file(path);
        let _ = fs::remove_file(ledger_path);
    }
}
