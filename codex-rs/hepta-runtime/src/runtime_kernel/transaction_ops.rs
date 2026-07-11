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
        transactions.sort_by(|left, right| right.created_at_unix_ms.cmp(&left.created_at_unix_ms));
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
        groups.sort_by(|left, right| right.opened_at_unix_ms.cmp(&left.opened_at_unix_ms));
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
        attempts.sort_by(|left, right| right.started_at_unix_ms.cmp(&left.started_at_unix_ms));
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

    fn find_conflicting_target_lock(
        &self,
        target_path: &str,
        allowed_owner_id: Option<&str>,
    ) -> Result<Option<WriteTargetLock>, HeptaError> {
        self.prune_stale_write_locks_internal(false)?;
        let normalized_target_path = normalize_path(PathBuf::from(target_path));
        let guard = self
            .write_lock_state
            .lock()
            .map_err(|_| HeptaError("write lock state mutex poisoned".into()))?;
        Ok(guard
            .target_locks
            .iter()
            .find(|lock| {
                paths_overlap(
                    Path::new(&lock.target_path),
                    normalized_target_path.as_path(),
                ) && allowed_owner_id
                    .map(|allowed_owner_id| lock.owner_id != allowed_owner_id)
                    .unwrap_or(true)
            })
            .cloned())
    }

    fn ensure_write_target_unlocked(
        &self,
        session_id: &str,
        target_path: &str,
        operation: &str,
    ) -> Result<(), HeptaError> {
        if let Some(lock) = self.find_conflicting_target_lock(target_path, None)? {
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
        Ok(())
    }

    fn acquire_group_rollback_locks(
        &self,
        session_id: &str,
        group_id: &str,
        attempt_id: &str,
        target_paths: &[String],
    ) -> Result<(), HeptaError> {
        let locked_at_unix_ms = current_unix_ms()?;
        let lease_expires_at_unix_ms = locked_at_unix_ms.saturating_add(WRITE_LOCK_LEASE_MS);
        self.prune_stale_write_locks_internal(false)?;
        {
            let mut guard = self
                .write_lock_state
                .lock()
                .map_err(|_| HeptaError("write lock state mutex poisoned".into()))?;
            for target_path in target_paths {
                let normalized_target_path = normalize_path(PathBuf::from(target_path));
                if let Some(conflict) = guard
                    .target_locks
                    .iter()
                    .find(|lock| {
                        lock.owner_id != group_id
                            && paths_overlap(
                                Path::new(&lock.target_path),
                                normalized_target_path.as_path(),
                            )
                    })
                    .cloned()
                {
                    let message = format!(
                        "write lock blocks rollback_group for {} (owner={} {})",
                        target_path, conflict.owner_kind, conflict.owner_id
                    );
                    drop(guard);
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
            }
            if !guard
                .group_locks
                .iter()
                .any(|lock| lock.session_id == session_id && lock.group_id == group_id)
            {
                guard.group_locks.push(WriteGroupLock {
                    session_id: session_id.to_string(),
                    group_id: group_id.to_string(),
                    owner_kind: "rollback_group".into(),
                    owner_id: attempt_id.to_string(),
                    rollback_attempt_id: Some(attempt_id.to_string()),
                    locked_at_unix_ms,
                    lease_expires_at_unix_ms,
                });
            } else if let Some(lock) = guard
                .group_locks
                .iter_mut()
                .find(|lock| lock.session_id == session_id && lock.group_id == group_id)
            {
                lock.owner_kind = "rollback_group".into();
                lock.owner_id = attempt_id.to_string();
                lock.rollback_attempt_id = Some(attempt_id.to_string());
                lock.locked_at_unix_ms = locked_at_unix_ms;
                lock.lease_expires_at_unix_ms = lease_expires_at_unix_ms;
            }
            for target_path in target_paths {
                let normalized_target_path = normalize_path(PathBuf::from(target_path));
                if !guard.target_locks.iter().any(|lock| {
                    lock.owner_id == group_id
                        && paths_overlap(
                            Path::new(&lock.target_path),
                            normalized_target_path.as_path(),
                        )
                }) {
                    guard.target_locks.push(WriteTargetLock {
                        session_id: session_id.to_string(),
                        target_path: normalized_target_path.display().to_string(),
                        owner_kind: "rollback_group".into(),
                        owner_id: group_id.to_string(),
                        rollback_group_id: Some(group_id.to_string()),
                        rollback_attempt_id: Some(attempt_id.to_string()),
                        locked_at_unix_ms,
                        lease_expires_at_unix_ms,
                    });
                } else if let Some(lock) = guard.target_locks.iter_mut().find(|lock| {
                    lock.owner_id == group_id
                        && paths_overlap(
                            Path::new(&lock.target_path),
                            normalized_target_path.as_path(),
                        )
                }) {
                    lock.owner_kind = "rollback_group".into();
                    lock.rollback_group_id = Some(group_id.to_string());
                    lock.rollback_attempt_id = Some(attempt_id.to_string());
                    lock.locked_at_unix_ms = locked_at_unix_ms;
                    lock.lease_expires_at_unix_ms = lease_expires_at_unix_ms;
                }
            }
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
        Ok(())
    }

    fn release_group_rollback_locks(
        &self,
        session_id: &str,
        group_id: &str,
    ) -> Result<(), HeptaError> {
        let (released_group_locks, released_target_locks) = {
            {
                let mut guard = self
                    .write_lock_state
                    .lock()
                    .map_err(|_| HeptaError("write lock state mutex poisoned".into()))?;
                let released_group_locks = guard
                    .group_locks
                    .iter()
                    .filter(|lock| lock.session_id == session_id && lock.group_id == group_id)
                    .count();
                let released_target_locks = guard
                    .target_locks
                    .iter()
                    .filter(|lock| lock.session_id == session_id && lock.owner_id == group_id)
                    .count();
                guard
                    .group_locks
                    .retain(|lock| !(lock.session_id == session_id && lock.group_id == group_id));
                guard
                    .target_locks
                    .retain(|lock| !(lock.session_id == session_id && lock.owner_id == group_id));
                (released_group_locks, released_target_locks)
            }
        };
        self.emit_event_with_payload(
            EventKind::WriteLocksReleased,
            Some(SessionId(session_id.to_string())),
            None,
            format!("released write locks for group {}", group_id),
            Some(json!({
                "schema_version": ROLLBACK_EVENT_PAYLOAD_SCHEMA_VERSION,
                "group_id": group_id,
                "released_group_locks": released_group_locks,
                "released_target_locks": released_target_locks,
            })),
        )?;
        Ok(())
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

    fn prepare_write_transaction(
        &self,
        tool_name: &str,
        input_json: &str,
    ) -> Result<Option<PreparedWriteTransaction>, HeptaError> {
        if tool_name != "write_file" {
            return Ok(None);
        }

        let requested_path =
            parse_required_string_field(input_json, "path").map_err(|err| HeptaError(err.0))?;
        let mode_requested = parse_optional_string_field(input_json, "mode")
            .map_err(|err| HeptaError(err.0))?
            .unwrap_or_else(|| "create".to_string());
        let preview_only = parse_optional_bool_field(input_json, "preview_only")
            .map_err(|err| HeptaError(err.0))?
            .unwrap_or(false);
        let workspace_root = self.workspace_root()?;
        let target_path = resolve_path_within_root(&workspace_root, Path::new(&requested_path));
        let target_existed_before = target_path.exists();
        let before_bytes = if preview_only || !target_existed_before {
            None
        } else {
            Some(fs::read(&target_path).map_err(|err| {
                HeptaError(format!(
                    "failed to read {} before write transaction capture: {}",
                    target_path.display(),
                    err
                ))
            })?)
        };

        Ok(Some(PreparedWriteTransaction {
            target_path: target_path.display().to_string(),
            mode_requested,
            preview_only,
            target_existed_before,
            before_bytes,
        }))
    }

    fn prepare_write_transaction_with_lock_check(
        &self,
        session_id: &str,
        tool_name: &str,
        input_json: &str,
    ) -> Result<Option<PreparedWriteTransaction>, HeptaError> {
        let prepared = self.prepare_write_transaction(tool_name, input_json)?;
        if let Some(prepared_write_transaction) = prepared.as_ref() {
            self.ensure_write_target_unlocked(
                session_id,
                &prepared_write_transaction.target_path,
                tool_name,
            )?;
        }
        Ok(prepared)
    }

    fn record_write_transaction_from_tool_result(
        &self,
        session_id: &SessionId,
        prepared: Option<PreparedWriteTransaction>,
        tool_output_json: Option<String>,
    ) -> Result<Option<String>, HeptaError> {
        let Some(prepared) = prepared else {
            return Ok(tool_output_json);
        };
        let Some(tool_output_json) = tool_output_json else {
            return Ok(None);
        };

        let mut output_value: Value = serde_json::from_str(&tool_output_json).map_err(|err| {
            HeptaError(format!(
                "failed to parse tool output JSON for write transaction capture: {}",
                err
            ))
        })?;
        if prepared.preview_only {
            return Ok(Some(output_value.to_string()));
        }

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
                if let Some(parent) = checkpoint_path.parent() {
                    fs::create_dir_all(parent).map_err(|err| {
                        HeptaError(format!(
                            "failed to create transaction checkpoint parent {}: {}",
                            parent.display(),
                            err
                        ))
                    })?;
                }
                fs::write(&checkpoint_path, before_bytes).map_err(|err| {
                    HeptaError(format!(
                        "failed to write transaction checkpoint {}: {}",
                        checkpoint_path.display(),
                        err
                    ))
                })?;
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
        let bytes_after = output_value
            .get("bytes_after")
            .and_then(Value::as_u64)
            .or_else(|| output_value.get("bytes_written").and_then(Value::as_u64))
            .unwrap_or(0);
        let entry = WriteTransactionEntry {
            transaction_id: transaction_id.clone(),
            session_id: session_id.0.clone(),
            action: "write_file".into(),
            target_path: prepared.target_path.clone(),
            created_at_unix_ms: current_unix_ms()?,
            mode: prepared.mode_requested.clone(),
            target_existed_before: prepared.target_existed_before,
            bytes_before: prepared
                .before_bytes
                .as_ref()
                .map(|bytes| bytes.len() as u64)
                .unwrap_or(0),
            bytes_after,
            rollback_strategy: rollback_strategy.into(),
            rollback_checkpoint_path: rollback_checkpoint_path.clone(),
            source_backup_path,
            rolled_back_at_unix_ms: None,
        };
        {
            let mut guard = self
                .write_transaction_state
                .lock()
                .map_err(|_| HeptaError("write transaction state mutex poisoned".into()))?;
            guard.push(entry.clone());
        }
        let active_group_id = self.active_write_transaction_group_id_for_session(&session_id.0)?;
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

        if let Some(object) = output_value.as_object_mut() {
            object.insert("transaction_id".into(), json!(transaction_id));
            object.insert("rollback_strategy".into(), json!(rollback_strategy));
            if let Some(rollback_checkpoint_path) = rollback_checkpoint_path {
                object.insert(
                    "rollback_checkpoint_path".into(),
                    json!(rollback_checkpoint_path),
                );
            }
            if let Some(active_group_id) = active_group_id {
                object.insert("transaction_group_id".into(), json!(active_group_id));
            }
        }

        Ok(Some(output_value.to_string()))
    }

    fn record_restore_backup_transaction(
        &self,
        session_id: &SessionId,
        restored_target_path: &str,
        target_existed_before_restore: bool,
        restored_bytes: u64,
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
            entries.sort_by(|left, right| right.created_at_unix_ms.cmp(&left.created_at_unix_ms));
            for (index, entry) in entries.into_iter().enumerate() {
                let keep_due_to_count = index < keep_latest_per_target;
                let age_matches = max_age_ms
                    .map(|max_age_ms| now.saturating_sub(entry.created_at_unix_ms) >= max_age_ms)
                    .unwrap_or(true);
                if !keep_due_to_count && age_matches {
                    if execute {
                        fs::remove_file(&entry.backup_path).map_err(|err| {
                            HeptaError(format!(
                                "failed to delete backup {}: {}",
                                entry.backup_path, err
                            ))
                        })?;
                    }
                    deleted_backups.push(entry);
                } else {
                    kept_backups.push(entry);
                }
            }
        }

        kept_backups.sort_by(|left, right| right.created_at_unix_ms.cmp(&left.created_at_unix_ms));
        deleted_backups
            .sort_by(|left, right| right.created_at_unix_ms.cmp(&left.created_at_unix_ms));

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

const ROLLBACK_GROUP_STATUS_SCHEMA_VERSION: u32 = 1;
const WRITE_LOCK_REPORT_SCHEMA_VERSION: u32 = 1;
const ROLLBACK_EVENT_PAYLOAD_SCHEMA_VERSION: u32 = 1;
