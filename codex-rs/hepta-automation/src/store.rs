use std::fs;
use std::path::Path;
use std::path::PathBuf;

use codex_hepta_contracts::AgentId;
use codex_hepta_paths::HeptaAgentLayout;
use codex_state::SqliteConfig;
use codex_utils_absolute_path::AbsolutePathBuf;
use sqlx::Row;
use sqlx::SqlitePool;

use crate::AUTOMATION_SCHEMA_VERSION;
use crate::AutomationError;
use crate::AutomationLease;
use crate::AutomationQueueReceipt;
use crate::AutomationSchedule;
use crate::AutomationTask;
use crate::AutomationTaskDraft;
use crate::AutomationTaskId;
use crate::AutomationTaskState;
use crate::model::client_message_id;

const AUTOMATION_DB_FILENAME: &str = "automation_1.sqlite3";
const MAX_TASK_PAGE: usize = 1_024;
static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");

#[derive(Clone)]
pub struct AutomationStore {
    pool: SqlitePool,
    owner_agent_id: AgentId,
    path: PathBuf,
}

impl AutomationStore {
    pub async fn open(layout: &HeptaAgentLayout) -> Result<Self, AutomationError> {
        Self::open_root(
            layout.automation_root().to_path_buf(),
            layout.agent_id().clone(),
        )
        .await
    }

    async fn open_root(root: PathBuf, owner_agent_id: AgentId) -> Result<Self, AutomationError> {
        create_private_directory(&root)?;
        let path = root.join(AUTOMATION_DB_FILENAME);
        let sqlite_home = AbsolutePathBuf::try_from(root).map_err(|_| AutomationError::Invalid)?;
        let pool = SqliteConfig::from_sqlite_home(sqlite_home)
            .open_durable_evidence_pool(&path)
            .await
            .map_err(unavailable)?;
        if MIGRATOR.run(&pool).await.is_err() {
            pool.close().await;
            return Err(AutomationError::Unavailable);
        }
        protect_database_file(&path)?;
        sqlx::query(
            "INSERT INTO automation_meta (singleton, schema_version, owner_agent_id)
             VALUES (1, ?, ?) ON CONFLICT(singleton) DO NOTHING",
        )
        .bind(i64::from(AUTOMATION_SCHEMA_VERSION))
        .bind(owner_agent_id.as_str())
        .execute(&pool)
        .await
        .map_err(unavailable)?;
        verify_store(&pool, &owner_agent_id).await?;
        Ok(Self {
            pool,
            owner_agent_id,
            path,
        })
    }

    pub fn owner_agent_id(&self) -> &AgentId {
        &self.owner_agent_id
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub async fn close(&self) {
        self.pool.close().await;
    }

    pub async fn create_task(
        &self,
        draft: &AutomationTaskDraft,
    ) -> Result<AutomationTask, AutomationError> {
        draft.validate()?;
        let (schedule_kind, interval_ms) = schedule_columns(draft.schedule)?;
        let result = sqlx::query(
            "INSERT INTO automation_tasks (
                task_id, owner_agent_id, thread_id, prompt, schedule_kind, interval_ms,
                state, next_run_at_ms, next_occurrence, created_at_ms, updated_at_ms
             ) VALUES (?, ?, ?, ?, ?, ?, 'enabled', ?, 1, ?, ?)",
        )
        .bind(draft.task_id.to_string())
        .bind(self.owner_agent_id.as_str())
        .bind(&draft.thread_id)
        .bind(&draft.prompt)
        .bind(schedule_kind)
        .bind(interval_ms.map(to_i64).transpose()?)
        .bind(to_i64(draft.first_run_at_ms)?)
        .bind(to_i64(draft.created_at_ms)?)
        .bind(to_i64(draft.created_at_ms)?)
        .execute(&self.pool)
        .await;
        match result {
            Ok(_) => self
                .task(draft.task_id)
                .await?
                .ok_or(AutomationError::Corrupt),
            Err(error) if is_constraint(&error) => Err(AutomationError::Conflict),
            Err(error) => Err(unavailable(error)),
        }
    }

