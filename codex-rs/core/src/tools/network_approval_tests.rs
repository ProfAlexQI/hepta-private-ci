use super::*;
use crate::sandboxing::SandboxPermissions;
use codex_network_proxy::BlockedRequestArgs;
use codex_protocol::models::PermissionProfile;
use codex_protocol::permissions::NetworkSandboxPolicy;
use codex_protocol::protocol::AskForApproval;
use codex_protocol::protocol::SandboxPolicy;
use core_test_support::PathBufExt;
use core_test_support::test_path_buf;
use pretty_assertions::assert_eq;
use tokio_util::sync::CancellationToken;

fn pending_key(host: HostApprovalKey, turn: &str, execution: &str) -> PendingHostApprovalKey {
    PendingHostApprovalKey {
        host,
        turn_id: turn.to_string(),
        execution_id: Some(execution.to_string()),
    }
}

#[test]
fn pending_approvals_are_deduped_within_one_execution() {
    let service = NetworkApprovalService::default();
    let key = HostApprovalKey {
        host: "example.com".to_string(),
        protocol: "http",
        port: 443,
    };

    let key = pending_key(key, "turn-1", "execution-1");
    let (first, first_is_owner) = service.get_or_create_pending_approval(key.clone());
    let (second, second_is_owner) = service.get_or_create_pending_approval(key);

    assert!(first_is_owner);
    assert!(!second_is_owner);
    assert!(Arc::ptr_eq(&first, &second));
}

#[test]
fn pending_approvals_do_not_dedupe_across_ports() {
    let service = NetworkApprovalService::default();
    let first_key = HostApprovalKey {
        host: "example.com".to_string(),
        protocol: "https",
        port: 443,
    };
    let second_key = HostApprovalKey {
        host: "example.com".to_string(),
        protocol: "https",
        port: 8443,
    };

    let (first, first_is_owner) =
        service.get_or_create_pending_approval(pending_key(first_key, "turn-1", "execution-1"));
    let (second, second_is_owner) =
        service.get_or_create_pending_approval(pending_key(second_key, "turn-1", "execution-1"));

    assert!(first_is_owner);
    assert!(second_is_owner);
    assert!(!Arc::ptr_eq(&first, &second));
}

#[test]
fn pending_approvals_do_not_dedupe_across_execution_or_turn() {
    let service = NetworkApprovalService::default();
    let host = HostApprovalKey {
        host: "example.com".to_string(),
        protocol: "https",
        port: 443,
    };
    let (first, first_owner) =
        service.get_or_create_pending_approval(pending_key(host.clone(), "turn-1", "execution-1"));
    let (second, second_owner) =
        service.get_or_create_pending_approval(pending_key(host.clone(), "turn-1", "execution-2"));
    let (third, third_owner) =
        service.get_or_create_pending_approval(pending_key(host, "turn-2", "execution-1"));

    assert!(first_owner && second_owner && third_owner);
    assert!(!Arc::ptr_eq(&first, &second));
    assert!(!Arc::ptr_eq(&first, &third));
}

#[tokio::test]
async fn abandoned_owner_denies_waiters_and_cancels_execution() {
    let service = NetworkApprovalService::default();
    let key = pending_key(
        HostApprovalKey {
            host: "example.com".to_string(),
            protocol: "https",
            port: 443,
        },
        "turn-1",
        "execution-1",
    );
    let (pending, is_owner) = service.get_or_create_pending_approval(key.clone());
    assert!(is_owner);
    let cancellation = CancellationToken::new();
    let owner = PendingHostApprovalOwner::new(
        &service,
        key,
        Arc::clone(&pending),
        Some(cancellation.clone()),
    );
    let waiter = tokio::spawn(async move { pending.wait_for_decision().await });

    drop(owner);

    assert_eq!(waiter.await.expect("waiter"), PendingApprovalDecision::Deny);
    assert!(cancellation.is_cancelled());
}

#[tokio::test]
async fn session_approved_hosts_preserve_protocol_and_port_scope() {
    let source = NetworkApprovalService::default();
    {
        let mut approved_hosts = source.session_approved_hosts.lock().await;
        approved_hosts.extend([
            HostApprovalKey {
                host: "example.com".to_string(),
                protocol: "https",
                port: 443,
            },
            HostApprovalKey {
                host: "example.com".to_string(),
                protocol: "https",
                port: 8443,
            },
            HostApprovalKey {
                host: "example.com".to_string(),
                protocol: "http",
                port: 80,
            },
        ]);
    }

    let seeded = NetworkApprovalService::default();
    assert!(source.sync_session_approved_hosts_to(&seeded).await);

    let mut copied = seeded
        .session_approved_hosts
        .lock()
        .await
        .iter()
        .cloned()
        .collect::<Vec<_>>();
    copied.sort_by(|a, b| (&a.host, a.protocol, a.port).cmp(&(&b.host, b.protocol, b.port)));

    assert_eq!(
        copied,
        vec![
            HostApprovalKey {
                host: "example.com".to_string(),
                protocol: "http",
                port: 80,
            },
            HostApprovalKey {
                host: "example.com".to_string(),
                protocol: "https",
                port: 443,
            },
            HostApprovalKey {
                host: "example.com".to_string(),
                protocol: "https",
                port: 8443,
            },
        ]
    );
}

