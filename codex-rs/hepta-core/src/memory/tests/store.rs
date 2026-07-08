use std::future::Future;
use std::pin::Pin;

use super::*;

struct StubMemoryReportStore;

impl MemoryStore for StubMemoryReportStore {
    async fn put(&self, _record: MemoryRecord) -> Result<(), crate::MemoryError> {
        Ok(())
    }

    async fn search(&self, _query: MemoryQuery) -> Result<Vec<MemoryRecord>, crate::MemoryError> {
        Ok(Vec::new())
    }
}

impl MemoryReportStore for StubMemoryReportStore {
    async fn search_report(
        &self,
        query: MemoryQuery,
    ) -> Result<MemoryQueryReport, crate::MemoryError> {
        Ok(MemoryQueryReport::from_hits(query, 0, Vec::new()))
    }
}

impl MemoryProvider for StubMemoryReportStore {
    async fn query(
        &self,
        request: ContextRecallRequest,
    ) -> Result<ContextRecallBundle, crate::MemoryError> {
        Ok(ContextRecallBundle {
            request,
            recent_entries: Vec::new(),
            transcript_hits: Vec::new(),
            durable_memory_hits: Vec::new(),
            summary_hits: Vec::new(),
            active_topic_sessions: Vec::new(),
            active_neurons: Vec::new(),
            budget: ContextBudget::default(),
            ranked_items: Vec::new(),
            omitted_by_budget: 0,
            truncated: false,
        })
    }

    async fn update_context(
        &self,
        request: ContextRecallRequest,
    ) -> Result<MemoryProviderContextUpdateEnvelope, crate::MemoryError> {
        let bundle = MemoryProvider::query(self, request).await?;
        Ok(MemoryProviderContextUpdateEnvelope::from_bundle(
            "stub",
            &bundle,
            ContextRecallLimitPressure::default(),
        ))
    }

    async fn report(
        &self,
        request: ContextRecallRequest,
    ) -> Result<MemoryProviderReport, crate::MemoryError> {
        Ok(MemoryProviderReport::from_update(
            MemoryProviderDescriptor::builtin(),
            MemoryProvider::update_context(self, request).await?,
        ))
    }

    async fn clear(
        &self,
        request: MemoryProviderClearRequest,
    ) -> Result<MemoryProviderClearReport, crate::MemoryError> {
        if request.dry_run {
            Ok(MemoryProviderClearReport::dry_run("stub", request.scope))
        } else {
            Ok(MemoryProviderClearReport::blocked("stub", request.scope))
        }
    }
}

impl MemoryProviderV2 for StubMemoryReportStore {
    async fn query(
        &self,
        request: ContextRecallRequest,
    ) -> Result<ContextRecallBundle, crate::MemoryError> {
        <Self as MemoryProvider>::query(self, request).await
    }

    async fn update_context(
        &self,
        request: ContextRecallRequest,
    ) -> Result<MemoryProviderContextUpdateEnvelope, crate::MemoryError> {
        <Self as MemoryProvider>::update_context(self, request).await
    }

    async fn propose_write(
        &self,
        queue: ContextMemoryFormationQueueReport,
    ) -> Result<MemoryProviderWriteProposalReport, crate::MemoryError> {
        Ok(MemoryProviderWriteProposalReport::from_formation_queue(
            "stub", &queue,
        ))
    }

    async fn add(
        &self,
        request: MemoryProviderAddRequest,
    ) -> Result<MemoryProviderAddReport, crate::MemoryError> {
        if request.dry_run {
            Ok(MemoryProviderAddReport::dry_run(&request.proposal))
        } else {
            Ok(MemoryProviderAddReport::blocked(&request.proposal))
        }
    }

    async fn clear(
        &self,
        request: MemoryProviderClearRequest,
    ) -> Result<MemoryProviderClearReport, crate::MemoryError> {
        <Self as MemoryProvider>::clear(self, request).await
    }

    async fn close(&self) -> Result<MemoryProviderCloseReport, crate::MemoryError> {
        Ok(MemoryProviderCloseReport::shadow_noop("stub"))
    }
}

fn assert_memory_report_store<T: MemoryReportStore>() {}
fn assert_memory_provider<T: MemoryProvider>() {}
fn assert_memory_provider_v2<T: MemoryProviderV2>() {}

fn reported_search_future<'a, T: MemoryReportStore + ?Sized>(
    store: &'a T,
    query: MemoryQuery,
) -> Pin<Box<dyn Future<Output = Result<MemoryQueryReport, crate::MemoryError>> + 'a>> {
    Box::pin(store.search_report(query))
}

fn update_context_future<'a, T: MemoryProvider + ?Sized>(
    store: &'a T,
    request: ContextRecallRequest,
) -> Pin<
    Box<dyn Future<Output = Result<MemoryProviderContextUpdateEnvelope, crate::MemoryError>> + 'a>,
> {
    Box::pin(store.update_context(request))
}

