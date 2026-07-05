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

fn assert_memory_report_store<T: MemoryReportStore>() {}

fn reported_search_future<'a, T: MemoryReportStore + ?Sized>(
    store: &'a T,
    query: MemoryQuery,
) -> Pin<Box<dyn Future<Output = Result<MemoryQueryReport, crate::MemoryError>> + 'a>> {
    Box::pin(store.search_report(query))
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