    pub async fn task(
        &self,
        task_id: AutomationTaskId,
    ) -> Result<Option<AutomationTask>, AutomationError> {
        sqlx::query(TASK_SELECT_BY_ID)
            .bind(task_id.to_string())
            .fetch_optional(&self.pool)
            .await
            .map_err(unavailable)?
            .map(|row| task_from_row(&row, &self.owner_agent_id))
            .transpose()
    }

    pub async fn list_tasks(&self, limit: usize) -> Result<Vec<AutomationTask>, AutomationError> {
        if !(1..=MAX_TASK_PAGE).contains(&limit) {
            return Err(AutomationError::Invalid);
        }
        let rows = sqlx::query(
            "SELECT task_id, owner_agent_id, thread_id, prompt, schedule_kind, interval_ms,
                    state, next_run_at_ms, next_occurrence, created_at_ms, updated_at_ms
             FROM automation_tasks ORDER BY created_at_ms, task_id LIMIT ?",
        )
        .bind(i64::try_from(limit).map_err(|_| AutomationError::Invalid)?)
        .fetch_all(&self.pool)
        .await
        .map_err(unavailable)?;
        rows.iter()
            .map(|row| task_from_row(row, &self.owner_agent_id))
            .collect()
    }

    pub async fn cancel_task(
        &self,
        task_id: AutomationTaskId,
        now_ms: u64,
    ) -> Result<AutomationTask, AutomationError> {
        let mut transaction = self.pool.begin().await.map_err(unavailable)?;
        let changed = sqlx::query(
            "UPDATE automation_tasks
             SET state = 'cancelled', next_run_at_ms = NULL, updated_at_ms = ?
             WHERE task_id = ? AND owner_agent_id = ? AND state IN ('enabled', 'disabled')",
        )
        .bind(to_i64(now_ms)?)
        .bind(task_id.to_string())
        .bind(self.owner_agent_id.as_str())
        .execute(&mut *transaction)
        .await
        .map_err(unavailable)?;
        if changed.rows_affected() != 1 {
            return Err(AutomationError::Conflict);
        }
        sqlx::query(
            "UPDATE automation_runs
             SET state = 'cancelled', lease_generation = NULL, lease_token = NULL,
                 lease_expires_at_ms = NULL
             WHERE task_id = ? AND state = 'pending'",
        )
        .bind(task_id.to_string())
        .execute(&mut *transaction)
        .await
        .map_err(unavailable)?;
        transaction.commit().await.map_err(unavailable)?;
        self.task(task_id).await?.ok_or(AutomationError::Corrupt)
    }

    pub async fn set_enabled(
        &self,
        task_id: AutomationTaskId,
        enabled: bool,
        resume_at_ms: Option<u64>,
        now_ms: u64,
    ) -> Result<AutomationTask, AutomationError> {
        let mut transaction = self.pool.begin().await.map_err(unavailable)?;
        let changed = if enabled {
            let resume_at_ms = resume_at_ms.ok_or(AutomationError::Invalid)?;
            sqlx::query(
                "UPDATE automation_tasks
                 SET state = 'enabled', next_run_at_ms = ?, updated_at_ms = ?
                 WHERE task_id = ? AND owner_agent_id = ? AND state = 'disabled'
                   AND NOT EXISTS (
                       SELECT 1 FROM automation_runs r
                       WHERE r.task_id = automation_tasks.task_id AND r.state = 'leased'
                   )",
            )
            .bind(to_i64(resume_at_ms)?)
            .bind(to_i64(now_ms)?)
            .bind(task_id.to_string())
            .bind(self.owner_agent_id.as_str())
            .execute(&mut *transaction)
            .await
            .map_err(unavailable)?
        } else {
            if resume_at_ms.is_some() {
                return Err(AutomationError::Invalid);
            }
            let changed = sqlx::query(
                "UPDATE automation_tasks
                 SET state = 'disabled', next_run_at_ms = NULL, updated_at_ms = ?
                 WHERE task_id = ? AND owner_agent_id = ? AND state = 'enabled'",
            )
            .bind(to_i64(now_ms)?)
            .bind(task_id.to_string())
            .bind(self.owner_agent_id.as_str())
            .execute(&mut *transaction)
            .await
            .map_err(unavailable)?;
            if changed.rows_affected() == 1 {
                sqlx::query(
                    "UPDATE automation_runs
                     SET state = 'cancelled', lease_generation = NULL, lease_token = NULL,
                         lease_expires_at_ms = NULL
                     WHERE task_id = ? AND state = 'pending'",
                )
                .bind(task_id.to_string())
                .execute(&mut *transaction)
                .await
                .map_err(unavailable)?;
            }
            changed
        };
        if changed.rows_affected() != 1 {
            return Err(AutomationError::Conflict);
        }
        transaction.commit().await.map_err(unavailable)?;
        self.task(task_id).await?.ok_or(AutomationError::Corrupt)
    }

