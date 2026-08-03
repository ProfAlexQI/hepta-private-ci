impl RuntimeKernel {
    fn next_write_transaction_group_id(
        &self,
        requested: Option<&str>,
    ) -> Result<String, HeptaError> {
        if let Some(requested) = requested.map(str::trim).filter(|value| !value.is_empty()) {
            let guard = self
                .write_transaction_group_state
                .lock()
                .map_err(|_| HeptaError("write transaction group state mutex poisoned".into()))?;
            if guard.groups.iter().any(|group| group.group_id == requested) {
                return Err(HeptaError(format!(
                    "write transaction group already exists: {}",
                    requested
                )));
            }
            return Ok(requested.to_string());
        }

        let now = current_unix_ms()?;
        let mut suffix = 1usize;
        let guard = self
            .write_transaction_group_state
            .lock()
            .map_err(|_| HeptaError("write transaction group state mutex poisoned".into()))?;
        loop {
            let candidate = format!("txngrp-{}-{}", now, suffix);
            if !guard.groups.iter().any(|group| group.group_id == candidate) {
                return Ok(candidate);
            }
            suffix += 1;
        }
    }

    fn find_write_transaction_group(
        &self,
        session_id: &str,
        group_id: &str,
    ) -> Result<(WriteTransactionGroup, bool), HeptaError> {
        let guard = self
            .write_transaction_group_state
            .lock()
            .map_err(|_| HeptaError("write transaction group state mutex poisoned".into()))?;
        let active = guard
            .active_bindings
            .iter()
            .any(|binding| binding.session_id == session_id && binding.active_group_id == group_id);
        let group = guard
            .groups
            .iter()
            .find(|group| group.session_id == session_id && group.group_id == group_id)
            .cloned()
            .ok_or_else(|| HeptaError(format!("unknown write transaction group: {}", group_id)))?;
        Ok((group, active))
    }

    fn write_transactions_for_session(
        &self,
        session_id: &str,
    ) -> Result<Vec<WriteTransactionEntry>, HeptaError> {
        let mut transactions = self
            .write_transaction_state
            .lock()
            .map_err(|_| HeptaError("write transaction state mutex poisoned".into()))?
            .iter()
            .filter(|entry| entry.session_id == session_id)
            .cloned()
            .collect::<Vec<_>>();
        transactions.sort_by_key(|transaction| std::cmp::Reverse(transaction.created_at_unix_ms));
        Ok(transactions)
    }

    fn write_transaction_groups_for_session(
        &self,
        session_id: &str,
    ) -> Result<Vec<WriteTransactionGroup>, HeptaError> {
        let mut groups = self
            .write_transaction_group_state
            .lock()
            .map_err(|_| HeptaError("write transaction group state mutex poisoned".into()))?
            .groups
            .iter()
            .filter(|group| group.session_id == session_id)
            .cloned()
            .collect::<Vec<_>>();
        groups.sort_by_key(|group| std::cmp::Reverse(group.opened_at_unix_ms));
        Ok(groups)
    }

    fn active_write_transaction_group_id_for_session(
        &self,
        session_id: &str,
    ) -> Result<Option<String>, HeptaError> {
        Ok(self
            .write_transaction_group_state
            .lock()
            .map_err(|_| HeptaError("write transaction group state mutex poisoned".into()))?
            .active_bindings
            .iter()
            .find(|binding| binding.session_id == session_id)
            .map(|binding| binding.active_group_id.clone()))
    }

    fn rollback_group_attempts_for_session(
        &self,
        session_id: &str,
    ) -> Result<Vec<RollbackGroupAttempt>, HeptaError> {
        let mut attempts = self
            .write_transaction_group_state
            .lock()
            .map_err(|_| HeptaError("write transaction group state mutex poisoned".into()))?
            .rollback_attempts
            .iter()
            .filter(|attempt| attempt.session_id == session_id)
            .cloned()
            .collect::<Vec<_>>();
        attempts.sort_by_key(|attempt| std::cmp::Reverse(attempt.started_at_unix_ms));
        Ok(attempts)
    }

    fn rollback_group_attempts(
        &self,
        session_id: &str,
        group_id: &str,
    ) -> Result<Vec<RollbackGroupAttempt>, HeptaError> {
        Ok(self
            .rollback_group_attempts_for_session(session_id)?
            .into_iter()
            .filter(|attempt| attempt.group_id == group_id)
            .collect())
    }

    fn latest_rollback_group_attempt(
        &self,
        session_id: &str,
        group_id: &str,
    ) -> Result<Option<RollbackGroupAttempt>, HeptaError> {
        let attempts = self.rollback_group_attempts(session_id, group_id)?;
        Ok(attempts
            .iter()
            .find(|attempt| attempt.superseded_by_attempt_id.is_none())
            .cloned()
            .or_else(|| attempts.into_iter().next()))
    }

    fn rollback_group_attempt_lifecycle(
        &self,
        session_id: &str,
        group_id: &str,
    ) -> Result<RollbackGroupAttemptLifecycle, HeptaError> {
        let attempts = self.rollback_group_attempts(session_id, group_id)?;
        Ok(RollbackGroupAttemptLifecycle {
            attempt_count: attempts.len(),
            superseded_attempt_count: attempts
                .iter()
                .filter(|attempt| attempt.superseded_by_attempt_id.is_some())
                .count(),
            active_attempt_id: attempts
                .iter()
                .find(|attempt| attempt.superseded_by_attempt_id.is_none())
                .map(|attempt| attempt.attempt_id.clone()),
        })
    }

    fn next_rollback_group_attempt_id(&self) -> Result<String, HeptaError> {
        let now = current_unix_ms()?;
        let mut suffix = 1usize;
        let guard = self
            .write_transaction_group_state
            .lock()
            .map_err(|_| HeptaError("write transaction group state mutex poisoned".into()))?;
        loop {
            let candidate = format!("rbk-{}-{}", now, suffix);
            if !guard
                .rollback_attempts
                .iter()
                .any(|attempt| attempt.attempt_id == candidate)
            {
                return Ok(candidate);
            }
            suffix += 1;
        }
    }

    fn write_locks_for_session(
        &self,
        session_id: &str,
    ) -> Result<(Vec<WriteTargetLock>, Vec<WriteGroupLock>), HeptaError> {
        self.prune_stale_write_locks_internal(false)?;
        let guard = self
            .write_lock_state
            .lock()
            .map_err(|_| HeptaError("write lock state mutex poisoned".into()))?;
        Ok((
            guard
                .target_locks
                .iter()
                .filter(|lock| lock.session_id == session_id)
                .cloned()
                .collect(),
            guard
                .group_locks
                .iter()
                .filter(|lock| lock.session_id == session_id)
                .cloned()
                .collect(),
        ))
    }

    fn rollback_group_attempt_by_id(
        &self,
        attempt_id: &str,
    ) -> Result<Option<RollbackGroupAttempt>, HeptaError> {
        Ok(self
            .write_transaction_group_state
            .lock()
            .map_err(|_| HeptaError("write transaction group state mutex poisoned".into()))?
            .rollback_attempts
            .iter()
            .find(|attempt| attempt.attempt_id == attempt_id)
            .cloned())
    }

    fn live_rollback_group_attempt_by_id(
        &self,
        attempt_id: &str,
    ) -> Result<Option<RollbackGroupAttempt>, HeptaError> {
        Ok(self
            .rollback_group_attempt_by_id(attempt_id)?
            .filter(|attempt| attempt.superseded_by_attempt_id.is_none()))
    }

    fn rollback_group_lock_diagnostics(
        &self,
        session_id: &str,
        group_id: &str,
        latest_attempt_id: Option<&str>,
    ) -> Result<rollback_locks::RollbackGroupLockDiagnostics, HeptaError> {
        let locks = self.write_locks()?;
        Ok(rollback_locks::collect_rollback_group_lock_diagnostics(
            session_id,
            group_id,
            latest_attempt_id,
            &locks,
        ))
    }

    fn prune_stale_write_locks_internal(
        &self,
        emit_event: bool,
    ) -> Result<WriteLockPruneReport, HeptaError> {
        let now_unix_ms = current_unix_ms()?;
        let report = {
            // Keep the global -> local order used by acquisition and Drop.
            let mut process_registry =
                process_write_reservation_registry().lock().map_err(|_| {
                    HeptaError("process write reservation registry mutex poisoned".into())
                })?;
            process_registry.active.retain(|entry| {
                entry.lock.owner_kind != "rollback_group"
                    || entry.lock.lease_expires_at_unix_ms > now_unix_ms
            });
            let mut guard = self
                .write_lock_state
                .lock()
                .map_err(|_| HeptaError("write lock state mutex poisoned".into()))?;
            let before_target_locks = guard.target_locks.len();
            let before_group_locks = guard.group_locks.len();
            guard
                .target_locks
                .retain(|lock| lock.lease_expires_at_unix_ms > now_unix_ms);
            guard
                .group_locks
                .retain(|lock| lock.lease_expires_at_unix_ms > now_unix_ms);
            guard
                .active_group_rollback_reservations
                .retain(|reservation| reservation.lease_expires_at_unix_ms > now_unix_ms);
            rollback_locks::build_write_lock_prune_report(
                now_unix_ms,
                before_target_locks,
                before_group_locks,
                guard.target_locks.len(),
                guard.group_locks.len(),
            )
        };
        if emit_event && (report.pruned_target_locks > 0 || report.pruned_group_locks > 0) {
            self.emit_event_with_payload(
                EventKind::WriteLocksPruned,
                Some(SessionId(self.active_session_id()?)),
                None,
                format!(
                    "pruned stale write locks: targets={} groups={}",
                    report.pruned_target_locks, report.pruned_group_locks
                ),
                Some(json!({
                    "schema_version": ROLLBACK_EVENT_PAYLOAD_SCHEMA_VERSION,
                    "now_unix_ms": report.now_unix_ms,
                    "pruned_target_locks": report.pruned_target_locks,
                    "pruned_group_locks": report.pruned_group_locks,
                    "remaining_target_locks": report.remaining_target_locks,
                    "remaining_group_locks": report.remaining_group_locks,
                })),
            )?;
        }
        Ok(report)
    }

    fn acquire_sealed_write_target_reservation(
        &self,
        session_id: &str,
        sealed_target: &SealedWriteTarget,
        operation: &str,
        group_reservation: Option<&GroupRollbackReservation>,
    ) -> Result<WriteTargetReservation, HeptaError> {
        self.prune_stale_write_locks_internal(false)?;
        if let Some(reservation) = group_reservation
            && reservation.session_id != session_id
        {
            return Err(HeptaError(
                "rollback group witness session does not match the mutation session".into(),
            ));
        }
        let target_path = sealed_target.canonical_path.display().to_string();
        let normalized_target_path = normalize_path(sealed_target.canonical_path.clone());
        let identity = sealed_write_identity(sealed_target);
        let mut cross_process_lease = None;
        let reservation_id = format!("write-reservation-{}", uuid::Uuid::new_v4());
        let locked_at_unix_ms = current_unix_ms()?;
        let lock = WriteTargetLock {
            session_id: session_id.to_string(),
            target_path: normalized_target_path.display().to_string(),
            owner_kind: "tool_execution_reservation".into(),
            owner_id: reservation_id.clone(),
            rollback_group_id: None,
            rollback_attempt_id: None,
            locked_at_unix_ms,
            lease_expires_at_unix_ms: u64::MAX,
        };
        let conflict = {
            // Lock ordering is process registry -> runtime-local observability.
            // Drop follows the same order, so no cross-kernel cycle is possible.
            let mut process_registry =
                process_write_reservation_registry().lock().map_err(|_| {
                    HeptaError("process write reservation registry mutex poisoned".into())
                })?;
            let mut local_state = self
                .write_lock_state
                .lock()
                .map_err(|_| HeptaError("write lock state mutex poisoned".into()))?;
            if let Some(reservation) = group_reservation {
                let exact_local_witness = local_state
                    .active_group_rollback_reservations
                    .iter()
                    .any(|active| {
                        active.token == reservation.token
                            && active.session_id == reservation.session_id
                            && active.group_id == reservation.group_id
                            && active.attempt_id == reservation.attempt_id
                    });
                let exact_group_identity_is_held = process_registry.active.iter().any(|entry| {
                    entry.lock.owner_kind == "rollback_group"
                        && entry.lock.owner_id == reservation.token
                        && entry.lock.session_id == reservation.session_id
                        && entry.lock.rollback_group_id.as_deref()
                            == Some(reservation.group_id.as_str())
                        && entry.lock.rollback_attempt_id.as_deref()
                            == Some(reservation.attempt_id.as_str())
                        && entry.identity == identity
                });
                if !exact_local_witness || !exact_group_identity_is_held {
                    return Err(HeptaError(format!(
                        "rollback group witness does not hold the exact sealed identity for {}",
                        target_path
                    )));
                }
            }
            let local_conflict = local_state
                .target_locks
                .iter()
                .chain(local_state.active_target_reservations.iter())
                .find(|candidate| {
                    paths_overlap(
                        Path::new(&candidate.target_path),
                        normalized_target_path.as_path(),
                    ) && group_reservation
                        .map(|reservation| candidate.owner_id != reservation.token)
                        .unwrap_or(true)
                })
                .cloned();
            let process_conflict = process_registry
                .active
                .iter()
                .find(|candidate| {
                    sealed_write_identities_conflict(&candidate.identity, &identity)
                        && group_reservation
                            .map(|reservation| candidate.lock.owner_id != reservation.token)
                            .unwrap_or(true)
                })
                .map(|candidate| candidate.lock.clone());
            let conflict = local_conflict.or(process_conflict);
            if conflict.is_none() {
                if group_reservation.is_none() {
                    cross_process_lease = Some(
                        runtime_kernel::cross_process_write_lock::acquire_cross_process_target_lease(
                            &sealed_target.workspace_root,
                            &[cross_process_target_identity(&identity)],
                        )
                        .map_err(|error| {
                            HeptaError(format!(
                                "tool_execution_reservation blocks {operation} for {target_path}: {}",
                                error.0
                            ))
                        })?,
                    );
                }
                process_registry.active.push(ProcessWriteReservationEntry {
                    lock: lock.clone(),
                    identity,
                });
                local_state.active_target_reservations.push(lock);
            }
            conflict
        };
        if let Some(lock) = conflict {
            let message = format!(
                "write lock blocks {} for {} (owner={} {})",
                operation, target_path, lock.owner_kind, lock.owner_id
            );
            self.emit_event_with_payload(
                EventKind::WriteLockConflict,
                Some(SessionId(session_id.to_string())),
                None,
                message.clone(),
                Some(json!({
                    "schema_version": ROLLBACK_EVENT_PAYLOAD_SCHEMA_VERSION,
                    "operation": operation,
                    "requested_target_path": target_path,
                    "conflicting_target_path": lock.target_path,
                    "conflicting_owner_kind": lock.owner_kind,
                    "conflicting_owner_id": lock.owner_id,
                    "conflicting_group_id": lock.rollback_group_id,
                    "conflicting_attempt_id": lock.rollback_attempt_id,
                })),
            )?;
            return Err(HeptaError(message));
        }
        Ok(WriteTargetReservation {
            reservation_id: reservation_id.clone(),
            write_lock_state: Arc::clone(&self.write_lock_state),
            process_reservation_id: Some(reservation_id),
            _cross_process_lease: cross_process_lease,
        })
    }

    #[cfg(test)]
    fn acquire_group_rollback_locks(
        &self,
        session_id: &str,
        group_id: &str,
        attempt_id: &str,
        target_paths: &[String],
    ) -> Result<GroupRollbackReservation, HeptaError> {
        self.acquire_group_rollback_locks_internal(
            session_id,
            group_id,
            attempt_id,
            None,
            target_paths,
        )
    }

    fn acquire_group_rollback_locks_internal(
        &self,
        session_id: &str,
        group_id: &str,
        attempt_id: &str,
        resumed_from_attempt_id: Option<&str>,
        target_paths: &[String],
    ) -> Result<GroupRollbackReservation, HeptaError> {
        let locked_at_unix_ms = current_unix_ms()?;
        // The non-cloneable owner token and OS advisory-lock witness, rather
        // than a wall-clock timeout, define this reservation's lifetime.
        let lease_expires_at_unix_ms = u64::MAX;
        self.prune_stale_write_locks_internal(false)?;
        let mut sealed_targets = Vec::<(String, SealedWriteIdentity)>::new();
        for target_path in target_paths {
            let candidate = self.seal_write_candidate(
                session_id,
                "rollback_write_transaction",
                "rollback_group",
                target_path,
                "overwrite",
                false,
            )?;
            if normalize_path(PathBuf::from(&candidate.target_path))
                != normalize_path(PathBuf::from(target_path))
            {
                return Err(HeptaError(format!(
                    "rollback group target identity changed since transaction capture: expected {} resolved {}",
                    target_path, candidate.target_path
                )));
            }
            verify_sealed_target_unchanged(
                &candidate.sealed_target,
                candidate.before_bytes.as_deref(),
            )?;
            let identity = sealed_write_identity(&candidate.sealed_target);
            if sealed_targets
                .iter()
                .any(|(_, existing)| sealed_write_identities_conflict(existing, &identity))
            {
                continue;
            }
            sealed_targets.push((candidate.target_path, identity));
        }
        sealed_targets.sort_by(|left, right| left.0.cmp(&right.0));

        let prior_cross_process_lease = if let Some(resumed_attempt_id) = resumed_from_attempt_id {
            let local_state = self
                .write_lock_state
                .lock()
                .map_err(|_| HeptaError("write lock state mutex poisoned".into()))?;
            Some(
                local_state
                    .active_group_rollback_reservations
                    .iter()
                    .find(|reservation| {
                        reservation.session_id == session_id
                            && reservation.group_id == group_id
                            && reservation.attempt_id == resumed_attempt_id
                    })
                    .map(|reservation| Arc::clone(&reservation.cross_process_lease))
                    .ok_or_else(|| {
                        HeptaError(format!(
                            "rollback group resume lacks the prior runtime-owned reservation for group {}",
                            group_id
                        ))
                    })?,
            )
        } else {
            None
        };
        let group_cross_process_lease = match prior_cross_process_lease {
            Some(lease) => lease,
            None => Arc::new(
                runtime_kernel::cross_process_write_lock::acquire_cross_process_target_lease(
                    &tool_workspace_root_path(),
                    &sealed_targets
                        .iter()
                        .map(|(_, identity)| cross_process_target_identity(identity))
                        .collect::<Vec<_>>(),
                )
                .map_err(|error| {
                    HeptaError(format!(
                        "write lock blocks rollback_group: cross-process reservation failed: {}",
                        error.0
                    ))
                })?,
            ),
        };
        let token = format!("rollback-group-reservation-{}", uuid::Uuid::new_v4());
        let conflict = {
            // Group acquisition is one process-global critical section. No
            // target is published unless every sealed identity is available.
            let mut process_registry =
                process_write_reservation_registry().lock().map_err(|_| {
                    HeptaError("process write reservation registry mutex poisoned".into())
                })?;
            let mut local_state = self
                .write_lock_state
                .lock()
                .map_err(|_| HeptaError("write lock state mutex poisoned".into()))?;

            let prior_reservation = resumed_from_attempt_id.and_then(|resumed_attempt_id| {
                local_state
                    .active_group_rollback_reservations
                    .iter()
                    .find(|reservation| {
                        reservation.session_id == session_id
                            && reservation.group_id == group_id
                            && reservation.attempt_id == resumed_attempt_id
                    })
                    .cloned()
            });
            if resumed_from_attempt_id.is_some() && prior_reservation.is_none() {
                return Err(HeptaError(format!(
                    "rollback group resume lacks the prior runtime-owned reservation for group {}",
                    group_id
                )));
            }
            let prior_token = prior_reservation
                .as_ref()
                .map(|reservation| reservation.token.as_str());
            if let Some(prior) = prior_reservation.as_ref() {
                let prior_identities = process_registry
                    .active
                    .iter()
                    .filter(|entry| {
                        entry.lock.owner_kind == "rollback_group"
                            && entry.lock.owner_id == prior.token
                            && entry.lock.session_id == prior.session_id
                            && entry.lock.rollback_group_id.as_deref()
                                == Some(prior.group_id.as_str())
                            && entry.lock.rollback_attempt_id.as_deref()
                                == Some(prior.attempt_id.as_str())
                    })
                    .map(|entry| &entry.identity)
                    .collect::<Vec<_>>();
                let exact_identity_set = prior_identities.len() == sealed_targets.len()
                    && sealed_targets.iter().all(|(_, identity)| {
                        prior_identities.iter().any(|prior_identity| {
                            sealed_write_namespace_identity_eq(prior_identity, identity)
                        })
                    });
                if !exact_identity_set {
                    return Err(HeptaError(format!(
                        "rollback group resume target identity set changed for group {}",
                        group_id
                    )));
                }
            }

            let same_group_conflict = process_registry.active.iter().find(|entry| {
                entry.lock.owner_kind == "rollback_group"
                    && entry.lock.rollback_group_id.as_deref() == Some(group_id)
                    && prior_token
                        .map(|allowed| entry.lock.owner_id != allowed)
                        .unwrap_or(true)
            });
            let local_group_conflict =
                local_state
                    .active_group_rollback_reservations
                    .iter()
                    .find(|reservation| {
                        reservation.group_id == group_id
                            && prior_token
                                .map(|allowed| reservation.token != allowed)
                                .unwrap_or(true)
                    });
            let mut conflict = same_group_conflict.map(|entry| {
                (
                    sealed_targets
                        .first()
                        .map(|(target_path, _)| target_path.clone())
                        .unwrap_or_else(|| entry.lock.target_path.clone()),
                    entry.lock.clone(),
                )
            });
            if conflict.is_none()
                && let Some(active) = local_group_conflict
            {
                let lock = local_state
                    .group_locks
                    .iter()
                    .find(|lock| lock.owner_id == active.token)
                    .map(|lock| WriteTargetLock {
                        session_id: lock.session_id.clone(),
                        target_path: sealed_targets
                            .first()
                            .map(|(target_path, _)| target_path.clone())
                            .unwrap_or_else(|| "<rollback-group>".into()),
                        owner_kind: lock.owner_kind.clone(),
                        owner_id: lock.owner_id.clone(),
                        rollback_group_id: Some(lock.group_id.clone()),
                        rollback_attempt_id: lock.rollback_attempt_id.clone(),
                        locked_at_unix_ms: lock.locked_at_unix_ms,
                        lease_expires_at_unix_ms: lock.lease_expires_at_unix_ms,
                    })
                    .unwrap_or_else(|| WriteTargetLock {
                        session_id: active.session_id.clone(),
                        target_path: sealed_targets
                            .first()
                            .map(|(target_path, _)| target_path.clone())
                            .unwrap_or_else(|| "<rollback-group>".into()),
                        owner_kind: "rollback_group".into(),
                        owner_id: active.token.clone(),
                        rollback_group_id: Some(active.group_id.clone()),
                        rollback_attempt_id: Some(active.attempt_id.clone()),
                        locked_at_unix_ms,
                        lease_expires_at_unix_ms: active.lease_expires_at_unix_ms,
                    });
                conflict = Some((lock.target_path.clone(), lock));
            }
            for (target_path, identity) in &sealed_targets {
                if conflict.is_some() {
                    break;
                }
                let normalized_target_path = normalize_path(PathBuf::from(target_path));
                let local_conflict = local_state
                    .target_locks
                    .iter()
                    .chain(local_state.active_target_reservations.iter())
                    .find(|lock| {
                        paths_overlap(
                            Path::new(&lock.target_path),
                            normalized_target_path.as_path(),
                        ) && prior_token
                            .map(|allowed| lock.owner_id != allowed)
                            .unwrap_or(true)
                    })
                    .cloned();
                let process_conflict = process_registry
                    .active
                    .iter()
                    .find(|entry| {
                        sealed_write_identities_conflict(&entry.identity, identity)
                            && prior_token
                                .map(|allowed| entry.lock.owner_id != allowed)
                                .unwrap_or(true)
                    })
                    .map(|entry| entry.lock.clone());
                if let Some(lock) = local_conflict.or(process_conflict) {
                    conflict = Some((target_path.clone(), lock));
                }
            }

            if conflict.is_none() {
                if let Some(prior) = prior_reservation.as_ref() {
                    process_registry.active.retain(|entry| {
                        !(entry.lock.owner_kind == "rollback_group"
                            && entry.lock.owner_id == prior.token
                            && entry.lock.session_id == prior.session_id
                            && entry.lock.rollback_group_id.as_deref()
                                == Some(prior.group_id.as_str())
                            && entry.lock.rollback_attempt_id.as_deref()
                                == Some(prior.attempt_id.as_str()))
                    });
                    local_state.group_locks.retain(|lock| {
                        !(lock.owner_id == prior.token
                            && lock.session_id == prior.session_id
                            && lock.group_id == prior.group_id)
                    });
                    local_state.target_locks.retain(|lock| {
                        !(lock.owner_id == prior.token
                            && lock.session_id == prior.session_id
                            && lock.rollback_group_id.as_deref() == Some(prior.group_id.as_str()))
                    });
                    local_state
                        .active_group_rollback_reservations
                        .retain(|reservation| reservation.token != prior.token);
                }
                for (target_path, identity) in &sealed_targets {
                    let lock = WriteTargetLock {
                        session_id: session_id.to_string(),
                        target_path: target_path.clone(),
                        owner_kind: "rollback_group".into(),
                        owner_id: token.clone(),
                        rollback_group_id: Some(group_id.to_string()),
                        rollback_attempt_id: Some(attempt_id.to_string()),
                        locked_at_unix_ms,
                        lease_expires_at_unix_ms,
                    };
                    process_registry.active.push(ProcessWriteReservationEntry {
                        lock,
                        identity: identity.clone(),
                    });
                }

                local_state.group_locks.push(WriteGroupLock {
                    session_id: session_id.to_string(),
                    group_id: group_id.to_string(),
                    owner_kind: "rollback_group".into(),
                    owner_id: token.clone(),
                    rollback_attempt_id: Some(attempt_id.to_string()),
                    locked_at_unix_ms,
                    lease_expires_at_unix_ms,
                });
                local_state
                    .target_locks
                    .extend(
                        sealed_targets
                            .iter()
                            .map(|(target_path, _)| WriteTargetLock {
                                session_id: session_id.to_string(),
                                target_path: target_path.clone(),
                                owner_kind: "rollback_group".into(),
                                owner_id: token.clone(),
                                rollback_group_id: Some(group_id.to_string()),
                                rollback_attempt_id: Some(attempt_id.to_string()),
                                locked_at_unix_ms,
                                lease_expires_at_unix_ms,
                            }),
                    );
                local_state.active_group_rollback_reservations.push(
                    ActiveGroupRollbackReservation {
                        token: token.clone(),
                        session_id: session_id.to_string(),
                        group_id: group_id.to_string(),
                        attempt_id: attempt_id.to_string(),
                        lease_expires_at_unix_ms,
                        cross_process_lease: Arc::clone(&group_cross_process_lease),
                    },
                );
            }
            conflict
        };
        if let Some((target_path, conflict)) = conflict {
            let message = format!(
                "write lock blocks rollback_group for {} (owner={} {})",
                target_path, conflict.owner_kind, conflict.owner_id
            );
            self.emit_event_with_payload(
                EventKind::WriteLockConflict,
                Some(SessionId(session_id.to_string())),
                None,
                message.clone(),
                Some(json!({
                    "schema_version": ROLLBACK_EVENT_PAYLOAD_SCHEMA_VERSION,
                    "operation": "rollback_group",
                    "requested_target_path": target_path,
                    "conflicting_target_path": conflict.target_path,
                    "conflicting_owner_kind": conflict.owner_kind,
                    "conflicting_owner_id": conflict.owner_id,
                    "conflicting_group_id": conflict.rollback_group_id,
                    "conflicting_attempt_id": conflict.rollback_attempt_id,
                })),
            )?;
            return Err(HeptaError(message));
        }
        self.emit_event_with_payload(
            EventKind::WriteLocksAcquired,
            Some(SessionId(session_id.to_string())),
            None,
            format!("acquired write locks for group {}", group_id),
            Some(json!({
                "schema_version": ROLLBACK_EVENT_PAYLOAD_SCHEMA_VERSION,
                "group_id": group_id,
                "attempt_id": attempt_id,
                "target_paths": target_paths,
                "target_lock_count": target_paths.len(),
                "locked_at_unix_ms": locked_at_unix_ms,
                "lease_expires_at_unix_ms": lease_expires_at_unix_ms,
            })),
        )?;
        Ok(GroupRollbackReservation {
            token,
            session_id: session_id.to_string(),
            group_id: group_id.to_string(),
            attempt_id: attempt_id.to_string(),
        })
    }

    fn release_group_rollback_reservation(
        &self,
        reservation: &GroupRollbackReservation,
    ) -> Result<(), HeptaError> {
        let (released_group_locks, released_target_locks) = {
            let mut process_registry =
                process_write_reservation_registry().lock().map_err(|_| {
                    HeptaError("process write reservation registry mutex poisoned".into())
                })?;
            let mut guard = self
                .write_lock_state
                .lock()
                .map_err(|_| HeptaError("write lock state mutex poisoned".into()))?;
            let exact_local_witness =
                guard
                    .active_group_rollback_reservations
                    .iter()
                    .any(|active| {
                        active.token == reservation.token
                            && active.session_id == reservation.session_id
                            && active.group_id == reservation.group_id
                            && active.attempt_id == reservation.attempt_id
                    });
            if !exact_local_witness {
                return Err(HeptaError(format!(
                    "runtime does not own rollback group reservation for group {} attempt {}",
                    reservation.group_id, reservation.attempt_id
                )));
            }
            let released_group_locks = guard
                .group_locks
                .iter()
                .filter(|lock| {
                    lock.owner_id == reservation.token
                        && lock.session_id == reservation.session_id
                        && lock.group_id == reservation.group_id
                        && lock.rollback_attempt_id.as_deref()
                            == Some(reservation.attempt_id.as_str())
                })
                .count();
            let released_target_locks = guard
                .target_locks
                .iter()
                .filter(|lock| {
                    lock.owner_id == reservation.token
                        && lock.session_id == reservation.session_id
                        && lock.rollback_group_id.as_deref() == Some(reservation.group_id.as_str())
                        && lock.rollback_attempt_id.as_deref()
                            == Some(reservation.attempt_id.as_str())
                })
                .count();
            process_registry.active.retain(|entry| {
                !(entry.lock.owner_kind == "rollback_group"
                    && entry.lock.owner_id == reservation.token
                    && entry.lock.session_id == reservation.session_id
                    && entry.lock.rollback_group_id.as_deref()
                        == Some(reservation.group_id.as_str())
                    && entry.lock.rollback_attempt_id.as_deref()
                        == Some(reservation.attempt_id.as_str()))
            });
            guard.group_locks.retain(|lock| {
                !(lock.owner_id == reservation.token
                    && lock.session_id == reservation.session_id
                    && lock.group_id == reservation.group_id
                    && lock.rollback_attempt_id.as_deref() == Some(reservation.attempt_id.as_str()))
            });
            guard.target_locks.retain(|lock| {
                !(lock.owner_id == reservation.token
                    && lock.session_id == reservation.session_id
                    && lock.rollback_group_id.as_deref() == Some(reservation.group_id.as_str())
                    && lock.rollback_attempt_id.as_deref() == Some(reservation.attempt_id.as_str()))
            });
            guard.active_group_rollback_reservations.retain(|active| {
                !(active.token == reservation.token
                    && active.session_id == reservation.session_id
                    && active.group_id == reservation.group_id
                    && active.attempt_id == reservation.attempt_id)
            });
            (released_group_locks, released_target_locks)
        };
        self.emit_event_with_payload(
            EventKind::WriteLocksReleased,
            Some(SessionId(reservation.session_id.clone())),
            None,
            format!("released write locks for group {}", reservation.group_id),
            Some(json!({
                "schema_version": ROLLBACK_EVENT_PAYLOAD_SCHEMA_VERSION,
                "group_id": reservation.group_id,
                "attempt_id": reservation.attempt_id,
                "released_group_locks": released_group_locks,
                "released_target_locks": released_target_locks,
            })),
        )?;
        Ok(())
    }

    #[cfg(test)]
    fn release_group_rollback_locks(
        &self,
        session_id: &str,
        group_id: &str,
    ) -> Result<(), HeptaError> {
        let active = {
            let guard = self
                .write_lock_state
                .lock()
                .map_err(|_| HeptaError("write lock state mutex poisoned".into()))?;
            guard
                .active_group_rollback_reservations
                .iter()
                .find(|reservation| {
                    reservation.session_id == session_id && reservation.group_id == group_id
                })
                .cloned()
                .ok_or_else(|| {
                    HeptaError(format!(
                        "runtime does not own rollback group reservation for group {}",
                        group_id
                    ))
                })?
        };
        self.release_group_rollback_reservation(&GroupRollbackReservation {
            token: active.token,
            session_id: active.session_id,
            group_id: active.group_id,
            attempt_id: active.attempt_id,
        })
    }

    fn append_transaction_to_active_group(
        &self,
        session_id: &str,
        transaction_id: &str,
    ) -> Result<(), HeptaError> {
        let mut guard = self
            .write_transaction_group_state
            .lock()
            .map_err(|_| HeptaError("write transaction group state mutex poisoned".into()))?;
        let Some(active_group_id) = guard
            .active_bindings
            .iter()
            .find(|binding| binding.session_id == session_id)
            .map(|binding| binding.active_group_id.clone())
        else {
            return Ok(());
        };
        let group = guard
            .groups
            .iter_mut()
            .find(|group| group.group_id == active_group_id && group.session_id == session_id)
            .ok_or_else(|| {
                HeptaError(format!(
                    "unknown write transaction group: {}",
                    active_group_id
                ))
            })?;
        if !group.transaction_ids.iter().any(|id| id == transaction_id) {
            group.transaction_ids.push(transaction_id.to_string());
        }
        Ok(())
    }

    fn next_write_transaction_id(&self) -> Result<String, HeptaError> {
        let mut suffix = 1usize;
        let now = current_unix_ms()?;
        let guard = self
            .write_transaction_state
            .lock()
            .map_err(|_| HeptaError("write transaction state mutex poisoned".into()))?;
        loop {
            let candidate = format!("txn-{}-{}", now, suffix);
            if !guard.iter().any(|entry| entry.transaction_id == candidate) {
                return Ok(candidate);
            }
            suffix += 1;
        }
    }

    fn prepare_read_capability(
        &self,
        session_id: &str,
        tool_name: &str,
        input_json: &str,
    ) -> Result<Option<PreparedReadCapability>, HeptaError> {
        let argument_name = match tool_name {
            "read_file" | "read" | "memory_get" => "path",
            _ => return Ok(None),
        };
        let requested_path = parse_required_string_field(input_json, argument_name)
            .map_err(|error| HeptaError(error.0))?;
        if tool_name == "memory_get" {
            validate_memory_get_read_path(&requested_path)?;
        }
        let default_scope = self.filesystem_scope_for_session(session_id)?;
        let scope = self
            .path_capability_gates_for_session(session_id)?
            .into_iter()
            .find(|gate| gate.tool_name == tool_name && gate.argument_name == argument_name)
            .map(|gate| gate.scope)
            .unwrap_or(default_scope);
        let workspace_root = self.workspace_root()?;
        seal_read_capability(
            &workspace_root,
            tool_name,
            argument_name,
            &requested_path,
            scope,
        )
        .map(Some)
    }

    #[cfg(test)]
    fn prepare_write_transaction_with_lock_check(
        &self,
        session_id: &str,
        tool_name: &str,
        input_json: &str,
    ) -> Result<Option<PreparedWriteTransaction>, HeptaError> {
        if tool_name != "write_file" {
            return Ok(None);
        }
        let mut prepared =
            self.prepare_write_transactions_with_lock_check(session_id, tool_name, input_json)?;
        Ok(prepared.transactions.pop())
    }

    fn prepare_write_transactions_with_lock_check(
        &self,
        session_id: &str,
        tool_name: &str,
        input_json: &str,
    ) -> Result<PreparedWriteReservationSet, HeptaError> {
        let preview_only =
            native_mutation_preview_only(input_json).map_err(|error| HeptaError(error.0))?;
        let mut prepared = PreparedWriteReservationSet::empty();
        match tool_name {
            "write_file" => {
                let requested_path = parse_required_string_field(input_json, "path")
                    .map_err(|error| HeptaError(error.0))?;
                let mode_requested = parse_optional_string_field(input_json, "mode")
                    .map_err(|error| HeptaError(error.0))?
                    .unwrap_or_else(|| "create".to_string());
                prepared.transactions.push(self.prepare_sealed_write_target(
                    session_id,
                    tool_name,
                    "write_file",
                    &requested_path,
                    &mode_requested,
                    preview_only,
                    None,
                )?);
            }
            "write" | "edit" => {
                let requested_path = parse_required_string_field(input_json, "path")
                    .map_err(|error| HeptaError(error.0))?;
                let operation = if tool_name == "write" {
                    "native_write"
                } else {
                    "native_edit"
                };
                let mut transaction = self.prepare_sealed_write_target(
                    session_id,
                    tool_name,
                    operation,
                    &requested_path,
                    "overwrite",
                    preview_only,
                    None,
                )?;
                if tool_name == "edit" && !transaction.target_existed_before {
                    return Err(HeptaError(format!(
                        "native edit target did not exist at authorization: {}",
                        transaction.target_path
                    )));
                }
                if !transaction.target_existed_before {
                    transaction.mode_requested = "create".into();
                }
                prepared.transactions.push(transaction);
            }
            "apply_patch" => {
                prepared.transactions =
                    self.prepare_native_patch_reservations(session_id, input_json, preview_only)?;
            }
            "tts" => {
                let (requested_path, argument_names) = native_tts_explicit_output_path(input_json)
                    .map_err(|error| HeptaError(error.0))?
                    .ok_or_else(|| {
                        HeptaError(
                            "tts requires an explicit path or filename for identity-bound execution"
                                .into(),
                        )
                    })?;
                let candidate = self.seal_write_candidate_for_arguments(
                    session_id,
                    tool_name,
                    "native_tts",
                    &requested_path,
                    "overwrite",
                    preview_only,
                    &argument_names,
                )?;
                let mut transaction =
                    self.reserve_sealed_write_candidate(session_id, candidate, None)?;
                if !transaction.target_existed_before {
                    transaction.mode_requested = "create".into();
                }
                if !preview_only {
                    transaction.staged_after_bytes =
                        Some(stage_native_tts_audio(input_json).map_err(|error| {
                            HeptaError(format!(
                                "failed to stage exact TTS effect before intent persistence: {}",
                                error.0
                            ))
                        })?);
                }
                prepared.transactions.push(transaction);
            }
            _ => {}
        }
        Ok(prepared)
    }

    fn prepare_sealed_write_target(
        &self,
        session_id: &str,
        tool_name: &str,
        operation: &str,
        requested_path: &str,
        mode_requested: &str,
        preview_only: bool,
        group_reservation: Option<&GroupRollbackReservation>,
    ) -> Result<PreparedWriteTransaction, HeptaError> {
        let candidate = self.seal_write_candidate(
            session_id,
            tool_name,
            operation,
            requested_path,
            mode_requested,
            preview_only,
        )?;
        self.reserve_sealed_write_candidate(session_id, candidate, group_reservation)
    }

    fn seal_write_candidate(
        &self,
        session_id: &str,
        tool_name: &str,
        operation: &str,
        requested_path: &str,
        mode_requested: &str,
        preview_only: bool,
    ) -> Result<SealedWriteCandidate, HeptaError> {
        self.seal_write_candidate_for_arguments(
            session_id,
            tool_name,
            operation,
            requested_path,
            mode_requested,
            preview_only,
            &["path"],
        )
    }

    fn seal_write_candidate_for_arguments(
        &self,
        session_id: &str,
        tool_name: &str,
        operation: &str,
        requested_path: &str,
        mode_requested: &str,
        preview_only: bool,
        argument_names: &[&str],
    ) -> Result<SealedWriteCandidate, HeptaError> {
        let workspace_root = self.workspace_root()?;
        let session_id = SessionId(session_id.to_string());
        let resolved_target =
            resolve_write_path_within_root(&workspace_root, Path::new(requested_path))?;
        self.ensure_resolved_write_path_scopes_allow_for_arguments(
            &session_id,
            tool_name,
            argument_names,
            requested_path,
            &resolved_target.canonical_path,
        )?;
        let target_path = resolved_target.canonical_path.clone();
        let (sealed_target, before_bytes) = seal_write_target(&workspace_root, resolved_target)?;
        let target_existed_before = sealed_target.target_identity.is_some();
        Ok(SealedWriteCandidate {
            operation: operation.to_string(),
            requested_path: requested_path.to_string(),
            target_path: target_path.display().to_string(),
            mode_requested: mode_requested.to_string(),
            preview_only,
            target_existed_before,
            before_bytes,
            sealed_target,
        })
    }

    fn reserve_sealed_write_candidate(
        &self,
        session_id: &str,
        candidate: SealedWriteCandidate,
        group_reservation: Option<&GroupRollbackReservation>,
    ) -> Result<PreparedWriteTransaction, HeptaError> {
        let reservation = self.acquire_sealed_write_target_reservation(
            session_id,
            &candidate.sealed_target,
            &candidate.operation,
            group_reservation,
        )?;
        verify_sealed_target_unchanged(
            &candidate.sealed_target,
            candidate.before_bytes.as_deref(),
        )?;
        Ok(PreparedWriteTransaction {
            operation: candidate.operation,
            requested_path: candidate.requested_path,
            target_path: candidate.target_path,
            mode_requested: candidate.mode_requested,
            preview_only: candidate.preview_only,
            target_existed_before: candidate.target_existed_before,
            before_bytes: candidate.before_bytes,
            staged_after_bytes: None,
            sealed_target: candidate.sealed_target,
            _reservation: reservation,
        })
    }

    fn prepare_native_patch_reservations(
        &self,
        session_id: &str,
        input_json: &str,
        preview_only: bool,
    ) -> Result<Vec<PreparedWriteTransaction>, HeptaError> {
        let input = parse_tool_input_object(input_json).map_err(|error| HeptaError(error.0))?;
        let patch = input
            .get("input")
            .or_else(|| input.get("patch"))
            .and_then(Value::as_str)
            .ok_or_else(|| {
                HeptaError("apply_patch requires string field 'input' or 'patch'".into())
            })?;
        let operations = parse_native_apply_patch(patch).map_err(|error| HeptaError(error.0))?;
        if !preview_only && operations.len() > 1 {
            return Err(HeptaError(
                "identity-bound apply_patch refuses non-preview multi-operation patches".into(),
            ));
        }
        let mut candidates = Vec::with_capacity(operations.len());
        for operation in operations {
            let (path, operation_name, mode) = match &operation {
                NativePatchOp::Add { path, .. } => (path, "native_patch_add", "create"),
                NativePatchOp::Update { path, .. } => (path, "native_patch_update", "overwrite"),
                NativePatchOp::Delete { path } => {
                    return Err(HeptaError(format!(
                        "identity-bound apply_patch refuses delete operation for {}",
                        path
                    )));
                }
            };
            let candidate = self.seal_write_candidate(
                session_id,
                "apply_patch",
                operation_name,
                path,
                mode,
                preview_only,
            )?;
            match operation {
                NativePatchOp::Add { .. } if candidate.target_existed_before => {
                    return Err(HeptaError(format!(
                        "cannot add existing sealed file {}",
                        candidate.target_path
                    )));
                }
                NativePatchOp::Update { .. } if !candidate.target_existed_before => {
                    return Err(HeptaError(format!(
                        "cannot update missing sealed file {}",
                        candidate.target_path
                    )));
                }
                _ => {}
            }
            candidates.push(candidate);
        }
        for (index, candidate) in candidates.iter().enumerate() {
            let identity = sealed_write_identity(&candidate.sealed_target);
            if candidates[..index].iter().any(|existing| {
                sealed_write_identities_conflict(
                    &sealed_write_identity(&existing.sealed_target),
                    &identity,
                )
            }) {
                return Err(HeptaError(format!(
                    "apply_patch contains repeated or overlapping sealed target {}",
                    candidate.target_path
                )));
            }
        }
        candidates.sort_by(|left, right| left.target_path.cmp(&right.target_path));
        candidates
            .into_iter()
            .map(|candidate| self.reserve_sealed_write_candidate(session_id, candidate, None))
            .collect()
    }

    fn record_mutation_transactions_from_tool_result(
        &self,
        session_id: &SessionId,
        prepared: &[PreparedWriteTransaction],
        tool_output_json: Option<String>,
        effect_plan_hash: Option<&::hepta_contracts::ContentHash>,
        effect_ack_hash: Option<&::hepta_contracts::ContentHash>,
    ) -> Result<runtime_kernel::execution_bus::CapturedTransactionResult, HeptaError> {
        if prepared.is_empty() {
            if effect_plan_hash.is_some() || effect_ack_hash.is_some() {
                return Err(HeptaError(
                    "non-mutation execution carried provider effect evidence".into(),
                ));
            }
            return Ok(runtime_kernel::execution_bus::CapturedTransactionResult {
                final_output_json: tool_output_json,
                evidence: runtime_kernel::execution_bus::CapturedTransaction::NotApplicable,
            });
        }
        if prepared.iter().all(|transaction| transaction.preview_only) {
            if effect_plan_hash.is_some() || effect_ack_hash.is_some() {
                return Err(HeptaError(
                    "mutation preview carried live provider effect evidence".into(),
                ));
            }
            let Some(tool_output_json) = tool_output_json else {
                return Err(HeptaError(
                    "mutation preview capture requires structured tool output".into(),
                ));
            };
            let output_value: Value = serde_json::from_str(&tool_output_json).map_err(|err| {
                HeptaError(format!(
                    "failed to parse tool output JSON for mutation preview capture: {err}"
                ))
            })?;
            return Ok(runtime_kernel::execution_bus::CapturedTransactionResult {
                final_output_json: Some(output_value.to_string()),
                evidence: runtime_kernel::execution_bus::CapturedTransaction::Preview,
            });
        }
        if prepared.iter().any(|transaction| transaction.preview_only) {
            return Err(HeptaError(
                "mutation transaction set mixes preview and live targets".into(),
            ));
        }
        let [prepared] = prepared else {
            return Err(HeptaError(
                "live multi-target mutation requires an atomic multi-entry receipt".into(),
            ));
        };
        let effect_plan_hash = effect_plan_hash.ok_or_else(|| {
            HeptaError("live mutation transaction lacks its staged effect-plan hash".into())
        })?;
        let effect_ack_hash = effect_ack_hash.ok_or_else(|| {
            HeptaError("live mutation transaction lacks its durable provider effect ACK".into())
        })?;
        let mut output_value: Value = match tool_output_json {
            Some(output) => serde_json::from_str(&output).map_err(|err| {
                HeptaError(format!(
                    "failed to parse tool output JSON for mutation transaction capture: {err}"
                ))
            })?,
            None => {
                let effect_observed =
                    committed_mutation_observed_after_error(prepared).map_err(|error| {
                        HeptaError(format!(
                            "mutation effect observation is ambiguous after tool error: {}",
                            error.0
                        ))
                    })?;
                if !effect_observed {
                    return Err(HeptaError(
                        "mutation tool failed with no applied sealed effect observed".into(),
                    ));
                }
                json!({
                    "identity_bound": true,
                    "transaction_capture": "applied_effect_observed_after_tool_error",
                })
            }
        };

        let transaction_id = self.next_write_transaction_id()?;
        let workspace_root = self.workspace_root()?;
        let target_path = PathBuf::from(&prepared.target_path);
        let source_backup_path = output_value
            .get("backup_path")
            .and_then(Value::as_str)
            .map(|value| value.to_string());
        let rollback_checkpoint_path = if prepared.target_existed_before {
            if let Some(source_backup_path) = source_backup_path.clone() {
                Some(source_backup_path)
            } else if let Some(before_bytes) = prepared.before_bytes.as_ref() {
                let checkpoint_path = preview_transaction_checkpoint_path(
                    &workspace_root,
                    &target_path,
                    &transaction_id,
                )?;
                write_new_file_within_root(&workspace_root, &checkpoint_path, before_bytes)?;
                Some(checkpoint_path.display().to_string())
            } else {
                None
            }
        } else {
            None
        };
        let rollback_strategy = if prepared.target_existed_before {
            "restore_checkpoint"
        } else {
            "delete_target"
        };
        let (after_bytes, after_identity) = read_committed_sealed_target(&prepared.sealed_target)?;
        let before_content_hash = prepared.before_bytes.as_deref().map(mutation_content_hash);
        let after_content_hash = mutation_content_hash(&after_bytes);
        let before_file_identity = prepared
            .sealed_target
            .target_identity
            .map(file_identity_label);
        let entry = WriteTransactionEntry {
            transaction_id: transaction_id.clone(),
            session_id: session_id.0.clone(),
            action: prepared.operation.clone(),
            target_path: prepared.target_path.clone(),
            created_at_unix_ms: current_unix_ms()?,
            mode: prepared.mode_requested.clone(),
            target_existed_before: prepared.target_existed_before,
            bytes_before: prepared
                .before_bytes
                .as_ref()
                .map(|bytes| bytes.len() as u64)
                .unwrap_or(0),
            bytes_after: after_bytes.len() as u64,
            before_content_hash,
            after_content_hash: Some(after_content_hash),
            effect_plan_hash: Some(effect_plan_hash.as_str().to_owned()),
            effect_ack_hash: Some(effect_ack_hash.as_str().to_owned()),
            before_file_identity,
            after_file_identity: Some(file_identity_label(after_identity)),
            rollback_strategy: rollback_strategy.into(),
            rollback_checkpoint_path: rollback_checkpoint_path.clone(),
            source_backup_path,
            rolled_back_at_unix_ms: None,
        };
        let entry_json = serde_json::to_string(&entry).map_err(|error| {
            HeptaError(format!(
                "failed to canonicalize write transaction evidence: {error}"
            ))
        })?;
        let entry_hash = runtime_kernel::context_freezer::framed_hash(
            "hepta.runtime.write-transaction-entry.v2",
            &[("canonical_json", entry_json.as_bytes())],
        );
        {
            let mut guard = self
                .write_transaction_state
                .lock()
                .map_err(|_| HeptaError("write transaction state mutex poisoned".into()))?;
            guard.push(entry.clone());
        }
        if let Some(object) = output_value.as_object_mut() {
            object.insert("transaction_id".into(), json!(transaction_id.clone()));
            object.insert("rollback_strategy".into(), json!(rollback_strategy));
            if let Some(rollback_checkpoint_path) = rollback_checkpoint_path {
                object.insert(
                    "rollback_checkpoint_path".into(),
                    json!(rollback_checkpoint_path),
                );
            }
        }
        let active_group_id =
            match self.active_write_transaction_group_id_for_session(&session_id.0) {
                Ok(group_id) => group_id,
                Err(error) => {
                    return Ok(captured_transaction_failure(
                        error,
                        &output_value,
                        &transaction_id,
                        None,
                        &entry_hash,
                    ));
                }
            };
        if let Some(active_group_id) = active_group_id.as_ref()
            && let Some(object) = output_value.as_object_mut()
        {
            object.insert("transaction_group_id".into(), json!(active_group_id));
        }
        if let Err(error) = self.append_transaction_to_active_group(&session_id.0, &transaction_id)
        {
            return Ok(captured_transaction_failure(
                error,
                &output_value,
                &transaction_id,
                active_group_id.as_deref(),
                &entry_hash,
            ));
        }
        if let Err(error) = self.emit_event(
            EventKind::WriteTransactionRecorded,
            Some(session_id.clone()),
            None,
            format!(
                "recorded write transaction {} for {}",
                entry.transaction_id, entry.target_path
            ),
        ) {
            return Ok(captured_transaction_failure(
                error,
                &output_value,
                &transaction_id,
                active_group_id.as_deref(),
                &entry_hash,
            ));
        }

        Ok(runtime_kernel::execution_bus::CapturedTransactionResult {
            final_output_json: Some(output_value.to_string()),
            evidence: runtime_kernel::execution_bus::CapturedTransaction::Recorded {
                transaction_id,
                group_id: active_group_id,
                entry_hash,
            },
        })
    }

    fn record_restore_backup_transaction(
        &self,
        session_id: &SessionId,
        restored_target_path: &str,
        target_existed_before_restore: bool,
        restored_bytes: u64,
        before_content_hash: Option<String>,
        after_content_hash: String,
        before_file_identity: Option<String>,
        after_file_identity: String,
        previous_target_backup_path: Option<String>,
        source_backup_path: String,
    ) -> Result<String, HeptaError> {
        let transaction_id = self.next_write_transaction_id()?;
        let rollback_strategy = if target_existed_before_restore {
            "restore_checkpoint"
        } else {
            "delete_target"
        };
        let entry = WriteTransactionEntry {
            transaction_id: transaction_id.clone(),
            session_id: session_id.0.clone(),
            action: "restore_backup".into(),
            target_path: restored_target_path.to_string(),
            created_at_unix_ms: current_unix_ms()?,
            mode: "restore_backup".into(),
            target_existed_before: target_existed_before_restore,
            bytes_before: previous_target_backup_path
                .as_ref()
                .map(|path| fs::metadata(path).map(|meta| meta.len()).unwrap_or(0))
                .unwrap_or(0),
            bytes_after: restored_bytes,
            before_content_hash,
            after_content_hash: Some(after_content_hash),
            effect_plan_hash: None,
            effect_ack_hash: None,
            before_file_identity,
            after_file_identity: Some(after_file_identity),
            rollback_strategy: rollback_strategy.into(),
            rollback_checkpoint_path: previous_target_backup_path,
            source_backup_path: Some(source_backup_path),
            rolled_back_at_unix_ms: None,
        };
        {
            let mut guard = self
                .write_transaction_state
                .lock()
                .map_err(|_| HeptaError("write transaction state mutex poisoned".into()))?;
            guard.push(entry.clone());
        }
        self.append_transaction_to_active_group(&session_id.0, &transaction_id)?;
        self.emit_event(
            EventKind::WriteTransactionRecorded,
            Some(session_id.clone()),
            None,
            format!(
                "recorded write transaction {} for {}",
                entry.transaction_id, entry.target_path
            ),
        )?;
        Ok(transaction_id)
    }

    pub(crate) fn apply_worker_patch_create_sealed(
        &self,
        authorization: worker_tasks::WorkerPatchApplyAuthorization,
        content: &[u8],
    ) -> Result<Option<String>, HeptaError> {
        let prepared = self.prepare_worker_patch_target(
            authorization.worker_session_id(),
            authorization.requested_path(),
            authorization.workspace_root(),
            "worker_task_patch_apply",
            "create",
        )?;
        if let Some(before) = prepared.before_bytes.as_deref() {
            if before == content {
                return Ok(None);
            }
            return Err(HeptaError(format!(
                "worker patch target already exists with different content: {}",
                prepared.target_path
            )));
        }

        let transaction_id = self.next_write_transaction_id()?;
        write_prepared_target(&prepared, "create", content)?;
        let (after_bytes, after_identity) = read_committed_sealed_target(&prepared.sealed_target)?;
        if after_bytes != content {
            return Err(HeptaError(format!(
                "worker patch committed bytes differ from sealed input: {}",
                prepared.target_path
            )));
        }
        let entry = WriteTransactionEntry {
            transaction_id: transaction_id.clone(),
            session_id: authorization.worker_session_id().to_string(),
            action: "worker_task_patch_apply".into(),
            target_path: prepared.target_path.clone(),
            created_at_unix_ms: current_unix_ms()?,
            mode: "create".into(),
            target_existed_before: false,
            bytes_before: 0,
            bytes_after: after_bytes.len() as u64,
            before_content_hash: None,
            after_content_hash: Some(mutation_content_hash(&after_bytes)),
            effect_plan_hash: None,
            effect_ack_hash: None,
            before_file_identity: None,
            after_file_identity: Some(file_identity_label(after_identity)),
            rollback_strategy: "delete_target".into(),
            rollback_checkpoint_path: None,
            source_backup_path: None,
            rolled_back_at_unix_ms: None,
        };
        self.write_transaction_state
            .lock()
            .map_err(|_| HeptaError("write transaction state mutex poisoned".into()))?
            .push(entry.clone());
        self.append_transaction_to_active_group(
            authorization.worker_session_id(),
            &transaction_id,
        )?;
        self.emit_event(
            EventKind::WriteTransactionRecorded,
            Some(SessionId(authorization.worker_session_id().to_string())),
            None,
            format!(
                "recorded worker patch transaction {} for {}",
                transaction_id, entry.target_path
            ),
        )?;
        Ok(Some(transaction_id))
    }

    fn prepare_worker_patch_target(
        &self,
        worker_session_id: &str,
        requested_path: &str,
        authorized_workspace_root: &str,
        operation: &str,
        mode_requested: &str,
    ) -> Result<PreparedWriteTransaction, HeptaError> {
        let workspace_root = self.workspace_root()?;
        let authorized_root = fs::canonicalize(authorized_workspace_root).map_err(|error| {
            HeptaError(format!(
                "worker patch authorized workspace cannot be resolved: {error}"
            ))
        })?;
        if authorized_root != workspace_root {
            return Err(HeptaError(
                "worker patch authorization workspace no longer matches runtime".into(),
            ));
        }
        let resolved_target =
            resolve_write_path_within_root(&workspace_root, Path::new(requested_path))?;
        if !resolved_target.canonical_path.starts_with(&workspace_root) {
            return Err(HeptaError(format!(
                "worker patch target is outside workspace: {}",
                resolved_target.canonical_path.display()
            )));
        }
        let target_path = resolved_target.canonical_path.clone();
        let (sealed_target, before_bytes) = seal_write_target(&workspace_root, resolved_target)?;
        self.reserve_sealed_write_candidate(
            worker_session_id,
            SealedWriteCandidate {
                operation: operation.into(),
                requested_path: requested_path.into(),
                target_path: target_path.display().to_string(),
                mode_requested: mode_requested.into(),
                preview_only: false,
                target_existed_before: sealed_target.target_identity.is_some(),
                before_bytes,
                sealed_target,
            },
            None,
        )
    }

    pub(crate) fn rollback_worker_patch_create_sealed(
        &self,
        authorization: worker_tasks::WorkerPatchRollbackAuthorization,
    ) -> Result<(), HeptaError> {
        let entry = self
            .write_transaction_state
            .lock()
            .map_err(|_| HeptaError("write transaction state mutex poisoned".into()))?
            .iter()
            .find(|entry| entry.transaction_id == authorization.transaction_id())
            .cloned()
            .ok_or_else(|| {
                HeptaError(format!(
                    "unknown worker patch transaction: {}",
                    authorization.transaction_id()
                ))
            })?;
        if entry.action != "worker_task_patch_apply"
            || entry.session_id != authorization.worker_session_id()
            || entry.rollback_strategy != "delete_target"
            || entry.target_existed_before
            || entry.rolled_back_at_unix_ms.is_some()
        {
            return Err(HeptaError(format!(
                "worker patch rollback receipt is not eligible: {}",
                entry.transaction_id
            )));
        }
        let mut prepared = self.prepare_worker_patch_target(
            authorization.worker_session_id(),
            authorization.requested_path(),
            authorization.workspace_root(),
            "worker_task_patch_rollback",
            "overwrite",
        )?;
        if normalize_path(PathBuf::from(&prepared.target_path))
            != normalize_path(PathBuf::from(&entry.target_path))
        {
            return Err(HeptaError(format!(
                "worker patch rollback target differs from receipt: {}",
                entry.transaction_id
            )));
        }
        let current = prepared.before_bytes.as_deref().ok_or_else(|| {
            HeptaError(format!(
                "worker patch rollback target disappeared: {}",
                entry.target_path
            ))
        })?;
        let expected_hash = entry.after_content_hash.as_deref().ok_or_else(|| {
            HeptaError(format!(
                "worker patch transaction is missing committed content hash: {}",
                entry.transaction_id
            ))
        })?;
        if mutation_content_hash(current) != expected_hash {
            return Err(HeptaError(format!(
                "worker patch rollback target changed since commit: {}",
                entry.target_path
            )));
        }
        delete_prepared_target(&mut prepared)?;
        let rolled_back_at_unix_ms = current_unix_ms()?;
        self.write_transaction_state
            .lock()
            .map_err(|_| HeptaError("write transaction state mutex poisoned".into()))?
            .iter_mut()
            .find(|stored| stored.transaction_id == entry.transaction_id)
            .ok_or_else(|| {
                HeptaError(format!(
                    "worker patch transaction disappeared: {}",
                    entry.transaction_id
                ))
            })?
            .rolled_back_at_unix_ms = Some(rolled_back_at_unix_ms);
        self.emit_event(
            EventKind::WriteRolledBack,
            Some(SessionId(authorization.worker_session_id().to_string())),
            None,
            format!(
                "rolled back worker patch transaction {} for {}",
                entry.transaction_id, entry.target_path
            ),
        )
    }

    fn write_maintenance_blob_sealed(
        &self,
        session_id: &str,
        operation: &str,
        requested_path: &str,
        content: &[u8],
    ) -> Result<String, HeptaError> {
        let mut prepared = self.prepare_sealed_write_target(
            session_id,
            operation,
            operation,
            requested_path,
            "overwrite",
            false,
            None,
        )?;
        write_prepared_target(&prepared, "overwrite", content)?;
        mark_prepared_target_written(&mut prepared, content.to_vec())?;
        Ok(prepared.target_path)
    }

    fn plan_backup_prune(
        &self,
        target_path: Option<&str>,
        keep_latest_per_target: usize,
        max_age_ms: Option<u64>,
        execute: bool,
    ) -> Result<BackupPruneReport, HeptaError> {
        let report = self.backup_index(target_path)?;
        let backup_root = report.backup_root.clone();
        let filter_target_path = report.filter_target_path.clone();
        let scanned_backups = report.backups.len();
        let now = current_unix_ms()?;
        let active_session_id = execute.then(|| self.active_session_id()).transpose()?;
        let canonical_backup_root = if execute {
            Some(fs::canonicalize(&backup_root).map_err(|error| {
                HeptaError(format!(
                    "failed to canonicalize backup root {} before prune: {error}",
                    backup_root
                ))
            })?)
        } else {
            None
        };

        let mut grouped = std::collections::BTreeMap::<String, Vec<BackupEntryReport>>::new();
        for backup in report.backups {
            grouped
                .entry(backup.target_path.clone())
                .or_default()
                .push(backup);
        }

        let mut kept_backups = Vec::new();
        let mut deleted_backups = Vec::new();

        for (_target, mut entries) in grouped {
            entries.sort_by_key(|entry| std::cmp::Reverse(entry.created_at_unix_ms));
            for (index, entry) in entries.into_iter().enumerate() {
                let keep_due_to_count = index < keep_latest_per_target;
                let age_matches = max_age_ms
                    .map(|max_age_ms| now.saturating_sub(entry.created_at_unix_ms) >= max_age_ms)
                    .unwrap_or(true);
                if !keep_due_to_count && age_matches {
                    if execute {
                        let mut prepared = self.prepare_sealed_write_target(
                            active_session_id
                                .as_deref()
                                .ok_or_else(|| HeptaError("active prune session missing".into()))?,
                            "backup_prune",
                            "backup_prune",
                            &entry.backup_path,
                            "overwrite",
                            false,
                            None,
                        )?;
                        let canonical_backup_root =
                            canonical_backup_root.as_ref().ok_or_else(|| {
                                HeptaError("canonical backup prune root missing".into())
                            })?;
                        let sealed_path = normalize_path(PathBuf::from(&prepared.target_path));
                        if !sealed_path.starts_with(canonical_backup_root)
                            || sealed_path
                                != normalize_path(PathBuf::from(entry.backup_path.as_str()))
                        {
                            return Err(HeptaError(format!(
                                "backup prune target changed after indexing: expected {} sealed {}",
                                entry.backup_path, prepared.target_path
                            )));
                        }
                        delete_prepared_target(&mut prepared)?;
                    }
                    deleted_backups.push(entry);
                } else {
                    kept_backups.push(entry);
                }
            }
        }

        kept_backups.sort_by_key(|backup| std::cmp::Reverse(backup.created_at_unix_ms));
        deleted_backups.sort_by_key(|backup| std::cmp::Reverse(backup.created_at_unix_ms));

        Ok(BackupPruneReport {
            backup_root,
            filter_target_path,
            keep_latest_per_target,
            max_age_ms,
            scanned_backups,
            executed: execute,
            deleted_count: deleted_backups.len(),
            kept_backups,
            deleted_backups,
        })
    }

    fn plan_history_merge(
        &self,
        target_session_id: &str,
        source_history: &[TurnRecord],
    ) -> Result<MergeHistoryPlan, HeptaError> {
        let target_history_signatures = self
            .history(Some(target_session_id), usize::MAX)?
            .into_iter()
            .map(|turn| turn_record_signature(&turn))
            .collect::<HashSet<_>>();

        let mut append_turns = Vec::new();
        let mut new_history_entries_to_append = Vec::new();
        let mut duplicate_history_entries_skipped = Vec::new();

        for turn in source_history.iter().rev() {
            let signature = turn_record_signature(turn);
            if target_history_signatures.contains(&signature) {
                duplicate_history_entries_skipped.push(signature);
                continue;
            }

            let mut cloned = turn.clone();
            cloned.session_id = target_session_id.to_string();
            append_turns.push(cloned);
            new_history_entries_to_append.push(signature);
        }

        new_history_entries_to_append.reverse();
        duplicate_history_entries_skipped.reverse();

        Ok(MergeHistoryPlan {
            append_turns,
            new_history_entries_to_append,
            duplicate_history_entries_skipped,
        })
    }
}

