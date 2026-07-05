CREATE TABLE inter_agent_mailbox_events (
    sequence_id INTEGER PRIMARY KEY AUTOINCREMENT,
    thread_id TEXT NOT NULL,
    event_type TEXT NOT NULL,
    mailbox_seq INTEGER,
    barrier_id TEXT,
    task_id TEXT,
    task_name TEXT,
    author_path TEXT,
    recipient_path TEXT,
    other_recipients_json TEXT,
    trigger_turn INTEGER,
    content_json TEXT,
    status TEXT NOT NULL,
    created_at_ms INTEGER NOT NULL,
    deadline_at_ms INTEGER,
    trace_id TEXT,
    live_blocking_enabled INTEGER NOT NULL DEFAULT 0,
    live_cutover_enabled INTEGER NOT NULL DEFAULT 0
);

CREATE INDEX idx_inter_agent_mailbox_events_thread_sequence
ON inter_agent_mailbox_events(thread_id, sequence_id ASC);

CREATE INDEX idx_inter_agent_mailbox_events_thread_mailbox
ON inter_agent_mailbox_events(thread_id, mailbox_seq ASC);

CREATE INDEX idx_inter_agent_mailbox_events_barrier
ON inter_agent_mailbox_events(barrier_id, sequence_id ASC);

CREATE INDEX idx_inter_agent_mailbox_events_task
ON inter_agent_mailbox_events(task_id, sequence_id ASC);
