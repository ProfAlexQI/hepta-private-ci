use crate::config_manager::ConfigManager;
use codex_core::CodexThread;
use codex_core::ThreadManager;
use codex_core::config::Config;
use codex_protocol::protocol::McpServerRefreshConfig;
use std::io;
use std::sync::Arc;
use std::sync::OnceLock;
use tokio::sync::Mutex;
use tracing::warn;

static MCP_REFRESH_SERIALIZER: OnceLock<Mutex<()>> = OnceLock::new();

fn mcp_refresh_serializer() -> &'static Mutex<()> {
    MCP_REFRESH_SERIALIZER.get_or_init(|| Mutex::new(()))
}

pub(crate) async fn queue_strict_refresh(
    thread_manager: &Arc<ThreadManager>,
    config_manager: &ConfigManager,
) -> io::Result<()> {
    let _refresh_guard = mcp_refresh_serializer().lock().await;
    config_manager
        .load_latest_config(/*fallback_cwd*/ None)
        .await?;
    let generation = thread_manager.begin_mcp_runtime_invalidation();
    thread_manager
        .wait_for_mcp_startups_before(generation)
        .await;
    let mut refreshes = Vec::new();
    for thread_id in thread_manager.mcp_threads_stale_before(generation).await {
        let thread = thread_manager
            .get_thread(thread_id)
            .await
            .map_err(|err| io::Error::other(format!("failed to load thread {thread_id}: {err}")))?;
        let plan =
            build_refresh_plan(thread_manager, config_manager, thread.config().await).await?;
        refreshes.push((thread_id, thread, plan));
    }
    for (_thread_id, thread, plan) in refreshes {
        apply_refresh(thread, plan).await;
    }
    Ok(())
}

pub(crate) async fn queue_best_effort_refresh(
    thread_manager: &Arc<ThreadManager>,
    config_manager: &ConfigManager,
) {
    let _refresh_guard = mcp_refresh_serializer().lock().await;
    let generation = thread_manager.begin_mcp_runtime_invalidation();
    thread_manager
        .wait_for_mcp_startups_before(generation)
        .await;
    for thread_id in thread_manager.mcp_threads_stale_before(generation).await {
        let thread = match thread_manager.get_thread(thread_id).await {
            Ok(thread) => thread,
            Err(err) => {
                warn!("failed to load thread {thread_id} for MCP refresh: {err}");
                continue;
            }
        };
        let plan =
            match build_refresh_plan(thread_manager, config_manager, thread.config().await).await {
                Ok(plan) => plan,
                Err(err) => {
                    warn!("failed to build MCP refresh config for thread {thread_id}: {err}");
                    continue;
                }
            };
        apply_refresh(thread, plan).await;
    }
}

struct McpRefreshPlan {
    thread_config: Config,
    runtime_config: McpServerRefreshConfig,
}

async fn build_refresh_plan(
    thread_manager: &ThreadManager,
    config_manager: &ConfigManager,
    thread_config: Arc<Config>,
) -> io::Result<McpRefreshPlan> {
    let config = config_manager
        .load_latest_config_for_thread(thread_config.as_ref())
        .await?;
    let mcp_servers = thread_manager
        .mcp_manager()
        .configured_servers(&config)
        .await;
    let runtime_config = McpServerRefreshConfig {
        mcp_servers: serde_json::to_value(mcp_servers).map_err(io::Error::other)?,
        mcp_oauth_credentials_store_mode: serde_json::to_value(
            config.mcp_oauth_credentials_store_mode,
        )
        .map_err(io::Error::other)?,
        elicitation_authority: Some(codex_core::connectors::mcp_elicitation_authority(&config)),
    };
    Ok(McpRefreshPlan {
        thread_config: config,
        runtime_config,
    })
}