fn pending_approval_signature(item: &PendingApproval) -> String {
    format!("{} ({})", item.tool_name, item.reason)
}

fn turn_record_signature(turn: &TurnRecord) -> String {
    format!(
        "input=\"{}\" tool={:?} final=\"{}\" blocked={:?}",
        turn.input, turn.invoked_tool, turn.final_text, turn.blocked_reason
    )
}

fn ordered_unique_difference(items: Vec<String>, other: &HashSet<String>) -> Vec<String> {
    let mut seen = HashSet::new();
    let mut diff = Vec::new();
    for item in items {
        if other.contains(&item) {
            continue;
        }
        if seen.insert(item.clone()) {
            diff.push(item);
        }
    }
    diff
}

fn parse_required_string_field(
    input_json: &str,
    field: &str,
) -> Result<String, hepta_core::ToolError> {
    let value: Value = serde_json::from_str(input_json)
        .map_err(|err| hepta_core::ToolError(format!("invalid JSON tool input: {}", err)))?;
    value
        .get(field)
        .and_then(Value::as_str)
        .map(|value| value.to_string())
        .ok_or_else(|| hepta_core::ToolError(format!("missing string field '{}'", field)))
}

fn parse_optional_string_field(
    input_json: &str,
    field: &str,
) -> Result<Option<String>, hepta_core::ToolError> {
    let value: Value = serde_json::from_str(input_json)
        .map_err(|err| hepta_core::ToolError(format!("invalid JSON tool input: {}", err)))?;
    match value.get(field) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(text)) => Ok(Some(text.clone())),
        Some(_) => Err(hepta_core::ToolError(format!(
            "field '{}' must be a string when present",
            field
        ))),
    }
}