#[tokio::test]
async fn sync_session_approved_hosts_to_replaces_existing_target_hosts() {
    let source = NetworkApprovalService::default();
    {
        let mut approved_hosts = source.session_approved_hosts.lock().await;
        approved_hosts.insert(HostApprovalKey {
            host: "source.example.com".to_string(),
            protocol: "https",
            port: 443,
        });
    }

    let target = NetworkApprovalService::default();
    {
        let mut approved_hosts = target.session_approved_hosts.lock().await;
        approved_hosts.insert(HostApprovalKey {
            host: "stale.example.com".to_string(),
            protocol: "https",
            port: 8443,
        });
    }

    assert!(source.sync_session_approved_hosts_to(&target).await);

    let copied = target
        .session_approved_hosts
        .lock()
        .await
        .iter()
        .cloned()
        .collect::<Vec<_>>();

    assert_eq!(
        copied,
        vec![HostApprovalKey {
            host: "source.example.com".to_string(),
            protocol: "https",
            port: 443,
        }]
    );
}

#[tokio::test]
async fn sync_session_approved_hosts_waits_for_an_in_flight_policy_commit() {
    let source = Arc::new(NetworkApprovalService::default());
    {
        let mut approved_hosts = source.session_approved_hosts.lock().await;
        approved_hosts.insert(HostApprovalKey {
            host: "committed.example.com".to_string(),
            protocol: "https",
            port: 443,
        });
    }
    let target = Arc::new(NetworkApprovalService::default());
    {
        let mut approved_hosts = target.session_approved_hosts.lock().await;
        approved_hosts.insert(HostApprovalKey {
            host: "pre-commit.example.com".to_string(),
            protocol: "https",
            port: 8443,
        });
    }

    let commit_permit = source
        .session_policy_commit_semaphore
        .acquire()
        .await
        .expect("the test service keeps its commit semaphore open");
    let sync_task = {
        let source = Arc::clone(&source);
        let target = Arc::clone(&target);
        tokio::spawn(async move {
            assert!(source.sync_session_approved_hosts_to(&target).await);
        })
    };
    tokio::task::yield_now().await;

    assert!(!sync_task.is_finished());
    assert!(
        target
            .session_approved_hosts
            .lock()
            .await
            .iter()
            .any(|host| host.host == "pre-commit.example.com")
    );

    drop(commit_permit);
    sync_task.await.expect("sync task should complete");

    let copied = target.session_approved_hosts.lock().await;
    assert_eq!(copied.len(), 1);
    assert!(
        copied
            .iter()
            .any(|host| host.host == "committed.example.com")
    );
}

#[tokio::test]
async fn sync_session_approved_hosts_fails_closed_and_clears_stale_target_hosts() {
    let source = NetworkApprovalService::default();
    {
        let mut approved_hosts = source.session_approved_hosts.lock().await;
        approved_hosts.insert(HostApprovalKey {
            host: "source.example.com".to_string(),
            protocol: "https",
            port: 443,
        });
    }
    source.session_policy_commit_semaphore.close();

    let target = NetworkApprovalService::default();
    {
        let mut approved_hosts = target.session_approved_hosts.lock().await;
        approved_hosts.insert(HostApprovalKey {
            host: "target.example.com".to_string(),
            protocol: "https",
            port: 8443,
        });
    }

    assert!(!source.sync_session_approved_hosts_to(&target).await);

    let copied = target.session_approved_hosts.lock().await;
    assert!(
        copied.is_empty(),
        "stale target approvals must not survive a failed sync"
    );
}

#[tokio::test]
async fn pending_waiters_receive_owner_decision() {
    let pending = Arc::new(PendingHostApproval::new());

    let waiter = {
        let pending = Arc::clone(&pending);
        tokio::spawn(async move { pending.wait_for_decision().await })
    };

    pending.set_decision(PendingApprovalDecision::AllowOnce);

    let decision = waiter.await.expect("waiter should complete");
    assert_eq!(decision, PendingApprovalDecision::AllowOnce);
}

