#![forbid(unsafe_code)]

use std::collections::HashSet;
use std::future::Future;
use std::time::Duration;

use anyhow::Result;
use anyhow::anyhow;

pub const DEFAULT_OVERALL_TIMEOUT: Duration = Duration::from_secs(30);
pub const MAX_PAGES: usize = 100;
/// Maximum catalog size accepted across all pages of one discovery request.
///
/// This matches the upstream MCP catalog limit while retaining a hard,
/// fail-closed aggregate bound for canonical and compatibility clients.
pub const MAX_ITEMS: usize = 2_048;
pub const MAX_CURSOR_BYTES: usize = 64 * 1_024;

/// Collects an MCP paginated response under one shared, fail-closed budget.
///
/// The callback receives the cursor for the next request. Keeping this crate
/// independent of an RMCP model version lets the canonical and compatibility
/// clients share exactly the same resource limits.
pub async fn collect_paginated<T, F, Fut>(
    method: &str,
    overall_timeout: Option<Duration>,
    mut fetch: F,
) -> Result<Vec<T>>
where
    F: FnMut(Option<String>) -> Fut,
    Fut: Future<Output = Result<(Vec<T>, Option<String>)>>,
{
    let collect = async {
        let mut collected = Vec::new();
        let mut cursor = None;
        let mut seen_cursors = HashSet::new();

        for page in 1..=MAX_PAGES {
            let (items, next_cursor) = fetch(cursor.clone()).await?;
            if collected.len().saturating_add(items.len()) > MAX_ITEMS {
                return Err(anyhow!(
                    "{method} exceeded the {MAX_ITEMS}-item pagination budget"
                ));
            }
            collected.extend(items);

            let Some(next_cursor) = next_cursor else {
                return Ok(collected);
            };
            if next_cursor.len() > MAX_CURSOR_BYTES {
                return Err(anyhow!(
                    "{method} returned a cursor larger than {MAX_CURSOR_BYTES} bytes"
                ));
            }
            if !seen_cursors.insert(next_cursor.clone()) {
                return Err(anyhow!("{method} returned a repeated pagination cursor"));
            }
            if page == MAX_PAGES {
                return Err(anyhow!(
                    "{method} exceeded the {MAX_PAGES}-page pagination budget"
                ));
            }
            cursor = Some(next_cursor);
        }
        unreachable!("bounded pagination loop always returns")
    };

    let timeout = overall_timeout.unwrap_or(DEFAULT_OVERALL_TIMEOUT);
    tokio::time::timeout(timeout, collect)
        .await
        .map_err(|_| anyhow!("{method} pagination timed out after {timeout:?}"))?
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn accepts_finite_pages() {
        let mut page = 0;
        let items = collect_paginated("tools/list", None, |_| {
            page += 1;
            async move {
                Ok(match page {
                    1 => (vec![1, 2], Some("next".to_string())),
                    _ => (vec![3], None),
                })
            }
        })
        .await
        .expect("finite pagination");
        assert_eq!(items, vec![1, 2, 3]);
    }

    #[tokio::test]
    async fn accepts_exact_item_budget_across_pages() {
        let mut page = 0;
        let items = collect_paginated("tools/list", None, |_| {
            page += 1;
            async move {
                Ok((
                    vec![page as u16; MAX_ITEMS / 2],
                    (page == 1).then(|| "next".to_string()),
                ))
            }
        })
        .await
        .expect("exact item budget");

        assert_eq!(items.len(), MAX_ITEMS);
        assert!(items[..MAX_ITEMS / 2].iter().all(|item| *item == 1));
        assert!(items[MAX_ITEMS / 2..].iter().all(|item| *item == 2));
    }

    #[tokio::test]
    async fn rejects_repeated_and_oversized_cursors() {
        let repeated = collect_paginated("tools/list", None, |_| async {
            Ok((Vec::<u8>::new(), Some("same".to_string())))
        })
        .await
        .expect_err("repeated cursor");
        assert!(repeated.to_string().contains("repeated pagination cursor"));

        let oversized = collect_paginated("tools/list", None, |_| async {
            Ok((Vec::<u8>::new(), Some("x".repeat(MAX_CURSOR_BYTES + 1))))
        })
        .await
        .expect_err("oversized cursor");
        assert!(oversized.to_string().contains("cursor larger"));
    }

    #[tokio::test]
    async fn rejects_page_and_item_budget_overruns() {
        let pages = collect_paginated("tools/list", None, |cursor| async move {
            let next = cursor
                .as_deref()
                .unwrap_or("0")
                .parse::<usize>()
                .expect("numeric cursor")
                + 1;
            Ok((Vec::<u8>::new(), Some(next.to_string())))
        })
        .await
        .expect_err("page budget");
        assert!(pages.to_string().contains("page pagination budget"));

        let items = collect_paginated("tools/list", None, |_| async {
            Ok((vec![0_u8; MAX_ITEMS + 1], None))
        })
        .await
        .expect_err("item budget");
        assert!(items.to_string().contains("item pagination budget"));
    }

    #[tokio::test]
    async fn rejects_single_item_past_budget_across_pages() {
        let mut page = 0;
        let error = collect_paginated("resources/list", None, |_| {
            page += 1;
            async move {
                Ok(match page {
                    1 => (vec![0_u8; MAX_ITEMS], Some("next".to_string())),
                    _ => (vec![0_u8], None),
                })
            }
        })
        .await
        .expect_err("single item beyond aggregate budget");

        assert!(error.to_string().contains("2048-item pagination budget"));
    }
}