fn parse_optional_bool_field(
    input_json: &str,
    field: &str,
) -> Result<Option<bool>, hepta_core::ToolError> {
    let value: Value = serde_json::from_str(input_json)
        .map_err(|err| hepta_core::ToolError(format!("invalid JSON tool input: {}", err)))?;
    match value.get(field) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::Bool(flag)) => Ok(Some(*flag)),
        Some(_) => Err(hepta_core::ToolError(format!(
            "field '{}' must be a boolean when present",
            field
        ))),
    }
}

fn native_mutation_preview_only(input_json: &str) -> Result<bool, hepta_core::ToolError> {
    let input = parse_tool_input_object(input_json)?;
    match input
        .get("preview_only")
        .or_else(|| input.get("dryRun"))
        .or_else(|| input.get("dry_run"))
    {
        None | Some(Value::Null) => Ok(false),
        Some(Value::Bool(value)) => Ok(*value),
        Some(_) => Err(hepta_core::ToolError(
            "preview_only/dryRun must be a boolean when present".into(),
        )),
    }
}

fn native_tts_explicit_output_path(
    input_json: &str,
) -> Result<Option<(String, Vec<&'static str>)>, hepta_core::ToolError> {
    let input = parse_tool_input_object(input_json)?;
    let path = match input.get("path") {
        None | Some(Value::Null) => None,
        Some(Value::String(path)) if !path.trim().is_empty() => Some(path.clone()),
        Some(Value::String(_)) => {
            return Err(hepta_core::ToolError(
                "tts path must not be empty when present".into(),
            ));
        }
        Some(_) => {
            return Err(hepta_core::ToolError(
                "tts path must be a string when present".into(),
            ));
        }
    };
    let filename = match input.get("filename") {
        None | Some(Value::Null) => None,
        Some(Value::String(path)) if !path.trim().is_empty() => Some(path.clone()),
        Some(Value::String(_)) => {
            return Err(hepta_core::ToolError(
                "tts filename must not be empty when present".into(),
            ));
        }
        Some(_) => {
            return Err(hepta_core::ToolError(
                "tts filename must be a string when present".into(),
            ));
        }
    };
    match (path, filename) {
        (Some(path), Some(filename)) if path != filename => Err(hepta_core::ToolError(
            "tts path and filename must identify the same output when both are present".into(),
        )),
        (Some(path), Some(_)) => Ok(Some((path, vec!["path", "filename"]))),
        (Some(path), None) => Ok(Some((path, vec!["path"]))),
        (None, Some(path)) => Ok(Some((path, vec!["filename"]))),
        (None, None) => Ok(None),
    }
}

