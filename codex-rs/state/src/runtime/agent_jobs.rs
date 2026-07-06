use super::agent_jobs_work_graph::AgentJobWorkGraphShadowEventSpec;
use super::agent_jobs_work_graph::append_agent_job_created_shadow_events_tx;
use super::agent_jobs_work_graph::json_value_kind;
use super::*;
use crate::model::AgentJobItemRow;
use serde_json::json;

impl StateRuntime {
    pub async fn create_agent_job(
        &self,
        params: &AgentJobCreateParams,
        items: &[AgentJobItemCreateParams],
    ) -> anyhow::Result<AgentJob> {
        let now = Utc::now().timestamp();
        let input_headers_json = serde_json::to_string(&params.input_headers)?;
        let output_schema_json = params
            .output_schema_json
            .as_ref()
            .map(serde_json::to_string)
            .transpose()?;
        let max_runtime_seconds = params
            .max_runtime_seconds
            .map(i64::try_from)
            .transpose()
            .map_err(|_| anyhow::anyhow!("invalid max_runtime_seconds value"))?;
        let mut tx = self.pool.begin().await?;
        sqlx::query(
            r#"
INSERT INTO agent_jobs (
    id,
    name,
    status,
    instruction,
    auto_export,
    max_runtime_seconds,
    output_schema_json,
    input_headers_json,
    input_csv_path,
    output_csv_path,
    created_at,
    updated_at,
    started_at,
    completed_at,
    last_error
) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, NULL, NULL, NULL)
            "#,
        )
        .bind(params.id.as_str())
        .bind(params.name.as_str())
        .bind(AgentJobStatus::Pending.as_str())
        .bind(params.instruction.as_str())
        .bind(i64::from(params.auto_export))
        .bind(max_runtime_seconds)
        .bind(output_schema_json)
        .bind(input_headers_json)
        .bind(params.input_csv_path.as_str())
        .bind(params.output_csv_path.as_str())
        .bind(now)
        .bind(now)
        .execute(&mut *tx)
        .await?;

        for item in items {
            let row_json = serde_json::to_string(&item.row_json)?;
            sqlx::query(
                r#"
INSERT INTO agent_job_items (
    job_id,
    item_id,
    row_index,
    source_id,
    row_json,
    status,
    assigned_thread_id,
    attempt_count,
    result_json,
    last_error,
    created_at,
    updated_at,
    completed_at,
    reported_at
) VALUES (?, ?, ?, ?, ?, ?, NULL, 0, NULL, NULL, ?, ?, NULL, NULL)
                "#,
            )
            .bind(params.id.as_str())
            .bind(item.item_id.as_str())
            .bind(item.row_index)
            .bind(item.source_id.as_deref())
            .bind(row_json)
            .bind(AgentJobItemStatus::Pending.as_str())
            .bind(now)
            .bind(now)
            .execute(&mut *tx)
            .await?;
        }

        append_agent_job_created_shadow_events_tx(&mut tx, params, items, now).await?;

        tx.commit().await?;

