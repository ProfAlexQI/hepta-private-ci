fn ready_paired_supervisor(
    release_name: &str,
) -> Result<(TestFleet, FakeControl, Supervisor<FakeDriver>, Instant), SupervisorError> {
    let fleet = TestFleet::new()?;
    let release_id = ReleaseId::parse(release_name)?;
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
    let first_release =
        AgentRelease::try_from(fleet.registry.resolve_release(&fleet.first, &release_id)?)?;
    let second_release =
        AgentRelease::try_from(fleet.registry.resolve_release(&fleet.second, &release_id)?)?;
    let control = FakeControl::default();
    let now = Instant::now();
    let (mut supervisor, report) =
        Supervisor::recover(fleet.registry.clone(), control.driver(), config(), now)?;
    assert_eq!(report, TickReport::default());
    supervisor.start_release(&fleet.first, first_release, now)?;
    supervisor.start_release(&fleet.second, second_release, now)?;
    control.set_healthy(&fleet.first);
    control.set_healthy(&fleet.second);
    assert_eq!(supervisor.tick(now), TickReport::default());
    control.set_matrix_healthy(&fleet.first);
    control.set_matrix_healthy(&fleet.second);
    assert_eq!(supervisor.tick(now), TickReport::default());
    Ok((fleet, control, supervisor, now))
}

#[test]
fn stop_supersedes_inflight_paired_restart_after_matrix_exits() -> Result<(), SupervisorError> {
    let (fleet, control, mut supervisor, now) =
        ready_paired_supervisor("paired-stop-supersedes-restart")?;
    let peer_before = supervisor.snapshot(&fleet.second).expect("peer snapshot");

    supervisor.restart(&fleet.first, now)?;
    assert!(supervisor.snapshot(&fleet.first).unwrap().restart_pending);
    assert_eq!(control.matrix_counts(&fleet.first), (0, 1, 0));
    assert_eq!(control.counts(&fleet.first), (0, 0, 0));

    supervisor.stop(&fleet.first, now)?;
    assert!(!supervisor.snapshot(&fleet.first).unwrap().restart_pending);
    assert_eq!(control.matrix_counts(&fleet.first), (0, 1, 0));
    assert_eq!(control.counts(&fleet.first), (0, 0, 0));

    control.set_matrix_exit(&fleet.first);
    assert_eq!(supervisor.tick(now), TickReport::default());
    assert_eq!(control.counts(&fleet.first), (0, 1, 0));
    control.set_exit(&fleet.first);
    assert_eq!(supervisor.tick(now), TickReport::default());

    let stopped = supervisor.snapshot(&fleet.first).expect("stopped snapshot");
    assert!(!stopped.active);
    assert!(!stopped.matrix.active);
    assert!(!stopped.restart_pending);
    assert_eq!(control.spawn_count(&fleet.first), 1);
    assert_eq!(control.matrix_spawn_count(&fleet.first), 1);
    let peer_after = supervisor.snapshot(&fleet.second).expect("peer snapshot");
    assert_eq!(peer_after.process_system_id, peer_before.process_system_id);
    assert_eq!(peer_after.spawn_generation, peer_before.spawn_generation);
    assert_eq!(
        peer_after.matrix.process_system_id,
        peer_before.matrix.process_system_id
    );
    assert_eq!(
        peer_after.matrix.attached_agent_generation,
        peer_before.matrix.attached_agent_generation
    );
    Ok(())
}

#[test]
fn kill_supersedes_inflight_paired_restart_without_replacement() -> Result<(), SupervisorError> {
    let (fleet, control, mut supervisor, now) =
        ready_paired_supervisor("paired-kill-supersedes-restart")?;
    let peer_before = supervisor.snapshot(&fleet.second).expect("peer snapshot");

    supervisor.restart(&fleet.first, now)?;
    assert_eq!(control.matrix_counts(&fleet.first), (0, 1, 0));
    assert_eq!(control.counts(&fleet.first), (0, 0, 0));
    supervisor.kill(&fleet.first)?;

    let killing = supervisor.snapshot(&fleet.first).expect("killing snapshot");
    assert!(!killing.restart_pending);
    assert_eq!(control.matrix_counts(&fleet.first), (0, 1, 1));
    assert_eq!(control.counts(&fleet.first), (0, 0, 1));
    let matrix_kill = killing
        .events
        .iter()
        .position(|event| event.kind == SupervisorEventKind::MatrixKillRequested)
        .expect("Matrix kill event");
    let agent_kill = killing
        .events
        .iter()
        .position(|event| event.kind == SupervisorEventKind::KillRequested)
        .expect("agent kill event");
    assert!(
        matrix_kill < agent_kill,
        "Matrix must be killed before agentd"
    );

    control.set_exit(&fleet.first);
    control.set_matrix_exit(&fleet.first);
    assert_eq!(supervisor.tick(now), TickReport::default());
    let stopped = supervisor.snapshot(&fleet.first).expect("stopped snapshot");
    assert!(!stopped.active);
    assert!(!stopped.matrix.active);
    assert!(!stopped.restart_pending);
    assert_eq!(control.spawn_count(&fleet.first), 1);
    assert_eq!(control.matrix_spawn_count(&fleet.first), 1);
    let peer_after = supervisor.snapshot(&fleet.second).expect("peer snapshot");
    assert_eq!(peer_after.process_system_id, peer_before.process_system_id);
    assert_eq!(peer_after.spawn_generation, peer_before.spawn_generation);
    assert_eq!(
        peer_after.matrix.process_system_id,
        peer_before.matrix.process_system_id
    );
    assert_eq!(
        peer_after.matrix.attached_agent_generation,
        peer_before.matrix.attached_agent_generation
    );
    Ok(())
}

#[test]
fn stale_deferred_drain_is_generation_fenced_from_replacement_starting()
-> Result<(), SupervisorError> {
    let (fleet, control, mut supervisor, now) =
        ready_paired_supervisor("paired-stale-drain-fence")?;
    let original = supervisor
        .snapshot(&fleet.first)
        .expect("original snapshot");
    let original_spawn_generation = original.spawn_generation.expect("spawn generation");

    supervisor.restart(&fleet.first, now)?;
    control.set_exit(&fleet.first);
    assert_eq!(supervisor.tick(now), TickReport::default());
    let replacement = supervisor
        .snapshot(&fleet.first)
        .expect("replacement snapshot");
    assert!(replacement.active);
    assert!(!replacement.healthy);
    assert!(replacement.spawn_generation.unwrap() > original_spawn_generation);
    assert!(!replacement.restart_pending);
    assert_eq!(control.spawn_count(&fleet.first), 2);

    control.set_matrix_exit(&fleet.first);
    assert_eq!(supervisor.tick(now), TickReport::default());
    let still_starting = supervisor
        .snapshot(&fleet.first)
        .expect("starting replacement snapshot");
    assert!(still_starting.active);
    assert!(!still_starting.healthy);
    assert!(!still_starting.matrix.active);
    assert_eq!(control.counts(&fleet.first), (0, 0, 0));
    Ok(())
}