fn parse_optional_usize_field(
    input_json: &str,
    field: &str,
) -> Result<Option<usize>, hepta_core::ToolError> {
    let value: Value = serde_json::from_str(input_json)
        .map_err(|err| hepta_core::ToolError(format!("invalid JSON tool input: {}", err)))?;
    match value.get(field) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::Number(number)) => number
            .as_u64()
            .map(|value| Some(value as usize))
            .ok_or_else(|| {
                hepta_core::ToolError(format!("field '{}' must be a non-negative integer", field))
            }),
        Some(_) => Err(hepta_core::ToolError(format!(
            "field '{}' must be an integer when present",
            field
        ))),
    }
}

fn tool_workspace_root_path() -> PathBuf {
    let root = discover_workspace_root();
    fs::canonicalize(&root).unwrap_or_else(|_| normalize_path(root))
}

fn discover_workspace_root() -> PathBuf {
    if let Ok(explicit) = std::env::var("HEPTA_WORKSPACE_ROOT") {
        let trimmed = explicit.trim();
        if !trimmed.is_empty() {
            return PathBuf::from(trimmed);
        }
    }

    let candidates = [
        std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
        PathBuf::from(env!("CARGO_MANIFEST_DIR")),
    ];
    for candidate in candidates {
        let mut cursor = candidate;
        loop {
            let codex_hepta_product_root = cursor.join("codex-rs/Cargo.toml").is_file()
                && cursor.join("codex-rs/core").is_dir()
                && cursor.join("codex-rs/cli").is_dir()
                && cursor.join("codex-rs/hepta-core").is_dir();
            let old_hepta_root = cursor.join("Cargo.toml").is_file()
                && cursor.join("crates").is_dir()
                && cursor.join("apps").is_dir();
            let codex_rust_workspace_root = cursor.join("Cargo.toml").is_file()
                && cursor.join("core").is_dir()
                && cursor.join("cli").is_dir()
                && cursor.join("hepta-core").is_dir();
            if codex_hepta_product_root || old_hepta_root {
                return cursor;
            }
            if codex_rust_workspace_root
                && cursor.file_name().and_then(|name| name.to_str()) == Some("codex-rs")
                && let Some(parent) = cursor.parent()
            {
                return parent.to_path_buf();
            }
            if !cursor.pop() {
                break;
            }
        }
    }

    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from("."))
}

fn default_write_lock_lease_expires_at_unix_ms() -> u64 {
    0
}

fn process_write_reservation_registry() -> &'static Mutex<ProcessWriteReservationRegistry> {
    static REGISTRY: OnceLock<Mutex<ProcessWriteReservationRegistry>> = OnceLock::new();
    REGISTRY.get_or_init(|| Mutex::new(ProcessWriteReservationRegistry::default()))
}

fn sealed_write_identity(target: &SealedWriteTarget) -> SealedWriteIdentity {
    let mut anchor_suffix = target
        .missing_parent_components
        .iter()
        .map(|component| {
            normalize_write_namespace_component(component, target.namespace_case_insensitive)
        })
        .collect::<Vec<_>>();
    anchor_suffix.push(normalize_write_namespace_component(
        &target.leaf_name,
        target.namespace_case_insensitive,
    ));
    SealedWriteIdentity {
        canonical_namespace: normalize_path(target.canonical_path.clone()),
        existing_target: target.target_identity,
        anchor: target.anchor_identity,
        anchor_suffix,
    }
}

fn cross_process_target_identity(
    identity: &SealedWriteIdentity,
) -> runtime_kernel::cross_process_write_lock::CrossProcessTargetIdentity {
    runtime_kernel::cross_process_write_lock::CrossProcessTargetIdentity::new(
        identity.canonical_namespace.clone(),
        identity.anchor.device,
        identity.anchor.inode,
        identity.anchor_suffix.clone(),
        identity
            .existing_target
            .map(|target| (target.device, target.inode)),
    )
}

fn sealed_write_identities_conflict(
    left: &SealedWriteIdentity,
    right: &SealedWriteIdentity,
) -> bool {
    paths_overlap(&left.canonical_namespace, &right.canonical_namespace)
        || left
            .existing_target
            .zip(right.existing_target)
            .map(|(left, right)| left == right)
            .unwrap_or(false)
        || (left.anchor == right.anchor && left.anchor_suffix == right.anchor_suffix)
}

fn sealed_write_namespace_identity_eq(
    left: &SealedWriteIdentity,
    right: &SealedWriteIdentity,
) -> bool {
    left.canonical_namespace == right.canonical_namespace
        && left.anchor == right.anchor
        && left.anchor_suffix == right.anchor_suffix
}

#[cfg(unix)]
fn normalize_write_namespace_component(
    component: &std::ffi::OsStr,
    case_insensitive: bool,
) -> std::ffi::OsString {
    if !case_insensitive {
        return component.to_os_string();
    }
    use std::os::unix::ffi::OsStrExt as _;
    use std::os::unix::ffi::OsStringExt as _;

    std::ffi::OsString::from_vec(
        component
            .as_bytes()
            .iter()
            .map(u8::to_ascii_lowercase)
            .collect(),
    )
}

#[cfg(not(unix))]
fn normalize_write_namespace_component(
    component: &std::ffi::OsStr,
    _case_insensitive: bool,
) -> std::ffi::OsString {
    component.to_os_string()
}