        let job_id = params.id.as_str();
        self.get_agent_job(job_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("failed to load created agent job {job_id}"))
    }

    pub async fn get_agent_job(&self, job_id: &str) -> anyhow::Result<Option<AgentJob>> {
        let row = sqlx::query_as::<_, AgentJobRow>(
            r#"
SELECT
    id,
    name,
    status,
    instruction,
    auto_export,
    max_runtime_seconds,
    output_schema_json,
    input_headers_json,
    input_csv_path,
    output_csv_path,
    created_at,
    updated_at,
    started_at,
    completed_at,
    last_error
FROM agent_jobs
WHERE id = ?
            "#,
        )
        .bind(job_id)
        .fetch_optional(self.pool.as_ref())
        .await?;
        row.map(AgentJob::try_from).transpose()
    }

    pub async fn list_agent_job_items(
        &self,
        job_id: &str,
        status: Option<AgentJobItemStatus>,
        limit: Option<usize>,
    ) -> anyhow::Result<Vec<AgentJobItem>> {
        let mut builder = QueryBuilder::<Sqlite>::new(
            r#"
SELECT
    job_id,
    item_id,
    row_index,
    source_id,
    row_json,
    status,
    assigned_thread_id,
    attempt_count,
    result_json,
    last_error,
    created_at,
    updated_at,
    completed_at,
    reported_at
FROM agent_job_items
WHERE job_id =
            "#,
        );
        builder.push_bind(job_id);
        if let Some(status) = status {
            builder.push(" AND status = ");
            builder.push_bind(status.as_str());
        }
        builder.push(" ORDER BY row_index ASC");
        if let Some(limit) = limit {
            builder.push(" LIMIT ");
            builder.push_bind(limit as i64);
        }
        let rows: Vec<AgentJobItemRow> = builder
            .build_query_as::<AgentJobItemRow>()
            .fetch_all(self.pool.as_ref())
            .await?;
        rows.into_iter().map(AgentJobItem::try_from).collect()
    }

    pub async fn get_agent_job_item(
        &self,
        job_id: &str,
        item_id: &str,
    ) -> anyhow::Result<Option<AgentJobItem>> {
        let row: Option<AgentJobItemRow> = sqlx::query_as::<_, AgentJobItemRow>(
            r#"
SELECT
    job_id,
    item_id,
    row_index,
    source_id,
    row_json,
    status,
    assigned_thread_id,
    attempt_count,
    result_json,
    last_error,
    created_at,
    updated_at,
    completed_at,
    reported_at
FROM agent_job_items
WHERE job_id = ? AND item_id = ?
            "#,
        )
        .bind(job_id)
        .bind(item_id)
        .fetch_optional(self.pool.as_ref())
        .await?;
        row.map(AgentJobItem::try_from).transpose()
    }

    pub async fn mark_agent_job_running(&self, job_id: &str) -> anyhow::Result<()> {
        let now = Utc::now().timestamp();
        let result = sqlx::query(
            r#"
UPDATE agent_jobs
SET
    status = ?,
    updated_at = ?,
    started_at = COALESCE(started_at, ?),
    completed_at = NULL,
    last_error = NULL
WHERE id = ?
            "#,
        )
        .bind(AgentJobStatus::Running.as_str())
        .bind(now)
        .bind(now)
        .bind(job_id)
        .execute(self.pool.as_ref())
        .await?;
        if result.rows_affected() > 0 {
            self.append_agent_job_status_shadow_event(
                job_id,
                AgentJobWorkGraphShadowEventSpec {
                    event_type: "agent_job_running",
                    status: AgentJobStatus::Running.as_str(),
                    summary: "agent job entered running state in shadow WorkGraph event stream",
                    action: "running",
                },
                json!({
                    "shadowOnly": true,
                }),
            )
            .await?;
        }
        Ok(())
    }

    pub async fn mark_agent_job_completed(&self, job_id: &str) -> anyhow::Result<()> {
        let now = Utc::now().timestamp();
        let result = sqlx::query(
            r#"
UPDATE agent_jobs
SET status = ?, updated_at = ?, completed_at = ?, last_error = NULL
WHERE id = ?
            "#,
        )
        .bind(AgentJobStatus::Completed.as_str())
        .bind(now)
        .bind(now)
        .bind(job_id)
        .execute(self.pool.as_ref())
        .await?;
        if result.rows_affected() > 0 {
            self.append_agent_job_status_shadow_event(
                job_id,
                AgentJobWorkGraphShadowEventSpec {
                    event_type: "agent_job_completed",
                    status: AgentJobStatus::Completed.as_str(),
                    summary:
                        "agent job reached completed terminal state in shadow WorkGraph event stream",
                    action: "completed",
                },
                json!({
                    "shadowOnly": true,
                }),
            )
            .await?;
        }
        Ok(())
    }

    pub async fn mark_agent_job_failed(
        &self,
        job_id: &str,
        error_message: &str,
    ) -> anyhow::Result<()> {
        let now = Utc::now().timestamp();
        let result = sqlx::query(
            r#"
UPDATE agent_jobs
SET status = ?, updated_at = ?, completed_at = ?, last_error = ?
WHERE id = ?
            "#,
        )
        .bind(AgentJobStatus::Failed.as_str())
        .bind(now)
        .bind(now)
        .bind(error_message)
        .bind(job_id)
        .execute(self.pool.as_ref())
        .await?;
        if result.rows_affected() > 0 {
            self.append_agent_job_status_shadow_event(
                job_id,
                AgentJobWorkGraphShadowEventSpec {
                    event_type: "agent_job_failed",
                    status: AgentJobStatus::Failed.as_str(),
                    summary: "agent job reached failed terminal state in shadow WorkGraph event stream",
                    action: "failed",
                },
                json!({
                    "errorMessage": error_message,
                    "shadowOnly": true,
                }),
            )
            .await?;
        }
        Ok(())
    }

    pub async fn mark_agent_job_cancelled(
        &self,
        job_id: &str,
        reason: &str,
    ) -> anyhow::Result<bool> {
        let now = Utc::now().timestamp();
        let result = sqlx::query(
            r#"
UPDATE agent_jobs
SET status = ?, updated_at = ?, completed_at = ?, last_error = ?
WHERE id = ? AND status IN (?, ?)
            "#,
        )
        .bind(AgentJobStatus::Cancelled.as_str())
        .bind(now)
        .bind(now)
        .bind(reason)
        .bind(job_id)
        .bind(AgentJobStatus::Pending.as_str())
        .bind(AgentJobStatus::Running.as_str())
        .execute(self.pool.as_ref())
        .await?;
        let cancelled = result.rows_affected() > 0;
        if cancelled {
            self.append_agent_job_status_shadow_event(
                job_id,
                AgentJobWorkGraphShadowEventSpec {
                    event_type: "agent_job_cancelled",
                    status: AgentJobStatus::Cancelled.as_str(),
                    summary:
                        "agent job reached cancelled terminal state in shadow WorkGraph event stream",
                    action: "cancelled",
                },
                json!({
                    "reason": reason,
                    "shadowOnly": true,
                }),
            )
            .await?;
        }
        Ok(cancelled)
    }

    pub async fn is_agent_job_cancelled(&self, job_id: &str) -> anyhow::Result<bool> {
        let row = sqlx::query(
            r#"
SELECT status
FROM agent_jobs
WHERE id = ?
            "#,
        )
        .bind(job_id)
        .fetch_optional(self.pool.as_ref())
        .await?;
        let Some(row) = row else {
            return Ok(false);
        };
        let status: String = row.try_get("status")?;
        Ok(AgentJobStatus::parse(status.as_str())? == AgentJobStatus::Cancelled)
    }

    pub async fn mark_agent_job_item_running(
        &self,
        job_id: &str,
        item_id: &str,
    ) -> anyhow::Result<bool> {
        let now = Utc::now().timestamp();
        let result = sqlx::query(
            r#"
UPDATE agent_job_items
SET
    status = ?,
    assigned_thread_id = NULL,
    attempt_count = attempt_count + 1,
    updated_at = ?,
    last_error = NULL
WHERE job_id = ? AND item_id = ? AND status = ?
            "#,
        )
        .bind(AgentJobItemStatus::Running.as_str())
        .bind(now)
        .bind(job_id)
        .bind(item_id)
        .bind(AgentJobItemStatus::Pending.as_str())
        .execute(self.pool.as_ref())
        .await?;
        let started = result.rows_affected() > 0;
        if started {
            self.append_agent_job_item_status_shadow_event(
                job_id,
                item_id,
                AgentJobWorkGraphShadowEventSpec {
                    event_type: "agent_job_item_started",
                    status: AgentJobItemStatus::Running.as_str(),
                    summary: "agent job item entered running state in shadow WorkGraph event stream",
                    action: "started",
                },
                json!({
                    "assignedThreadId": Value::Null,
                    "shadowOnly": true,
                }),
            )
            .await?;
        }
        Ok(started)
    }

    pub async fn mark_agent_job_item_running_with_thread(
        &self,
        job_id: &str,
        item_id: &str,
        thread_id: &str,
    ) -> anyhow::Result<bool> {
        let now = Utc::now().timestamp();
        let result = sqlx::query(
            r#"
UPDATE agent_job_items
SET
    status = ?,
    assigned_thread_id = ?,
    attempt_count = attempt_count + 1,
    updated_at = ?,
    last_error = NULL
WHERE job_id = ? AND item_id = ? AND status = ?
            "#,
        )
        .bind(AgentJobItemStatus::Running.as_str())
        .bind(thread_id)
        .bind(now)
        .bind(job_id)
        .bind(item_id)
        .bind(AgentJobItemStatus::Pending.as_str())
        .execute(self.pool.as_ref())
        .await?;
        let started = result.rows_affected() > 0;
        if started {
            self.append_agent_job_item_status_shadow_event(
                job_id,
                item_id,
                AgentJobWorkGraphShadowEventSpec {
                    event_type: "agent_job_item_started",
                    status: AgentJobItemStatus::Running.as_str(),
                    summary:
                        "agent job item entered running state with assigned worker in shadow WorkGraph event stream",
                    action: "started",
                },
                json!({
                    "assignedThreadId": thread_id,
                    "shadowOnly": true,
                }),
            )
            .await?;
        }
        Ok(started)
    }

    pub async fn mark_agent_job_item_pending(
        &self,
        job_id: &str,
        item_id: &str,
        error_message: Option<&str>,
    ) -> anyhow::Result<bool> {
        let now = Utc::now().timestamp();
        let result = sqlx::query(
            r#"
UPDATE agent_job_items
SET
    status = ?,
    assigned_thread_id = NULL,
    updated_at = ?,
    last_error = ?
WHERE job_id = ? AND item_id = ? AND status = ?
            "#,
        )
        .bind(AgentJobItemStatus::Pending.as_str())
        .bind(now)
        .bind(error_message)
        .bind(job_id)
        .bind(item_id)
        .bind(AgentJobItemStatus::Running.as_str())
        .execute(self.pool.as_ref())
        .await?;
        let returned = result.rows_affected() > 0;
        if returned {
            self.append_agent_job_item_status_shadow_event(
                job_id,
                item_id,
                AgentJobWorkGraphShadowEventSpec {
                    event_type: "agent_job_item_returned_pending",
                    status: AgentJobItemStatus::Pending.as_str(),
                    summary: "agent job item returned to pending state in shadow WorkGraph event stream",
                    action: "pending",
                },
                json!({
                    "errorMessage": error_message,
                    "shadowOnly": true,
                }),
            )
            .await?;
        }
        Ok(returned)
    }

    pub async fn set_agent_job_item_thread(
        &self,
        job_id: &str,
        item_id: &str,
        thread_id: &str,
    ) -> anyhow::Result<bool> {
        let now = Utc::now().timestamp();
        let result = sqlx::query(
            r#"
UPDATE agent_job_items
SET assigned_thread_id = ?, updated_at = ?
WHERE job_id = ? AND item_id = ? AND status = ?
            "#,
        )
        .bind(thread_id)
        .bind(now)
        .bind(job_id)
        .bind(item_id)
        .bind(AgentJobItemStatus::Running.as_str())
        .execute(self.pool.as_ref())
        .await?;
        Ok(result.rows_affected() > 0)
    }

    pub async fn report_agent_job_item_result(
        &self,
        job_id: &str,
        item_id: &str,
        reporting_thread_id: &str,
        result_json: &Value,
        task_result_envelope_json: Option<&Value>,
    ) -> anyhow::Result<bool> {
        let now = Utc::now().timestamp();
        let serialized = serde_json::to_string(result_json)?;
        let result = sqlx::query(
            r#"
UPDATE agent_job_items
SET
    status = ?,
    result_json = ?,
    reported_at = ?,
    completed_at = ?,
    updated_at = ?,
    last_error = NULL,
    assigned_thread_id = NULL
WHERE
    job_id = ?
    AND item_id = ?
    AND status = ?
    AND assigned_thread_id = ?
            "#,
        )
        .bind(AgentJobItemStatus::Completed.as_str())
        .bind(serialized)
        .bind(now)
        .bind(now)
        .bind(now)
        .bind(job_id)
        .bind(item_id)
        .bind(AgentJobItemStatus::Running.as_str())
        .bind(reporting_thread_id)
        .execute(self.pool.as_ref())
        .await?;
        let accepted = result.rows_affected() > 0;
        if accepted {
            let mut payload_json = json!({
                "reportingThreadId": reporting_thread_id,
                "resultKind": json_value_kind(result_json),
                "taskResultEnvelopeRecorded": task_result_envelope_json.is_some(),
                "shadowOnly": true,
            });
            if let Some(task_result_envelope_json) = task_result_envelope_json {
                payload_json["taskResultEnvelope"] = task_result_envelope_json.clone();
            }
            self.append_agent_job_item_status_shadow_event(
                job_id,
                item_id,
                AgentJobWorkGraphShadowEventSpec {
                    event_type: "agent_job_item_result_accepted",
                    status: AgentJobItemStatus::Completed.as_str(),
                    summary: "agent job item result accepted as shadow TaskResult terminal event",
                    action: "result",
                },
                payload_json,
            )
            .await?;
        }
        Ok(accepted)
    }

    pub async fn mark_agent_job_item_completed(
        &self,
        job_id: &str,
        item_id: &str,
    ) -> anyhow::Result<bool> {
        let now = Utc::now().timestamp();
        let result = sqlx::query(
            r#"
UPDATE agent_job_items
SET
    status = ?,
    completed_at = ?,
    updated_at = ?,
    assigned_thread_id = NULL
WHERE
    job_id = ?
    AND item_id = ?
    AND status = ?
    AND result_json IS NOT NULL
            "#,
        )
        .bind(AgentJobItemStatus::Completed.as_str())
        .bind(now)
        .bind(now)
        .bind(job_id)
        .bind(item_id)
        .bind(AgentJobItemStatus::Running.as_str())
        .execute(self.pool.as_ref())
        .await?;
        let completed = result.rows_affected() > 0;
        if completed {
            self.append_agent_job_item_status_shadow_event(
                job_id,
                item_id,
                AgentJobWorkGraphShadowEventSpec {
                    event_type: "agent_job_item_completed",
                    status: AgentJobItemStatus::Completed.as_str(),
                    summary: "agent job item completed from existing result in shadow WorkGraph event stream",
                    action: "completed",
                },
                json!({
                    "shadowOnly": true,
                }),
            )
            .await?;
        }
        Ok(completed)
    }

    pub async fn mark_agent_job_item_failed(
        &self,
        job_id: &str,
        item_id: &str,
        error_message: &str,
    ) -> anyhow::Result<bool> {
        let now = Utc::now().timestamp();
        let result = sqlx::query(
            r#"
UPDATE agent_job_items
SET
    status = ?,
    completed_at = ?,
    updated_at = ?,
    last_error = ?,
    assigned_thread_id = NULL
WHERE
    job_id = ?
    AND item_id = ?
    AND status = ?
            "#,
        )
        .bind(AgentJobItemStatus::Failed.as_str())
        .bind(now)
        .bind(now)
        .bind(error_message)
        .bind(job_id)
        .bind(item_id)
        .bind(AgentJobItemStatus::Running.as_str())
        .execute(self.pool.as_ref())
        .await?;
        let failed = result.rows_affected() > 0;
        if failed {
            self.append_agent_job_item_status_shadow_event(
                job_id,
                item_id,
                AgentJobWorkGraphShadowEventSpec {
                    event_type: "agent_job_item_failed",
                    status: AgentJobItemStatus::Failed.as_str(),
                    summary:
                        "agent job item reached failed terminal state in shadow WorkGraph event stream",
                    action: "failed",
                },
                json!({
                    "errorMessage": error_message,
                    "shadowOnly": true,
                }),
            )
            .await?;
        }
        Ok(failed)
    }

    pub async fn get_agent_job_progress(&self, job_id: &str) -> anyhow::Result<AgentJobProgress> {
        let row = sqlx::query(
            r#"
SELECT
    COUNT(*) AS total_items,
    SUM(CASE WHEN status = ? THEN 1 ELSE 0 END) AS pending_items,
    SUM(CASE WHEN status = ? THEN 1 ELSE 0 END) AS running_items,
    SUM(CASE WHEN status = ? THEN 1 ELSE 0 END) AS completed_items,
    SUM(CASE WHEN status = ? THEN 1 ELSE 0 END) AS failed_items
FROM agent_job_items
WHERE job_id = ?
            "#,
        )
        .bind(AgentJobItemStatus::Pending.as_str())
        .bind(AgentJobItemStatus::Running.as_str())
        .bind(AgentJobItemStatus::Completed.as_str())
        .bind(AgentJobItemStatus::Failed.as_str())
        .bind(job_id)
        .fetch_one(self.pool.as_ref())
        .await?;

        let total_items: i64 = row.try_get("total_items")?;
        let pending_items: Option<i64> = row.try_get("pending_items")?;
        let running_items: Option<i64> = row.try_get("running_items")?;
        let completed_items: Option<i64> = row.try_get("completed_items")?;
        let failed_items: Option<i64> = row.try_get("failed_items")?;
        Ok(AgentJobProgress {
            total_items: usize::try_from(total_items).unwrap_or_default(),
            pending_items: usize::try_from(pending_items.unwrap_or_default()).unwrap_or_default(),
            running_items: usize::try_from(running_items.unwrap_or_default()).unwrap_or_default(),
            completed_items: usize::try_from(completed_items.unwrap_or_default())
                .unwrap_or_default(),
            failed_items: usize::try_from(failed_items.unwrap_or_default()).unwrap_or_default(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::test_support::unique_temp_dir;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    async fn create_running_single_item_job(
        runtime: &StateRuntime,
    ) -> anyhow::Result<(String, String, String)> {
        let job_id = "job-1".to_string();
        let item_id = "item-1".to_string();
        let thread_id = "thread-1".to_string();
        runtime
            .create_agent_job(
                &AgentJobCreateParams {
                    id: job_id.clone(),
                    name: "test-job".to_string(),
                    instruction: "Return a result".to_string(),
                    auto_export: true,
                    max_runtime_seconds: None,
                    output_schema_json: None,
                    input_headers: vec!["path".to_string()],
                    input_csv_path: "/tmp/in.csv".to_string(),
                    output_csv_path: "/tmp/out.csv".to_string(),
                },
                &[AgentJobItemCreateParams {
                    item_id: item_id.clone(),
                    row_index: 0,
                    source_id: None,
                    row_json: json!({"path":"file-1"}),
                }],
            )
            .await?;
        runtime.mark_agent_job_running(job_id.as_str()).await?;
        let marked_running = runtime
            .mark_agent_job_item_running_with_thread(
                job_id.as_str(),
                item_id.as_str(),
                thread_id.as_str(),
            )
            .await?;
        assert!(marked_running);
        Ok((job_id, item_id, thread_id))
    }

    #[tokio::test]
    async fn report_agent_job_item_result_completes_item_atomically() -> anyhow::Result<()> {
        let codex_home = unique_temp_dir();
        let runtime = StateRuntime::init(codex_home, "test-provider".to_string()).await?;
        let (job_id, item_id, thread_id) = create_running_single_item_job(runtime.as_ref()).await?;

        let accepted = runtime
            .report_agent_job_item_result(
                job_id.as_str(),
                item_id.as_str(),
                thread_id.as_str(),
                &json!({"ok": true}),
                None,
            )
            .await?;
        assert!(accepted);

        let item = runtime
            .get_agent_job_item(job_id.as_str(), item_id.as_str())
            .await?
            .expect("job item should exist");
        assert_eq!(item.status, AgentJobItemStatus::Completed);
        assert_eq!(item.result_json, Some(json!({"ok": true})));
        assert_eq!(item.assigned_thread_id, None);
        assert_eq!(item.last_error, None);
        assert!(item.reported_at.is_some());
        assert!(item.completed_at.is_some());
        let progress = runtime.get_agent_job_progress(job_id.as_str()).await?;
        assert_eq!(
            progress,
            AgentJobProgress {
                total_items: 1,
                pending_items: 0,
                running_items: 0,
                completed_items: 1,
                failed_items: 0,
            }
        );
        Ok(())
    }

    #[tokio::test]
    async fn work_graph_shadow_events_track_accepted_agent_job_result() -> anyhow::Result<()> {
        let codex_home = unique_temp_dir();
        let runtime = StateRuntime::init(codex_home, "test-provider".to_string()).await?;
        let (job_id, item_id, thread_id) = create_running_single_item_job(runtime.as_ref()).await?;

        let accepted = runtime
            .report_agent_job_item_result(
                job_id.as_str(),
                item_id.as_str(),
                thread_id.as_str(),
                &json!({"ok": true}),
                Some(&json!({
                    "schemaVersion": "hepta.task_result.v1",
                    "taskId": format!("agent-job:{job_id}:{item_id}"),
                    "status": "completed",
                })),
            )
            .await?;
        assert!(accepted);
        runtime.mark_agent_job_completed(job_id.as_str()).await?;

        let events = runtime
            .list_agent_job_work_graph_shadow_events(job_id.as_str())
            .await?;
        let event_types = events
            .iter()
            .map(|event| event.event_type.as_str())
            .collect::<Vec<_>>();
        assert_eq!(
            event_types,
            vec![
                "agent_job_created",
                "agent_job_item_created",
                "agent_job_running",
                "agent_job_item_started",
                "agent_job_item_result_accepted",
                "agent_job_completed",
            ]
        );
        assert!(
            events
                .iter()
                .all(|event| !event.live_blocking_enabled && !event.live_cutover_enabled)
        );
        let result_event = events
            .iter()
            .find(|event| event.event_type == "agent_job_item_result_accepted")
            .expect("accepted result event should be recorded");
        assert_eq!(
            result_event.payload_json["taskResultEnvelopeRecorded"],
            json!(true)
        );
        assert_eq!(
            result_event.payload_json["taskResultEnvelope"]["schemaVersion"],
            json!("hepta.task_result.v1")
        );
        assert_eq!(
            result_event.payload_json["taskResultEnvelope"]["taskId"],
            json!(format!("agent-job:{}:{}", result_event.job_id, item_id))
        );
        let task_id = format!("agent-job:{}:{}", result_event.job_id, item_id);
        let task_result = runtime
            .get_agent_job_task_result_envelope_by_task_id(task_id.as_str())
            .await?
            .expect("task result envelope should be readable by task id");
        assert_eq!(task_result["status"], json!("completed"));
        assert_eq!(task_result["schemaVersion"], json!("hepta.task_result.v1"));

        let diff = runtime
            .get_agent_job_work_graph_shadow_projection_diff(job_id.as_str())
            .await?;
        assert_eq!(
            diff,
            AgentJobWorkGraphShadowProjectionDiff {
                job_id: job_id.clone(),
                progress: AgentJobProgress {
                    total_items: 1,
                    pending_items: 0,
                    running_items: 0,
                    completed_items: 1,
                    failed_items: 0,
                },
                projection: AgentJobWorkGraphShadowProjection {
                    job_id,
                    total_events: 6,
                    distinct_tasks: 2,
                    latest_sequence_id: Some(6),
                    item_started_events: 1,
                    item_completed_events: 1,
                    item_failed_events: 0,
                    job_terminal_events: 1,
                    live_blocking_event_count: 0,
                    live_cutover_event_count: 0,
                },
                completed_item_delta: 0,
                failed_item_delta: 0,
                projection_matches_items: true,
            }
        );
        Ok(())
    }

    #[tokio::test]
    async fn work_graph_shadow_events_track_admission_decision() -> anyhow::Result<()> {
        let codex_home = unique_temp_dir();
        let runtime = StateRuntime::init(codex_home, "test-provider".to_string()).await?;
        let (job_id, _item_id, _thread_id) =
            create_running_single_item_job(runtime.as_ref()).await?;
        let task_id = format!("agent-job:{job_id}");

        runtime
            .append_agent_job_admission_shadow_decision(
                job_id.as_str(),
                task_id.as_str(),
                "allow_shadow_no_live_blocking",
                json!({
                    "decision": "allow_shadow_no_live_blocking",
                    "shadowOnly": true,
                }),
                Some("trace-1"),
            )
            .await?;

        let events = runtime
            .list_agent_job_work_graph_shadow_events(job_id.as_str())
            .await?;
        let admission_event = events
            .iter()
            .find(|event| event.event_type == "agent_job_admission_shadow_decision")
            .expect("admission shadow decision event should be recorded");
        assert_eq!(admission_event.status, "allow_shadow_no_live_blocking");
        assert_eq!(admission_event.trace_id.as_deref(), Some("trace-1"));
        assert!(!admission_event.live_blocking_enabled);
        assert!(!admission_event.live_cutover_enabled);
        Ok(())
    }

    #[tokio::test]
    async fn work_graph_promotion_review_readback_tracks_shadow_packets() -> anyhow::Result<()> {
        let codex_home = unique_temp_dir();
        let runtime = StateRuntime::init(codex_home, "test-provider".to_string()).await?;
        let (job_id, _item_id, _thread_id) =
            create_running_single_item_job(runtime.as_ref()).await?;
        let task_id = format!("agent-job:{job_id}");

        runtime
            .append_agent_job_admission_shadow_decision(
                job_id.as_str(),
                task_id.as_str(),
                "allow_shadow_no_live_blocking",
                json!({
                    "decision": "allow_shadow_no_live_blocking",
                    "shadowOnly": true,
                }),
                Some("trace-review"),
            )
            .await?;
        runtime
            .append_agent_job_promotion_readiness_matrix_shadow(
                job_id.as_str(),
                task_id.as_str(),
                "promotion_matrix_not_ready_shadow_no_live_cutover",
                json!({
                    "decision": "promotion_matrix_not_ready_shadow_no_live_cutover",
                    "promotionStage": "shadow_only",
                    "liveCutoverEnabled": false,
                }),
                Some("trace-review"),
            )
            .await?;
        runtime
            .append_agent_job_operator_review_promotion_packet_shadow(
                job_id.as_str(),
                task_id.as_str(),
                "operator_review_packet_blocked_shadow_no_live_cutover",
                json!({
                    "decision": "operator_review_packet_blocked_shadow_no_live_cutover",
                    "promotionAllowed": false,
                    "operatorApprovalRecorded": false,
                    "approvalRecordMutationEnabled": false,
                    "liveCutoverEnabled": false,
                }),
                Some("trace-review"),
            )
            .await?;
        runtime
            .append_agent_job_promotion_review_replay_consistency_shadow(
                job_id.as_str(),
                task_id.as_str(),
                "promotion_review_replay_consistent_shadow_no_live_cutover",
                json!({
                    "decision": "promotion_review_replay_consistent_shadow_no_live_cutover",
                    "replayConsistent": true,
                    "promotionAllowed": false,
                    "operatorApprovalRecorded": false,
                    "approvalRecordMutationEnabled": false,
                    "liveCutoverEnabled": false,
                }),
                Some("trace-review"),
            )
            .await?;
        runtime
            .append_agent_job_promotion_closeout_receipt_shadow(
                job_id.as_str(),
                task_id.as_str(),
                "promotion_closeout_receipt_recorded_shadow_no_live_cutover",
                json!({
                    "decision": "promotion_closeout_receipt_recorded_shadow_no_live_cutover",
                    "reviewedButNotPromoted": true,
                    "promotionAllowed": false,
                    "operatorApprovalRecorded": false,
                    "approvalRecordMutationEnabled": false,
                    "liveCutoverEnabled": false,
                }),
                Some("trace-review"),
            )
            .await?;
        runtime
            .append_agent_job_promotion_closeout_replay_consistency_shadow(
                job_id.as_str(),
                task_id.as_str(),
                "promotion_closeout_replay_consistent_shadow_no_live_cutover",
                json!({
                    "decision": "promotion_closeout_replay_consistent_shadow_no_live_cutover",
                    "replayConsistent": true,
                    "promotionAllowed": false,
                    "operatorApprovalRecorded": false,
                    "approvalRecordMutationEnabled": false,
                    "liveCutoverEnabled": false,
                }),
                Some("trace-review"),
            )
            .await?;
        runtime
            .append_agent_job_promotion_review_audit_chain_receipt_shadow(
                job_id.as_str(),
                task_id.as_str(),
                "promotion_review_audit_chain_recorded_shadow_no_live_cutover",
                json!({
                    "decision": "promotion_review_audit_chain_recorded_shadow_no_live_cutover",
                    "terminalAuditReady": true,
                    "chainReadbackReady": true,
                    "promotionAllowed": false,
                    "operatorApprovalRecorded": false,
                    "approvalRecordMutationEnabled": false,
                    "liveCutoverEnabled": false,
                }),
                Some("trace-review"),
            )
            .await?;
        runtime
            .append_agent_job_reviewed_flag_precondition_plan_shadow(
                job_id.as_str(),
                task_id.as_str(),
                "reviewed_flag_precondition_plan_recorded_shadow_no_live_cutover",
                json!({
                    "decision": "reviewed_flag_precondition_plan_recorded_shadow_no_live_cutover",
                    "dryRunPlanReady": true,
                    "reviewedFlagMutationEnabled": false,
                    "promotionAllowed": false,
                    "operatorApprovalRecorded": false,
                    "approvalRecordMutationEnabled": false,
                    "liveCutoverEnabled": false,
                }),
                Some("trace-review"),
            )
            .await?;
        runtime
            .append_agent_job_reviewed_flag_precondition_plan_replay_consistency_shadow(
                job_id.as_str(),
                task_id.as_str(),
                "reviewed_flag_precondition_plan_replay_consistent_shadow_no_live_cutover",
                json!({
                    "decision": "reviewed_flag_precondition_plan_replay_consistent_shadow_no_live_cutover",
                    "replayConsistent": true,
                    "reviewedFlagMutationEnabled": false,
                    "promotionAllowed": false,
                    "operatorApprovalRecorded": false,
                    "approvalRecordMutationEnabled": false,
                    "liveCutoverEnabled": false,
                }),
                Some("trace-review"),
            )
            .await?;
        runtime
            .append_agent_job_reviewed_flag_readiness_closeout_receipt_shadow(
                job_id.as_str(),
                task_id.as_str(),
                "reviewed_flag_readiness_closeout_recorded_shadow_no_live_cutover",
                json!({
                    "decision": "reviewed_flag_readiness_closeout_recorded_shadow_no_live_cutover",
                    "plannedButNotMutable": true,
                    "terminalCloseoutReady": true,
                    "reviewedFlagMutationEnabled": false,
                    "promotionAllowed": false,
                    "operatorApprovalRecorded": false,
                    "approvalRecordMutationEnabled": false,
                    "liveCutoverEnabled": false,
                }),
                Some("trace-review"),
            )
            .await?;
        runtime
            .append_agent_job_reviewed_flag_readiness_closeout_replay_consistency_shadow(
                job_id.as_str(),
                task_id.as_str(),
                "reviewed_flag_readiness_closeout_replay_consistent_shadow_no_live_cutover",
                json!({
                    "decision": "reviewed_flag_readiness_closeout_replay_consistent_shadow_no_live_cutover",
                    "replayConsistent": true,
                    "plannedButNotMutable": true,
                    "reviewedFlagMutationEnabled": false,
                    "promotionAllowed": false,
                    "operatorApprovalRecorded": false,
                    "approvalRecordMutationEnabled": false,
                    "liveCutoverEnabled": false,
                }),
                Some("trace-review"),
            )
            .await?;
        runtime
            .append_agent_job_reviewed_flag_audit_chain_closeout_receipt_shadow(
                job_id.as_str(),
                task_id.as_str(),
                "reviewed_flag_audit_chain_closeout_recorded_shadow_no_live_cutover",
                json!({
                    "decision": "reviewed_flag_audit_chain_closeout_recorded_shadow_no_live_cutover",
                    "terminalReviewedFlagAuditReady": true,
                    "plannedButNotMutable": true,
                    "reviewedFlagMutationEnabled": false,
                    "promotionAllowed": false,
                    "operatorApprovalRecorded": false,
                    "approvalRecordMutationEnabled": false,
                    "liveCutoverEnabled": false,
                }),
                Some("trace-review"),
            )
            .await?;

        let events = runtime
            .list_agent_job_work_graph_shadow_events(job_id.as_str())
            .await?;
        let review_event = events
            .iter()
            .find(|event| event.event_type == "agent_job_operator_review_promotion_packet")
            .expect("operator review packet event should be recorded");
        assert_eq!(
            review_event.status,
            "operator_review_packet_blocked_shadow_no_live_cutover"
        );
        assert_eq!(review_event.trace_id.as_deref(), Some("trace-review"));
        assert!(!review_event.live_blocking_enabled);
        assert!(!review_event.live_cutover_enabled);
        let replay_event = events
            .iter()
            .find(|event| event.event_type == "agent_job_promotion_review_replay_consistency")
            .expect("promotion review replay consistency event should be recorded");
        assert_eq!(
            replay_event.status,
            "promotion_review_replay_consistent_shadow_no_live_cutover"
        );
        assert_eq!(replay_event.trace_id.as_deref(), Some("trace-review"));
        assert!(!replay_event.live_blocking_enabled);
        assert!(!replay_event.live_cutover_enabled);
        let closeout_event = events
            .iter()
            .find(|event| event.event_type == "agent_job_promotion_closeout_receipt")
            .expect("promotion closeout receipt event should be recorded");
        assert_eq!(
            closeout_event.status,
            "promotion_closeout_receipt_recorded_shadow_no_live_cutover"
        );
        assert_eq!(closeout_event.trace_id.as_deref(), Some("trace-review"));
        assert!(!closeout_event.live_blocking_enabled);
        assert!(!closeout_event.live_cutover_enabled);
        let closeout_replay_event = events
            .iter()
            .find(|event| event.event_type == "agent_job_promotion_closeout_replay_consistency")
            .expect("promotion closeout replay consistency event should be recorded");
        assert_eq!(
            closeout_replay_event.status,
            "promotion_closeout_replay_consistent_shadow_no_live_cutover"
        );
        assert_eq!(
            closeout_replay_event.trace_id.as_deref(),
            Some("trace-review")
        );
        assert!(!closeout_replay_event.live_blocking_enabled);
        assert!(!closeout_replay_event.live_cutover_enabled);
        let audit_chain_event = events
            .iter()
            .find(|event| event.event_type == "agent_job_promotion_review_audit_chain_receipt")
            .expect("promotion review audit chain receipt event should be recorded");
        assert_eq!(
            audit_chain_event.status,
            "promotion_review_audit_chain_recorded_shadow_no_live_cutover"
        );
        assert_eq!(audit_chain_event.trace_id.as_deref(), Some("trace-review"));
        assert!(!audit_chain_event.live_blocking_enabled);
        assert!(!audit_chain_event.live_cutover_enabled);
        let reviewed_flag_plan_event = events
            .iter()
            .find(|event| event.event_type == "agent_job_reviewed_flag_precondition_plan")
            .expect("reviewed flag precondition plan event should be recorded");
        assert_eq!(
            reviewed_flag_plan_event.status,
            "reviewed_flag_precondition_plan_recorded_shadow_no_live_cutover"
        );
        assert_eq!(
            reviewed_flag_plan_event.trace_id.as_deref(),
            Some("trace-review")
        );
        assert!(!reviewed_flag_plan_event.live_blocking_enabled);
        assert!(!reviewed_flag_plan_event.live_cutover_enabled);
        let reviewed_flag_plan_replay_event = events
            .iter()
            .find(|event| {
                event.event_type == "agent_job_reviewed_flag_precondition_plan_replay_consistency"
            })
            .expect("reviewed flag precondition plan replay event should be recorded");
        assert_eq!(
            reviewed_flag_plan_replay_event.status,
            "reviewed_flag_precondition_plan_replay_consistent_shadow_no_live_cutover"
        );
        assert_eq!(
            reviewed_flag_plan_replay_event.trace_id.as_deref(),
            Some("trace-review")
        );
        assert!(!reviewed_flag_plan_replay_event.live_blocking_enabled);
        assert!(!reviewed_flag_plan_replay_event.live_cutover_enabled);
        let reviewed_flag_closeout_event = events
            .iter()
            .find(|event| event.event_type == "agent_job_reviewed_flag_readiness_closeout_receipt")
            .expect("reviewed flag readiness closeout event should be recorded");
        assert_eq!(
            reviewed_flag_closeout_event.status,
            "reviewed_flag_readiness_closeout_recorded_shadow_no_live_cutover"
        );
        assert_eq!(
            reviewed_flag_closeout_event.trace_id.as_deref(),
            Some("trace-review")
        );
        assert!(!reviewed_flag_closeout_event.live_blocking_enabled);
        assert!(!reviewed_flag_closeout_event.live_cutover_enabled);
        let reviewed_flag_closeout_replay_event = events
            .iter()
            .find(|event| {
                event.event_type == "agent_job_reviewed_flag_readiness_closeout_replay_consistency"
            })
            .expect("reviewed flag readiness closeout replay event should be recorded");
        assert_eq!(
            reviewed_flag_closeout_replay_event.status,
            "reviewed_flag_readiness_closeout_replay_consistent_shadow_no_live_cutover"
        );
        assert_eq!(
            reviewed_flag_closeout_replay_event.trace_id.as_deref(),
            Some("trace-review")
        );
        assert!(!reviewed_flag_closeout_replay_event.live_blocking_enabled);
        assert!(!reviewed_flag_closeout_replay_event.live_cutover_enabled);
        let reviewed_flag_audit_chain_closeout_event = events
            .iter()
            .find(|event| {
                event.event_type == "agent_job_reviewed_flag_audit_chain_closeout_receipt"
            })
            .expect("reviewed flag audit-chain closeout event should be recorded");
        assert_eq!(
            reviewed_flag_audit_chain_closeout_event.status,
            "reviewed_flag_audit_chain_closeout_recorded_shadow_no_live_cutover"
        );
        assert_eq!(
            reviewed_flag_audit_chain_closeout_event.trace_id.as_deref(),
            Some("trace-review")
        );
        assert!(!reviewed_flag_audit_chain_closeout_event.live_blocking_enabled);
        assert!(!reviewed_flag_audit_chain_closeout_event.live_cutover_enabled);

        let readback = runtime
            .get_agent_job_work_graph_promotion_review_readback(job_id.as_str())
            .await?;
        assert_eq!(readback.job_id, job_id);
        assert_eq!(readback.admission_shadow_decision_events, 1);
        assert_eq!(readback.promotion_readiness_matrix_events, 1);
        assert_eq!(readback.operator_review_promotion_packet_events, 1);
        assert_eq!(readback.promotion_review_replay_consistency_events, 1);
        assert_eq!(readback.promotion_closeout_receipt_events, 1);
        assert_eq!(readback.promotion_closeout_replay_consistency_events, 1);
        assert_eq!(readback.promotion_review_audit_chain_receipt_events, 1);
        assert_eq!(readback.reviewed_flag_precondition_plan_events, 1);
        assert_eq!(
            readback.reviewed_flag_precondition_plan_replay_consistency_events,
            1
        );
        assert_eq!(readback.reviewed_flag_readiness_closeout_receipt_events, 1);
        assert_eq!(
            readback.reviewed_flag_readiness_closeout_replay_consistency_events,
            1
        );
        assert_eq!(
            readback.reviewed_flag_audit_chain_closeout_receipt_events,
            1
        );
        assert_eq!(readback.live_blocking_event_count, 0);
        assert_eq!(readback.live_cutover_event_count, 0);
        assert!(readback.readback_ready);
        assert!(readback.replay_consistency_ready);
        assert!(readback.closeout_receipt_ready);
        assert!(readback.closeout_replay_consistency_ready);
        assert!(readback.audit_chain_receipt_ready);
        assert!(readback.reviewed_flag_precondition_plan_ready);
        assert!(readback.reviewed_flag_precondition_plan_replay_consistency_ready);
        assert!(readback.reviewed_flag_readiness_closeout_receipt_ready);
        assert!(readback.reviewed_flag_readiness_closeout_replay_consistency_ready);
        assert!(readback.reviewed_flag_audit_chain_closeout_receipt_ready);
        assert_eq!(
            readback
                .latest_admission_shadow_decision
                .as_ref()
                .and_then(|value| value.get("decision")),
            Some(&json!("allow_shadow_no_live_blocking"))
        );
        assert_eq!(
            readback
                .latest_promotion_readiness_matrix
                .as_ref()
                .and_then(|value| value.get("decision")),
            Some(&json!("promotion_matrix_not_ready_shadow_no_live_cutover"))
        );
        assert_eq!(
            readback
                .latest_operator_review_promotion_packet
                .as_ref()
                .and_then(|value| value.get("promotionAllowed")),
            Some(&json!(false))
        );
        assert_eq!(
            readback
                .latest_promotion_review_replay_consistency
                .as_ref()
                .and_then(|value| value.get("replayConsistent")),
            Some(&json!(true))
        );
        assert_eq!(
            readback
                .latest_promotion_closeout_receipt
                .as_ref()
                .and_then(|value| value.get("reviewedButNotPromoted")),
            Some(&json!(true))
        );
        assert_eq!(
            readback
                .latest_promotion_closeout_replay_consistency
                .as_ref()
                .and_then(|value| value.get("replayConsistent")),
            Some(&json!(true))
        );
        assert_eq!(
            readback
                .latest_promotion_review_audit_chain_receipt
                .as_ref()
                .and_then(|value| value.get("terminalAuditReady")),
            Some(&json!(true))
        );
        assert_eq!(
            readback
                .latest_reviewed_flag_precondition_plan
                .as_ref()
                .and_then(|value| value.get("dryRunPlanReady")),
            Some(&json!(true))
        );
        assert_eq!(
            readback
                .latest_reviewed_flag_precondition_plan_replay_consistency
                .as_ref()
                .and_then(|value| value.get("replayConsistent")),
            Some(&json!(true))
        );
        assert_eq!(
            readback
                .latest_reviewed_flag_readiness_closeout_receipt
                .as_ref()
                .and_then(|value| value.get("plannedButNotMutable")),
            Some(&json!(true))
        );
        assert_eq!(
            readback
                .latest_reviewed_flag_readiness_closeout_replay_consistency
                .as_ref()
                .and_then(|value| value.get("replayConsistent")),
            Some(&json!(true))
        );
        assert_eq!(
            readback
                .latest_reviewed_flag_audit_chain_closeout_receipt
                .as_ref()
                .and_then(|value| value.get("terminalReviewedFlagAuditReady")),
            Some(&json!(true))
        );
        let generic_specs = [
            AgentJobWorkGraphAuditChainSegmentSpec {
                segment_id: "admission_shadow_decision",
                event_type: "agent_job_admission_shadow_decision",
                replay_consistency_field: None,
            },
            AgentJobWorkGraphAuditChainSegmentSpec {
                segment_id: "promotion_readiness_matrix",
                event_type: "agent_job_promotion_readiness_matrix",
                replay_consistency_field: None,
            },
            AgentJobWorkGraphAuditChainSegmentSpec {
                segment_id: "operator_review_promotion_packet",
                event_type: "agent_job_operator_review_promotion_packet",
                replay_consistency_field: None,
            },
            AgentJobWorkGraphAuditChainSegmentSpec {
                segment_id: "promotion_review_replay_consistency",
                event_type: "agent_job_promotion_review_replay_consistency",
                replay_consistency_field: Some("replayConsistent"),
            },
            AgentJobWorkGraphAuditChainSegmentSpec {
                segment_id: "promotion_closeout_receipt",
                event_type: "agent_job_promotion_closeout_receipt",
                replay_consistency_field: None,
            },
            AgentJobWorkGraphAuditChainSegmentSpec {
                segment_id: "promotion_closeout_replay_consistency",
                event_type: "agent_job_promotion_closeout_replay_consistency",
                replay_consistency_field: Some("replayConsistent"),
            },
            AgentJobWorkGraphAuditChainSegmentSpec {
                segment_id: "promotion_review_audit_chain_receipt",
                event_type: "agent_job_promotion_review_audit_chain_receipt",
                replay_consistency_field: None,
            },
            AgentJobWorkGraphAuditChainSegmentSpec {
                segment_id: "reviewed_flag_precondition_plan",
                event_type: "agent_job_reviewed_flag_precondition_plan",
                replay_consistency_field: None,
            },
            AgentJobWorkGraphAuditChainSegmentSpec {
                segment_id: "reviewed_flag_precondition_plan_replay_consistency",
                event_type: "agent_job_reviewed_flag_precondition_plan_replay_consistency",
                replay_consistency_field: Some("replayConsistent"),
            },
            AgentJobWorkGraphAuditChainSegmentSpec {
                segment_id: "reviewed_flag_readiness_closeout_receipt",
                event_type: "agent_job_reviewed_flag_readiness_closeout_receipt",
                replay_consistency_field: None,
            },
            AgentJobWorkGraphAuditChainSegmentSpec {
                segment_id: "reviewed_flag_readiness_closeout_replay_consistency",
                event_type: "agent_job_reviewed_flag_readiness_closeout_replay_consistency",
                replay_consistency_field: Some("replayConsistent"),
            },
            AgentJobWorkGraphAuditChainSegmentSpec {
                segment_id: "reviewed_flag_audit_chain_closeout_receipt",
                event_type: "agent_job_reviewed_flag_audit_chain_closeout_receipt",
                replay_consistency_field: None,
            },
        ];
        let generic_readback = runtime
            .get_agent_job_work_graph_audit_chain_readback(job_id.as_str(), &generic_specs)
            .await?;
        assert_eq!(generic_readback.job_id, job_id);
        assert_eq!(generic_readback.segments.len(), 12);
        assert_eq!(generic_readback.live_blocking_event_count, 0);
        assert_eq!(generic_readback.live_cutover_event_count, 0);
        assert!(generic_readback.chain_readback_ready);
        assert!(generic_readback.chain_replay_consistent);
        assert!(generic_readback.no_live_guardrails_ready);
        assert!(generic_readback.chain_ready);
        let replay_segment = generic_readback
            .segments
            .iter()
            .find(|segment| segment.segment_id == "promotion_review_replay_consistency")
            .expect("promotion review replay segment should be present");
        assert_eq!(replay_segment.event_count, 1);
        assert_eq!(
            replay_segment.latest_decision,
            "promotion_review_replay_consistent_shadow_no_live_cutover"
        );
        assert!(replay_segment.replay_consistent);
        assert!(replay_segment.ready);
        Ok(())
    }

    #[tokio::test]
    async fn report_agent_job_item_result_rejects_late_reports() -> anyhow::Result<()> {
        let codex_home = unique_temp_dir();
        let runtime = StateRuntime::init(codex_home, "test-provider".to_string()).await?;
        let (job_id, item_id, thread_id) = create_running_single_item_job(runtime.as_ref()).await?;

        let marked_failed = runtime
            .mark_agent_job_item_failed(job_id.as_str(), item_id.as_str(), "missing report")
            .await?;
        assert!(marked_failed);
        let accepted = runtime
            .report_agent_job_item_result(
                job_id.as_str(),
                item_id.as_str(),
                thread_id.as_str(),
                &json!({"late": true}),
                None,
            )
            .await?;
        assert!(!accepted);

        let item = runtime
            .get_agent_job_item(job_id.as_str(), item_id.as_str())
            .await?
            .expect("job item should exist");
        assert_eq!(item.status, AgentJobItemStatus::Failed);
        assert_eq!(item.result_json, None);
        assert_eq!(item.last_error, Some("missing report".to_string()));
        Ok(())
    }

    #[tokio::test]
    async fn work_graph_shadow_projection_diff_tracks_failed_items() -> anyhow::Result<()> {
        let codex_home = unique_temp_dir();
        let runtime = StateRuntime::init(codex_home, "test-provider".to_string()).await?;
        let (job_id, item_id, _thread_id) =
            create_running_single_item_job(runtime.as_ref()).await?;

        let marked_failed = runtime
            .mark_agent_job_item_failed(job_id.as_str(), item_id.as_str(), "missing report")
            .await?;
        assert!(marked_failed);

        let diff = runtime
            .get_agent_job_work_graph_shadow_projection_diff(job_id.as_str())
            .await?;
        assert_eq!(
            diff,
            AgentJobWorkGraphShadowProjectionDiff {
                job_id: job_id.clone(),
                progress: AgentJobProgress {
                    total_items: 1,
                    pending_items: 0,
                    running_items: 0,
                    completed_items: 0,
                    failed_items: 1,
                },
                projection: AgentJobWorkGraphShadowProjection {
                    job_id,
                    total_events: 5,
                    distinct_tasks: 2,
                    latest_sequence_id: Some(5),
                    item_started_events: 1,
                    item_completed_events: 0,
                    item_failed_events: 1,
                    job_terminal_events: 0,
                    live_blocking_event_count: 0,
                    live_cutover_event_count: 0,
                },
                completed_item_delta: 0,
                failed_item_delta: 0,
                projection_matches_items: true,
            }
        );
        Ok(())
    }
}
