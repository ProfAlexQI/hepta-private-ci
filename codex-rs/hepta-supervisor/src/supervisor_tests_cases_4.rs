#[test]
fn live_but_unhealthy_matrix_is_bounded_and_restarted_without_peer_churn()
-> Result<(), SupervisorError> {
    let fleet = TestFleet::new()?;
    let release_id = ReleaseId::parse("paired-unhealthy-v1")?;
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

    control.set_matrix_unhealthy(&fleet.first);
    assert_eq!(supervisor.tick(now), TickReport::default());
    assert!(supervisor.snapshot(&fleet.first).unwrap().matrix.degraded);
    assert!(supervisor.snapshot(&fleet.second).unwrap().matrix.healthy);
    assert_eq!(control.counts(&fleet.first), (0, 0, 0));
    assert_eq!(control.counts(&fleet.second), (0, 0, 0));

    assert_eq!(
        supervisor.tick(now + Duration::from_millis(11)),
        TickReport::default()
    );
    assert_eq!(control.matrix_counts(&fleet.first), (0, 1, 0));
    assert_eq!(control.matrix_counts(&fleet.second), (0, 0, 0));
    assert_eq!(
        supervisor.tick(now + Duration::from_millis(22)),
        TickReport::default()
    );
    assert_eq!(control.matrix_counts(&fleet.first), (0, 1, 1));
    assert_eq!(control.matrix_counts(&fleet.second), (0, 0, 0));
    control.set_matrix_exit(&fleet.first);
    assert_eq!(
        supervisor.tick(now + Duration::from_millis(23)),
        TickReport::default()
    );
    assert_eq!(control.spawn_count(&fleet.first), 1);
    assert_eq!(control.spawn_count(&fleet.second), 1);
    assert_eq!(control.matrix_spawn_count(&fleet.second), 1);

    assert_eq!(
        supervisor.tick(now + Duration::from_millis(300)),
        TickReport::default()
    );
    assert_eq!(control.matrix_spawn_count(&fleet.first), 2);
    assert_eq!(control.matrix_spawn_count(&fleet.second), 1);
    assert!(supervisor.snapshot(&fleet.second).unwrap().matrix.healthy);
    Ok(())
}

#[test]
fn recovery_does_not_infer_signed_commit_from_matching_target_only() -> Result<(), SupervisorError>
{
    let fleet = TestFleet::new()?;
    let source = ReleaseId::parse("signed-source")?;
    let target = ReleaseId::parse("signed-target")?;
    let source_program = fleet.program("signed-recovery")?;
    for release_id in [&source, &target] {
        fleet
            .registry
            .install_release(release_id.clone(), &source_program, Vec::new())?;
        fleet.registry.allow_release(&fleet.first, release_id)?;
    }

    // Leave the durable release state looking as though the target is active,
    // but provide no durable proof for the signed operation's source,
    // control-revision, lifecycle-generation, or daemon authority epoch.
    fleet.registry.compare_and_set_release_state(
        &fleet.first,
        0,
        Some(target.clone()),
        Some(source),
    )?;
    let starting =
        fleet
            .registry
            .compare_and_transition(&fleet.first, 0, AgentLifecycle::Starting)?;
    let running = fleet.registry.compare_and_transition(
        &fleet.first,
        starting.generation,
        AgentLifecycle::Running,
    )?;
    let record = fleet
        .registry
        .load()?
        .agent(&fleet.first)
        .cloned()
        .expect("registered agent");
    assert_eq!(record.lifecycle.generation, running.generation);

    let intent = crate::signed_intent::SignedSupervisorIntent::new(
        Sha256Digest::for_bytes(b"unrelated-grant"),
        fleet.first.to_string(),
        crate::H7H89ProductionTransition::Upgrade,
        "unrelated-source",
        target.to_string(),
        7,
        1,
        999,
        crate::signed_intent::SignedIntentStatus::Queued,
    )
    .expect("synthetic unresolved intent");
    crate::signed_intent::write_intent(record.layout.run_root(), &intent)
        .expect("persist unresolved intent");

    let error = match Supervisor::recover(
        fleet.registry.clone(),
        FakeControl::default().driver(),
        config(),
        Instant::now(),
    ) {
        Ok(_) => panic!("matching target must not infer a signed commit"),
        Err(error) => error,
    };
    assert!(matches!(
        error,
        SupervisorError::SignedIntentRecoveryRequired(agent_id) if agent_id == fleet.first
    ));
    assert_eq!(
        crate::signed_intent::read_intent(record.layout.run_root())
            .expect("read unresolved intent")
            .expect("intent remains durable")
            .status,
        crate::signed_intent::SignedIntentStatus::Queued
    );
    Ok(())
}

fn write_matrix_binding(
    registry: &FleetRegistry,
    agent_id: &AgentId,
    revision: u64,
) -> Result<(), SupervisorError> {
    let record = registry
        .load()?
        .agent(agent_id)
        .cloned()
        .ok_or_else(|| SupervisorError::UnknownAgent(agent_id.clone()))?;
    let binding = serde_json::json!({
        "schema_version": 1,
        "agent_id": agent_id,
        "revision": revision,
        "homeserver": "https://matrix.example.test",
        "expected_mxid": "@hepta:example.test",
        "expected_device_id": "HEPTA1",
        "allowed_rooms": ["!room:example.test"],
        "allowed_senders": ["@operator:example.test"],
        "require_explicit_mention": true
    });
    std::fs::write(
        record.layout.matrix_public_binding(),
        serde_json::to_vec(&binding)
            .map_err(|error| SupervisorError::Invalid(error.to_string()))?,
    )?;
    Ok(())
}

fn finish_release_drain(
    supervisor: &mut Supervisor<FakeDriver>,
    control: &FakeControl,
    agent_id: &AgentId,
    now: Instant,
) {
    control.set_drained(agent_id);
    assert_eq!(supervisor.tick(now), TickReport::default());
    control.set_exit(agent_id);
    assert_eq!(supervisor.tick(now), TickReport::default());
}
