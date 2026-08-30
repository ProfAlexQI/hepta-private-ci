#[test]
fn successful_upgrade_and_explicit_rollback_change_only_target_agent() -> Result<(), SupervisorError>
{
    let fleet = TestFleet::new()?;
    let control = FakeControl::default();
    let now = Instant::now();
    let (mut supervisor, _) =
        Supervisor::recover(fleet.registry.clone(), control.driver(), config(), now)?;
    supervisor.start_release(
        &fleet.first,
        fleet.release("release-v1")?,
        now,
    )?;
    supervisor.start_release(
        &fleet.second,
        fleet.release("peer-release")?,
        now,
    )?;
    control.set_healthy(&fleet.first);
    control.set_healthy(&fleet.second);
    assert_eq!(supervisor.tick(now), TickReport::default());
    let peer_before = supervisor.snapshot(&fleet.second).expect("peer snapshot");

    supervisor.upgrade(
        &fleet.first,
        fleet.release("release-v2")?,
        now,
    )?;
    assert!(matches!(
        supervisor.restart(&fleet.first, now),
        Err(SupervisorError::ReleaseChangePending(agent_id)) if agent_id == fleet.first
    ));
    finish_release_drain(&mut supervisor, &control, &fleet.first, now);
    control.set_healthy(&fleet.first);
    assert_eq!(supervisor.tick(now), TickReport::default());
    let upgraded = supervisor
        .snapshot(&fleet.first)
        .expect("upgraded snapshot");
    assert_eq!(upgraded.active_release.as_deref(), Some("release-v2"));
    assert_eq!(upgraded.previous_release.as_deref(), Some("release-v1"));
    assert!(!upgraded.release_change_pending);
    assert!(upgraded.events.iter().any(|event| {
        matches!(
            &event.kind,
            SupervisorEventKind::UpgradeCommitted { previous, target }
                if previous == "release-v1" && target == "release-v2"
        )
    }));

    supervisor.rollback(&fleet.first, now)?;
    finish_release_drain(&mut supervisor, &control, &fleet.first, now);
    control.set_healthy(&fleet.first);
    assert_eq!(supervisor.tick(now), TickReport::default());
    let rolled_back = supervisor
        .snapshot(&fleet.first)
        .expect("rollback snapshot");
    assert_eq!(rolled_back.active_release.as_deref(), Some("release-v1"));
    assert_eq!(rolled_back.previous_release.as_deref(), Some("release-v2"));
    assert!(!rolled_back.release_change_pending);
    assert!(rolled_back.events.iter().any(|event| {
        matches!(
            &event.kind,
            SupervisorEventKind::ExplicitRollbackCommitted { previous, target }
                if previous == "release-v2" && target == "release-v1"
        )
    }));

    let peer_after = supervisor.snapshot(&fleet.second).expect("peer snapshot");
    assert_eq!(peer_after.process_system_id, peer_before.process_system_id);
    assert_eq!(peer_after.spawn_generation, peer_before.spawn_generation);
    assert_eq!(control.counts(&fleet.second), (0, 0, 0));
    Ok(())
}

#[test]
fn failed_spawn_and_failed_health_each_auto_rollback_once() -> Result<(), SupervisorError> {
    let fleet = TestFleet::new()?;
    let control = FakeControl::default();
    let now = Instant::now();
    let (mut supervisor, _) =
        Supervisor::recover(fleet.registry.clone(), control.driver(), config(), now)?;
    supervisor.start_release(
        &fleet.first,
        fleet.release("release-v1")?,
        now,
    )?;
    control.set_healthy(&fleet.first);
    supervisor.tick(now);

    control.reject_spawn_program(fleet.program("release-spawn-fails")?);
    supervisor.upgrade(
        &fleet.first,
        fleet.release("release-spawn-fails")?,
        now,
    )?;
    finish_release_drain(&mut supervisor, &control, &fleet.first, now);
    control.set_healthy(&fleet.first);
    assert_eq!(supervisor.tick(now), TickReport::default());
    let recovered = supervisor
        .snapshot(&fleet.first)
        .expect("recovered snapshot");
    assert_eq!(recovered.active_release.as_deref(), Some("release-v1"));
    assert!(!recovered.release_change_pending);

    supervisor.upgrade(
        &fleet.first,
        fleet.release("release-health-fails")?,
        now,
    )?;
    finish_release_drain(&mut supervisor, &control, &fleet.first, now);
    assert_eq!(
        supervisor.tick(now + Duration::from_millis(11)),
        TickReport::default()
    );
    control.set_exit(&fleet.first);
    assert_eq!(
        supervisor.tick(now + Duration::from_millis(12)),
        TickReport::default()
    );
    control.set_healthy(&fleet.first);
    assert_eq!(
        supervisor.tick(now + Duration::from_millis(12)),
        TickReport::default()
    );
    let final_snapshot = supervisor.snapshot(&fleet.first).expect("final snapshot");
    assert_eq!(final_snapshot.active_release.as_deref(), Some("release-v1"));
    assert!(!final_snapshot.release_change_pending);
    assert!(final_snapshot.events.iter().any(|event| {
        matches!(
            &event.kind,
            SupervisorEventKind::AutomaticRollbackCommitted { failed, restored }
                if failed == "release-health-fails" && restored == "release-v1"
        )
    }));
    let spawn_count = control.spawn_count(&fleet.first);
    assert_eq!(
        supervisor.tick(now + Duration::from_secs(10)),
        TickReport::default()
    );
    assert_eq!(control.spawn_count(&fleet.first), spawn_count);
    Ok(())
}

