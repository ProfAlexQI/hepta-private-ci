use std::sync::Arc;
use std::time::Duration;

use codex_hepta_automation::AutomationAdmission;
use codex_hepta_automation::AutomationError;
use codex_hepta_automation::AutomationFuture;
use codex_hepta_automation::AutomationQueueReceipt;
use codex_hepta_automation::AutomationSchedule;
use codex_hepta_automation::AutomationScheduler;
use codex_hepta_automation::AutomationStore;
use codex_hepta_automation::AutomationTaskDraft;
use codex_hepta_automation::AutomationTaskId;
use codex_hepta_automation::AutomationTaskState;
use codex_hepta_automation::AutomationTick;
use codex_hepta_automation::AutomationTurnQueue;
use codex_hepta_contracts::AgentId;
use codex_hepta_fleet::AgentManifest;
use codex_hepta_fleet::FleetRegistry;
use codex_hepta_fleet::ResourceBudget;
use codex_hepta_fleet::WorkspaceBinding;
use codex_hepta_paths::HeptaAgentLayout;
use codex_hepta_paths::HeptaFleetRoot;
use pretty_assertions::assert_eq;
use tokio::sync::Mutex;
use tokio::sync::Notify;

const AGENT_IDS: [&str; 5] = [
    "018f4f72-5f8f-7cc1-8f55-df9fb3aa2c12",
    "019153a4-3088-7e03-a56a-9b1964f75dd3",
    "019153a4-3088-7e03-a56a-9b1964f75dd4",
    "019153a4-3088-7e03-a56a-9b1964f75dd5",
    "019153a4-3088-7e03-a56a-9b1964f75dd6",
];
const THREAD_ID: &str = "019153a4-3088-7e03-a56a-9b1964f75ddd";

struct FleetFixture {
    _temp: tempfile::TempDir,
    layouts: Vec<HeptaAgentLayout>,
}

impl FleetFixture {
    #[allow(
        clippy::expect_used,
        reason = "test fixture construction must fail loudly"
    )]
    fn new(count: usize) -> Self {
        let temp = tempfile::tempdir().expect("temp root");
        let root = temp.path().canonicalize().expect("canonical temp root");
        let fleet_root = HeptaFleetRoot::parse(root.join("fleet")).expect("fleet root");
        let registry = FleetRegistry::initialize(fleet_root.clone()).expect("fleet registry");
        let mut layouts = Vec::new();
        for (index, raw_agent_id) in AGENT_IDS.iter().take(count).enumerate() {
            let workspace = root.join(format!("workspace-{index}"));
            std::fs::create_dir(&workspace).expect("workspace");
            let workspace = workspace.canonicalize().expect("canonical workspace");
            let agent_id = AgentId::parse(*raw_agent_id).expect("agent id");
            let manifest = AgentManifest::new(
                agent_id,
                WorkspaceBinding::new(workspace, &fleet_root).expect("workspace binding"),
                ResourceBudget::local_default(),
            )
            .expect("manifest");
            layouts.push(registry.register(manifest).expect("register agent").layout);
        }
        Self {
            _temp: temp,
            layouts,
        }
    }
}

#[derive(Default)]
struct RecordingQueue {
    admissions: Mutex<Vec<AutomationAdmission>>,
    entered: Notify,
    release: Option<Arc<Notify>>,
}

impl RecordingQueue {
    fn blocked(release: Arc<Notify>) -> Self {
        Self {
            admissions: Mutex::new(Vec::new()),
            entered: Notify::new(),
            release: Some(release),
        }
    }

    async fn admissions(&self) -> Vec<AutomationAdmission> {
        self.admissions.lock().await.clone()
    }
}

impl AutomationTurnQueue for RecordingQueue {
    fn enqueue(
        &self,
        admission: AutomationAdmission,
    ) -> AutomationFuture<'_, AutomationQueueReceipt> {
        Box::pin(async move {
            self.entered.notify_one();
            if let Some(release) = &self.release {
                release.notified().await;
            }
            self.admissions.lock().await.push(admission.clone());
            Ok(AutomationQueueReceipt {
                queued_submission_id: format!(
                    "queue-{}-{}",
                    admission.task_id, admission.occurrence
                ),
                client_user_message_id: admission.client_user_message_id,
            })
        })
    }
}

#[allow(
    clippy::expect_used,
    reason = "test task identifiers are fixed valid fixtures"
)]
fn draft(id: &str, schedule: AutomationSchedule, due: u64) -> AutomationTaskDraft {
    let mut draft = AutomationTaskDraft::new(THREAD_ID, "perform durable work", schedule, due, 1);
    draft.task_id = AutomationTaskId::parse(id).expect("task id");
    draft
}