    /// Releases work held by an older process generation. The caller must own
    /// the per-Agent writer lock before invoking this immediate recovery path.
    pub async fn recover_stale_generation(
        &self,
        current_generation: u64,
    ) -> Result<u64, AutomationError> {
        if current_generation == 0 {
            return Err(AutomationError::Invalid);
        }
        let recovered = sqlx::query(
            "UPDATE automation_runs
             SET state = 'pending', lease_generation = NULL, lease_token = NULL,
                 lease_expires_at_ms = NULL
             WHERE state = 'leased' AND lease_generation != ?",
        )
        .bind(to_i64(current_generation)?)
        .execute(&self.pool)
        .await
        .map_err(unavailable)?;
        Ok(recovered.rows_affected())
    }

    pub async fn claim_due(
        &self,
        now_ms: u64,
        generation: u64,
        lease_duration_ms: u64,
    ) -> Result<Option<AutomationLease>, AutomationError> {
        if generation == 0 || lease_duration_ms == 0 {
            return Err(AutomationError::Invalid);
        }
        let lease_expires_at_ms = now_ms
            .checked_add(lease_duration_ms)
            .ok_or(AutomationError::Invalid)?;
        let mut transaction = self.pool.begin().await.map_err(unavailable)?;

        let reclaim = sqlx::query(
            "SELECT r.task_id, r.occurrence, r.scheduled_for_ms, r.client_user_message_id
             FROM automation_runs r
             JOIN automation_tasks t ON t.task_id = r.task_id
             WHERE t.owner_agent_id = ? AND t.state = 'enabled'
               AND (r.state = 'pending'
                    OR (r.state = 'leased' AND r.lease_expires_at_ms <= ?))
             ORDER BY r.scheduled_for_ms, r.task_id, r.occurrence LIMIT 1",
        )
        .bind(self.owner_agent_id.as_str())
        .bind(to_i64(now_ms)?)
        .fetch_optional(&mut *transaction)
        .await
        .map_err(unavailable)?;

        let (task_id, occurrence, scheduled_for_ms, client_id) = if let Some(row) = reclaim {
            let task_id =
                AutomationTaskId::parse(&row.try_get::<String, _>("task_id").map_err(unavailable)?)
                    .map_err(|_| AutomationError::Corrupt)?;
            (
                task_id,
                to_u64(row.try_get("occurrence").map_err(unavailable)?)?,
                to_u64(row.try_get("scheduled_for_ms").map_err(unavailable)?)?,
                row.try_get("client_user_message_id").map_err(unavailable)?,
            )
        } else {
            let row = sqlx::query(
                "SELECT t.task_id, t.next_occurrence, t.next_run_at_ms
                 FROM automation_tasks t
                 WHERE t.owner_agent_id = ? AND t.state = 'enabled'
                   AND t.next_run_at_ms IS NOT NULL AND t.next_run_at_ms <= ?
                   AND NOT EXISTS (
                       SELECT 1 FROM automation_runs r
                       WHERE r.task_id = t.task_id AND r.state IN ('pending', 'leased')
                   )
                 ORDER BY t.next_run_at_ms, t.task_id LIMIT 1",
            )
            .bind(self.owner_agent_id.as_str())
            .bind(to_i64(now_ms)?)
            .fetch_optional(&mut *transaction)
            .await
            .map_err(unavailable)?;
            let Some(row) = row else {
                transaction.commit().await.map_err(unavailable)?;
                return Ok(None);
            };
            let task_id =
                AutomationTaskId::parse(&row.try_get::<String, _>("task_id").map_err(unavailable)?)
                    .map_err(|_| AutomationError::Corrupt)?;
            let occurrence = to_u64(row.try_get("next_occurrence").map_err(unavailable)?)?;
            let scheduled_for_ms = to_u64(row.try_get("next_run_at_ms").map_err(unavailable)?)?;
            let client_id = client_message_id(&self.owner_agent_id, task_id, occurrence);
            sqlx::query(
                "INSERT INTO automation_runs (
                    task_id, occurrence, scheduled_for_ms, client_user_message_id, state
                 ) VALUES (?, ?, ?, ?, 'pending')",
            )
            .bind(task_id.to_string())
            .bind(to_i64(occurrence)?)
            .bind(to_i64(scheduled_for_ms)?)
            .bind(&client_id)
            .execute(&mut *transaction)
            .await
            .map_err(unavailable)?;
            let advanced = sqlx::query(
                "UPDATE automation_tasks SET next_occurrence = next_occurrence + 1
                 WHERE task_id = ? AND next_occurrence = ?",
            )
            .bind(task_id.to_string())
            .bind(to_i64(occurrence)?)
            .execute(&mut *transaction)
            .await
            .map_err(unavailable)?;
            if advanced.rows_affected() != 1 {
                return Err(AutomationError::Conflict);
            }
            (task_id, occurrence, scheduled_for_ms, client_id)
        };