#[test]
fn paired_companions_stop_before_agent_restart_and_fail_independently()
-> Result<(), SupervisorError> {
    let fleet = TestFleet::new()?;
    let release_id = ReleaseId::parse("paired-v1")?;
    let agentd_program = fleet.program(&format!("{release_id}-agentd"))?;
    let matrixd_program = fleet.program(&format!("{release_id}-matrixd"))?;
    fleet.registry.install_release_bundle(
        release_id.clone(),
        &agentd_program,
        Vec::new(),
        Some(&matrixd_program),
        Vec::new(),
    )?;
    for agent_id in [&fleet.first, &fleet.second] {
        fleet.registry.allow_release(agent_id, &release_id)?;
        write_matrix_binding(&fleet.registry, agent_id, 1)?;
    }
    let paired =
        AgentRelease::try_from(fleet.registry.resolve_release(&fleet.first, &release_id)?)?;
    let peer_paired =
        AgentRelease::try_from(fleet.registry.resolve_release(&fleet.second, &release_id)?)?;

    let control = FakeControl::default();
    let now = Instant::now();
    let (mut supervisor, report) =
        Supervisor::recover(fleet.registry.clone(), control.driver(), config(), now)?;
    assert_eq!(report, TickReport::default());
    supervisor.start_release(&fleet.first, paired, now)?;
    supervisor.start_release(&fleet.second, peer_paired, now)?;
    control.set_healthy(&fleet.first);
    control.set_healthy(&fleet.second);
    assert_eq!(supervisor.tick(now), TickReport::default());
    assert_eq!(control.matrix_spawn_count(&fleet.first), 1);
    assert_eq!(control.matrix_spawn_count(&fleet.second), 1);
    control.set_matrix_healthy(&fleet.first);
    control.set_matrix_healthy(&fleet.second);
    assert_eq!(supervisor.tick(now), TickReport::default());
    let first_snapshot = supervisor.snapshot(&fleet.first).unwrap();
    assert!(first_snapshot.matrix.healthy);
    assert_eq!(
        first_snapshot.matrix.attached_agent_generation,
        first_snapshot.spawn_generation
    );
    assert_ne!(
        first_snapshot.matrix.attached_agent_generation,
        first_snapshot.runtime_generation
    );
    let second_snapshot = supervisor.snapshot(&fleet.second).unwrap();
    assert!(second_snapshot.matrix.healthy);
    assert_eq!(
        second_snapshot.matrix.attached_agent_generation,
        second_snapshot.spawn_generation
    );
    assert_ne!(
        second_snapshot.matrix.attached_agent_generation,
        second_snapshot.runtime_generation
    );

    supervisor.restart(&fleet.first, now)?;
    assert_eq!(control.matrix_counts(&fleet.first), (0, 1, 0));
    assert_eq!(control.counts(&fleet.first), (0, 0, 0));
    assert!(supervisor.snapshot(&fleet.second).unwrap().matrix.healthy);

    control.set_matrix_exit(&fleet.first);
    assert_eq!(supervisor.tick(now), TickReport::default());
    assert_eq!(control.counts(&fleet.first), (1, 0, 0));
    control.set_drained(&fleet.first);
    assert_eq!(supervisor.tick(now), TickReport::default());
    assert_eq!(control.counts(&fleet.first), (1, 1, 0));
    control.set_exit(&fleet.first);
    assert_eq!(supervisor.tick(now), TickReport::default());
    control.set_healthy(&fleet.first);
    assert_eq!(supervisor.tick(now), TickReport::default());
    assert_eq!(control.matrix_spawn_count(&fleet.first), 2);
    control.set_matrix_healthy(&fleet.first);
    assert_eq!(supervisor.tick(now), TickReport::default());

    control.set_matrix_exit(&fleet.second);
    assert_eq!(supervisor.tick(now), TickReport::default());
    assert_eq!(
        fleet
            .registry
            .load()?
            .agent(&fleet.second)
            .unwrap()
            .lifecycle
            .lifecycle,
        AgentLifecycle::Running
    );
    assert!(supervisor.snapshot(&fleet.second).unwrap().matrix.degraded);
    assert!(supervisor.snapshot(&fleet.first).unwrap().matrix.healthy);
    assert_eq!(control.counts(&fleet.second), (0, 0, 0));
    assert_eq!(
        supervisor.tick(now + Duration::from_millis(300)),
        TickReport::default()
    );
    assert_eq!(control.matrix_spawn_count(&fleet.second), 2);
    Ok(())
}