#[test]
fn allow_once_and_allow_for_session_both_allow_network() {
    assert_eq!(
        PendingApprovalDecision::AllowOnce.to_network_decision(),
        NetworkDecision::Allow
    );
    assert_eq!(
        PendingApprovalDecision::AllowForSession.to_network_decision(),
        NetworkDecision::Allow
    );
}

#[test]
fn only_never_policy_disables_network_approval_flow() {
    assert!(!allows_network_approval_flow(AskForApproval::Never));
    assert!(allows_network_approval_flow(AskForApproval::OnRequest));
    assert!(allows_network_approval_flow(AskForApproval::OnFailure));
    assert!(allows_network_approval_flow(AskForApproval::UnlessTrusted));
}

#[test]
fn network_approval_flow_is_limited_to_restricted_sandbox_modes() {
    assert!(permission_profile_allows_network_approval_flow(
        &PermissionProfile::from_legacy_sandbox_policy(&SandboxPolicy::new_read_only_policy())
    ));
    assert!(permission_profile_allows_network_approval_flow(
        &PermissionProfile::from_legacy_sandbox_policy(&SandboxPolicy::new_workspace_write_policy())
    ));
    assert!(!permission_profile_allows_network_approval_flow(
        &PermissionProfile::Disabled
    ));
    assert!(!permission_profile_allows_network_approval_flow(
        &PermissionProfile::External {
            network: NetworkSandboxPolicy::Restricted,
        }
    ));
}

fn denied_blocked_request(host: &str) -> BlockedRequest {
    BlockedRequest::new(BlockedRequestArgs {
        host: host.to_string(),
        reason: "not_allowed".to_string(),
        client: None,
        method: None,
        mode: None,
        protocol: "http".to_string(),
        decision: Some("deny".to_string()),
        source: Some("decider".to_string()),
        port: Some(80),
    })
}

async fn register_call_with_default_shell_trigger(
    service: &NetworkApprovalService,
    registration_id: &str,
) -> CancellationToken {
    let cancellation_token = CancellationToken::new();
    service
        .register_call(
            registration_id.to_string(),
            "turn-1".to_string(),
            GuardianNetworkAccessTrigger {
                call_id: "call-1".to_string(),
                tool_name: "shell_command".to_string(),
                command: vec!["curl".to_string(), "https://example.com".to_string()],
                cwd: test_path_buf("/tmp").abs(),
                sandbox_permissions: SandboxPermissions::UseDefault,
                additional_permissions: None,
                justification: None,
                tty: None,
            },
            "curl https://example.com".to_string(),
            cancellation_token.clone(),
        )
        .await;
    cancellation_token
}

#[tokio::test]
async fn active_call_preserves_triggering_command_context() {
    let service = NetworkApprovalService::default();
    let expected = GuardianNetworkAccessTrigger {
        call_id: "call-1".to_string(),
        tool_name: "shell_command".to_string(),
        command: vec!["curl".to_string(), "https://example.com".to_string()],
        cwd: test_path_buf("/repo").abs(),
        sandbox_permissions: SandboxPermissions::UseDefault,
        additional_permissions: None,
        justification: Some("fetch release metadata".to_string()),
        tty: None,
    };

    service
        .register_call(
            "registration-1".to_string(),
            "turn-1".to_string(),
            expected.clone(),
            "curl https://example.com".to_string(),
            CancellationToken::new(),
        )
        .await;

    let call = service
        .resolve_single_active_call()
        .await
        .expect("single active call should resolve");

    assert_eq!(&call.trigger, &expected);
    assert_eq!(call.command, "curl https://example.com");
}

#[tokio::test]
async fn record_blocked_request_sets_policy_outcome_for_owner_call() {
    let service = NetworkApprovalService::default();
    let cancellation_token =
        register_call_with_default_shell_trigger(&service, "registration-1").await;

    service
        .record_blocked_request(denied_blocked_request("example.com"))
        .await;

    assert!(cancellation_token.is_cancelled());
    assert_eq!(
            service.take_call_outcome("registration-1").await,
            Some(NetworkApprovalOutcome::DeniedByPolicy(
                "Network access to \"example.com\" was blocked: domain is not on the allowlist for the current sandbox mode.".to_string()
            ))
        );
}

