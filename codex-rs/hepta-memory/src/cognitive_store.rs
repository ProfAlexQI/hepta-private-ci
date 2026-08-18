use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use codex_hepta_contracts::AgentId;
use codex_hepta_contracts::Sha256Digest;
use codex_hepta_paths::HeptaAgentLayout;
use codex_state::SqliteConfig;
use codex_utils_absolute_path::AbsolutePathBuf;
use sqlx::Row;
use sqlx::SqlitePool;

use crate::cognitive_model::COGNITIVE_SCHEMA_VERSION;
use crate::cognitive_model::CognitiveAccess;
use crate::cognitive_model::CognitiveScope;
use crate::cognitive_model::MAX_SOURCE_BYTES;
use crate::cognitive_model::SourceDraft;
use crate::cognitive_model::SourceEventId;
use crate::cognitive_model::SourceRevisionId;

const COGNITIVE_DB_FILENAME: &str = "cognitive_1.sqlite3";
static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");

#[derive(Debug, thiserror::Error)]
pub enum CognitiveStoreError {
    #[error("invalid cognitive record: {0}")]
    Invalid(String),
    #[error("cognitive scope denied: {0}")]
    AccessDenied(String),
    #[error("cognitive revision conflict: {0}")]
    Conflict(String),
    #[error("cognitive store is corrupt: {0}")]
    Corrupt(String),
    #[error("cognitive store is unavailable: {0}")]
    Unavailable(String),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Serialize)]
pub struct ProjectionGeneration(pub(crate) u64);

impl ProjectionGeneration {
    pub fn get(self) -> u64 {
        self.0
    }
}

#[derive(Clone)]
pub struct CognitiveStore {
    pub(crate) pool: SqlitePool,
    pub(crate) owner_agent_id: AgentId,
    path: PathBuf,
}

impl CognitiveStore {
    pub(crate) fn from_read_only_pool(
        pool: SqlitePool,
        owner_agent_id: AgentId,
        path: PathBuf,
    ) -> Self {
        Self {
            pool,
            owner_agent_id,
            path,
        }
    }

    pub async fn open(layout: &HeptaAgentLayout) -> Result<Self, CognitiveStoreError> {
        let root = layout.cognitive_root();
        create_private_directory(root)?;
        let path = root.join(COGNITIVE_DB_FILENAME);
        let sqlite_home = AbsolutePathBuf::try_from(root.to_path_buf())
            .map_err(|error| CognitiveStoreError::Invalid(error.to_string()))?;
        let pool = SqliteConfig::from_sqlite_home(sqlite_home)
            .open_durable_evidence_pool(&path)
            .await
            .map_err(unavailable)?;
        MIGRATOR
            .run(&pool)
            .await
            .map_err(|error| CognitiveStoreError::Unavailable(error.to_string()))?;
        protect_database_file(&path)?;
        sqlx::query(
            "INSERT INTO cognitive_meta (singleton, schema_version, owner_agent_id)
             VALUES (1, ?, ?) ON CONFLICT(singleton) DO NOTHING",
        )
        .bind(i64::from(COGNITIVE_SCHEMA_VERSION))
        .bind(layout.agent_id().as_str())
        .execute(&pool)
        .await
        .map_err(unavailable)?;
        verify_store(&pool, layout.agent_id()).await?;
        Ok(Self {
            pool,
            owner_agent_id: layout.agent_id().clone(),
            path,
        })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn owner_agent_id(&self) -> &AgentId {
        &self.owner_agent_id
    }

    pub async fn append_source(
        &self,
        access: &CognitiveAccess,
        draft: &SourceDraft,
    ) -> Result<SourceRevisionId, CognitiveStoreError> {
        self.authorize(access, &draft.scope)?;
        validate_key(&draft.event_key, "source event key")?;
        if draft.content.is_empty() || draft.content.len() > MAX_SOURCE_BYTES {
            return Err(CognitiveStoreError::Invalid(format!(
                "source content must contain 1..={MAX_SOURCE_BYTES} bytes"
            )));
        }
        let source_id = SourceEventId::for_event(
            &self.owner_agent_id,
            &draft.scope,
            draft.kind,
            &draft.event_key,
        );
        let content_sha256 = Sha256Digest::for_bytes(&draft.content);
        let (scope_kind, workspace_sha256) = draft.scope.database_parts();
        let recorded_at = now_unix_seconds()?;
        let mut transaction = self.pool.begin().await.map_err(unavailable)?;
        let insert = sqlx::query(
            "INSERT INTO source_ledger (
                source_id, source_revision, owner_agent_id, scope_kind, workspace_sha256,
                source_kind, content, content_sha256, observed_at_unix_seconds,
                recorded_at_unix_seconds
             ) VALUES (?, 1, ?, ?, ?, ?, ?, ?, ?, ?)
             ON CONFLICT(source_id, source_revision) DO NOTHING",
        )
        .bind(source_id.as_str())
        .bind(self.owner_agent_id.as_str())
        .bind(scope_kind)
        .bind(workspace_sha256)
        .bind(draft.kind.as_str())
        .bind(&draft.content)
        .bind(content_sha256.as_str())
        .bind(draft.observed_at_unix_seconds)
        .bind(recorded_at)
        .execute(&mut *transaction)
        .await
        .map_err(unavailable)?;
        if insert.rows_affected() == 0 {
            let exact: i64 = sqlx::query_scalar(
                "SELECT COUNT(*) FROM source_ledger
                 WHERE source_id = ? AND source_revision = 1 AND owner_agent_id = ?
                   AND scope_kind = ? AND workspace_sha256 IS ? AND source_kind = ?
                   AND content = ? AND content_sha256 = ? AND observed_at_unix_seconds = ?",
            )
            .bind(source_id.as_str())
            .bind(self.owner_agent_id.as_str())
            .bind(scope_kind)
            .bind(workspace_sha256)
            .bind(draft.kind.as_str())
            .bind(&draft.content)
            .bind(content_sha256.as_str())
            .bind(draft.observed_at_unix_seconds)
            .fetch_one(&mut *transaction)
            .await
            .map_err(unavailable)?;
            if exact != 1 {
                return Err(CognitiveStoreError::Conflict(format!(
                    "source event {} was replayed with different content",
                    source_id.as_str()
                )));
            }
        }
        transaction.commit().await.map_err(unavailable)?;
        Ok(SourceRevisionId::new(source_id))
    }