        let lease_token = uuid::Uuid::now_v7().to_string();
        let leased = sqlx::query(
            "UPDATE automation_runs
             SET state = 'leased', lease_generation = ?, lease_token = ?, lease_expires_at_ms = ?
             WHERE task_id = ? AND occurrence = ?
               AND (state = 'pending' OR (state = 'leased' AND lease_expires_at_ms <= ?))",
        )
        .bind(to_i64(generation)?)
        .bind(&lease_token)
        .bind(to_i64(lease_expires_at_ms)?)
        .bind(task_id.to_string())
        .bind(to_i64(occurrence)?)
        .bind(to_i64(now_ms)?)
        .execute(&mut *transaction)
        .await
        .map_err(unavailable)?;
        if leased.rows_affected() != 1 {
            return Err(AutomationError::Conflict);
        }
        let task_row = sqlx::query(TASK_SELECT_BY_ID)
            .bind(task_id.to_string())
            .fetch_one(&mut *transaction)
            .await
            .map_err(unavailable)?;
        let task = task_from_row(&task_row, &self.owner_agent_id)?;
        transaction.commit().await.map_err(unavailable)?;
        Ok(Some(AutomationLease {
            task,
            occurrence,
            scheduled_for_ms,
            client_user_message_id: client_id,
            lease_generation: generation,
            lease_token,
            lease_expires_at_ms,
        }))
    }

    pub async fn mark_submitted(
        &self,
        lease: &AutomationLease,
        receipt: &AutomationQueueReceipt,
        submitted_at_ms: u64,
    ) -> Result<AutomationTask, AutomationError> {
        if lease.task.owner_agent_id != self.owner_agent_id
            || receipt.client_user_message_id != lease.client_user_message_id
            || receipt.queued_submission_id.is_empty()
        {
            return Err(AutomationError::AccessDenied);
        }
        let mut transaction = self.pool.begin().await.map_err(unavailable)?;
        let run = sqlx::query(
            "UPDATE automation_runs
             SET state = 'submitted', lease_generation = NULL, lease_token = NULL,
                 lease_expires_at_ms = NULL, queued_submission_id = ?, submitted_at_ms = ?
             WHERE task_id = ? AND occurrence = ? AND state = 'leased'
               AND lease_generation = ? AND lease_token = ?
               AND client_user_message_id = ?",
        )
        .bind(&receipt.queued_submission_id)
        .bind(to_i64(submitted_at_ms)?)
        .bind(lease.task.task_id.to_string())
        .bind(to_i64(lease.occurrence)?)
        .bind(to_i64(lease.lease_generation)?)
        .bind(&lease.lease_token)
        .bind(&lease.client_user_message_id)
        .execute(&mut *transaction)
        .await
        .map_err(unavailable)?;
        if run.rows_affected() != 1 {
            return Err(AutomationError::Conflict);
        }

        // Control operations can disable or cancel a task while an already admitted
        // occurrence is in flight. The queue admission cannot be revoked after the
        // App Server accepts it, but its completion must never resurrect the task or
        // overwrite a later control-plane decision from the stale lease snapshot.
        let current_row = sqlx::query(TASK_SELECT_BY_ID)
            .bind(lease.task.task_id.to_string())
            .fetch_one(&mut *transaction)
            .await
            .map_err(unavailable)?;
        let current = task_from_row(&current_row, &self.owner_agent_id)?;
        let (next_state, next_run) = match current.state {
            AutomationTaskState::Enabled => {
                let next_run = current.schedule.next_after(lease.scheduled_for_ms)?;
                let next_state = if next_run.is_some() {
                    AutomationTaskState::Enabled
                } else {
                    AutomationTaskState::Completed
                };
                (next_state, next_run)
            }
            AutomationTaskState::Disabled
            | AutomationTaskState::Cancelled
            | AutomationTaskState::Completed => (current.state, None),
        };
        let updated = sqlx::query(
            "UPDATE automation_tasks
             SET state = ?, next_run_at_ms = ?, updated_at_ms = ?
             WHERE task_id = ? AND owner_agent_id = ?",
        )
        .bind(next_state.as_str())
        .bind(next_run.map(to_i64).transpose()?)
        .bind(to_i64(submitted_at_ms)?)
        .bind(lease.task.task_id.to_string())
        .bind(self.owner_agent_id.as_str())
        .execute(&mut *transaction)
        .await
        .map_err(unavailable)?;
        if updated.rows_affected() != 1 {
            return Err(AutomationError::Corrupt);
        }
        transaction.commit().await.map_err(unavailable)?;
        self.task(lease.task.task_id)
            .await?
            .ok_or(AutomationError::Corrupt)
    }

    pub async fn release_for_retry(&self, lease: &AutomationLease) -> Result<(), AutomationError> {
        if lease.task.owner_agent_id != self.owner_agent_id {
            return Err(AutomationError::AccessDenied);
        }
        let updated = sqlx::query(
            "UPDATE automation_runs
             SET state = 'pending', lease_generation = NULL, lease_token = NULL,
                 lease_expires_at_ms = NULL
             WHERE task_id = ? AND occurrence = ? AND state = 'leased'
               AND lease_generation = ? AND lease_token = ?",
        )
        .bind(lease.task.task_id.to_string())
        .bind(to_i64(lease.occurrence)?)
        .bind(to_i64(lease.lease_generation)?)
        .bind(&lease.lease_token)
        .execute(&self.pool)
        .await
        .map_err(unavailable)?;
        if updated.rows_affected() != 1 {
            return Err(AutomationError::Conflict);
        }
        Ok(())
    }
}

