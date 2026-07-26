use std::sync::Arc;
use std::time::Duration;

use crate::outgoing_message::OutgoingMessageSender;
use codex_app_server_protocol::ServerNotification;
use codex_app_server_protocol::SkillsChangedNotification;
use codex_core::ThreadManager;
use codex_core::config::Config;
use codex_core::skills::SkillsLoadInput;
use codex_core::skills::SkillsManager;
use codex_file_watcher::FileWatcher;
use codex_file_watcher::FileWatcherSubscriber;
use codex_file_watcher::Receiver;
use codex_file_watcher::ThrottledWatchReceiver;
use codex_file_watcher::WatchPath;
use codex_file_watcher::WatchRegistration;
use codex_protocol::protocol::SkillScope;
use codex_protocol::protocol::TurnEnvironmentSelection;
use codex_skills::system_cache_root_dir;
use codex_utils_absolute_path::AbsolutePathBuf;
use tracing::warn;

#[cfg(not(test))]
const WATCHER_THROTTLE_INTERVAL: Duration = Duration::from_secs(10);
#[cfg(test)]
const WATCHER_THROTTLE_INTERVAL: Duration = Duration::from_millis(50);

pub(crate) struct SkillsWatcher {
    subscriber: FileWatcherSubscriber,
}

impl SkillsWatcher {
    pub(crate) fn new(
        skills_manager: Arc<SkillsManager>,
        codex_home: &AbsolutePathBuf,
        outgoing: Arc<OutgoingMessageSender>,
    ) -> Arc<Self> {
        let file_watcher = match FileWatcher::new() {
            Ok(file_watcher) => Arc::new(file_watcher),
            Err(err) => {
                warn!("failed to initialize skills file watcher: {err}");
                Arc::new(FileWatcher::noop())
            }
        };
        let (subscriber, rx) = file_watcher.add_subscriber();
        Self::spawn_event_loop(
            rx,
            skills_manager,
            system_cache_root_dir(codex_home),
            outgoing,
        );
        Arc::new(Self { subscriber })
    }

    pub(crate) async fn register_thread_config(
        &self,
        config: &Config,
        thread_manager: &ThreadManager,
        environments: &[TurnEnvironmentSelection],
    ) -> WatchRegistration {
        let Some(environment_selection) = environments.first() else {
            return WatchRegistration::default();
        };
        let Some(environment) = thread_manager
            .environment_manager()
            .get_environment(&environment_selection.environment_id)
        else {
            warn!(
                "failed to register skills watcher for unknown environment `{}`",
                environment_selection.environment_id
            );
            return WatchRegistration::default();
        };
        if environment.is_remote() {
            return WatchRegistration::default();
        }

        let plugins_input = config.plugins_config_input();
        let plugins_manager = thread_manager.plugins_manager();
        let plugin_outcome = plugins_manager.plugins_for_config(&plugins_input).await;
        let skills_input = SkillsLoadInput::new(
            config.cwd.clone(),
            plugin_outcome.effective_plugin_skill_roots(),
            config.config_layer_stack.clone(),
            config.bundled_skills_enabled(),
        );
        let roots = thread_manager
            .skills_manager()
            .skill_roots_for_config(&skills_input, Some(environment.get_filesystem()))
            .await
            .into_iter()
            .filter(|root| should_watch_skill_scope(root.scope))
            .map(|root| WatchPath {
                path: root.path.into_path_buf(),
                recursive: true,
            })
            .collect();
        self.subscriber.register_paths(roots)
    }

    fn spawn_event_loop(
        rx: Receiver,
        skills_manager: Arc<SkillsManager>,
        system_skills_root: AbsolutePathBuf,
        outgoing: Arc<OutgoingMessageSender>,
    ) {
        let mut rx = ThrottledWatchReceiver::new(rx, WATCHER_THROTTLE_INTERVAL);
        let Ok(handle) = tokio::runtime::Handle::try_current() else {
            warn!("skills watcher listener skipped: no Tokio runtime available");
            return;
        };
        handle.spawn(async move {
            while let Some(event) = rx.recv().await {
                if !should_process_skill_event(&event.paths, &system_skills_root) {
                    continue;
                }
                skills_manager.clear_cache();
                outgoing
                    .send_server_notification(ServerNotification::SkillsChanged(
                        SkillsChangedNotification {},
                    ))
                    .await;
            }
        });
    }
}

fn should_watch_skill_scope(scope: SkillScope) -> bool {
    scope != SkillScope::System
}

fn should_process_skill_event(
    paths: &[std::path::PathBuf],
    system_skills_root: &AbsolutePathBuf,
) -> bool {
    paths
        .iter()
        .any(|path| !path.starts_with(system_skills_root.as_path()))
}

#[cfg(test)]
mod tests {
    use super::should_process_skill_event;
    use super::should_watch_skill_scope;
    use codex_protocol::protocol::SkillScope;
    use codex_skills::system_cache_root_dir;
    use codex_utils_absolute_path::AbsolutePathBuf;
    use pretty_assertions::assert_eq;

    #[test]
    fn generated_system_scope_is_not_watched() {
        assert_eq!(
            [
                SkillScope::User,
                SkillScope::Repo,
                SkillScope::System,
                SkillScope::Admin,
            ]
            .map(should_watch_skill_scope),
            [true, true, false, true]
        );
    }

    #[test]
    fn pure_system_cache_events_are_ignored() {
        let codex_home = AbsolutePathBuf::try_from(std::env::temp_dir().join("codex-home"))
            .expect("absolute temporary Codex home");
        let system_root = system_cache_root_dir(&codex_home);
        let system_event = system_root.join("imagegen/SKILL.md").into_path_buf();
        let user_event = codex_home.join("skills/local/SKILL.md").into_path_buf();

        assert_eq!(
            should_process_skill_event(std::slice::from_ref(&system_event), &system_root),
            false
        );
        assert_eq!(
            should_process_skill_event(&[system_event, user_event], &system_root),
            true
        );
    }
}