fn resolve_path_within_root(root: &Path, requested: &Path) -> PathBuf {
    let candidate = if requested.is_absolute() {
        requested.to_path_buf()
    } else {
        root.join(requested)
    };

    if let Ok(canonical) = fs::canonicalize(&candidate) {
        canonical
    } else {
        normalize_path(candidate)
    }
}

#[derive(Debug)]
struct ResolvedWriteTarget {
    canonical_path: PathBuf,
    canonical_anchor: PathBuf,
    missing_parent_components: Vec<std::ffi::OsString>,
    leaf_name: std::ffi::OsString,
}

/// Resolves the deepest existing parent rather than falling back to a lexical
/// path when the leaf does not exist. Existing symlinks are therefore either
/// canonicalized or rejected; they can never disappear into a lexical suffix.
fn resolve_write_path_within_root(
    root: &Path,
    requested: &Path,
) -> Result<ResolvedWriteTarget, HeptaError> {
    let candidate = normalize_path(if requested.is_absolute() {
        requested.to_path_buf()
    } else {
        root.join(requested)
    });
    let leaf_name = candidate
        .file_name()
        .map(std::ffi::OsStr::to_os_string)
        .ok_or_else(|| {
            HeptaError(format!(
                "write target must name a file: {}",
                candidate.display()
            ))
        })?;
    let mut cursor = candidate
        .parent()
        .map(Path::to_path_buf)
        .ok_or_else(|| HeptaError("write target has no parent directory".into()))?;
    let mut missing_parent_components = Vec::new();
    let canonical_anchor = loop {
        match fs::canonicalize(&cursor) {
            Ok(canonical) => {
                let metadata = fs::metadata(&canonical).map_err(|error| {
                    HeptaError(format!(
                        "failed to inspect canonical write ancestor {}: {error}",
                        canonical.display()
                    ))
                })?;
                if !metadata.is_dir() {
                    return Err(HeptaError(format!(
                        "write ancestor is not a directory: {}",
                        canonical.display()
                    )));
                }
                break normalize_path(canonical);
            }
            Err(canonical_error) => match fs::symlink_metadata(&cursor) {
                Ok(_) => {
                    return Err(HeptaError(format!(
                        "existing write ancestor cannot be canonicalized: {}: {canonical_error}",
                        cursor.display()
                    )));
                }
                Err(metadata_error) if metadata_error.kind() == std::io::ErrorKind::NotFound => {
                    let component = cursor
                        .file_name()
                        .map(std::ffi::OsStr::to_os_string)
                        .ok_or_else(|| {
                            HeptaError(format!(
                                "failed to find an existing write ancestor for {}",
                                candidate.display()
                            ))
                        })?;
                    if !cursor.pop() {
                        return Err(HeptaError(format!(
                            "failed to find an existing write ancestor for {}",
                            candidate.display()
                        )));
                    }
                    missing_parent_components.push(component);
                }
                Err(metadata_error) => {
                    return Err(HeptaError(format!(
                        "failed to inspect write ancestor {}: {metadata_error}",
                        cursor.display()
                    )));
                }
            },
        }
    };
    missing_parent_components.reverse();
    let mut canonical_path = canonical_anchor.clone();
    for component in &missing_parent_components {
        canonical_path.push(component);
    }
    canonical_path.push(&leaf_name);

    Ok(ResolvedWriteTarget {
        canonical_path,
        canonical_anchor,
        missing_parent_components,
        leaf_name,
    })
}

const PREPARED_READ_MAX_BYTES: u64 = 16 * 1024 * 1024;

fn validate_memory_get_read_path(requested_path: &str) -> Result<(), HeptaError> {
    let requested = Path::new(requested_path);
    if requested.is_absolute() {
        return Err(HeptaError(
            "memory_get only permits relative MEMORY.md or memory/*.md paths".into(),
        ));
    }
    let mut components = Vec::new();
    for component in requested.components() {
        match component {
            Component::Normal(component) => components.push(component),
            Component::CurDir => {}
            Component::ParentDir | Component::RootDir | Component::Prefix(_) => {
                return Err(HeptaError(
                    "memory_get only permits relative MEMORY.md or memory/*.md paths".into(),
                ));
            }
        }
    }
    let allowed = match components.as_slice() {
        [file] => *file == std::ffi::OsStr::new("MEMORY.md"),
        [directory, file] => {
            *directory == std::ffi::OsStr::new("memory")
                && file
                    .to_str()
                    .is_some_and(|name| name.len() > 3 && name.ends_with(".md"))
        }
        _ => false,
    };
    if allowed {
        Ok(())
    } else {
        Err(HeptaError(
            "memory_get only permits relative MEMORY.md or memory/*.md paths".into(),
        ))
    }
}

fn resolve_read_components(
    workspace_root: &Path,
    requested_path: &str,
    scope: FilesystemScope,
) -> Result<(PathBuf, Vec<std::ffi::OsString>, PathBuf), HeptaError> {
    let requested = Path::new(requested_path);
    let (anchor_path, relative) = if requested.is_absolute() {
        match scope {
            FilesystemScope::AnyPath => (
                PathBuf::from("/"),
                requested.strip_prefix(Path::new("/")).map_err(|_| {
                    HeptaError(format!("invalid absolute read path: {requested_path}"))
                })?,
            ),
            FilesystemScope::WorkspaceOnly => (
                workspace_root.to_path_buf(),
                requested.strip_prefix(workspace_root).map_err(|_| {
                    HeptaError(format!(
                        "filesystem scope workspace_only blocks read path {} outside workspace {}",
                        requested_path,
                        workspace_root.display()
                    ))
                })?,
            ),
        }
    } else {
        (workspace_root.to_path_buf(), requested)
    };
    let mut components = Vec::new();
    for component in relative.components() {
        match component {
            Component::Normal(component) => components.push(component.to_os_string()),
            Component::CurDir => {}
            Component::ParentDir | Component::RootDir | Component::Prefix(_) => {
                return Err(HeptaError(format!(
                    "sealed read path contains a forbidden component: {requested_path}"
                )));
            }
        }
    }
    if components.is_empty() {
        return Err(HeptaError(format!(
            "sealed read target must name a file: {requested_path}"
        )));
    }
    let mut resolved_path = anchor_path.clone();
    for component in &components {
        resolved_path.push(component);
    }
    Ok((anchor_path, components, resolved_path))
}

#[cfg(unix)]
fn read_file_version(metadata: &fs::Metadata) -> (u64, i64, i64, i64, i64) {
    use std::os::unix::fs::MetadataExt as _;

    (
        metadata.size(),
        metadata.mtime(),
        metadata.mtime_nsec(),
        metadata.ctime(),
        metadata.ctime_nsec(),
    )
}

#[cfg(unix)]
fn ensure_single_link_read_target(
    metadata: &fs::Metadata,
    target_path: &Path,
) -> Result<(), HeptaError> {
    use std::os::unix::fs::MetadataExt as _;

    let link_count = metadata.nlink();
    if link_count == 1 {
        Ok(())
    } else {
        Err(HeptaError(format!(
            "sealed read refuses target with {link_count} hard links: {}",
            target_path.display()
        )))
    }
}

#[cfg(unix)]
fn openat_read_directory(
    directory: &fs::File,
    component: &std::ffi::OsStr,
) -> Result<fs::File, HeptaError> {
    openat_file(
        directory,
        component,
        libc::O_RDONLY | libc::O_DIRECTORY | libc::O_NOFOLLOW | libc::O_CLOEXEC,
        0,
    )
    .map_err(|error| {
        HeptaError(format!(
            "sealed read refused directory component {:?}: {error}",
            component
        ))
    })
}

#[cfg(unix)]
fn seal_read_capability(
    workspace_root: &Path,
    tool_name: &str,
    argument_name: &str,
    requested_path: &str,
    scope: FilesystemScope,
) -> Result<PreparedReadCapability, HeptaError> {
    use std::io::Read as _;

    let (anchor_path, relative_components, resolved_path) =
        resolve_read_components(workspace_root, requested_path, scope)?;
    let anchor_metadata = fs::symlink_metadata(&anchor_path).map_err(|error| {
        HeptaError(format!(
            "failed to inspect sealed read anchor {}: {error}",
            anchor_path.display()
        ))
    })?;
    if !anchor_metadata.is_dir() || anchor_metadata.file_type().is_symlink() {
        return Err(HeptaError(format!(
            "sealed read anchor is not a real directory: {}",
            anchor_path.display()
        )));
    }
    let anchor_identity = file_identity(&anchor_metadata);
    let anchor_directory = fs::File::open(&anchor_path).map_err(|error| {
        HeptaError(format!(
            "failed to retain sealed read anchor {}: {error}",
            anchor_path.display()
        ))
    })?;
    let opened_anchor = anchor_directory.metadata().map_err(|error| {
        HeptaError(format!(
            "failed to inspect retained read anchor {}: {error}",
            anchor_path.display()
        ))
    })?;
    if !opened_anchor.is_dir() || file_identity(&opened_anchor) != anchor_identity {
        return Err(HeptaError(format!(
            "sealed read anchor changed while being retained: {}",
            anchor_path.display()
        )));
    }

    let (leaf_name, directory_components) = relative_components
        .split_last()
        .ok_or_else(|| HeptaError("sealed read target has no leaf".into()))?;
    let mut parent = anchor_directory
        .try_clone()
        .map_err(|error| HeptaError(format!("failed to duplicate sealed read anchor: {error}")))?;
    for component in directory_components {
        parent = openat_read_directory(&parent, component)?;
    }
    let parent_metadata = parent.metadata().map_err(|error| {
        HeptaError(format!(
            "failed to inspect sealed read parent {}: {error}",
            resolved_path.display()
        ))
    })?;
    let parent_identity = file_identity(&parent_metadata);
    let mut retained_file = openat_file(
        &parent,
        leaf_name,
        libc::O_RDONLY | libc::O_NONBLOCK | libc::O_NOFOLLOW | libc::O_CLOEXEC,
        0,
    )
    .map_err(|error| {
        HeptaError(format!(
            "sealed read refused target {}: {error}",
            resolved_path.display()
        ))
    })?;
    let before_metadata = retained_file.metadata().map_err(|error| {
        HeptaError(format!(
            "failed to inspect sealed read target {}: {error}",
            resolved_path.display()
        ))
    })?;
    if !before_metadata.is_file() {
        return Err(HeptaError(format!(
            "sealed read target is not a regular file: {}",
            resolved_path.display()
        )));
    }
    ensure_single_link_read_target(&before_metadata, &resolved_path)?;
    if before_metadata.len() > PREPARED_READ_MAX_BYTES {
        return Err(HeptaError(format!(
            "sealed read target exceeds {} bytes: {}",
            PREPARED_READ_MAX_BYTES,
            resolved_path.display()
        )));
    }
    let captured_file_identity = file_identity(&before_metadata);
    let before_version = read_file_version(&before_metadata);
    let mut bytes = Vec::with_capacity(before_metadata.len() as usize);
    retained_file.read_to_end(&mut bytes).map_err(|error| {
        HeptaError(format!(
            "failed to capture sealed read bytes {}: {error}",
            resolved_path.display()
        ))
    })?;
    let after_metadata = retained_file.metadata().map_err(|error| {
        HeptaError(format!(
            "failed to revalidate sealed read target {}: {error}",
            resolved_path.display()
        ))
    })?;
    ensure_single_link_read_target(&after_metadata, &resolved_path)?;
    if !after_metadata.is_file()
        || file_identity(&after_metadata) != captured_file_identity
        || read_file_version(&after_metadata) != before_version
        || after_metadata.len() != bytes.len() as u64
    {
        return Err(HeptaError(format!(
            "sealed read target changed while bytes were captured: {}",
            resolved_path.display()
        )));
    }
    std::str::from_utf8(&bytes).map_err(|error| {
        HeptaError(format!(
            "sealed read target is not UTF-8 text {}: {error}",
            resolved_path.display()
        ))
    })?;
    let content_hash = runtime_kernel::context_freezer::framed_hash(
        "hepta.runtime.sealed-read-content.v1",
        &[("bytes", &bytes)],
    )
    .into_inner();
    Ok(PreparedReadCapability {
        tool_name: tool_name.to_string(),
        argument_name: argument_name.to_string(),
        requested_path: requested_path.to_string(),
        resolved_path,
        anchor_path,
        relative_components,
        anchor_identity,
        parent_identity,
        file_identity: captured_file_identity,
        content_hash,
        bytes,
        anchor_directory,
        retained_file,
    })
}

#[cfg(not(unix))]
fn seal_read_capability(
    _workspace_root: &Path,
    _tool_name: &str,
    _argument_name: &str,
    _requested_path: &str,
    _scope: FilesystemScope,
) -> Result<PreparedReadCapability, HeptaError> {
    Err(HeptaError(
        "sealed read capabilities require Unix openat semantics".into(),
    ))
}

impl PreparedReadCapability {
    #[cfg(unix)]
    fn verify_namespace_unchanged(&self) -> Result<(), HeptaError> {
        let anchor_now = fs::symlink_metadata(&self.anchor_path).map_err(|error| {
            HeptaError(format!(
                "sealed read anchor disappeared after authorization {}: {error}",
                self.anchor_path.display()
            ))
        })?;
        let retained_anchor = self.anchor_directory.metadata().map_err(|error| {
            HeptaError(format!(
                "failed to inspect retained read anchor {}: {error}",
                self.anchor_path.display()
            ))
        })?;
        if !anchor_now.is_dir()
            || anchor_now.file_type().is_symlink()
            || file_identity(&anchor_now) != self.anchor_identity
            || file_identity(&retained_anchor) != self.anchor_identity
        {
            return Err(HeptaError(format!(
                "sealed read anchor identity changed after authorization: {}",
                self.anchor_path.display()
            )));
        }
        let (leaf_name, directory_components) = self
            .relative_components
            .split_last()
            .ok_or_else(|| HeptaError("sealed read target has no leaf".into()))?;
        let mut parent = self.anchor_directory.try_clone().map_err(|error| {
            HeptaError(format!("failed to duplicate retained read anchor: {error}"))
        })?;
        for component in directory_components {
            parent = openat_read_directory(&parent, component)?;
        }
        let parent_metadata = parent.metadata().map_err(|error| {
            HeptaError(format!(
                "failed to inspect sealed read parent {}: {error}",
                self.resolved_path.display()
            ))
        })?;
        if file_identity(&parent_metadata) != self.parent_identity {
            return Err(HeptaError(format!(
                "sealed read ancestor identity changed after authorization: {}",
                self.resolved_path.display()
            )));
        }
        let current = openat_file(
            &parent,
            leaf_name,
            libc::O_RDONLY | libc::O_NONBLOCK | libc::O_NOFOLLOW | libc::O_CLOEXEC,
            0,
        )
        .map_err(|error| {
            HeptaError(format!(
                "sealed read target changed after authorization {}: {error}",
                self.resolved_path.display()
            ))
        })?;
        let current_metadata = current.metadata().map_err(|error| {
            HeptaError(format!(
                "failed to inspect current sealed read target {}: {error}",
                self.resolved_path.display()
            ))
        })?;
        let retained_metadata = self.retained_file.metadata().map_err(|error| {
            HeptaError(format!(
                "failed to inspect retained read target {}: {error}",
                self.resolved_path.display()
            ))
        })?;
        ensure_single_link_read_target(&current_metadata, &self.resolved_path)?;
        ensure_single_link_read_target(&retained_metadata, &self.resolved_path)?;
        if !current_metadata.is_file()
            || !retained_metadata.is_file()
            || file_identity(&current_metadata) != self.file_identity
            || file_identity(&retained_metadata) != self.file_identity
        {
            return Err(HeptaError(format!(
                "sealed read target identity changed after authorization: {}",
                self.resolved_path.display()
            )));
        }
        let current_hash = runtime_kernel::context_freezer::framed_hash(
            "hepta.runtime.sealed-read-content.v1",
            &[("bytes", &self.bytes)],
        );
        if current_hash.as_str() != self.content_hash.as_str() {
            return Err(HeptaError(
                "sealed read captured bytes failed integrity validation".into(),
            ));
        }
        Ok(())
    }

    #[cfg(not(unix))]
    fn verify_namespace_unchanged(&self) -> Result<(), HeptaError> {
        Err(HeptaError(
            "sealed read capabilities require Unix openat semantics".into(),
        ))
    }
}

fn preflight_prepared_read(
    tool_name: &str,
    prepared: &PreparedReadCapability,
    input_json: &str,
) -> Result<(), hepta_core::ToolError> {
    if prepared.tool_name != tool_name {
        return Err(hepta_core::ToolError(format!(
            "sealed read capability tool mismatch: prepared={} invoked={tool_name}",
            prepared.tool_name
        )));
    }
    let requested_path = parse_required_string_field(input_json, &prepared.argument_name)?;
    if requested_path != prepared.requested_path {
        return Err(hepta_core::ToolError(
            "sealed read capability path differs from authorized arguments".into(),
        ));
    }
    prepared
        .verify_namespace_unchanged()
        .map_err(|error| hepta_core::ToolError(error.0))
}

#[cfg(unix)]
fn file_identity(metadata: &fs::Metadata) -> FileIdentity {
    use std::os::unix::fs::MetadataExt as _;

    FileIdentity {
        device: metadata.dev(),
        inode: metadata.ino(),
    }
}

fn file_identity_label(identity: FileIdentity) -> String {
    format!("device:{}:inode:{}", identity.device, identity.inode)
}

fn mutation_content_hash(content: &[u8]) -> String {
    runtime_kernel::context_freezer::framed_hash(
        "hepta.runtime.mutation-content.v1",
        &[("content", content)],
    )
    .into_inner()
}

#[cfg(unix)]
fn ensure_single_link_mutable_target(
    metadata: &fs::Metadata,
    target_path: &Path,
) -> Result<(), HeptaError> {
    use std::os::unix::fs::MetadataExt as _;

    let link_count = metadata.nlink();
    if link_count == 1 {
        Ok(())
    } else {
        Err(HeptaError(format!(
            "identity-bound mutation refuses target with {link_count} hard links: {}",
            target_path.display()
        )))
    }
}

#[cfg(unix)]
fn directory_namespace_case_insensitive(directory: &fs::File) -> bool {
    #[cfg(target_os = "macos")]
    {
        use std::os::fd::AsRawFd as _;

        // Darwin exposes the volume policy without probing the namespace.
        // ASCII folding covers ordinary case aliases. APFS Unicode
        // normalization remains an explicit process-local residual.
        (unsafe { libc::fpathconf(directory.as_raw_fd(), libc::_PC_CASE_SENSITIVE) } == 0)
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = directory;
        // Linux has no portable fpathconf equivalent for casefolded
        // directories; canonical path and inode/anchor identities still bind
        // normal case-sensitive filesystems.
        false
    }
}