async fn apply_refresh(thread: Arc<CodexThread>, plan: McpRefreshPlan) {
    let McpRefreshPlan {
        thread_config,
        runtime_config,
    } = plan;
    thread
        .refresh_mcp_config(thread_config, runtime_config)
        .await;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::extensions::guardian_agent_spawner;
    use crate::extensions::thread_extensions;
    use async_trait::async_trait;
    use codex_arg0::Arg0DispatchPaths;
    use codex_config::CloudRequirementsLoader;
    use codex_config::LoaderOverrides;
    use codex_config::ThreadConfigContext;
    use codex_config::ThreadConfigLoadError;
    use codex_config::ThreadConfigLoadErrorCode;
    use codex_config::ThreadConfigLoader;
    use codex_config::ThreadConfigSource;
    use codex_core::config::ConfigOverrides;
    use codex_core::init_state_db;
    use codex_core::thread_store_from_config;
    use codex_exec_server::EnvironmentManager;
    use codex_login::AuthManager;
    use codex_login::CodexAuth;
    use codex_protocol::protocol::SessionSource;
    use codex_utils_absolute_path::AbsolutePathBuf;
    use pretty_assertions::assert_eq;
    use std::future::Future;
    use std::sync::atomic::AtomicUsize;
    use std::sync::atomic::Ordering;
    use tempfile::TempDir;
    use tokio::task::yield_now;

    #[tokio::test]
    async fn overlapping_refreshes_are_process_serialized() {
        let guard = mcp_refresh_serializer().lock().await;
        let entered = Arc::new(AtomicUsize::new(0));
        let entered_after_lock = Arc::clone(&entered);
        let waiter = tokio::spawn(async move {
            let _guard = mcp_refresh_serializer().lock().await;
            entered_after_lock.store(1, Ordering::Release);
        });
        yield_now().await;
        assert_eq!(entered.load(Ordering::Acquire), 0);
        drop(guard);
        waiter.await.expect("refresh waiter");
        assert_eq!(entered.load(Ordering::Acquire), 1);
    }

    #[test]
    fn strict_refresh_reports_thread_planning_failures() -> anyhow::Result<()> {
        run_refresh_test(|| async {
            let (_temp_dir, thread_manager, config_manager, _loader) = refresh_test_state().await?;
            let err = queue_strict_refresh(&thread_manager, &config_manager)
                .await
                .expect_err("strict refresh should fail");
            assert_eq!(err.to_string(), "failed to load refresh config");
            Ok(())
        })
    }

    #[test]
    fn best_effort_refresh_attempts_every_loaded_thread() -> anyhow::Result<()> {
        run_refresh_test(|| async {
            let (_temp_dir, thread_manager, config_manager, loader) = refresh_test_state().await?;
            queue_best_effort_refresh(&thread_manager, &config_manager).await;
            assert_eq!(loader.good_loads.load(Ordering::Relaxed), 1);
            assert_eq!(loader.bad_loads.load(Ordering::Relaxed), 1);
            let mut good_generation = None;
            let mut bad_generation = None;
            for thread_id in thread_manager.list_thread_ids().await {
                let config = thread_manager.get_thread(thread_id).await?.config().await;
                if config.cwd == loader.good_cwd {
                    good_generation = Some(config.config_generation().value());
                }
                if config.cwd == loader.bad_cwd {
                    bad_generation = Some(config.config_generation().value());
                }
            }
            assert_eq!(good_generation, Some(1));
            assert_eq!(bad_generation, Some(0));
            let current_generation = thread_manager.config_generation_source().current();
            assert_eq!(current_generation, 1);
            assert_eq!(
                thread_manager
                    .mcp_threads_stale_before(current_generation)
                    .await
                    .len(),
                2,
            );
            Ok(())
        })
    }

    fn run_refresh_test<F, Fut>(test: F) -> anyhow::Result<()>
    where
        F: FnOnce() -> Fut + Send + 'static,
        Fut: Future<Output = anyhow::Result<()>> + Send + 'static,
    {
        std::thread::Builder::new()
            .name("mcp-refresh-test".into())
            .stack_size(8 * 1024 * 1024)
            .spawn(move || {
                tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()?
                    .block_on(test())
            })?
            .join()
            .map_err(|_| anyhow::anyhow!("MCP refresh test thread panicked"))?
    }

    async fn refresh_test_state() -> anyhow::Result<(
        TempDir,
        Arc<ThreadManager>,
        ConfigManager,
        Arc<CountingThreadConfigLoader>,
    )> {
        let temp_dir = TempDir::new()?;
        let good_cwd = temp_dir.path().join("good");
        let bad_cwd = temp_dir.path().join("bad");
        std::fs::create_dir_all(&good_cwd)?;
        std::fs::create_dir_all(&bad_cwd)?;

        let initial_config_manager =
            ConfigManager::without_managed_config_for_tests(temp_dir.path().to_path_buf());
        let good_config = initial_config_manager
            .load_for_cwd(
                /*request_overrides*/ None,
                ConfigOverrides::default(),
                Some(good_cwd.clone()),
            )
            .await?;
        let bad_config = initial_config_manager
            .load_for_cwd(
                /*request_overrides*/ None,
                ConfigOverrides::default(),
                Some(bad_cwd.clone()),
            )
            .await?;

        let auth_manager = AuthManager::from_auth_for_testing(CodexAuth::from_api_key("dummy"));
        let state_db = init_state_db(&good_config)
            .await
            .expect("refresh tests require state db");
        let thread_store = thread_store_from_config(&good_config, Some(state_db.clone()));
        let thread_manager = Arc::new_cyclic(|thread_manager| {
            ThreadManager::new(
                &good_config,
                auth_manager,
                SessionSource::Exec,
                Arc::new(EnvironmentManager::default_for_tests()),
                thread_extensions(guardian_agent_spawner(thread_manager.clone())),
                /*analytics_events_client*/ None,
                thread_store,
                Some(state_db.clone()),
                "11111111-1111-4111-8111-111111111111".to_string(),
                /*attestation_provider*/ None,
            )
        });
        thread_manager.start_thread(good_config).await?;
        thread_manager.start_thread(bad_config).await?;

        let loader = Arc::new(CountingThreadConfigLoader {
            good_cwd: AbsolutePathBuf::try_from(good_cwd)?,
            bad_cwd: AbsolutePathBuf::try_from(bad_cwd)?,
            good_loads: AtomicUsize::new(0),
            bad_loads: AtomicUsize::new(0),
        });
        let config_manager = ConfigManager::new(
            temp_dir.path().to_path_buf(),
            Vec::new(),
            LoaderOverrides::without_managed_config_for_tests(),
            /*strict_config*/ false,
            CloudRequirementsLoader::default(),
            Arg0DispatchPaths::default(),
            loader.clone(),
        )
        .with_config_generation_source(thread_manager.config_generation_source());

        Ok((temp_dir, thread_manager, config_manager, loader))
    }

    struct CountingThreadConfigLoader {
        good_cwd: AbsolutePathBuf,
        bad_cwd: AbsolutePathBuf,
        good_loads: AtomicUsize,
        bad_loads: AtomicUsize,
    }

    #[async_trait]
    impl ThreadConfigLoader for CountingThreadConfigLoader {
        async fn load(
            &self,
            context: ThreadConfigContext,
        ) -> Result<Vec<ThreadConfigSource>, ThreadConfigLoadError> {
            if context.cwd.as_ref() == Some(&self.good_cwd) {
                self.good_loads.fetch_add(1, Ordering::Relaxed);
            }
            if context.cwd.as_ref() == Some(&self.bad_cwd) {
                self.bad_loads.fetch_add(1, Ordering::Relaxed);
                return Err(ThreadConfigLoadError::new(
                    ThreadConfigLoadErrorCode::Internal,
                    /*status_code*/ None,
                    "failed to load refresh config",
                ));
            }
            Ok(Vec::new())
        }
    }
}