const TASK_SELECT_BY_ID: &str =
    "SELECT task_id, owner_agent_id, thread_id, prompt, schedule_kind, interval_ms,
            state, next_run_at_ms, next_occurrence, created_at_ms, updated_at_ms
     FROM automation_tasks WHERE task_id = ?";

fn task_from_row(
    row: &sqlx::sqlite::SqliteRow,
    expected_owner: &AgentId,
) -> Result<AutomationTask, AutomationError> {
    let owner = AgentId::parse(
        row.try_get::<String, _>("owner_agent_id")
            .map_err(unavailable)?,
    )
    .map_err(|_| AutomationError::Corrupt)?;
    if &owner != expected_owner {
        return Err(AutomationError::AccessDenied);
    }
    let task_id =
        AutomationTaskId::parse(&row.try_get::<String, _>("task_id").map_err(unavailable)?)
            .map_err(|_| AutomationError::Corrupt)?;
    let kind: String = row.try_get("schedule_kind").map_err(unavailable)?;
    let interval: Option<i64> = row.try_get("interval_ms").map_err(unavailable)?;
    let schedule = match (kind.as_str(), interval) {
        ("once", None) => AutomationSchedule::Once,
        ("fixed_interval", Some(interval)) => AutomationSchedule::FixedInterval {
            interval_ms: to_u64(interval)?,
        },
        _ => return Err(AutomationError::Corrupt),
    };
    schedule.validate().map_err(|_| AutomationError::Corrupt)?;
    Ok(AutomationTask {
        task_id,
        owner_agent_id: owner,
        thread_id: row.try_get("thread_id").map_err(unavailable)?,
        prompt: row.try_get("prompt").map_err(unavailable)?,
        schedule,
        state: AutomationTaskState::parse(
            &row.try_get::<String, _>("state").map_err(unavailable)?,
        )?,
        next_run_at_ms: row
            .try_get::<Option<i64>, _>("next_run_at_ms")
            .map_err(unavailable)?
            .map(to_u64)
            .transpose()?,
        next_occurrence: to_u64(row.try_get("next_occurrence").map_err(unavailable)?)?,
        created_at_ms: to_u64(row.try_get("created_at_ms").map_err(unavailable)?)?,
        updated_at_ms: to_u64(row.try_get("updated_at_ms").map_err(unavailable)?)?,
    })
}