fn propose_write_future<'a, T: MemoryProviderV2 + ?Sized>(
    store: &'a T,
    queue: ContextMemoryFormationQueueReport,
) -> Pin<
    Box<
        dyn Future<Output = Result<MemoryProviderWriteProposalReport, crate::MemoryError>>
            + Send
            + 'a,
    >,
> {
    Box::pin(store.propose_write(queue))
}

#[test]
fn memory_report_store_trait_supports_report_queries() {
    assert_memory_report_store::<StubMemoryReportStore>();

    let _future = reported_search_future(
        &StubMemoryReportStore,
        MemoryQuery {
            text: "snapshot".into(),
            limit: 2,
        },
    );
}

#[test]
fn memory_provider_trait_supports_query_update_report_and_clear_boundaries() {
    assert_memory_provider::<StubMemoryReportStore>();

    let request = ContextRecallRequest {
        session_id: SessionId("session-1".into()),
        query_text: Some("payload should not enter update envelope".into()),
        recent_window_limit: 2,
        transcript_limit: 2,
        memory_limit: 2,
        allow_cross_session: false,
    };
    let _future = update_context_future(&StubMemoryReportStore, request);

    let envelope = MemoryProviderContextUpdateEnvelope::from_bundle(
        "stub",
        &ContextRecallBundle {
            request: ContextRecallRequest {
                session_id: SessionId("session-1".into()),
                query_text: Some("hidden query".into()),
                recent_window_limit: 2,
                transcript_limit: 2,
                memory_limit: 2,
                allow_cross_session: false,
            },
            recent_entries: Vec::new(),
            transcript_hits: Vec::new(),
            durable_memory_hits: Vec::new(),
            summary_hits: Vec::new(),
            active_topic_sessions: Vec::new(),
            active_neurons: Vec::new(),
            budget: ContextBudget::default(),
            ranked_items: Vec::new(),
            omitted_by_budget: 0,
            truncated: false,
        },
        ContextRecallLimitPressure::default(),
    );

    assert!(envelope.has_payload_light_boundary());
    let json = serde_json::to_string(&envelope).expect("provider envelope should serialize");
    assert!(!json.contains("hidden query"));
    assert!(!json.contains("payload should not enter"));
    assert!(!json.contains("\"runtime_activation\":true"));
    assert!(!json.contains("\"write_performed\":true"));

    let clear = MemoryProviderClearReport::blocked("stub", MemoryProviderClearScope::All);
    assert!(clear.blocked);
    assert!(clear.has_no_side_effects());
}

#[test]
fn memory_provider_v2_trait_supports_shadow_write_lifecycle_boundaries() {
    assert_memory_provider_v2::<StubMemoryReportStore>();

    let queue = ContextMemoryFormationQueueReport::default();
    let _future = propose_write_future(&StubMemoryReportStore, queue.clone());
    let proposal = MemoryProviderWriteProposalReport::from_formation_queue("stub", &queue);
    let dry_run_add = MemoryProviderAddReport::dry_run(&proposal);
    let blocked_add = MemoryProviderAddReport::blocked(&proposal);
    let close = MemoryProviderCloseReport::shadow_noop("stub");
    let clear = MemoryProviderClearReport::blocked("stub", MemoryProviderClearScope::All);
    let update = MemoryProviderContextUpdateEnvelope::from_bundle(
        "stub",
        &ContextRecallBundle {
            request: ContextRecallRequest {
                session_id: SessionId("session-1".into()),
                query_text: Some("hidden query".into()),
                recent_window_limit: 2,
                transcript_limit: 2,
                memory_limit: 2,
                allow_cross_session: false,
            },
            recent_entries: Vec::new(),
            transcript_hits: Vec::new(),
            durable_memory_hits: Vec::new(),
            summary_hits: Vec::new(),
            active_topic_sessions: Vec::new(),
            active_neurons: Vec::new(),
            budget: ContextBudget::default(),
            ranked_items: Vec::new(),
            omitted_by_budget: 0,
            truncated: false,
        },
        ContextRecallLimitPressure::default(),
    );
    let audit = MemoryProviderV2AuditReport::from_parts(
        MemoryProviderDescriptor::builtin(),
        update,
        proposal.clone(),
        blocked_add.clone(),
        clear,
        close.clone(),
    );

    assert!(proposal.has_shadow_boundary_integrity());
    assert!(dry_run_add.has_no_side_effects());
    assert!(blocked_add.blocked);
    assert!(blocked_add.has_no_side_effects());
    assert!(close.has_no_side_effects());
    assert!(audit.has_shadow_boundary_integrity());

    let json = serde_json::to_string(&audit).expect("provider v2 audit should serialize");
    assert!(!json.contains("hidden query"));
    assert!(!json.contains("session-1"));
    assert!(!json.contains("\"candidate_payload_exported\":true"));
    assert!(!json.contains("\"write_performed\":true"));
    assert!(!json.contains("\"graph_write_performed\":true"));
    assert!(!json.contains("\"runtime_activation\":true"));
}