#[tokio::test]
async fn one_shot_periodic_disable_and_cancel_are_durable() {
    let fixture = FleetFixture::new(1);
    let store = AutomationStore::open(&fixture.layouts[0])
        .await
        .expect("open store");
    let queue = Arc::new(RecordingQueue::default());
    let scheduler = AutomationScheduler::new(
        store.clone(),
        Arc::clone(&queue),
        7,
        Duration::from_secs(30),
        Duration::from_secs(2),
    )
    .expect("scheduler");

    let one = draft(
        "019153a4-3088-7000-a56a-9b1964f75001",
        AutomationSchedule::Once,
        10,
    );
    store.create_task(&one).await.expect("one-shot");
    assert!(matches!(
        scheduler.tick(10).await.expect("one-shot tick"),
        AutomationTick::Submitted { occurrence: 1, .. }
    ));
    assert_eq!(
        store.task(one.task_id).await.expect("read").unwrap().state,
        AutomationTaskState::Completed
    );

    let periodic = draft(
        "019153a4-3088-7000-a56a-9b1964f75002",
        AutomationSchedule::FixedInterval { interval_ms: 5_000 },
        20,
    );
    store.create_task(&periodic).await.expect("periodic");
    scheduler.tick(20).await.expect("periodic tick");
    assert_eq!(
        store
            .task(periodic.task_id)
            .await
            .expect("read")
            .unwrap()
            .next_run_at_ms,
        Some(5_020)
    );
    store
        .set_enabled(periodic.task_id, false, None, 21)
        .await
        .expect("disable");
    assert_eq!(
        scheduler.tick(9_000).await.expect("disabled tick"),
        AutomationTick::Idle
    );
    store
        .set_enabled(periodic.task_id, true, Some(10_000), 22)
        .await
        .expect("enable");
    scheduler.tick(10_000).await.expect("resumed tick");
    store
        .cancel_task(periodic.task_id, 23)
        .await
        .expect("cancel");
    assert_eq!(
        store
            .task(periodic.task_id)
            .await
            .expect("read")
            .unwrap()
            .state,
        AutomationTaskState::Cancelled
    );
    assert_eq!(queue.admissions().await.len(), 3);
}

#[tokio::test]
async fn restart_reclaims_same_occurrence_with_same_core_client_id_and_fences_old_lease() {
    let fixture = FleetFixture::new(5);
    let first = AutomationStore::open(&fixture.layouts[0])
        .await
        .expect("first open");
    let mut peers = Vec::new();
    for (index, layout) in fixture.layouts.iter().enumerate().skip(1) {
        let store = AutomationStore::open(layout).await.expect("open peer");
        let task_id = format!("019153a4-3088-71{index:02x}-a56a-9b1964f76{index:03x}");
        store
            .create_task(&draft(
                &task_id,
                AutomationSchedule::Once,
                1_000 + u64::try_from(index).expect("peer index"),
            ))
            .await
            .expect("create peer task");
        peers.push(store);
    }
    let task = draft(
        "019153a4-3088-7000-a56a-9b1964f75003",
        AutomationSchedule::Once,
        100,
    );
    first.create_task(&task).await.expect("create task");
    let old = first
        .claim_due(100, 1, 60_000)
        .await
        .expect("claim")
        .expect("due lease");
    drop(first);

    let restarted = AutomationStore::open(&fixture.layouts[0])
        .await
        .expect("reopen");
    assert_eq!(
        restarted
            .recover_stale_generation(2)
            .await
            .expect("recover"),
        1
    );
    let new = restarted
        .claim_due(101, 2, 60_000)
        .await
        .expect("reclaim")
        .expect("recovered lease");
    assert_eq!(new.occurrence, old.occurrence);
    assert_eq!(new.client_user_message_id, old.client_user_message_id);
    assert_ne!(new.lease_token, old.lease_token);
    assert!(matches!(
        restarted.release_for_retry(&old).await,
        Err(AutomationError::Conflict)
    ));
    for peer in peers {
        let tasks = peer.list_tasks(1).await.expect("peer remains available");
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].owner_agent_id, *peer.owner_agent_id());
        assert_eq!(tasks[0].state, AutomationTaskState::Enabled);
    }
}