#[cfg(unix)]
fn os_component_cstring(component: &std::ffi::OsStr) -> Result<std::ffi::CString, HeptaError> {
    use std::os::unix::ffi::OsStrExt as _;

    std::ffi::CString::new(component.as_bytes()).map_err(|_| {
        HeptaError(format!(
            "write path component contains a NUL byte: {:?}",
            component
        ))
    })
}

#[cfg(unix)]
fn openat_file(
    directory: &fs::File,
    name: &std::ffi::OsStr,
    flags: libc::c_int,
    mode: libc::mode_t,
) -> Result<fs::File, std::io::Error> {
    use std::os::fd::AsRawFd as _;
    use std::os::fd::FromRawFd as _;
    use std::os::unix::ffi::OsStrExt as _;

    let name = std::ffi::CString::new(name.as_bytes())
        .map_err(|_| std::io::Error::from_raw_os_error(libc::EINVAL))?;
    // SAFETY: `name` is NUL terminated, `directory` owns a valid descriptor,
    // and a successful descriptor is transferred exactly once into `File`.
    let descriptor = unsafe {
        libc::openat(
            directory.as_raw_fd(),
            name.as_ptr(),
            flags,
            libc::c_uint::from(mode),
        )
    };
    if descriptor < 0 {
        Err(std::io::Error::last_os_error())
    } else {
        // SAFETY: `openat` returned a new owned descriptor.
        Ok(unsafe { fs::File::from_raw_fd(descriptor) })
    }
}

#[cfg(unix)]
fn openat_directory(directory: &fs::File, name: &std::ffi::OsStr) -> Result<fs::File, HeptaError> {
    openat_file(
        directory,
        name,
        libc::O_RDONLY | libc::O_DIRECTORY | libc::O_NOFOLLOW | libc::O_CLOEXEC,
        0,
    )
    .map_err(|error| {
        HeptaError(format!(
            "failed to open sealed write directory component {:?}: {error}",
            name
        ))
    })
}

#[cfg(unix)]
fn mkdirat_directory(directory: &fs::File, name: &std::ffi::OsStr) -> Result<(), HeptaError> {
    use std::os::fd::AsRawFd as _;

    let name = os_component_cstring(name)?;
    // SAFETY: `name` is NUL terminated and `directory` owns a valid descriptor.
    let result = unsafe { libc::mkdirat(directory.as_raw_fd(), name.as_ptr(), 0o755) };
    if result == 0 {
        Ok(())
    } else {
        Err(HeptaError(format!(
            "sealed write parent changed after authorization at {:?}: {}",
            name,
            std::io::Error::last_os_error()
        )))
    }
}

#[cfg(unix)]
fn entry_exists_at(directory: &fs::File, name: &std::ffi::OsStr) -> Result<bool, HeptaError> {
    use std::os::fd::AsRawFd as _;

    let name = os_component_cstring(name)?;
    let mut stat = std::mem::MaybeUninit::<libc::stat>::uninit();
    // SAFETY: `stat` points to writable storage, `name` is NUL terminated, and
    // `directory` owns a valid descriptor.
    let result = unsafe {
        libc::fstatat(
            directory.as_raw_fd(),
            name.as_ptr(),
            stat.as_mut_ptr(),
            libc::AT_SYMLINK_NOFOLLOW,
        )
    };
    if result == 0 {
        Ok(true)
    } else {
        let error = std::io::Error::last_os_error();
        if error.kind() == std::io::ErrorKind::NotFound {
            Ok(false)
        } else {
            Err(HeptaError(format!(
                "failed to inspect sealed write component {:?}: {error}",
                name
            )))
        }
    }
}

#[cfg(unix)]
fn validate_sealed_anchor(target: &SealedWriteTarget) -> Result<fs::File, HeptaError> {
    let canonical_now = fs::canonicalize(&target.canonical_anchor).map_err(|error| {
        HeptaError(format!(
            "sealed write ancestor disappeared after authorization: {}: {error}",
            target.canonical_anchor.display()
        ))
    })?;
    if normalize_path(canonical_now) != target.canonical_anchor {
        return Err(HeptaError(format!(
            "sealed write ancestor changed after authorization: {}",
            target.canonical_anchor.display()
        )));
    }
    let current_metadata = fs::metadata(&target.canonical_anchor).map_err(|error| {
        HeptaError(format!(
            "failed to inspect sealed write ancestor {}: {error}",
            target.canonical_anchor.display()
        ))
    })?;
    let handle_metadata = target.anchor_directory.metadata().map_err(|error| {
        HeptaError(format!(
            "failed to inspect sealed write ancestor handle {}: {error}",
            target.canonical_anchor.display()
        ))
    })?;
    if !current_metadata.is_dir()
        || file_identity(&current_metadata) != target.anchor_identity
        || file_identity(&handle_metadata) != target.anchor_identity
    {
        return Err(HeptaError(format!(
            "sealed write ancestor identity changed after authorization: {}",
            target.canonical_anchor.display()
        )));
    }
    target.anchor_directory.try_clone().map_err(|error| {
        HeptaError(format!(
            "failed to duplicate sealed write ancestor handle {}: {error}",
            target.canonical_anchor.display()
        ))
    })
}

#[cfg(unix)]
fn open_sealed_parent(
    target: &SealedWriteTarget,
    create_missing: bool,
) -> Result<Option<fs::File>, HeptaError> {
    let mut directory = validate_sealed_anchor(target)?;
    for component in &target.missing_parent_components {
        if !create_missing {
            if entry_exists_at(&directory, component)? {
                return Err(HeptaError(format!(
                    "sealed write parent changed after authorization at {:?}",
                    component
                )));
            }
            return Ok(None);
        }
        mkdirat_directory(&directory, component)?;
        directory = openat_directory(&directory, component)?;
    }
    Ok(Some(directory))
}

/// Reopens a parent suffix that this authorized mutation has already created.
///
/// Pre-commit callers must use `open_sealed_parent`: an entry appearing in a
/// previously missing suffix is a namespace race until the authorized write
/// itself materializes it. Post-commit callers instead need to traverse that
/// now-existing suffix without creating anything. Each hop remains relative
/// to the retained anchor descriptor and rejects symlinks via
/// `O_NOFOLLOW | O_DIRECTORY | O_CLOEXEC` in `openat_directory`.
#[cfg(unix)]
fn open_committed_sealed_parent(target: &SealedWriteTarget) -> Result<fs::File, HeptaError> {
    let mut directory = validate_sealed_anchor(target)?;
    for component in &target.missing_parent_components {
        directory = openat_directory(&directory, component).map_err(|error| {
            HeptaError(format!(
                "failed to reopen committed sealed write parent at {:?}: {}",
                component, error.0
            ))
        })?;
    }
    Ok(directory)
}

#[cfg(unix)]
fn open_checked_existing_target(
    target: &SealedWriteTarget,
    flags: libc::c_int,
) -> Result<fs::File, HeptaError> {
    let expected_identity = target
        .target_identity
        .ok_or_else(|| HeptaError("sealed write target was not present at authorization".into()))?;
    let parent = open_sealed_parent(target, false)?.ok_or_else(|| {
        HeptaError("sealed write target parent disappeared after authorization".into())
    })?;
    let file = openat_file(
        &parent,
        &target.leaf_name,
        flags | libc::O_NOFOLLOW | libc::O_CLOEXEC,
        0,
    )
    .map_err(|error| {
        HeptaError(format!(
            "sealed write target changed after authorization: {}: {error}",
            target.canonical_path.display()
        ))
    })?;
    let metadata = file.metadata().map_err(|error| {
        HeptaError(format!(
            "failed to inspect sealed write target {}: {error}",
            target.canonical_path.display()
        ))
    })?;
    if !metadata.is_file() || file_identity(&metadata) != expected_identity {
        return Err(HeptaError(format!(
            "sealed write target identity changed after authorization: {}",
            target.canonical_path.display()
        )));
    }
    ensure_single_link_mutable_target(&metadata, &target.canonical_path)?;
    Ok(file)
}

