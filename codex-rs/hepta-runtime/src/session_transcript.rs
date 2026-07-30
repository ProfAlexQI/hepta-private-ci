use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::Hash;
use std::hash::Hasher;
use std::path::PathBuf;

use hepta_core::HeptaError;
use serde::Deserialize;
use serde::Serialize;

use crate::current_unix_ms;
use crate::delivery_queue::ReadbackEvidenceLedger;

pub const DEFAULT_SESSION_TRANSCRIPT_PATH: &str = ".hepta/session-transcripts-v0.json";
pub const DEFAULT_SESSION_TRANSCRIPT_STORE_ID: &str = "hepta-native-session-transcripts";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SessionTranscriptStatus {
    Active,
    Compacted,
    Archived,
}

impl SessionTranscriptStatus {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Compacted => "compacted",
            Self::Archived => "archived",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionTranscriptStoreFile {
    pub version: u32,
    pub store_id: String,
    pub created_at_unix_ms: u64,
    pub updated_at_unix_ms: u64,
    #[serde(default = "default_next_sequence")]
    pub next_sequence: u64,
    #[serde(default)]
    pub sessions: Vec<SessionTranscriptRecord>,
    #[serde(default)]
    pub handoffs: Vec<SessionTranscriptAppendHandoffRecord>,
    #[serde(default)]
    pub events: Vec<SessionTranscriptEvent>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionTranscriptRecord {
    pub session_id: String,
    pub title: String,
    pub status: SessionTranscriptStatus,
    pub turn_count: usize,
    pub compacted_turn_count: usize,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary_preview: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub readback_evidence_id: Option<String>,
    #[serde(default)]
    pub turns: Vec<SessionTranscriptTurn>,
    pub created_at_unix_ms: u64,
    pub updated_at_unix_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionTranscriptTurn {
    pub turn_id: String,
    #[serde(default)]
    pub sequence: u64,
    pub role: String,
    #[serde(default = "default_turn_kind")]
    pub turn_kind: SessionTranscriptTurnKind,
    pub text_preview: String,
    pub provenance_hash: String,
    pub created_at_unix_ms: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionTranscriptTurnKind {
    System,
    User,
    AssistantFinal,
    ToolProgressDraft,
    ToolResult,
}

impl SessionTranscriptTurnKind {
    pub const fn label(self) -> &'static str {
        match self {
            Self::System => "system",
            Self::User => "user",
            Self::AssistantFinal => "assistant_final",
            Self::ToolProgressDraft => "tool_progress_draft",
            Self::ToolResult => "tool_result",
        }
    }

    pub const fn is_progress_only(self) -> bool {
        matches!(self, Self::ToolProgressDraft)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionTranscriptAppendHandoffRecord {
    pub handoff_id: String,
    pub session_id: String,
    pub turn_id: String,
    #[serde(default)]
    pub sequence: u64,
    pub role: String,
    #[serde(default = "default_turn_kind")]
    pub turn_kind: SessionTranscriptTurnKind,
    pub text_preview: String,
    pub policy_decision: String,
    pub operator_confirmed: bool,
    pub idempotency_key: String,
    pub readback_evidence_id: String,
    pub created_at_unix_ms: u64,
    pub transcript_mutated_by_gate: bool,
    pub model_invoked_by_gate: bool,
    pub external_send_performed_by_gate: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionTranscriptEvent {
    pub event_id: String,
    pub event_type: String,
    pub session_id: String,
    pub occurred_at_unix_ms: u64,
    pub summary: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SessionTranscriptReport {
    pub store_path: String,
    pub store: SessionTranscriptStoreFile,
    pub active_count: usize,
    pub compacted_count: usize,
    pub archived_count: usize,
    pub session_count: usize,
    pub persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SessionTranscriptSessionReport {
    pub store_path: String,
    pub session: SessionTranscriptRecord,
    pub duplicate_session_id: bool,
    pub persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SessionTranscriptTurnReport {
    pub store_path: String,
    pub session_id: String,
    pub turn: SessionTranscriptTurn,
    pub sequence: u64,
    pub turn_count: usize,
    pub persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SessionTranscriptAppendHandoffInput {
    pub session_id: String,
    pub title: String,
    pub role: String,
    pub text: String,
    pub policy_decision: String,
    pub operator_confirmed: bool,
    pub idempotency_key: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SessionTranscriptAppendHandoffReport {
    pub store_path: String,
    pub evidence_ledger_path: String,
    pub handoff: SessionTranscriptAppendHandoffRecord,
    pub duplicate_idempotency_key: bool,
    pub transcript_store_mutated_by_gate: bool,
    pub model_invoked_by_gate: bool,
    pub external_send_performed_by_gate: bool,
    pub persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SessionTranscriptWindowTurn {
    pub session_id: String,
    pub sequence: u64,
    pub turn_id: String,
    pub role: String,
    pub turn_kind: SessionTranscriptTurnKind,
    pub text_preview: String,
    pub provenance_hash: String,
    pub created_at_unix_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SessionTranscriptReadWindowReport {
    pub store_path: String,
    pub session_id: Option<String>,
    pub start_sequence: Option<u64>,
    pub limit: usize,
    pub reverse: bool,
    pub include_progress: bool,
    pub returned_count: usize,
    pub highest_sequence: u64,
    pub turns: Vec<SessionTranscriptWindowTurn>,
    pub persisted: bool,
}

pub struct SessionTranscriptStore {
    path: PathBuf,
}

impl SessionTranscriptStore {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn default_in_current_dir() -> Result<Self, HeptaError> {
        Ok(Self::new(crate::default_state_path(
            DEFAULT_SESSION_TRANSCRIPT_PATH,
        )?))
    }

    pub fn path_display(&self) -> String {
        self.path.display().to_string()
    }

    pub fn report(&self, now_unix_ms: Option<u64>) -> Result<SessionTranscriptReport, HeptaError> {
        let now = now_unix_ms.unwrap_or(current_unix_ms()?);
        let store = self.load_or_default(now)?;
        Ok(SessionTranscriptReport {
            store_path: self.path_display(),
            active_count: count_status(&store, SessionTranscriptStatus::Active),
            compacted_count: count_status(&store, SessionTranscriptStatus::Compacted),
            archived_count: count_status(&store, SessionTranscriptStatus::Archived),
            session_count: store.sessions.len(),
            persisted: self.path.exists(),
            store,
        })
    }

    pub fn create_session(
        &self,
        session_id: &str,
        title: &str,
    ) -> Result<SessionTranscriptSessionReport, HeptaError> {
        let now = current_unix_ms()?;
        let mut store = self.load_or_default(now)?;
        let session_id = normalize_session_id(session_id)?;
        let title = normalize_non_empty(title, "title")?;
        if let Some(existing) = store
            .sessions
            .iter()
            .find(|session| session.session_id == session_id)
            .cloned()
        {
            return Ok(SessionTranscriptSessionReport {
                store_path: self.path_display(),
                session: existing,
                duplicate_session_id: true,
                persisted: self.path.exists(),
            });
        }
        let session = SessionTranscriptRecord {
            session_id: session_id.clone(),
            title,
            status: SessionTranscriptStatus::Active,
            turn_count: 0,
            compacted_turn_count: 0,
            summary_preview: None,
            readback_evidence_id: None,
            turns: Vec::new(),
            created_at_unix_ms: now,
            updated_at_unix_ms: now,
        };
        store.sessions.push(session.clone());
        push_event(
            &mut store,
            "session_created",
            &session_id,
            now,
            "session transcript created locally",
        );
        self.save(&mut store, now)?;
        Ok(SessionTranscriptSessionReport {
            store_path: self.path_display(),
            session,
            duplicate_session_id: false,
            persisted: true,
        })
    }

    pub fn append_turn(
        &self,
        session_id: &str,
        role: &str,
        text: &str,
    ) -> Result<SessionTranscriptTurnReport, HeptaError> {
        let now = current_unix_ms()?;
        let mut store = self.load_or_default(now)?;
        let session_id = normalize_session_id(session_id)?;
        let role = normalize_role(role)?;
        let text = normalize_non_empty(text, "text")?;
        let turn_kind = default_turn_kind_for_role(&role);
        self.append_turn_inner(&mut store, now, &session_id, &role, turn_kind, &text)
    }

    pub fn append_turn_with_kind(
        &self,
        session_id: &str,
        role: &str,
        turn_kind: SessionTranscriptTurnKind,
        text: &str,
    ) -> Result<SessionTranscriptTurnReport, HeptaError> {
        let now = current_unix_ms()?;
        let mut store = self.load_or_default(now)?;
        let session_id = normalize_session_id(session_id)?;
        let role = normalize_role(role)?;
        let text = normalize_non_empty(text, "text")?;
        self.append_turn_inner(&mut store, now, &session_id, &role, turn_kind, &text)
    }

    fn append_turn_inner(
        &self,
        store: &mut SessionTranscriptStoreFile,
        now: u64,
        session_id: &str,
        role: &str,
        turn_kind: SessionTranscriptTurnKind,
        text: &str,
    ) -> Result<SessionTranscriptTurnReport, HeptaError> {
        ensure_turn_kind_matches_role(role, turn_kind)?;
        let sequence = allocate_sequence(store);
        let session = find_session_mut(store, session_id)?;
        if session.status == SessionTranscriptStatus::Archived {
            return Err(HeptaError(format!(
                "session {session_id} is archived and cannot accept turns"
            )));
        }
        let turn = SessionTranscriptTurn {
            turn_id: format!("turn-{}-{sequence}", now),
            sequence,
            role: role.to_string(),
            turn_kind,
            text_preview: redact_preview(text),
            provenance_hash: stable_hash(&format!("{session_id}\n{sequence}\n{text}\n{now}")),
            created_at_unix_ms: now,
        };
        session.turns.push(turn.clone());
        session.turn_count = session.turns.len();
        session.updated_at_unix_ms = now;
        let turn_count = session.turn_count;
        push_event(
            store,
            "turn_appended",
            session_id,
            now,
            "session transcript turn appended locally",
        );
        self.save(store, now)?;
        Ok(SessionTranscriptTurnReport {
            store_path: self.path_display(),
            session_id: session_id.to_string(),
            turn,
            sequence,
            turn_count,
            persisted: true,
        })
    }

    pub fn compact_session(
        &self,
        session_id: &str,
        summary: &str,
        readback_evidence_id: &str,
    ) -> Result<SessionTranscriptSessionReport, HeptaError> {
        let now = current_unix_ms()?;
        let mut store = self.load_or_default(now)?;
        let session_id = normalize_session_id(session_id)?;
        let summary = normalize_non_empty(summary, "summary")?;
        let readback_evidence_id =
            normalize_non_empty(readback_evidence_id, "readback evidence id")?;
        let session = find_session_mut(&mut store, &session_id)?;
        if session.status == SessionTranscriptStatus::Archived {
            return Err(HeptaError(format!(
                "session {session_id} is archived and cannot be compacted"
            )));
        }
        session.status = SessionTranscriptStatus::Compacted;
        session.compacted_turn_count = session
            .compacted_turn_count
            .saturating_add(session.turns.len());
        session.turns.clear();
        session.turn_count = 0;
        session.summary_preview = Some(redact_preview(&summary));
        session.readback_evidence_id = Some(readback_evidence_id.clone());
        session.updated_at_unix_ms = now;
        let session = session.clone();
        push_event(
            &mut store,
            "session_compacted",
            &session_id,
            now,
            "session transcript compacted with readback evidence",
        );
        self.save(&mut store, now)?;
        Ok(SessionTranscriptSessionReport {
            store_path: self.path_display(),
            session,
            duplicate_session_id: false,
            persisted: true,
        })
    }

    pub fn gated_append_turn_handoff(
        &self,
        evidence_ledger: &ReadbackEvidenceLedger,
        input: SessionTranscriptAppendHandoffInput,
    ) -> Result<SessionTranscriptAppendHandoffReport, HeptaError> {
        let now = current_unix_ms()?;
        let mut store = self.load_or_default(now)?;
        let session_id = normalize_session_id(&input.session_id)?;
        let title = normalize_non_empty(&input.title, "title")?;
        let role = normalize_role(&input.role)?;
        let text = normalize_non_empty(&input.text, "text")?;
        let policy_decision = normalize_non_empty(&input.policy_decision, "policy decision")?;
        let idempotency_key = normalize_non_empty(&input.idempotency_key, "idempotency key")?;
        if !input.operator_confirmed {
            return Err(HeptaError(format!(
                "session transcript handoff for {session_id} requires explicit operator confirmation"
            )));
        }
        if !policy_allows_handoff(&policy_decision) {
            return Err(HeptaError(format!(
                "session transcript handoff for {session_id} requires allow/approved policy decision"
            )));
        }
        if let Some(existing) = store
            .handoffs
            .iter()
            .find(|handoff| handoff.idempotency_key == idempotency_key)
            .cloned()
        {
            return Ok(SessionTranscriptAppendHandoffReport {
                store_path: self.path_display(),
                evidence_ledger_path: evidence_ledger.path_display(),
                handoff: existing,
                duplicate_idempotency_key: true,
                transcript_store_mutated_by_gate: false,
                model_invoked_by_gate: false,
                external_send_performed_by_gate: false,
                persisted: self.path.exists(),
            });
        }
        let session_index = if let Some(index) = store
            .sessions
            .iter()
            .position(|session| session.session_id == session_id)
        {
            index
        } else {
            let session = SessionTranscriptRecord {
                session_id: session_id.clone(),
                title,
                status: SessionTranscriptStatus::Active,
                turn_count: 0,
                compacted_turn_count: 0,
                summary_preview: None,
                readback_evidence_id: None,
                turns: Vec::new(),
                created_at_unix_ms: now,
                updated_at_unix_ms: now,
            };
            store.sessions.push(session);
            push_event(
                &mut store,
                "session_created_by_handoff",
                &session_id,
                now,
                "session transcript created by gated local handoff",
            );
            store.sessions.len() - 1
        };
        if store.sessions[session_index].status == SessionTranscriptStatus::Archived {
            return Err(HeptaError(format!(
                "session {session_id} is archived and cannot accept handoff turns"
            )));
        }
        let turn_kind = default_turn_kind_for_role(&role);
        let sequence = allocate_sequence(&mut store);
        let turn = {
            let session = &mut store.sessions[session_index];
            let turn = SessionTranscriptTurn {
                turn_id: format!("turn-{}-{sequence}", now),
                sequence,
                role: role.clone(),
                turn_kind,
                text_preview: redact_preview(&text),
                provenance_hash: stable_hash(&format!("{session_id}\n{sequence}\n{text}\n{now}")),
                created_at_unix_ms: now,
            };
            session.turns.push(turn.clone());
            session.turn_count = session.turns.len();
            session.updated_at_unix_ms = now;
            turn
        };
        let handoff_id = format!("sesshandoff-{}-{}", now, store.handoffs.len() + 1);
        let evidence = evidence_ledger.append(
            "session_transcript_handoff",
            &handoff_id,
            "turn_appended",
            &format!(
                "session transcript turn handoff appended for {session_id}; model invocation and external send not performed by this gate"
            ),
        )?;
        let handoff = SessionTranscriptAppendHandoffRecord {
            handoff_id: handoff_id.clone(),
            session_id: session_id.clone(),
            turn_id: turn.turn_id,
            sequence,
            role,
            turn_kind,
            text_preview: turn.text_preview,
            policy_decision,
            operator_confirmed: input.operator_confirmed,
            idempotency_key,
            readback_evidence_id: evidence.entry.evidence_id,
            created_at_unix_ms: now,
            transcript_mutated_by_gate: true,
            model_invoked_by_gate: false,
            external_send_performed_by_gate: false,
        };
        store.handoffs.push(handoff.clone());
        store.handoffs.truncate(1024);
        push_event(
            &mut store,
            "handoff_turn_appended",
            &session_id,
            now,
            "session transcript handoff appended with readback evidence",
        );
        self.save(&mut store, now)?;
        Ok(SessionTranscriptAppendHandoffReport {
            store_path: self.path_display(),
            evidence_ledger_path: evidence_ledger.path_display(),
            handoff,
            duplicate_idempotency_key: false,
            transcript_store_mutated_by_gate: true,
            model_invoked_by_gate: false,
            external_send_performed_by_gate: false,
            persisted: true,
        })
    }

    pub fn read_window(
        &self,
        session_id: Option<&str>,
        start_sequence: Option<u64>,
        limit: usize,
        reverse: bool,
        include_progress: bool,
    ) -> Result<SessionTranscriptReadWindowReport, HeptaError> {
        let now = current_unix_ms()?;
        let store = self.load_or_default(now)?;
        let session_id = session_id.map(normalize_session_id).transpose()?;
        let limit = limit.clamp(1, 500);
        let mut turns = Vec::new();
        let mut highest_sequence = 0;
        for session in &store.sessions {
            if session_id
                .as_ref()
                .is_some_and(|requested| requested != &session.session_id)
            {
                continue;
            }
            for turn in &session.turns {
                highest_sequence = highest_sequence.max(turn.sequence);
                if start_sequence.is_some_and(|start| turn.sequence < start) {
                    continue;
                }
                if !include_progress && turn.turn_kind.is_progress_only() {
                    continue;
                }
                turns.push(SessionTranscriptWindowTurn {
                    session_id: session.session_id.clone(),
                    sequence: turn.sequence,
                    turn_id: turn.turn_id.clone(),
                    role: turn.role.clone(),
                    turn_kind: turn.turn_kind,
                    text_preview: turn.text_preview.clone(),
                    provenance_hash: turn.provenance_hash.clone(),
                    created_at_unix_ms: turn.created_at_unix_ms,
                });
            }
        }
        turns.sort_by_key(|turn| turn.sequence);
        if reverse {
            turns.reverse();
        }
        turns.truncate(limit);
        Ok(SessionTranscriptReadWindowReport {
            store_path: self.path_display(),
            session_id,
            start_sequence,
            limit,
            reverse,
            include_progress,
            returned_count: turns.len(),
            highest_sequence,
            turns,
            persisted: self.path.exists(),
        })
    }

    fn load_or_default(&self, now_unix_ms: u64) -> Result<SessionTranscriptStoreFile, HeptaError> {
        if !self.path.exists() {
            return Ok(SessionTranscriptStoreFile {
                version: 1,
                store_id: DEFAULT_SESSION_TRANSCRIPT_STORE_ID.into(),
                created_at_unix_ms: now_unix_ms,
                updated_at_unix_ms: now_unix_ms,
                next_sequence: default_next_sequence(),
                sessions: Vec::new(),
                handoffs: Vec::new(),
                events: Vec::new(),
            });
        }
        let text = fs::read_to_string(&self.path).map_err(|err| {
            HeptaError(format!(
                "failed to read session-transcripts {}: {err}",
                self.path.display()
            ))
        })?;
        let mut store: SessionTranscriptStoreFile = serde_json::from_str(&text).map_err(|err| {
            HeptaError(format!(
                "failed to parse session-transcripts {}: {err}",
                self.path.display()
            ))
        })?;
        if store.version != 1 {
            return Err(HeptaError(format!(
                "unsupported session-transcripts version {} in {}",
                store.version,
                self.path.display()
            )));
        }
        repair_sequences(&mut store);
        store.events.truncate(1024);
        Ok(store)
    }

    fn save(
        &self,
        store: &mut SessionTranscriptStoreFile,
        now_unix_ms: u64,
    ) -> Result<(), HeptaError> {
        store.updated_at_unix_ms = now_unix_ms;
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent).map_err(|err| {
                HeptaError(format!(
                    "failed to create session-transcripts directory {}: {err}",
                    parent.display()
                ))
            })?;
        }
        let text = serde_json::to_string_pretty(store)
            .map_err(|err| HeptaError(format!("failed to serialize session-transcripts: {err}")))?;
        fs::write(&self.path, text).map_err(|err| {
            HeptaError(format!(
                "failed to write session-transcripts {}: {err}",
                self.path.display()
            ))
        })
    }
}

fn count_status(store: &SessionTranscriptStoreFile, status: SessionTranscriptStatus) -> usize {
    store
        .sessions
        .iter()
        .filter(|session| session.status == status)
        .count()
}

fn find_session_mut<'a>(
    store: &'a mut SessionTranscriptStoreFile,
    session_id: &str,
) -> Result<&'a mut SessionTranscriptRecord, HeptaError> {
    store
        .sessions
        .iter_mut()
        .find(|session| session.session_id == session_id)
        .ok_or_else(|| HeptaError(format!("session transcript not found: {session_id}")))
}

fn normalize_session_id(value: &str) -> Result<String, HeptaError> {
    let value = normalize_non_empty(value, "session id")?;
    if value.contains('\n') || value.contains('\r') || value.contains("..") {
        return Err(HeptaError(
            "session id must be single-line and scoped".into(),
        ));
    }
    Ok(value)
}

fn normalize_role(value: &str) -> Result<String, HeptaError> {
    let value = normalize_non_empty(value, "role")?;
    match value.as_str() {
        "system" | "user" | "assistant" | "tool" => Ok(value),
        other => Err(HeptaError(format!("unsupported transcript role: {other}"))),
    }
}

fn normalize_non_empty(value: &str, label: &str) -> Result<String, HeptaError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(HeptaError(format!(
            "session transcript {label} must not be empty"
        )));
    }
    Ok(trimmed.to_string())
}

fn policy_allows_handoff(policy_decision: &str) -> bool {
    let policy = policy_decision.to_ascii_lowercase();
    policy.contains("allow") || policy.contains("approved")
}

const fn default_next_sequence() -> u64 {
    1
}

const fn default_turn_kind() -> SessionTranscriptTurnKind {
    SessionTranscriptTurnKind::AssistantFinal
}

fn default_turn_kind_for_role(role: &str) -> SessionTranscriptTurnKind {
    match role {
        "system" => SessionTranscriptTurnKind::System,
        "user" => SessionTranscriptTurnKind::User,
        "tool" => SessionTranscriptTurnKind::ToolResult,
        _ => SessionTranscriptTurnKind::AssistantFinal,
    }
}

fn ensure_turn_kind_matches_role(
    role: &str,
    turn_kind: SessionTranscriptTurnKind,
) -> Result<(), HeptaError> {
    match (role, turn_kind) {
        ("system", SessionTranscriptTurnKind::System)
        | ("user", SessionTranscriptTurnKind::User)
        | ("assistant", SessionTranscriptTurnKind::AssistantFinal)
        | ("assistant", SessionTranscriptTurnKind::ToolProgressDraft)
        | ("tool", SessionTranscriptTurnKind::ToolResult) => Ok(()),
        _ => Err(HeptaError(format!(
            "turn kind {} is not valid for transcript role {role}",
            turn_kind.label()
        ))),
    }
}

fn allocate_sequence(store: &mut SessionTranscriptStoreFile) -> u64 {
    repair_sequences(store);
    let sequence = store.next_sequence.max(default_next_sequence());
    store.next_sequence = sequence.saturating_add(1);
    sequence
}

fn repair_sequences(store: &mut SessionTranscriptStoreFile) {
    let mut highest = store
        .sessions
        .iter()
        .flat_map(|session| session.turns.iter().map(|turn| turn.sequence))
        .max()
        .unwrap_or(0);
    for session in &mut store.sessions {
        for turn in &mut session.turns {
            if turn.sequence == 0 {
                highest = highest.saturating_add(1);
                turn.sequence = highest;
            }
        }
    }
    store.next_sequence = store
        .next_sequence
        .max(highest.saturating_add(1))
        .max(default_next_sequence());
}

fn redact_preview(value: &str) -> String {
    let value = value.trim();
    let preview = if value.chars().count() > 160 {
        value.chars().take(160).collect::<String>()
    } else {
        value.to_string()
    };
    preview
        .split_whitespace()
        .map(|part| {
            if part.len() > 48 || part.contains("token=") || part.contains("secret") {
                "<redacted>"
            } else {
                part
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn stable_hash(value: &str) -> String {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}

fn push_event(
    store: &mut SessionTranscriptStoreFile,
    event_type: &str,
    session_id: &str,
    now_unix_ms: u64,
    summary: &str,
) {
    store.events.push(SessionTranscriptEvent {
        event_id: format!("sesevt-{}-{}", now_unix_ms, store.events.len() + 1),
        event_type: event_type.into(),
        session_id: session_id.into(),
        occurred_at_unix_ms: now_unix_ms,
        summary: summary.into(),
    });
    store.events.truncate(1024);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_file(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "hepta-session-transcript-test-{}-{}-{name}.json",
            std::process::id(),
            current_unix_ms().unwrap_or(0)
        ))
    }

    #[test]
    fn transcript_store_creates_appends_and_compacts_with_readback() {
        let path = temp_file("compact");
        let store = SessionTranscriptStore::new(&path);
        let created = store.create_session("session-main", "Main").unwrap();
        assert_eq!(created.session.status, SessionTranscriptStatus::Active);
        let duplicate = store
            .create_session("session-main", "Main duplicate")
            .unwrap();
        assert!(duplicate.duplicate_session_id);
        let user_turn = store
            .append_turn("session-main", "user", "hello token=secret")
            .unwrap();
        assert_eq!(user_turn.turn_count, 1);
        assert!(user_turn.turn.text_preview.contains("<redacted>"));
        store
            .append_turn("session-main", "assistant", "hi there")
            .unwrap();
        let compacted = store
            .compact_session("session-main", "summary of two turns", "rb-session-1")
            .unwrap();
        assert_eq!(compacted.session.status, SessionTranscriptStatus::Compacted);
        assert_eq!(compacted.session.compacted_turn_count, 2);
        assert_eq!(
            compacted.session.readback_evidence_id.as_deref(),
            Some("rb-session-1")
        );
        let report = store.report(None).unwrap();
        assert_eq!(report.compacted_count, 1);
        let _ = fs::remove_file(path);
    }

    #[test]
    fn transcript_store_rejects_bad_roles_and_traversing_session_ids() {
        let path = temp_file("reject");
        let store = SessionTranscriptStore::new(&path);
        assert!(store.create_session("../bad", "Bad").is_err());
        store.create_session("session-ok", "OK").unwrap();
        assert!(store.append_turn("session-ok", "hacker", "bad").is_err());
        let _ = fs::remove_file(path);
    }

    #[test]
    fn transcript_store_gated_append_handoff_records_readback_without_side_effects() {
        use crate::ReadbackEvidenceLedger;

        let path = temp_file("handoff");
        let ledger_path = temp_file("handoff-ledger");
        let store = SessionTranscriptStore::new(&path);
        let ledger = ReadbackEvidenceLedger::new(&ledger_path);
        let input = SessionTranscriptAppendHandoffInput {
            session_id: "session-inbound".into(),
            title: "Inbound".into(),
            role: "user".into(),
            text: "hello token=secret".into(),
            policy_decision: "approved-session-handoff".into(),
            operator_confirmed: true,
            idempotency_key: "session-handoff-1".into(),
        };
        let denied = SessionTranscriptAppendHandoffInput {
            operator_confirmed: false,
            ..input.clone()
        };
        assert!(store.gated_append_turn_handoff(&ledger, denied).is_err());
        let handoff = store
            .gated_append_turn_handoff(&ledger, input.clone())
            .unwrap();
        assert!(handoff.transcript_store_mutated_by_gate);
        assert!(!handoff.model_invoked_by_gate);
        assert!(!handoff.external_send_performed_by_gate);
        assert!(handoff.handoff.text_preview.contains("<redacted>"));
        let duplicate = store.gated_append_turn_handoff(&ledger, input).unwrap();
        assert!(duplicate.duplicate_idempotency_key);
        assert!(!duplicate.transcript_store_mutated_by_gate);
        let report = store.report(None).unwrap();
        assert_eq!(report.session_count, 1);
        assert_eq!(report.store.handoffs.len(), 1);
        assert_eq!(report.store.sessions[0].turn_count, 1);
        let evidence = ledger.report(None).unwrap();
        assert_eq!(evidence.evidence_count, 1);
        let _ = fs::remove_file(path);
        let _ = fs::remove_file(ledger_path);
    }

    #[test]
    fn transcript_store_assigns_monotonic_sequences_and_filters_progress_drafts() {
        let path = temp_file("sequence-window");
        let store = SessionTranscriptStore::new(&path);
        store.create_session("session-main", "Main").unwrap();
        let user = store
            .append_turn("session-main", "user", "please run the tool")
            .unwrap();
        let progress = store
            .append_turn_with_kind(
                "session-main",
                "assistant",
                SessionTranscriptTurnKind::ToolProgressDraft,
                "running cargo test",
            )
            .unwrap();
        let final_reply = store
            .append_turn("session-main", "assistant", "tests passed")
            .unwrap();

        assert_eq!(user.sequence, 1);
        assert_eq!(progress.sequence, 2);
        assert_eq!(final_reply.sequence, 3);
        assert_eq!(
            progress.turn.turn_kind,
            SessionTranscriptTurnKind::ToolProgressDraft
        );

        let final_only = store
            .read_window(Some("session-main"), Some(1), 10, false, false)
            .unwrap();
        assert_eq!(final_only.returned_count, 2);
        assert_eq!(
            final_only
                .turns
                .iter()
                .map(|turn| turn.sequence)
                .collect::<Vec<_>>(),
            vec![1, 3]
        );
        assert!(
            final_only
                .turns
                .iter()
                .all(|turn| !turn.turn_kind.is_progress_only())
        );

        let with_progress = store
            .read_window(Some("session-main"), Some(1), 10, true, true)
            .unwrap();
        assert_eq!(with_progress.returned_count, 3);
        assert_eq!(with_progress.turns[0].sequence, 3);
        assert_eq!(with_progress.highest_sequence, 3);
        assert!(
            store
                .append_turn_with_kind(
                    "session-main",
                    "user",
                    SessionTranscriptTurnKind::ToolProgressDraft,
                    "bad progress role",
                )
                .is_err()
        );
        let _ = fs::remove_file(path);
    }
}