#[tokio::test]
async fn disabling_an_inflight_lease_never_resurrects_the_task() {
    let fixture = FleetFixture::new(1);
    let store = AutomationStore::open(&fixture.layouts[0])
        .await
        .expect("open store");
    let task = draft(
        "019153a4-3088-7000-a56a-9b1964f75004",
        AutomationSchedule::FixedInterval { interval_ms: 5_000 },
        100,
    );
    store.create_task(&task).await.expect("create task");
    let lease = store
        .claim_due(100, 1, 60_000)
        .await
        .expect("claim")
        .expect("due lease");

    let disabled = store
        .set_enabled(task.task_id, false, None, 101)
        .await
        .expect("disable in-flight task");
    assert_eq!(disabled.state, AutomationTaskState::Disabled);
    assert!(matches!(
        store.set_enabled(task.task_id, true, Some(200), 102).await,
        Err(AutomationError::Conflict)
    ));

    let submitted = store
        .mark_submitted(
            &lease,
            &AutomationQueueReceipt {
                queued_submission_id: "queue-in-flight".to_string(),
                client_user_message_id: lease.client_user_message_id.clone(),
            },
            103,
        )
        .await
        .expect("finish admitted occurrence");
    assert_eq!(submitted.state, AutomationTaskState::Disabled);
    assert_eq!(submitted.next_run_at_ms, None);
    let resumed = store
        .set_enabled(task.task_id, true, Some(200), 104)
        .await
        .expect("resume after admitted occurrence finishes");
    assert_eq!(resumed.state, AutomationTaskState::Enabled);
    assert_eq!(resumed.next_run_at_ms, Some(200));
}

#[tokio::test]
async fn copied_database_cannot_cross_agent_owner_boundary() {
    let fixture = FleetFixture::new(2);
    let first = AutomationStore::open(&fixture.layouts[0])
        .await
        .expect("first store");
    first
        .create_task(&draft(
            "019153a4-3088-7000-a56a-9b1964f75005",
            AutomationSchedule::Once,
            100,
        ))
        .await
        .expect("first task");
    let first_path = first.path().to_path_buf();
    first.close().await;

    let second_path = fixture.layouts[1]
        .automation_root()
        .join("automation_1.sqlite3");
    std::fs::copy(first_path, second_path).expect("copy owner-bound database");
    assert!(matches!(
        AutomationStore::open(&fixture.layouts[1]).await,
        Err(AutomationError::AccessDenied)
    ));
}

#[tokio::test]
async fn five_real_agent_identities_are_isolated_and_one_blocked_backlog_cannot_starve_peers() {
    let fixture = FleetFixture::new(5);
    let mut stores = Vec::new();
    for (index, layout) in fixture.layouts.iter().enumerate() {
        let store = AutomationStore::open(layout).await.expect("agent store");
        let count = if index == 0 { 32 } else { 1 };
        for task_index in 0..count {
            let task_id = format!(
                "019153a4-3088-7{:03x}-a56a-9b1964f7{:04x}",
                index,
                task_index + 0x100
            );
            store
                .create_task(&draft(&task_id, AutomationSchedule::Once, 1))
                .await
                .expect("create isolated task");
        }
        stores.push(store);
    }
    let unique_paths = stores
        .iter()
        .map(|store| store.path().to_path_buf())
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(unique_paths.len(), 5);
    assert_eq!(
        stores
            .iter()
            .map(|store| store.owner_agent_id().clone())
            .collect::<std::collections::BTreeSet<_>>()
            .len(),
        5
    );

    let release_a = Arc::new(Notify::new());
    let queue_a = Arc::new(RecordingQueue::blocked(Arc::clone(&release_a)));
    let scheduler_a = Arc::new(
        AutomationScheduler::new(
            stores[0].clone(),
            Arc::clone(&queue_a),
            1,
            Duration::from_secs(30),
            Duration::from_secs(10),
        )
        .expect("A scheduler"),
    );
    let a_task = {
        let scheduler_a = Arc::clone(&scheduler_a);
        tokio::spawn(async move { scheduler_a.tick(1).await })
    };
    queue_a.entered.notified().await;

    for (index, store) in stores.iter().enumerate().skip(1) {
        let queue = Arc::new(RecordingQueue::default());
        let scheduler = AutomationScheduler::new(
            store.clone(),
            Arc::clone(&queue),
            u64::try_from(index + 1).expect("generation"),
            Duration::from_secs(30),
            Duration::from_secs(2),
        )
        .expect("peer scheduler");
        let outcome = tokio::time::timeout(Duration::from_secs(1), scheduler.tick(1))
            .await
            .expect("peer must not wait for A")
            .expect("peer tick");
        assert!(matches!(outcome, AutomationTick::Submitted { .. }));
        let admissions = queue.admissions().await;
        assert_eq!(admissions.len(), 1);
        assert_eq!(admissions[0].agent_id, *store.owner_agent_id());
    }

    assert!(!a_task.is_finished(), "A remains deliberately blocked");
    release_a.notify_waiters();
    assert!(matches!(
        a_task.await.expect("join A").expect("A tick"),
        AutomationTick::Submitted { .. }
    ));
}