#[cfg(unix)]
fn seal_write_target(
    workspace_root: &Path,
    resolved: ResolvedWriteTarget,
) -> Result<(SealedWriteTarget, Option<Vec<u8>>), HeptaError> {
    use std::io::Read as _;

    let canonical_anchor = resolved.canonical_anchor;
    let canonical_before_open = fs::canonicalize(&canonical_anchor).map_err(|error| {
        HeptaError(format!(
            "failed to revalidate write ancestor {}: {error}",
            canonical_anchor.display()
        ))
    })?;
    if normalize_path(canonical_before_open) != canonical_anchor {
        return Err(HeptaError(format!(
            "write ancestor changed before it could be sealed: {}",
            canonical_anchor.display()
        )));
    }
    let expected_anchor_metadata = fs::metadata(&canonical_anchor).map_err(|error| {
        HeptaError(format!(
            "failed to inspect write ancestor {}: {error}",
            canonical_anchor.display()
        ))
    })?;
    let anchor_identity = file_identity(&expected_anchor_metadata);
    let anchor_directory = fs::File::open(&canonical_anchor).map_err(|error| {
        HeptaError(format!(
            "failed to open write ancestor {}: {error}",
            canonical_anchor.display()
        ))
    })?;
    let opened_anchor_metadata = anchor_directory.metadata().map_err(|error| {
        HeptaError(format!(
            "failed to inspect write ancestor handle {}: {error}",
            canonical_anchor.display()
        ))
    })?;
    let canonical_after_open = fs::canonicalize(&canonical_anchor).map_err(|error| {
        HeptaError(format!(
            "failed to revalidate opened write ancestor {}: {error}",
            canonical_anchor.display()
        ))
    })?;
    let current_anchor_metadata = fs::metadata(&canonical_anchor).map_err(|error| {
        HeptaError(format!(
            "failed to inspect opened write ancestor path {}: {error}",
            canonical_anchor.display()
        ))
    })?;
    if !opened_anchor_metadata.is_dir()
        || normalize_path(canonical_after_open) != canonical_anchor
        || file_identity(&opened_anchor_metadata) != anchor_identity
        || file_identity(&current_anchor_metadata) != anchor_identity
    {
        return Err(HeptaError(format!(
            "write ancestor changed while being sealed: {}",
            canonical_anchor.display()
        )));
    }

    let mut sealed_target = SealedWriteTarget {
        workspace_root: workspace_root.to_path_buf(),
        canonical_path: resolved.canonical_path,
        canonical_anchor,
        missing_parent_components: resolved.missing_parent_components,
        leaf_name: resolved.leaf_name,
        anchor_identity,
        target_identity: None,
        namespace_case_insensitive: directory_namespace_case_insensitive(&anchor_directory),
        anchor_directory,
    };
    let before_bytes = if sealed_target.missing_parent_components.is_empty() {
        let parent = validate_sealed_anchor(&sealed_target)?;
        match openat_file(
            &parent,
            &sealed_target.leaf_name,
            libc::O_RDONLY | libc::O_NOFOLLOW | libc::O_CLOEXEC,
            0,
        ) {
            Ok(mut file) => {
                let metadata = file.metadata().map_err(|error| {
                    HeptaError(format!(
                        "failed to inspect write target {}: {error}",
                        sealed_target.canonical_path.display()
                    ))
                })?;
                if !metadata.is_file() {
                    return Err(HeptaError(format!(
                        "write target is not a regular file: {}",
                        sealed_target.canonical_path.display()
                    )));
                }
                ensure_single_link_mutable_target(&metadata, &sealed_target.canonical_path)?;
                sealed_target.target_identity = Some(file_identity(&metadata));
                let mut bytes = Vec::new();
                file.read_to_end(&mut bytes).map_err(|error| {
                    HeptaError(format!(
                        "failed to read {} before write transaction capture: {error}",
                        sealed_target.canonical_path.display()
                    ))
                })?;
                Some(bytes)
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => None,
            Err(error) => {
                return Err(HeptaError(format!(
                    "write target cannot be sealed without following a link: {}: {error}",
                    sealed_target.canonical_path.display()
                )));
            }
        }
    } else {
        let anchor = validate_sealed_anchor(&sealed_target)?;
        let first_missing = sealed_target
            .missing_parent_components
            .first()
            .ok_or_else(|| HeptaError("sealed write parent suffix disappeared".into()))?;
        if entry_exists_at(&anchor, first_missing)? {
            return Err(HeptaError(format!(
                "write parent changed while being sealed at {:?}",
                first_missing
            )));
        }
        None
    };
    Ok((sealed_target, before_bytes))
}

#[cfg(not(unix))]
fn seal_write_target(
    _workspace_root: &Path,
    _resolved: ResolvedWriteTarget,
) -> Result<(SealedWriteTarget, Option<Vec<u8>>), HeptaError> {
    Err(HeptaError(
        "identity-bound write reservations require Unix openat semantics".into(),
    ))
}

#[cfg(unix)]
fn verify_sealed_target_unchanged(
    target: &SealedWriteTarget,
    expected_before: Option<&[u8]>,
) -> Result<(), HeptaError> {
    use std::io::Read as _;
    use std::io::Seek as _;

    if target.target_identity.is_some() {
        let mut file = open_checked_existing_target(target, libc::O_RDONLY)?;
        file.seek(std::io::SeekFrom::Start(0)).map_err(|error| {
            HeptaError(format!(
                "failed to seek sealed write target {}: {error}",
                target.canonical_path.display()
            ))
        })?;
        let mut current = Vec::new();
        file.read_to_end(&mut current).map_err(|error| {
            HeptaError(format!(
                "failed to read sealed write target {}: {error}",
                target.canonical_path.display()
            ))
        })?;
        if Some(current.as_slice()) != expected_before {
            return Err(HeptaError(format!(
                "sealed write target contents changed after authorization: {}",
                target.canonical_path.display()
            )));
        }
        return Ok(());
    }

    let Some(parent) = open_sealed_parent(target, false)? else {
        return Ok(());
    };
    if entry_exists_at(&parent, &target.leaf_name)? {
        return Err(HeptaError(format!(
            "sealed write target appeared after authorization: {}",
            target.canonical_path.display()
        )));
    }
    Ok(())
}

#[cfg(unix)]
fn open_existing_target_for_atomic_replace(
    target: &SealedWriteTarget,
    flags: libc::c_int,
) -> Result<fs::File, HeptaError> {
    let expected_identity = target
        .target_identity
        .ok_or_else(|| HeptaError("sealed write target was not present at authorization".into()))?;
    let parent = open_sealed_parent(target, false)?.ok_or_else(|| {
        HeptaError("sealed write target parent disappeared after authorization".into())
    })?;
    let file = openat_file(
        &parent,
        &target.leaf_name,
        flags | libc::O_NOFOLLOW | libc::O_CLOEXEC,
        0,
    )
    .map_err(|error| {
        HeptaError(format!(
            "sealed write target changed after authorization: {}: {error}",
            target.canonical_path.display()
        ))
    })?;
    let metadata = file.metadata().map_err(|error| {
        HeptaError(format!(
            "failed to inspect sealed write target {}: {error}",
            target.canonical_path.display()
        ))
    })?;
    if !metadata.is_file() || file_identity(&metadata) != expected_identity {
        return Err(HeptaError(format!(
            "sealed write target identity changed after authorization: {}",
            target.canonical_path.display()
        )));
    }
    // A new hard link may appear after authorization. Atomic replacement does
    // not mutate that old inode, so it is safe to retain the outside alias with
    // its old bytes. In-place mutation is deliberately forbidden below.
    Ok(file)
}

#[cfg(unix)]
fn read_committed_sealed_target(
    target: &SealedWriteTarget,
) -> Result<(Vec<u8>, FileIdentity), HeptaError> {
    use std::io::Read as _;

    let parent = open_committed_sealed_parent(target)?;
    let mut file = openat_file(
        &parent,
        &target.leaf_name,
        libc::O_RDONLY | libc::O_NOFOLLOW | libc::O_CLOEXEC,
        0,
    )
    .map_err(|error| {
        HeptaError(format!(
            "failed to open committed sealed target {}: {error}",
            target.canonical_path.display()
        ))
    })?;
    let metadata = file.metadata().map_err(|error| {
        HeptaError(format!(
            "failed to inspect committed sealed target {}: {error}",
            target.canonical_path.display()
        ))
    })?;
    if !metadata.is_file() {
        return Err(HeptaError(format!(
            "committed sealed target is not a regular file: {}",
            target.canonical_path.display()
        )));
    }
    let mut content = Vec::new();
    file.read_to_end(&mut content).map_err(|error| {
        HeptaError(format!(
            "failed to read committed sealed target {}: {error}",
            target.canonical_path.display()
        ))
    })?;
    Ok((content, file_identity(&metadata)))
}

#[cfg(unix)]
fn committed_mutation_observed_after_error(
    prepared: &PreparedWriteTransaction,
) -> Result<bool, HeptaError> {
    let target = &prepared.sealed_target;
    let Some(parent) = open_sealed_parent(target, false)? else {
        return if prepared.target_existed_before {
            Err(HeptaError(
                "previously existing sealed target parent disappeared after tool error".into(),
            ))
        } else {
            Ok(false)
        };
    };
    if !entry_exists_at(&parent, &target.leaf_name)? {
        return if prepared.target_existed_before {
            Err(HeptaError(
                "previously existing sealed target disappeared after tool error".into(),
            ))
        } else {
            Ok(false)
        };
    }
    let (after, identity) = read_committed_sealed_target(target)?;
    Ok(!prepared.target_existed_before
        || prepared.before_bytes.as_deref() != Some(after.as_slice())
        || target.target_identity != Some(identity))
}

#[cfg(not(unix))]
fn read_committed_sealed_target(
    _target: &SealedWriteTarget,
) -> Result<(Vec<u8>, FileIdentity), HeptaError> {
    Err(HeptaError(
        "identity-bound mutation receipts require Unix openat semantics".into(),
    ))
}

#[cfg(not(unix))]
fn committed_mutation_observed_after_error(
    _prepared: &PreparedWriteTransaction,
) -> Result<bool, HeptaError> {
    Err(HeptaError(
        "identity-bound mutation observation requires Unix openat semantics".into(),
    ))
}

#[cfg(unix)]
fn verify_atomic_replace_source_unchanged(
    target: &SealedWriteTarget,
    expected_before: Option<&[u8]>,
) -> Result<(), HeptaError> {
    use std::io::Read as _;

    if target.target_identity.is_some() {
        let mut file = open_existing_target_for_atomic_replace(target, libc::O_RDONLY)?;
        let mut current = Vec::new();
        file.read_to_end(&mut current).map_err(|error| {
            HeptaError(format!(
                "failed to read sealed write target {}: {error}",
                target.canonical_path.display()
            ))
        })?;
        if expected_before != Some(current.as_slice()) {
            return Err(HeptaError(format!(
                "sealed write target contents changed after authorization: {}",
                target.canonical_path.display()
            )));
        }
        return Ok(());
    }

    let Some(parent) = open_sealed_parent(target, false)? else {
        return Ok(());
    };
    if entry_exists_at(&parent, &target.leaf_name)? {
        return Err(HeptaError(format!(
            "sealed write target appeared after authorization: {}",
            target.canonical_path.display()
        )));
    }
    Ok(())
}

#[cfg(not(unix))]
fn verify_atomic_replace_source_unchanged(
    _target: &SealedWriteTarget,
    _expected_before: Option<&[u8]>,
) -> Result<(), HeptaError> {
    Err(HeptaError(
        "identity-bound mutation source verification requires Unix openat semantics".into(),
    ))
}

#[cfg(unix)]
fn unlink_staging_entry(parent: &fs::File, name: &std::ffi::OsStr) {
    use std::os::fd::AsRawFd as _;

    let Ok(name) = os_component_cstring(name) else {
        return;
    };
    // SAFETY: `name` is NUL terminated and `parent` owns a valid descriptor.
    // Cleanup is best-effort and only targets the UUID staging leaf.
    unsafe {
        libc::unlinkat(parent.as_raw_fd(), name.as_ptr(), 0);
    }
}

#[cfg(unix)]
#[derive(Debug)]
enum StagedFileInstallOutcome {
    NotApplied(HeptaError),
    Applied,
    DurabilityAmbiguous(HeptaError),
}

#[cfg(all(unix, test))]
std::thread_local! {
    static FAIL_ATOMIC_INSTALL_AFTER_COMMIT: std::cell::Cell<bool> =
        const { std::cell::Cell::new(false) };
}

#[cfg(all(unix, test))]
fn inject_atomic_install_post_commit_failure_for_test() {
    FAIL_ATOMIC_INSTALL_AFTER_COMMIT.with(|fault| fault.set(true));
}

#[cfg(all(unix, test))]
fn take_atomic_install_post_commit_failure() -> bool {
    FAIL_ATOMIC_INSTALL_AFTER_COMMIT.with(|fault| fault.replace(false))
}

#[cfg(all(unix, not(test)))]
fn take_atomic_install_post_commit_failure() -> bool {
    false
}

#[cfg(unix)]
fn install_staged_file(
    parent: &fs::File,
    staging_name: &std::ffi::OsStr,
    target_name: &std::ffi::OsStr,
    replace_existing: bool,
) -> StagedFileInstallOutcome {
    use std::os::fd::AsRawFd as _;

    let staging_name = match os_component_cstring(staging_name) {
        Ok(name) => name,
        Err(error) => return StagedFileInstallOutcome::NotApplied(error),
    };
    let target_name = match os_component_cstring(target_name) {
        Ok(name) => name,
        Err(error) => return StagedFileInstallOutcome::NotApplied(error),
    };
    let result = if replace_existing {
        // SAFETY: both names are NUL terminated and both directory descriptors
        // are retained handles to the same sealed parent.
        unsafe {
            libc::renameat(
                parent.as_raw_fd(),
                staging_name.as_ptr(),
                parent.as_raw_fd(),
                target_name.as_ptr(),
            )
        }
    } else {
        // `linkat` gives creation no-replace semantics. The fully fsynced
        // staging inode becomes visible atomically only if the target is still
        // absent.
        // SAFETY: both names are NUL terminated and `parent` is valid.
        unsafe {
            libc::linkat(
                parent.as_raw_fd(),
                staging_name.as_ptr(),
                parent.as_raw_fd(),
                target_name.as_ptr(),
                0,
            )
        }
    };
    if result != 0 {
        return StagedFileInstallOutcome::NotApplied(HeptaError(format!(
            "sealed namespace changed before atomic install: {}",
            std::io::Error::last_os_error()
        )));
    }
    if !replace_existing {
        // The target now owns a second link to the staged inode; remove the
        // private staging name while retaining the installed target.
        let unlink_result = unsafe { libc::unlinkat(parent.as_raw_fd(), staging_name.as_ptr(), 0) };
        if unlink_result != 0 {
            return StagedFileInstallOutcome::DurabilityAmbiguous(HeptaError(format!(
                "failed to remove installed staging name: {}",
                std::io::Error::last_os_error()
            )));
        }
    }
    if take_atomic_install_post_commit_failure() {
        return StagedFileInstallOutcome::DurabilityAmbiguous(HeptaError(
            "injected failure after atomic install commit".into(),
        ));
    }
    match parent.sync_all() {
        Ok(()) => StagedFileInstallOutcome::Applied,
        Err(error) => StagedFileInstallOutcome::DurabilityAmbiguous(HeptaError(format!(
            "failed to fsync sealed parent after atomic install: {error}"
        ))),
    }
}

#[cfg(unix)]
fn write_sealed_target_atomically(
    target: &SealedWriteTarget,
    expected_before: Option<&[u8]>,
    mode: &str,
    content: &[u8],
) -> Result<(), HeptaError> {
    use std::io::Read as _;
    use std::io::Write as _;
    use std::os::unix::fs::MetadataExt as _;

    let target_existed = target.target_identity.is_some();
    if target_existed && mode == "create" {
        return Err(HeptaError(format!(
            "sealed create target already existed at authorization: {}",
            target.canonical_path.display()
        )));
    }
    if !matches!(mode, "create" | "overwrite" | "append") {
        return Err(HeptaError(format!(
            "unsupported sealed write mode {mode}: {}",
            target.canonical_path.display()
        )));
    }

    let (current, permissions) = if target_existed {
        let mut file = open_existing_target_for_atomic_replace(target, libc::O_RDONLY)?;
        let metadata = file.metadata().map_err(|error| {
            HeptaError(format!(
                "failed to inspect sealed write target {}: {error}",
                target.canonical_path.display()
            ))
        })?;
        let mut current = Vec::new();
        file.read_to_end(&mut current).map_err(|error| {
            HeptaError(format!(
                "failed to read sealed write target {}: {error}",
                target.canonical_path.display()
            ))
        })?;
        if expected_before != Some(current.as_slice()) {
            return Err(HeptaError(format!(
                "sealed write target contents changed after authorization: {}",
                target.canonical_path.display()
            )));
        }
        (current, (metadata.mode() & 0o7777) as libc::mode_t)
    } else {
        (Vec::new(), 0o600 as libc::mode_t)
    };

    let final_bytes = if mode == "append" {
        let mut final_bytes = current;
        final_bytes.extend_from_slice(content);
        final_bytes
    } else {
        content.to_vec()
    };
    let parent = open_sealed_parent(target, true)?
        .ok_or_else(|| HeptaError("sealed write parent could not be opened for staging".into()))?;
    let staging_name =
        std::ffi::OsString::from(format!(".hepta-stage-{}", uuid::Uuid::new_v4().simple()));
    let result = (|| {
        let mut staged = openat_file(
            &parent,
            &staging_name,
            libc::O_WRONLY | libc::O_CREAT | libc::O_EXCL | libc::O_NOFOLLOW | libc::O_CLOEXEC,
            permissions,
        )
        .map_err(|error| {
            HeptaError(format!(
                "failed to create atomic staging inode for {}: {error}",
                target.canonical_path.display()
            ))
        })?;
        staged.write_all(&final_bytes).map_err(|error| {
            HeptaError(format!(
                "failed to write atomic staging inode for {}: {error}",
                target.canonical_path.display()
            ))
        })?;
        staged.sync_all().map_err(|error| {
            HeptaError(format!(
                "failed to fsync atomic staging inode for {}: {error}",
                target.canonical_path.display()
            ))
        })?;

        if target_existed {
            let mut current_file = open_existing_target_for_atomic_replace(target, libc::O_RDONLY)?;
            let mut current_now = Vec::new();
            current_file
                .read_to_end(&mut current_now)
                .map_err(|error| {
                    HeptaError(format!(
                        "failed to revalidate sealed write target {}: {error}",
                        target.canonical_path.display()
                    ))
                })?;
            if expected_before != Some(current_now.as_slice()) {
                return Err(HeptaError(format!(
                    "sealed write target contents changed before atomic install: {}",
                    target.canonical_path.display()
                )));
            }
        } else if entry_exists_at(&parent, &target.leaf_name)? {
            return Err(HeptaError(format!(
                "sealed write target appeared before atomic install: {}",
                target.canonical_path.display()
            )));
        }

        match install_staged_file(&parent, &staging_name, &target.leaf_name, target_existed) {
            StagedFileInstallOutcome::Applied => Ok(()),
            StagedFileInstallOutcome::NotApplied(error) => Err(error),
            StagedFileInstallOutcome::DurabilityAmbiguous(error) => Err(HeptaError(format!(
                "mutation_durability_ambiguous: {}",
                error.0
            ))),
        }
    })();
    if result.is_err() {
        unlink_staging_entry(&parent, &staging_name);
    }
    result
}

#[cfg(unix)]
fn create_new_through_seal(
    target: &SealedWriteTarget,
    content: &[u8],
    append: bool,
) -> Result<(), HeptaError> {
    let mode = if append { "append" } else { "create" };

    write_sealed_target_atomically(target, None, mode, content)
}

#[cfg(unix)]
fn write_prepared_target(
    prepared: &PreparedWriteTransaction,
    mode: &str,
    content: &[u8],
) -> Result<(), HeptaError> {
    write_sealed_target_atomically(
        &prepared.sealed_target,
        prepared.before_bytes.as_deref(),
        mode,
        content,
    )
}

#[cfg(not(unix))]
fn verify_sealed_target_unchanged(
    _target: &SealedWriteTarget,
    _expected_before: Option<&[u8]>,
) -> Result<(), HeptaError> {
    Err(HeptaError(
        "identity-bound write reservations require Unix openat semantics".into(),
    ))
}

#[cfg(not(unix))]
fn write_prepared_target(
    _prepared: &PreparedWriteTransaction,
    _mode: &str,
    _content: &[u8],
) -> Result<(), HeptaError> {
    Err(HeptaError(
        "identity-bound write reservations require Unix openat semantics".into(),
    ))
}

fn mark_prepared_target_written(
    prepared: &mut PreparedWriteTransaction,
    content: Vec<u8>,
) -> Result<(), HeptaError> {
    #[cfg(unix)]
    {
        let parent = open_committed_sealed_parent(&prepared.sealed_target)?;
        let file = openat_file(
            &parent,
            &prepared.sealed_target.leaf_name,
            libc::O_RDONLY | libc::O_NOFOLLOW | libc::O_CLOEXEC,
            0,
        )
        .map_err(|error| {
            HeptaError(format!(
                "failed to inspect committed sealed target {}: {error}",
                prepared.target_path
            ))
        })?;
        let metadata = file.metadata().map_err(|error| {
            HeptaError(format!(
                "failed to inspect committed sealed target {}: {error}",
                prepared.target_path
            ))
        })?;
        if !metadata.is_file() {
            return Err(HeptaError(format!(
                "committed sealed target is not a regular file: {}",
                prepared.target_path
            )));
        }
        prepared.sealed_target.target_identity = Some(file_identity(&metadata));
        prepared.target_existed_before = true;
        prepared.before_bytes = Some(content);
        Ok(())
    }
    #[cfg(not(unix))]
    {
        let _ = (prepared, content);
        Err(HeptaError(
            "identity-bound write reservations require Unix openat semantics".into(),
        ))
    }
}

fn delete_prepared_target(prepared: &mut PreparedWriteTransaction) -> Result<(), HeptaError> {
    #[cfg(unix)]
    {
        use std::os::fd::AsRawFd as _;

        verify_sealed_target_unchanged(&prepared.sealed_target, prepared.before_bytes.as_deref())?;
        let parent = open_sealed_parent(&prepared.sealed_target, false)?.ok_or_else(|| {
            HeptaError("sealed delete target parent disappeared after authorization".into())
        })?;
        let _target = open_checked_existing_target(&prepared.sealed_target, libc::O_RDONLY)?;
        let leaf_name = os_component_cstring(&prepared.sealed_target.leaf_name)?;
        // SAFETY: `leaf_name` is NUL terminated and `parent` owns a valid
        // descriptor. With flags=0, unlinkat removes the directory entry and
        // never follows a symlink target.
        let result = unsafe { libc::unlinkat(parent.as_raw_fd(), leaf_name.as_ptr(), 0) };
        if result != 0 {
            return Err(HeptaError(format!(
                "failed to delete sealed target {}: {}",
                prepared.target_path,
                std::io::Error::last_os_error()
            )));
        }
        prepared.sealed_target.target_identity = None;
        prepared.target_existed_before = false;
        prepared.before_bytes = None;
        Ok(())
    }
    #[cfg(not(unix))]
    {
        let _ = prepared;
        Err(HeptaError(
            "identity-bound write reservations require Unix openat semantics".into(),
        ))
    }
}

fn read_existing_file_within_root(root: &Path, source_path: &Path) -> Result<Vec<u8>, HeptaError> {
    let canonical_root = fs::canonicalize(root).map_err(|error| {
        HeptaError(format!(
            "failed to canonicalize internal read root {}: {error}",
            root.display()
        ))
    })?;
    let resolved = resolve_write_path_within_root(&canonical_root, source_path)?;
    if !resolved.canonical_path.starts_with(&canonical_root) {
        return Err(HeptaError(format!(
            "internal read path escapes canonical root: {}",
            resolved.canonical_path.display()
        )));
    }
    let (sealed, before_bytes) = seal_write_target(&canonical_root, resolved)?;
    before_bytes.ok_or_else(|| {
        HeptaError(format!(
            "internal read target is missing: {}",
            sealed.canonical_path.display()
        ))
    })
}

fn write_new_file_within_root(
    root: &Path,
    target_path: &Path,
    content: &[u8],
) -> Result<(), HeptaError> {
    let canonical_root = fs::canonicalize(root).map_err(|error| {
        HeptaError(format!(
            "failed to canonicalize internal write root {}: {error}",
            root.display()
        ))
    })?;
    let resolved = resolve_write_path_within_root(&canonical_root, target_path)?;
    if !resolved.canonical_path.starts_with(&canonical_root) {
        return Err(HeptaError(format!(
            "internal write path escapes canonical root: {}",
            resolved.canonical_path.display()
        )));
    }
    let (sealed, before_bytes) = seal_write_target(&canonical_root, resolved)?;
    if before_bytes.is_some() {
        return Err(HeptaError(format!(
            "internal write refuses to replace existing path {}",
            sealed.canonical_path.display()
        )));
    }
    #[cfg(unix)]
    {
        create_new_through_seal(&sealed, content, false)
    }
    #[cfg(not(unix))]
    {
        let _ = (sealed, content);
        Err(HeptaError(
            "identity-bound write reservations require Unix openat semantics".into(),
        ))
    }
}

fn prepared_native_target<'a>(
    prepared: &'a [PreparedWriteTransaction],
    requested_path: &str,
    operation: &str,
) -> Result<&'a PreparedWriteTransaction, hepta_core::ToolError> {
    let mut matches = prepared.iter().filter(|transaction| {
        transaction.requested_path == requested_path && transaction.operation == operation
    });
    let transaction = matches.next().ok_or_else(|| {
        hepta_core::ToolError(format!(
            "{} request no longer matches an identity-bound reservation for {}",
            operation, requested_path
        ))
    })?;
    if matches.next().is_some() {
        return Err(hepta_core::ToolError(format!(
            "{} has duplicate identity-bound reservations for {}",
            operation, requested_path
        )));
    }
    Ok(transaction)
}

fn parsed_native_patch(
    input_json: &str,
) -> Result<(bool, Vec<NativePatchOp>), hepta_core::ToolError> {
    let input = parse_tool_input_object(input_json)?;
    let patch = input
        .get("input")
        .or_else(|| input.get("patch"))
        .and_then(Value::as_str)
        .ok_or_else(|| {
            hepta_core::ToolError("apply_patch requires string field 'input' or 'patch'".into())
        })?;
    Ok((
        native_mutation_preview_only(input_json)?,
        parse_native_apply_patch(patch)?,
    ))
}

fn preflight_prepared_native_mutation(
    tool: &str,
    prepared: &[PreparedWriteTransaction],
    input_json: &str,
) -> Result<(), hepta_core::ToolError> {
    let preview_only = native_mutation_preview_only(input_json)?;
    match tool {
        "write" | "edit" => {
            let requested_path = parse_required_string_field(input_json, "path")?;
            let operation = if tool == "write" {
                "native_write"
            } else {
                "native_edit"
            };
            let transaction = prepared_native_target(prepared, &requested_path, operation)?;
            if prepared.len() != 1 || transaction.preview_only != preview_only {
                return Err(hepta_core::ToolError(format!(
                    "{} request no longer matches its sealed reservation",
                    tool
                )));
            }
            if tool == "write" {
                parse_required_string_field(input_json, "content")?;
            } else {
                let input = parse_tool_input_object(input_json)?;
                let edits = input
                    .get("edits")
                    .and_then(Value::as_array)
                    .ok_or_else(|| {
                        hepta_core::ToolError("edit requires array field 'edits'".into())
                    })?;
                for edit in edits {
                    edit.get("oldText")
                        .or_else(|| edit.get("old_text"))
                        .and_then(Value::as_str)
                        .ok_or_else(|| {
                            hepta_core::ToolError("each edit requires oldText".into())
                        })?;
                    edit.get("newText")
                        .or_else(|| edit.get("new_text"))
                        .and_then(Value::as_str)
                        .ok_or_else(|| {
                            hepta_core::ToolError("each edit requires newText".into())
                        })?;
                }
            }
        }
        "apply_patch" => {
            let (parsed_preview, operations) = parsed_native_patch(input_json)?;
            if !parsed_preview && operations.len() > 1 {
                return Err(hepta_core::ToolError(
                    "identity-bound apply_patch refuses non-preview multi-operation patches".into(),
                ));
            }
            if parsed_preview != preview_only || operations.len() != prepared.len() {
                return Err(hepta_core::ToolError(
                    "apply_patch request no longer matches its identity-bound sealed reservation set"
                        .into(),
                ));
            }
            for operation in operations {
                let (path, expected_operation) = match operation {
                    NativePatchOp::Add { path, .. } => (path, "native_patch_add"),
                    NativePatchOp::Update { path, .. } => (path, "native_patch_update"),
                    NativePatchOp::Delete { path } => {
                        return Err(hepta_core::ToolError(format!(
                            "identity-bound apply_patch refuses delete operation for {}",
                            path
                        )));
                    }
                };
                let transaction = prepared_native_target(prepared, &path, expected_operation)?;
                if transaction.preview_only != preview_only {
                    return Err(hepta_core::ToolError(
                        "apply_patch preview mode changed after reservation".into(),
                    ));
                }
            }
        }
        "tts" => {
            parse_required_string_field(input_json, "text")?;
            let (requested_path, _) =
                native_tts_explicit_output_path(input_json)?.ok_or_else(|| {
                    hepta_core::ToolError(
                        "tts requires an explicit path or filename for identity-bound execution"
                            .into(),
                    )
                })?;
            let transaction = prepared_native_target(prepared, &requested_path, "native_tts")?;
            if prepared.len() != 1 || transaction.preview_only != preview_only {
                return Err(hepta_core::ToolError(
                    "tts request no longer matches its sealed reservation".into(),
                ));
            }
        }
        _ => {
            return Err(hepta_core::ToolError(format!(
                "{} is not an identity-bound native mutation",
                tool
            )));
        }
    }
    for transaction in prepared {
        verify_atomic_replace_source_unchanged(
            &transaction.sealed_target,
            transaction.before_bytes.as_deref(),
        )
        .map_err(|error| hepta_core::ToolError(error.0))?;
    }
    Ok(())
}