fn schedule_columns(
    schedule: AutomationSchedule,
) -> Result<(&'static str, Option<u64>), AutomationError> {
    schedule.validate()?;
    Ok(match schedule {
        AutomationSchedule::Once => ("once", None),
        AutomationSchedule::FixedInterval { interval_ms } => ("fixed_interval", Some(interval_ms)),
    })
}

async fn verify_store(pool: &SqlitePool, owner_agent_id: &AgentId) -> Result<(), AutomationError> {
    let quick_check = sqlx::query_scalar::<_, String>("PRAGMA quick_check(1)")
        .fetch_all(pool)
        .await
        .map_err(unavailable)?;
    if quick_check != ["ok"] {
        return Err(AutomationError::Corrupt);
    }
    if !sqlx::query("PRAGMA foreign_key_check")
        .fetch_all(pool)
        .await
        .map_err(unavailable)?
        .is_empty()
    {
        return Err(AutomationError::Corrupt);
    }
    let row = sqlx::query(
        "SELECT schema_version, owner_agent_id FROM automation_meta WHERE singleton = 1",
    )
    .fetch_one(pool)
    .await
    .map_err(unavailable)?;
    let schema: i64 = row.try_get("schema_version").map_err(unavailable)?;
    let owner: String = row.try_get("owner_agent_id").map_err(unavailable)?;
    if schema != i64::from(AUTOMATION_SCHEMA_VERSION) {
        return Err(AutomationError::Corrupt);
    }
    if owner != owner_agent_id.as_str() {
        return Err(AutomationError::AccessDenied);
    }
    let foreign: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM automation_tasks WHERE owner_agent_id != ?")
            .bind(owner_agent_id.as_str())
            .fetch_one(pool)
            .await
            .map_err(unavailable)?;
    if foreign != 0 {
        return Err(AutomationError::AccessDenied);
    }
    Ok(())
}

fn is_constraint(error: &sqlx::Error) -> bool {
    matches!(error, sqlx::Error::Database(database) if database.is_unique_violation())
}

fn to_i64(value: u64) -> Result<i64, AutomationError> {
    i64::try_from(value).map_err(|_| AutomationError::Invalid)
}

fn to_u64(value: i64) -> Result<u64, AutomationError> {
    u64::try_from(value).map_err(|_| AutomationError::Corrupt)
}

fn unavailable(_error: impl std::fmt::Display) -> AutomationError {
    AutomationError::Unavailable
}

fn create_private_directory(path: &Path) -> Result<(), AutomationError> {
    fs::create_dir_all(path).map_err(unavailable)?;
    if path.canonicalize().map_err(unavailable)? != path {
        return Err(AutomationError::Invalid);
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700)).map_err(unavailable)?;
    }
    Ok(())
}

fn protect_database_file(path: &Path) -> Result<(), AutomationError> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o600)).map_err(unavailable)?;
    }
    Ok(())
}
