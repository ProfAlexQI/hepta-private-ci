#[test]
fn hung_agent_is_stopped_and_killed_without_blocking_peer() -> Result<(), SupervisorError> {
    let fleet = TestFleet::new()?;
    let control = FakeControl::default();
    let now = Instant::now();
    let (mut supervisor, recovered) =
        Supervisor::recover(fleet.registry.clone(), control.driver(), config(), now)?;
    assert_eq!(recovered, TickReport::default());
    supervisor.start(&fleet.first, fleet.command()?, now)?;
    supervisor.start(&fleet.second, fleet.command()?, now)?;
    control.set_healthy(&fleet.second);
    control.push_logs(&fleet.first, 10);
    control.push_logs(&fleet.second, 10);

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
    assert_eq!(
        supervisor.tick(now + Duration::from_millis(11)),
        TickReport::default()
    );
    assert_eq!(control.counts(&fleet.first), (0, 1, 0));
    assert_eq!(
        supervisor.tick(now + Duration::from_millis(22)),
        TickReport::default()
    );
    assert_eq!(control.counts(&fleet.first), (0, 1, 1));
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
    control.set_exit(&fleet.first);
    assert_eq!(
        supervisor.tick(now + Duration::from_millis(23)),
        TickReport::default()
    );

    let first = supervisor.snapshot(&fleet.first).expect("first slot");
    let second = supervisor.snapshot(&fleet.second).expect("second slot");
    assert!(!first.active);
    assert!(second.active);
    assert_eq!((first.logs.len(), second.logs.len()), (3, 3));
    assert!(first.events.len() <= 8);
    assert!(second.events.len() <= 8);
    assert!(first.logs.iter().all(|log| log.bytes.len() <= 8));
    assert!(second.logs.iter().all(|log| log.bytes.len() <= 8));
    Ok(())
}

#[test]
fn restart_drains_one_agent_and_spawns_a_new_generation() -> Result<(), SupervisorError> {
    let fleet = TestFleet::new()?;
    let control = FakeControl::default();
    let now = Instant::now();
    let (mut supervisor, _) =
        Supervisor::recover(fleet.registry.clone(), control.driver(), config(), now)?;
    supervisor.start(&fleet.first, fleet.command()?, now)?;
    control.set_healthy(&fleet.first);
    supervisor.tick(now);
    supervisor.restart(&fleet.first, now)?;
    control.set_drained(&fleet.first);
    supervisor.tick(now);
    assert_eq!(control.counts(&fleet.first), (1, 1, 0));
    control.set_exit(&fleet.first);
    supervisor.tick(now);

    assert_eq!(control.spawn_count(&fleet.first), 2);
    assert_eq!(
        fleet
            .registry
            .load()?
            .agent(&fleet.first)
            .unwrap()
            .lifecycle
            .lifecycle,
        AgentLifecycle::Starting
    );
    Ok(())
}

#[test]
fn recovery_adopts_one_orphan_and_rejects_another() -> Result<(), SupervisorError> {
    let fleet = TestFleet::new()?;
    let control = FakeControl::default();
    let now = Instant::now();
    let (mut first_supervisor, _) =
        Supervisor::recover(fleet.registry.clone(), control.driver(), config(), now)?;
    first_supervisor.start(&fleet.first, fleet.command()?, now)?;
    first_supervisor.start(&fleet.second, fleet.command()?, now)?;
    control.set_healthy(&fleet.first);
    control.set_healthy(&fleet.second);
    first_supervisor.tick(now);
    drop(first_supervisor);
    control.reject_adoption(fleet.second.clone());

    let (supervisor, report) =
        Supervisor::recover(fleet.registry.clone(), control.driver(), config(), now)?;
    assert_eq!(report, TickReport::default());
    assert!(supervisor.snapshot(&fleet.first).unwrap().active);
    assert!(!supervisor.snapshot(&fleet.second).unwrap().active);
    assert!(
        supervisor
            .snapshot(&fleet.first)
            .unwrap()
            .events
            .iter()
            .any(|event| event.kind == SupervisorEventKind::OrphanAdopted)
    );
    assert!(
        supervisor
            .snapshot(&fleet.second)
            .unwrap()
            .events
            .iter()
            .any(|event| event.kind == SupervisorEventKind::OrphanRejected)
    );
    assert_eq!(
        fleet
            .registry
            .load()?
            .agent(&fleet.second)
            .unwrap()
            .lifecycle
            .lifecycle,
        AgentLifecycle::Failed
    );
    Ok(())
}

