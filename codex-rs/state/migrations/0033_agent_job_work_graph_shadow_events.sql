CREATE TABLE agent_job_work_graph_shadow_events (
    sequence_id INTEGER PRIMARY KEY AUTOINCREMENT,
    job_id TEXT NOT NULL,
    item_id TEXT,
    event_type TEXT NOT NULL,
    task_id TEXT NOT NULL,
    status TEXT NOT NULL,
    summary TEXT NOT NULL,
    payload_json TEXT NOT NULL,
    trace_id TEXT,
    span_id TEXT NOT NULL,
    source_surface_id TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    live_blocking_enabled INTEGER NOT NULL DEFAULT 0,
    live_cutover_enabled INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY(job_id) REFERENCES agent_jobs(id) ON DELETE CASCADE
);

CREATE INDEX idx_agent_job_work_graph_shadow_events_job_sequence
ON agent_job_work_graph_shadow_events(job_id, sequence_id ASC);

CREATE INDEX idx_agent_job_work_graph_shadow_events_task
ON agent_job_work_graph_shadow_events(task_id, sequence_id ASC);

CREATE INDEX idx_agent_job_work_graph_shadow_events_type
ON agent_job_work_graph_shadow_events(event_type, sequence_id ASC);