#[tokio::test]
async fn unavailable_policy_commit_serializer_denies_and_cancels_owner_call() {
    let service = NetworkApprovalService::default();
    let cancellation_token =
        register_call_with_default_shell_trigger(&service, "registration-1").await;
    let owner_call = service.resolve_single_active_call().await;

    service.session_policy_commit_semaphore.close();
    let permit = service
        .acquire_session_policy_commit_permit_or_deny_owner(owner_call.as_ref())
        .await;

    assert!(permit.is_none());
    assert!(cancellation_token.is_cancelled());
    let err = service
        .finish_call("registration-1")
        .await
        .expect_err("serializer failure must finish as a policy denial");
    assert!(
        matches!(err, ToolError::Rejected(message) if message == NETWORK_APPROVAL_POLICY_COMMIT_SERIALIZATION_UNAVAILABLE_MESSAGE)
    );
    assert_eq!(service.take_call_outcome("registration-1").await, None);
}

#[tokio::test]
async fn blocked_request_policy_does_not_override_user_denial_outcome() {
    let service = NetworkApprovalService::default();
    register_call_with_default_shell_trigger(&service, "registration-1").await;

    service
        .record_call_outcome("registration-1", NetworkApprovalOutcome::DeniedByUser)
        .await;
    service
        .record_blocked_request(denied_blocked_request("example.com"))
        .await;

    assert_eq!(
        service.take_call_outcome("registration-1").await,
        Some(NetworkApprovalOutcome::DeniedByUser)
    );
}

#[tokio::test]
async fn finish_call_returns_denial_and_unregisters_active_call() {
    let service = NetworkApprovalService::default();
    register_call_with_default_shell_trigger(&service, "registration-1").await;

    service
        .record_call_outcome(
            "registration-1",
            NetworkApprovalOutcome::DeniedByPolicy("network denied".to_string()),
        )
        .await;

    let err = service
        .finish_call("registration-1")
        .await
        .expect_err("denial should be returned");

    assert!(matches!(err, ToolError::Rejected(message) if message == "network denied"));
    assert!(service.resolve_single_active_call().await.is_none());
    assert_eq!(service.take_call_outcome("registration-1").await, None);
}

#[tokio::test]
async fn deferred_finish_reuses_denial_result_after_first_consumer() {
    let service = NetworkApprovalService::default();
    let cancellation_token =
        register_call_with_default_shell_trigger(&service, "registration-1").await;
    let deferred = DeferredNetworkApproval {
        registration_id: "registration-1".to_string(),
        cancellation_token,
        finish_outcome: Arc::new(OnceCell::new()),
    };
    service
        .record_call_outcome(
            "registration-1",
            NetworkApprovalOutcome::DeniedByPolicy("network denied".to_string()),
        )
        .await;

    let first = deferred
        .finish(&service)
        .await
        .expect_err("first consumer should see denial");
    let second = deferred
        .finish(&service)
        .await
        .expect_err("second consumer should reuse denial");

    assert!(matches!(first, ToolError::Rejected(message) if message == "network denied"));
    assert!(matches!(second, ToolError::Rejected(message) if message == "network denied"));
}

#[tokio::test]
async fn deferred_finish_fails_closed_when_owner_disappears_without_an_outcome() {
    let service = NetworkApprovalService::default();
    let cancellation_token = CancellationToken::new();
    cancellation_token.cancel();
    let deferred = DeferredNetworkApproval {
        registration_id: "abandoned-registration".to_string(),
        cancellation_token,
        finish_outcome: Arc::new(OnceCell::new()),
    };

    let err = deferred
        .finish(&service)
        .await
        .expect_err("an abandoned approval must fail closed");

    assert!(
        matches!(err, ToolError::Rejected(message) if message == ABANDONED_NETWORK_APPROVAL_MESSAGE)
    );
}

#[tokio::test]
async fn record_call_outcome_ignores_inactive_call() {
    let service = NetworkApprovalService::default();
    let cancellation_token =
        register_call_with_default_shell_trigger(&service, "registration-1").await;
    service.unregister_call("registration-1").await;

    service
        .record_call_outcome(
            "registration-1",
            NetworkApprovalOutcome::DeniedByPolicy("network denied".to_string()),
        )
        .await;

    assert!(!cancellation_token.is_cancelled());
    assert_eq!(service.take_call_outcome("registration-1").await, None);
}

#[tokio::test]
async fn record_blocked_request_ignores_ambiguous_unattributed_blocked_requests() {
    let service = NetworkApprovalService::default();
    register_call_with_default_shell_trigger(&service, "registration-1").await;
    register_call_with_default_shell_trigger(&service, "registration-2").await;

    service
        .record_blocked_request(denied_blocked_request("example.com"))
        .await;

    assert_eq!(service.take_call_outcome("registration-1").await, None);
    assert_eq!(service.take_call_outcome("registration-2").await, None);
}