    pub(crate) fn authorize(
        &self,
        access: &CognitiveAccess,
        scope: &CognitiveScope,
    ) -> Result<(), CognitiveStoreError> {
        if access.agent_id() != &self.owner_agent_id {
            return Err(CognitiveStoreError::AccessDenied(
                "cross-agent cognitive access requires an executable federation capability"
                    .to_string(),
            ));
        }
        if !scope.permits(access) {
            return Err(CognitiveStoreError::AccessDenied(
                "workspace-private cognitive record does not match the caller workspace"
                    .to_string(),
            ));
        }
        Ok(())
    }
}

pub(crate) fn validate_key(value: &str, label: &str) -> Result<(), CognitiveStoreError> {
    if value.trim().is_empty() || value.len() > 512 || value.as_bytes().contains(&0) {
        return Err(CognitiveStoreError::Invalid(format!(
            "{label} must contain 1..=512 non-NUL bytes"
        )));
    }
    Ok(())
}

pub(crate) fn decode_scope(
    row: &sqlx::sqlite::SqliteRow,
) -> Result<CognitiveScope, CognitiveStoreError> {
    CognitiveScope::parse(
        row.try_get("scope_kind").map_err(unavailable)?,
        row.try_get("workspace_sha256").map_err(unavailable)?,
    )
    .map_err(CognitiveStoreError::Corrupt)
}

pub(crate) fn unavailable(error: impl std::fmt::Display) -> CognitiveStoreError {
    CognitiveStoreError::Unavailable(error.to_string())
}

fn now_unix_seconds() -> Result<i64, CognitiveStoreError> {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| CognitiveStoreError::Unavailable(error.to_string()))?
        .as_secs();
    i64::try_from(seconds)
        .map_err(|_| CognitiveStoreError::Unavailable("system clock overflow".to_string()))
}

async fn verify_store(pool: &SqlitePool, owner: &AgentId) -> Result<(), CognitiveStoreError> {
    let quick_check = sqlx::query_scalar::<_, String>("PRAGMA quick_check(1)")
        .fetch_all(pool)
        .await
        .map_err(unavailable)?;
    if quick_check != ["ok"] {
        return Err(CognitiveStoreError::Corrupt(
            "SQLite quick_check rejected the cognitive store".to_string(),
        ));
    }
    let foreign_key_errors = sqlx::query("PRAGMA foreign_key_check")
        .fetch_all(pool)
        .await
        .map_err(unavailable)?;
    if !foreign_key_errors.is_empty() {
        return Err(CognitiveStoreError::Corrupt(
            "SQLite foreign_key_check rejected the cognitive store".to_string(),
        ));
    }
    let required_schema_objects: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM sqlite_schema WHERE name IN (
            'source_ledger_no_update', 'source_ledger_no_delete',
            'memory_revisions_no_update', 'memory_revisions_no_delete',
            'memory_citations_no_update', 'memory_citations_no_delete',
            'memory_fts', 'kg_entity_fts'
         )",
    )
    .fetch_one(pool)
    .await
    .map_err(unavailable)?;
    if required_schema_objects != 8 {
        return Err(CognitiveStoreError::Corrupt(
            "required immutable-storage or FTS5 schema objects are missing".to_string(),
        ));
    }
    let row = sqlx::query(
        "SELECT schema_version, owner_agent_id FROM cognitive_meta WHERE singleton = 1",
    )
    .fetch_one(pool)
    .await
    .map_err(unavailable)?;
    let schema_version: i64 = row.try_get("schema_version").map_err(unavailable)?;
    let stored_owner: String = row.try_get("owner_agent_id").map_err(unavailable)?;
    if schema_version != i64::from(COGNITIVE_SCHEMA_VERSION) {
        return Err(CognitiveStoreError::Corrupt(format!(
            "unsupported cognitive schema version {schema_version}"
        )));
    }
    if stored_owner != owner.as_str() {
        return Err(CognitiveStoreError::AccessDenied(format!(
            "cognitive database belongs to agent {stored_owner}, not {owner}"
        )));
    }
    Ok(())
}

fn create_private_directory(path: &Path) -> Result<(), CognitiveStoreError> {
    fs::create_dir_all(path).map_err(unavailable)?;
    if path.canonicalize().map_err(unavailable)? != path {
        return Err(CognitiveStoreError::Invalid(
            "per-agent cognitive root must be canonical and must not traverse a symlink"
                .to_string(),
        ));
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700)).map_err(unavailable)?;
    }
    Ok(())
}

fn protect_database_file(path: &Path) -> Result<(), CognitiveStoreError> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o600)).map_err(unavailable)?;
    }
    Ok(())
}