#[test]
fn recovery_closes_running_release_state_crash_window() -> Result<(), SupervisorError> {
    let fleet = TestFleet::new()?;
    let release_id = ReleaseId::parse("release-after-crash")?;
    let source = fleet._temp.path().join("release-after-crash");
    std::fs::write(&source, b"#!/bin/sh\nexit 0\n")?;
    fleet
        .registry
        .install_release(release_id.clone(), &source, Vec::new())?;
    fleet.registry.allow_release(&fleet.first, &release_id)?;
    let product_release =
        AgentRelease::try_from(fleet.registry.resolve_release(&fleet.first, &release_id)?)?;

    let control = FakeControl::default();
    let now = Instant::now();
    let (mut first_supervisor, _) =
        Supervisor::recover(fleet.registry.clone(), control.driver(), config(), now)?;
    first_supervisor.start_release(&fleet.first, product_release, now)?;
    control.set_healthy(&fleet.first);

    // Model a daemon crash after the Running lifecycle became durable but before
    // its corresponding current-release revision was appended.
    let starting = fleet
        .registry
        .load()?
        .agent(&fleet.first)
        .expect("registered agent")
        .lifecycle
        .clone();
    fleet.registry.compare_and_transition(
        &fleet.first,
        starting.generation,
        AgentLifecycle::Running,
    )?;
    drop(first_supervisor);

    let (recovered, report) =
        Supervisor::recover(fleet.registry.clone(), control.driver(), config(), now)?;
    assert_eq!(report, TickReport::default());
    let snapshot = recovered.snapshot(&fleet.first).expect("recovered slot");
    assert!(snapshot.active);
    assert_eq!(
        snapshot.active_release.as_deref(),
        Some(release_id.as_str())
    );
    let durable = fleet.registry.load()?;
    let release_state = &durable
        .agent(&fleet.first)
        .expect("registered agent")
        .release_state;
    assert_eq!(release_state.current.as_ref(), Some(&release_id));
    assert_eq!(release_state.previous, None);
    Ok(())
}

#[test]
fn stale_runtime_is_fenced_without_touching_peer() -> Result<(), SupervisorError> {
    let fleet = TestFleet::new()?;
    let control = FakeControl::default();
    let now = Instant::now();
    let (mut supervisor, _) =
        Supervisor::recover(fleet.registry.clone(), control.driver(), config(), now)?;
    supervisor.start(&fleet.first, fleet.command()?, now)?;
    supervisor.start(&fleet.second, fleet.command()?, now)?;
    control.set_healthy(&fleet.first);
    control.set_healthy(&fleet.second);
    supervisor.tick(now);
    let first = fleet
        .registry
        .load()?
        .agent(&fleet.first)
        .unwrap()
        .lifecycle
        .clone();
    fleet.registry.compare_and_transition(
        &fleet.first,
        first.generation,
        AgentLifecycle::Draining,
    )?;

    supervisor.tick(now);
    assert_eq!(control.counts(&fleet.first).2, 1);
    assert_eq!(control.counts(&fleet.second).2, 0);
    assert!(
        supervisor
            .snapshot(&fleet.first)
            .unwrap()
            .events
            .iter()
            .any(|event| matches!(event.kind, SupervisorEventKind::GenerationFenced { .. }))
    );
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
    Ok(())
}