fn native_mutation_result(
    output: serde_json::Map<String, Value>,
) -> Result<ToolResult, hepta_core::ToolError> {
    let content = output
        .get("content")
        .and_then(Value::as_str)
        .unwrap_or("native mutation completed")
        .to_string();
    Ok(ToolResult {
        content,
        structured_json: Some(Value::Object(output).to_string()),
    })
}

fn invoke_prepared_native_mutation(
    tool: &str,
    prepared: &[PreparedWriteTransaction],
    input_json: &str,
    provider_identity: &ProviderExecutionIdentity,
) -> Result<ToolResult, hepta_core::ToolError> {
    let _provider_identity = provider_identity;
    preflight_prepared_native_mutation(tool, prepared, input_json)?;
    match tool {
        "write" => invoke_prepared_native_write(&prepared[0], input_json),
        "edit" => invoke_prepared_native_edit(&prepared[0], input_json),
        "apply_patch" => invoke_prepared_native_patch(prepared, input_json),
        "tts" => invoke_prepared_native_tts(&prepared[0], input_json),
        _ => Err(hepta_core::ToolError(format!(
            "{} is not an identity-bound native mutation",
            tool
        ))),
    }
}

fn invoke_prepared_native_write(
    prepared: &PreparedWriteTransaction,
    input_json: &str,
) -> Result<ToolResult, hepta_core::ToolError> {
    let content = parse_required_string_field(input_json, "content")?;
    let preview_only = native_mutation_preview_only(input_json)?;
    if !preview_only {
        write_prepared_target(prepared, &prepared.mode_requested, content.as_bytes())
            .map_err(|error| hepta_core::ToolError(error.0))?;
    }
    let bytes_after = if prepared.mode_requested == "append" {
        prepared
            .before_bytes
            .as_ref()
            .map(Vec::len)
            .unwrap_or_default()
            .saturating_add(content.len())
    } else {
        content.len()
    };
    let mut output = native_compat_base("write", if preview_only { "preview" } else { "ok" });
    output.insert("bytes_after".into(), json!(bytes_after));
    output.insert(
        "content".into(),
        Value::String(format!(
            "{} {} bytes to {}{}",
            if preview_only { "would write" } else { "wrote" },
            content.len(),
            prepared.target_path,
            if prepared.target_existed_before {
                " (overwrote existing file)"
            } else {
                ""
            }
        )),
    );
    output.insert(
        "result".into(),
        json!({
            "path": prepared.target_path,
            "bytes": content.len(),
            "existed_before": prepared.target_existed_before,
            "preview_only": preview_only,
            "identity_bound": true
        }),
    );
    native_mutation_result(output)
}

fn invoke_prepared_native_edit(
    prepared: &PreparedWriteTransaction,
    input_json: &str,
) -> Result<ToolResult, hepta_core::ToolError> {
    let input = parse_tool_input_object(input_json)?;
    let edits = input
        .get("edits")
        .and_then(Value::as_array)
        .ok_or_else(|| hepta_core::ToolError("edit requires array field 'edits'".into()))?;
    let mut content = String::from_utf8(prepared.before_bytes.clone().unwrap_or_default())
        .map_err(|error| {
            hepta_core::ToolError(format!(
                "failed to read existing content from {} as UTF-8: {error}",
                prepared.target_path
            ))
        })?;
    for edit in edits {
        let old_text = edit
            .get("oldText")
            .or_else(|| edit.get("old_text"))
            .and_then(Value::as_str)
            .ok_or_else(|| hepta_core::ToolError("each edit requires oldText".into()))?;
        let new_text = edit
            .get("newText")
            .or_else(|| edit.get("new_text"))
            .and_then(Value::as_str)
            .ok_or_else(|| hepta_core::ToolError("each edit requires newText".into()))?;
        let count = content.matches(old_text).count();
        if count != 1 {
            return Err(hepta_core::ToolError(format!(
                "oldText must match exactly once; matched {} times",
                count
            )));
        }
        content = content.replacen(old_text, new_text, 1);
    }
    let preview_only = native_mutation_preview_only(input_json)?;
    if !preview_only {
        write_prepared_target(prepared, "overwrite", content.as_bytes())
            .map_err(|error| hepta_core::ToolError(error.0))?;
    }
    let mut output = native_compat_base("edit", if preview_only { "preview" } else { "ok" });
    output.insert("bytes_after".into(), json!(content.len()));
    output.insert(
        "content".into(),
        Value::String(format!(
            "{} {} edit(s) in {}",
            if preview_only {
                "would apply"
            } else {
                "applied"
            },
            edits.len(),
            prepared.target_path
        )),
    );
    output.insert(
        "result".into(),
        json!({
            "path": prepared.target_path,
            "edits_applied": edits.len(),
            "preview_only": preview_only,
            "identity_bound": true
        }),
    );
    native_mutation_result(output)
}

fn invoke_prepared_native_patch(
    prepared: &[PreparedWriteTransaction],
    input_json: &str,
) -> Result<ToolResult, hepta_core::ToolError> {
    let (preview_only, operations) = parsed_native_patch(input_json)?;
    let mut planned = Vec::with_capacity(operations.len());
    for operation in operations {
        match operation {
            NativePatchOp::Add { path, content } => {
                let transaction = prepared_native_target(prepared, &path, "native_patch_add")?;
                planned.push((transaction, content, "add", 0usize));
            }
            NativePatchOp::Update { path, old, new } => {
                let transaction = prepared_native_target(prepared, &path, "native_patch_update")?;
                let current =
                    std::str::from_utf8(transaction.before_bytes.as_deref().unwrap_or_default())
                        .map_err(|error| {
                            hepta_core::ToolError(format!(
                                "failed to read {} as UTF-8: {error}",
                                transaction.target_path
                            ))
                        })?;
                let count = current.matches(&old).count();
                if count != 1 {
                    return Err(hepta_core::ToolError(format!(
                        "patch update for {} matched old hunk {} times; expected exactly once",
                        transaction.target_path, count
                    )));
                }
                planned.push((
                    transaction,
                    current.replacen(&old, &new, 1),
                    "update",
                    old.len(),
                ));
            }
            NativePatchOp::Delete { path } => {
                return Err(hepta_core::ToolError(format!(
                    "identity-bound apply_patch refuses delete operation for {}",
                    path
                )));
            }
        }
    }
    if !preview_only {
        for (transaction, content, operation, _) in &planned {
            let mode = if *operation == "add" {
                "create"
            } else {
                "overwrite"
            };
            write_prepared_target(transaction, mode, content.as_bytes())
                .map_err(|error| hepta_core::ToolError(error.0))?;
        }
    }
    let summaries = planned
        .iter()
        .map(|(transaction, content, operation, old_bytes)| {
            if *operation == "add" {
                json!({"op":"add","path":transaction.target_path,"bytes":content.len()})
            } else {
                json!({"op":"update","path":transaction.target_path,"old_bytes":old_bytes,"new_bytes":content.len()})
            }
        })
        .collect::<Vec<_>>();
    let bytes_after = planned
        .iter()
        .map(|(_, content, _, _)| content.len())
        .sum::<usize>();
    let mut output = native_compat_base("apply_patch", if preview_only { "preview" } else { "ok" });
    output.insert("bytes_after".into(), json!(bytes_after));
    output.insert(
        "content".into(),
        Value::String(format!(
            "{} {} patch operation(s)",
            if preview_only {
                "would apply"
            } else {
                "applied"
            },
            summaries.len()
        )),
    );
    output.insert(
        "result".into(),
        json!({
            "operation_count": summaries.len(),
            "operations": summaries,
            "preview_only": preview_only,
            "identity_bound": true
        }),
    );
    native_mutation_result(output)
}

fn invoke_prepared_native_tts(
    prepared: &PreparedWriteTransaction,
    input_json: &str,
) -> Result<ToolResult, hepta_core::ToolError> {
    let text = parse_required_string_field(input_json, "text")?;
    let preview_only = native_mutation_preview_only(input_json)?;
    if preview_only {
        let mut output = native_compat_base("tts", "preview");
        output.insert(
            "content".into(),
            Value::String(format!(
                "would synthesize {} chars to {}",
                text.chars().count(),
                prepared.target_path
            )),
        );
        output.insert(
            "result".into(),
            json!({
                "path": prepared.target_path,
                "chars": text.chars().count(),
                "dryRun": true,
                "identity_bound": true
            }),
        );
        return native_mutation_result(output);
    }
    let audio = prepared.staged_after_bytes.as_deref().ok_or_else(|| {
        hepta_core::ToolError(
            "live tts provider lacks the exact audio bytes staged before durable intent".into(),
        )
    })?;
    write_prepared_target(prepared, &prepared.mode_requested, audio)
        .map_err(|error| hepta_core::ToolError(error.0))?;
    let mut result = native_compat_base("tts", "ok");
    result.insert("bytes_after".into(), json!(audio.len()));
    result.insert(
        "content".into(),
        Value::String(format!("synthesized speech to {}", prepared.target_path)),
    );
    result.insert(
        "result".into(),
        json!({
            "path": prepared.target_path,
            "chars": text.chars().count(),
            "format": "aiff",
            "synthesis_staged_before_intent": true,
            "provider_installed_staged_bytes": true,
            "live_adapter_invoked": true,
            "identity_bound": true
        }),
    );
    native_mutation_result(result)
}

#[cfg(target_os = "macos")]
fn stage_native_tts_audio(input_json: &str) -> Result<Vec<u8>, hepta_core::ToolError> {
    let text = parse_required_string_field(input_json, "text")?;
    let staging = tempfile::Builder::new()
        .prefix("hepta-tts-stage-")
        .tempdir()
        .map_err(|error| {
            hepta_core::ToolError(format!(
                "failed to create private TTS staging directory: {error}"
            ))
        })?;
    let staged_output = staging.path().join("speech.aiff");
    let output = std::process::Command::new("/usr/bin/say")
        .arg("-o")
        .arg(&staged_output)
        .arg("--")
        .arg(&text)
        .output()
        .map_err(|error| hepta_core::ToolError(format!("failed to run macOS say: {error}")))?;
    if !output.status.success() {
        return Err(hepta_core::ToolError(format!(
            "macOS say failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )));
    }
    fs::read(&staged_output).map_err(|error| {
        hepta_core::ToolError(format!(
            "failed to read staged TTS output {}: {error}",
            staged_output.display()
        ))
    })
}

#[cfg(not(target_os = "macos"))]
fn stage_native_tts_audio(_input_json: &str) -> Result<Vec<u8>, hepta_core::ToolError> {
    Err(hepta_core::ToolError(
        "live tts requires the macOS say adapter; no target effect was attempted".into(),
    ))
}

fn invoke_prepared_write_file(
    prepared: &PreparedWriteTransaction,
    input_json: &str,
    provider_identity: &ProviderExecutionIdentity,
) -> Result<ToolResult, hepta_core::ToolError> {
    let _provider_identity = provider_identity;
    let requested_path = parse_required_string_field(input_json, "path")?;
    let content = parse_required_string_field(input_json, "content")?;
    let mode =
        parse_optional_string_field(input_json, "mode")?.unwrap_or_else(|| "create".to_string());
    let preview_only = parse_optional_bool_field(input_json, "preview_only")?.unwrap_or(false);
    if requested_path != prepared.requested_path
        || mode != prepared.mode_requested
        || preview_only != prepared.preview_only
    {
        return Err(hepta_core::ToolError(
            "write_file request no longer matches its sealed reservation".into(),
        ));
    }
    if !matches!(mode.as_str(), "create" | "overwrite" | "append") {
        return Err(hepta_core::ToolError(format!(
            "unsupported write mode {} for {}",
            mode, prepared.target_path
        )));
    }

    let before_bytes = prepared.before_bytes.as_deref().unwrap_or_default();
    let before_text = std::str::from_utf8(before_bytes).map_err(|error| {
        hepta_core::ToolError(format!(
            "failed to read existing content from {} as UTF-8: {error}",
            prepared.target_path
        ))
    })?;
    let after_content = match mode.as_str() {
        "append" => format!("{before_text}{content}"),
        "create" | "overwrite" => content.clone(),
        _ => {
            return Err(hepta_core::ToolError(format!(
                "unsupported write mode {} for {}",
                mode, prepared.target_path
            )));
        }
    };
    let bytes_before = before_text.len();
    let bytes_after = after_content.len();
    let content_changed = before_text != after_content;
    let backup_planned = prepared.target_existed_before && mode == "overwrite";
    let change_summary = summarize_write_change(
        mode.as_str(),
        prepared.target_existed_before,
        content_changed,
        bytes_before,
        bytes_after,
    );

    if preview_only {
        verify_sealed_target_unchanged(&prepared.sealed_target, prepared.before_bytes.as_deref())
            .map_err(|error| hepta_core::ToolError(error.0))?;
        let mut output = serde_json::Map::new();
        output.insert("path".into(), json!(prepared.target_path));
        output.insert("bytes_written".into(), json!(0));
        output.insert("mode_requested".into(), json!(mode.clone()));
        output.insert("mode_applied".into(), json!(mode.clone()));
        output.insert(
            "existed_before".into(),
            json!(prepared.target_existed_before),
        );
        output.insert("preview_only".into(), json!(true));
        output.insert("content_changed".into(), json!(content_changed));
        output.insert("bytes_before".into(), json!(bytes_before));
        output.insert("bytes_after".into(), json!(bytes_after));
        output.insert("backup_planned".into(), json!(backup_planned));
        output.insert("backup_created".into(), json!(false));
        if backup_planned {
            let backup_path = preview_backup_path(
                &prepared.sealed_target.workspace_root,
                Path::new(&prepared.target_path),
            )?;
            output.insert(
                "backup_path".into(),
                json!(backup_path.display().to_string()),
            );
        }
        output.insert("change_summary".into(), json!(change_summary.clone()));
        return Ok(ToolResult {
            content: format!(
                "write_file:{} => preview {}",
                prepared.target_path, change_summary
            ),
            structured_json: Some(Value::Object(output).to_string()),
        });
    }

    verify_sealed_target_unchanged(&prepared.sealed_target, prepared.before_bytes.as_deref())
        .map_err(|error| hepta_core::ToolError(error.0))?;
    let mut backup_path = None;
    if backup_planned {
        let planned = preview_backup_path(
            &prepared.sealed_target.workspace_root,
            Path::new(&prepared.target_path),
        )?;
        write_new_file_within_root(
            &prepared.sealed_target.workspace_root,
            &planned,
            before_bytes,
        )
        .map_err(|error| hepta_core::ToolError(error.0))?;
        backup_path = Some(planned);
    }
    write_prepared_target(prepared, mode.as_str(), content.as_bytes())
        .map_err(|error| hepta_core::ToolError(error.0))?;

    let mut output = serde_json::Map::new();
    output.insert("path".into(), json!(prepared.target_path));
    output.insert("bytes_written".into(), json!(content.len()));
    output.insert("mode_requested".into(), json!(mode.clone()));
    output.insert("mode_applied".into(), json!(mode.clone()));
    output.insert(
        "existed_before".into(),
        json!(prepared.target_existed_before),
    );
    output.insert("preview_only".into(), json!(false));
    output.insert("content_changed".into(), json!(content_changed));
    output.insert("bytes_before".into(), json!(bytes_before));
    output.insert("bytes_after".into(), json!(bytes_after));
    output.insert("backup_planned".into(), json!(backup_planned));
    output.insert("backup_created".into(), json!(backup_path.is_some()));
    if let Some(backup_path) = backup_path.as_ref() {
        output.insert(
            "backup_path".into(),
            json!(backup_path.display().to_string()),
        );
    }
    output.insert("change_summary".into(), json!(change_summary));
    Ok(ToolResult {
        content: format!(
            "write_file:{} => {} bytes ({})",
            prepared.target_path,
            content.len(),
            mode
        ),
        structured_json: Some(Value::Object(output).to_string()),
    })
}

fn validate_provider_execution_identity(
    expected_attempt_id: &str,
    expected_idempotency_key: &str,
    presented_attempt_id: &str,
    presented_idempotency_key: &str,
) -> Result<(), hepta_core::ToolError> {
    uuid::Uuid::parse_str(expected_attempt_id).map_err(|_| {
        hepta_core::ToolError("provider execution identity has an invalid execution attempt".into())
    })?;
    let prefix = format!("hepta-execution:{expected_attempt_id}:sha256:");
    let Some(digest) = expected_idempotency_key.strip_prefix(&prefix) else {
        return Err(hepta_core::ToolError(
            "expected provider idempotency key is not bound to its execution attempt".into(),
        ));
    };
    if digest.len() != 64
        || !digest
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(hepta_core::ToolError(
            "expected provider idempotency key has an invalid canonical digest".into(),
        ));
    }
    if presented_attempt_id != expected_attempt_id
        || presented_idempotency_key != expected_idempotency_key
    {
        return Err(hepta_core::ToolError(
            "presented provider execution identity differs from the staged execution intent".into(),
        ));
    }
    Ok(())
}

fn normalize_path(path: PathBuf) -> PathBuf {
    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                normalized.pop();
            }
            Component::RootDir | Component::Prefix(_) | Component::Normal(_) => {
                normalized.push(component.as_os_str());
            }
        }
    }
    normalized
}

fn paths_overlap(left: &Path, right: &Path) -> bool {
    let normalized_left = normalize_path(left.to_path_buf());
    let normalized_right = normalize_path(right.to_path_buf());
    normalized_left == normalized_right
        || normalized_left.starts_with(&normalized_right)
        || normalized_right.starts_with(&normalized_left)
}

fn captured_transaction_failure(
    error: HeptaError,
    output_value: &Value,
    transaction_id: &str,
    group_id: Option<&str>,
    entry_hash: &::hepta_contracts::ContentHash,
) -> runtime_kernel::execution_bus::CapturedTransactionResult {
    runtime_kernel::execution_bus::CapturedTransactionResult {
        final_output_json: Some(output_value.to_string()),
        evidence: runtime_kernel::execution_bus::CapturedTransaction::Failed {
            error: error.0,
            transaction_id: Some(transaction_id.to_string()),
            group_id: group_id.map(str::to_string),
            entry_hash: Some(entry_hash.clone()),
        },
    }
}

const ROLLBACK_GROUP_STATUS_SCHEMA_VERSION: u32 = 1;
const WRITE_LOCK_REPORT_SCHEMA_VERSION: u32 = 1;
const ROLLBACK_EVENT_PAYLOAD_SCHEMA_VERSION: u32 = 1;
