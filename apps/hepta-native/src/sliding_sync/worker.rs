use super::*;

/// The entry point for the worker task that runs Matrix-related operations.
///
/// All this task does is wait for [`MatrixRequests`] from the main UI thread
/// and then executes them within an async runtime context.
pub(super) async fn matrix_worker_task(
    mut request_receiver: UnboundedReceiver<MatrixRequest>,
    login_sender: Sender<LoginRequest>,
) -> Result<()> {
    log!("Started matrix_worker_task.");
    // The async tasks that are spawned to subscribe to changes in our own user's read receipts for each timeline.
    let mut subscribers_own_user_read_receipts: HashMap<TimelineKind, JoinHandle<()>> =
        HashMap::new();
    // The async tasks that are spawned to subscribe to changes in the pinned events for each room.
    let mut subscribers_pinned_events: HashMap<OwnedRoomId, JoinHandle<()>> = HashMap::new();

    while let Some(request) = request_receiver.recv().await {
        match request {
            MatrixRequest::Login(login_request) => {
                if let Err(e) = login_sender.send(login_request).await {
                    error!("Error sending login request to login_sender: {e:?}");
                    Cx::post_action(LoginAction::LoginFailure(String::from(
                        "BUG: failed to send login request to login worker task.",
                    )));
                }
            }

            MatrixRequest::Logout { is_desktop } => {
                log!("Received MatrixRequest::Logout, is_desktop: {}", is_desktop);
                let _logout_task = Handle::current().spawn(async move {
                    log!("Starting logout task");
                    // Use the state machine implementation
                    match logout_with_state_machine(is_desktop).await {
                        Ok(()) => {
                            log!("Logout completed successfully via state machine");
                        }
                        Err(e) => {
                            error!("Logout failed: {e:?}");
                        }
                    }
                });
            }

            MatrixRequest::PaginateTimeline {
                timeline_kind,
                num_events,
                direction,
            } => {
                let Some((timeline, sender)) = get_timeline_and_sender(&timeline_kind) else {
                    log!("Skipping pagination request for unknown {timeline_kind}");
                    continue;
                };

                // Spawn a new async task that will make the actual pagination request.
                let _paginate_task = Handle::current().spawn(async move {
                    log!("Starting {direction} pagination request for {timeline_kind}...");
                    sender.send(TimelineUpdate::PaginationRunning(direction)).unwrap();
                    SignalToUI::set_ui_signal();

                    let res = if direction == PaginationDirection::Forwards {
                        timeline.paginate_forwards(num_events).await
                    } else {
                        timeline.paginate_backwards(num_events).await
                    };

                    match res {
                        Ok(fully_paginated) => {
                            log!("Completed {direction} pagination request for {timeline_kind}, hit {} of timeline? {}",
                                if direction == PaginationDirection::Forwards { "end" } else { "start" },
                                if fully_paginated { "yes" } else { "no" },
                            );
                            sender.send(TimelineUpdate::PaginationIdle {
                                fully_paginated,
                                direction,
                            }).unwrap();
                            SignalToUI::set_ui_signal();
                        }
                        Err(error) => {
                            error!("Error sending {direction} pagination request for {timeline_kind}: {error:?}");
                            sender.send(TimelineUpdate::PaginationError {
                                error,
                                direction,
                            }).unwrap();
                            SignalToUI::set_ui_signal();
                        }
                    }
                });
            }

            MatrixRequest::EditMessage {
                timeline_kind,
                timeline_event_item_id,
                edited_content,
            } => {
                let Some((timeline, sender)) = get_timeline_and_sender(&timeline_kind) else {
                    log!("BUG: {timeline_kind} not found for edit request");
                    continue;
                };

                // Spawn a new async task that will make the actual edit request.
                let _edit_task = Handle::current().spawn(async move {
                    log!("Sending request to edit message {timeline_event_item_id:?} in {timeline_kind}...");
                    let result = timeline.edit(&timeline_event_item_id, edited_content).await;
                    match result {
                        Ok(_) => log!("Successfully edited message {timeline_event_item_id:?} in {timeline_kind}."),
                        Err(ref e) => error!("Error editing message {timeline_event_item_id:?} in {timeline_kind}: {e:?}"),
                    }
                    sender.send(TimelineUpdate::MessageEdited {
                        timeline_event_item_id,
                        result,
                    }).unwrap();
                    SignalToUI::set_ui_signal();
                });
            }

            MatrixRequest::FetchDetailsForEvent {
                timeline_kind,
                event_id,
            } => {
                let Some((timeline, sender)) = get_timeline_and_sender(&timeline_kind) else {
                    log!("BUG: {timeline_kind} not found for fetch details for event request");
                    continue;
                };

                let _fetch_task = Handle::current().spawn(async move {
                    // log!("Sending request to fetch details for event {event_id} in {timeline_kind}...");
                    let result = timeline.fetch_details_for_event(&event_id).await;
                    match &result {
                        Ok(_) => {
                            // log!("Successfully fetched details for event {event_id} in {timeline_kind}.");
                        }
                        Err(_e) => {
                            // error!("Error fetching details for event {event_id} in {timeline_kind}: {_e:?}");
                        }
                    }
                    if sender
                        .send(TimelineUpdate::EventDetailsFetched { event_id, result })
                        .is_err()
                    {
                        error!("Failed to send fetched event details to UI for {timeline_kind}");
                    }
                    SignalToUI::set_ui_signal();
                });
            }

            MatrixRequest::FetchThreadSummaryDetails {
                timeline_kind,
                thread_root_event_id,
                timeline_item_index,
            } => {
                let Some((timeline, sender)) = get_timeline_and_sender(&timeline_kind) else {
                    log!("BUG: {timeline_kind} not found for fetch thread summary details request");
                    continue;
                };

                let _fetch_task = Handle::current().spawn(async move {
                    let (num_replies, latest_reply_event) = fetch_thread_summary_details(
                        timeline.room(),
                        &thread_root_event_id,
                    ).await;
                    let latest_reply_preview_text = match latest_reply_event.as_ref() {
                        Some(event) => text_preview_of_latest_thread_reply(timeline.room(), event).await,
                        None => None,
                    };

                    if sender.send(TimelineUpdate::ThreadSummaryDetailsFetched {
                        thread_root_event_id,
                        timeline_item_index,
                        num_replies,
                        latest_reply_preview_text,
                    }).is_err() {
                        error!("Failed to send fetched thread summary details to UI for {timeline_kind}");
                    }
                    SignalToUI::set_ui_signal();
                });
            }

            MatrixRequest::FetchEditHistory {
                timeline_kind,
                event_id,
            } => {
                let Some((timeline, sender)) = get_timeline_and_sender(&timeline_kind) else {
                    log!("BUG: {timeline_kind} not found for fetch edit history request");
                    continue;
                };

                let _fetch_task = Handle::current().spawn(async move {
                    let result = fetch_message_edit_history(timeline.room(), &event_id).await;
                    if sender
                        .send(TimelineUpdate::EditHistoryFetched { event_id, result })
                        .is_err()
                    {
                        error!("Failed to send fetched edit history to UI for {timeline_kind}");
                    }
                    SignalToUI::set_ui_signal();
                });
            }

            MatrixRequest::FetchEventSource {
                timeline_kind,
                event_id,
            } => {
                let Some((timeline, sender)) = get_timeline_and_sender(&timeline_kind) else {
                    log!("BUG: {timeline_kind} not found for event source fetch request");
                    continue;
                };

                let _fetch_source_task = Handle::current().spawn(async move {
                    let result = fetch_event_source_json(timeline.room(), &event_id).await;
                    if sender
                        .send(TimelineUpdate::EventSourceFetched { event_id, result })
                        .is_err()
                    {
                        error!("Failed to send fetched event source to UI for {timeline_kind}");
                    }
                    SignalToUI::set_ui_signal();
                });
            }

            MatrixRequest::SearchMessagesServer {
                timeline_kind,
                query,
                filter,
                limit,
                next_batch,
            } => {
                let Some((timeline, sender)) = get_timeline_and_sender(&timeline_kind) else {
                    log!("BUG: {timeline_kind} not found for server message search request");
                    continue;
                };

                let _search_task = Handle::current().spawn(async move {
                    let room = timeline.room();
                    let result = search_room_messages_server(
                        room.client().clone(),
                        room.room_id().to_owned(),
                        query,
                        filter,
                        limit,
                        next_batch,
                    )
                    .await;
                    if sender
                        .send(TimelineUpdate::MessageSearchServerResult { result })
                        .is_err()
                    {
                        error!(
                            "Failed to send server message search result to UI for {timeline_kind}"
                        );
                    }
                    SignalToUI::set_ui_signal();
                });
            }

            MatrixRequest::SyncRoomMemberList { timeline_kind } => {
                let Some((timeline, sender)) = get_timeline_and_sender(&timeline_kind) else {
                    log!("BUG: {timeline_kind} not found for sync members list request");
                    continue;
                };

                let _fetch_task = Handle::current().spawn(async move {
                    log!("Sending sync room members request for {timeline_kind}...");
                    timeline.fetch_members().await;
                    log!("Completed sync room members request for {timeline_kind}.");
                    sender.send(TimelineUpdate::RoomMembersSynced).unwrap();
                    SignalToUI::set_ui_signal();
                });
            }

            MatrixRequest::CreateThreadTimeline {
                room_id,
                thread_root_event_id,
            } => {
                let main_room_timeline = {
                    let mut all_joined_rooms = ALL_JOINED_ROOMS.lock().unwrap();
                    let Some(room_info) = all_joined_rooms.get_mut(&room_id) else {
                        error!(
                            "BUG: room info not found for create thread timeline request, room {room_id}"
                        );
                        continue;
                    };
                    if room_info
                        .thread_timelines
                        .contains_key(&thread_root_event_id)
                    {
                        continue;
                    }
                    let newly_pending = room_info
                        .pending_thread_timelines
                        .insert(thread_root_event_id.clone());
                    if !newly_pending {
                        continue;
                    }
                    room_info.main_timeline.timeline.clone()
                };

                let _create_thread_timeline_task = Handle::current().spawn(async move {
                    log!("Creating thread-focused timeline for room {room_id}, thread {thread_root_event_id}...");
                    let build_result = main_room_timeline.room()
                        .timeline_builder()
                        .with_focus(TimelineFocus::Thread {
                            root_event_id: thread_root_event_id.clone(),
                        })
                        .track_read_marker_and_receipts(TimelineReadReceiptTracking::AllEvents)
                        .build()
                        .await;

                    match build_result {
                        Ok(thread_timeline) => {
                            let mut all_joined_rooms = ALL_JOINED_ROOMS.lock().unwrap();
                            let Some(room_info) = all_joined_rooms.get_mut(&room_id) else {
                                return;
                            };
                            log!("Successfully created thread-focused timeline for room {room_id}, thread {thread_root_event_id}.");
                            let thread_timeline = Arc::new(thread_timeline);
                            let (timeline_update_sender, timeline_update_receiver) = crossbeam_channel::unbounded();
                            let (request_sender, request_receiver) = watch::channel(Vec::new());
                            let timeline_subscriber_handler_task = Handle::current().spawn(
                                timeline_subscriber_handler(
                                    main_room_timeline.room().clone(),
                                    thread_timeline.clone(),
                                    timeline_update_sender.clone(),
                                    request_receiver,
                                    Some(thread_root_event_id.clone()),
                                )
                            );
                            room_info
                                .pending_thread_timelines
                                .remove(&thread_root_event_id);
                            room_info.thread_timelines.insert(
                                thread_root_event_id.clone(),
                                PerTimelineDetails {
                                    timeline: thread_timeline,
                                    timeline_update_sender,
                                    timeline_singleton_endpoints: Some((
                                        timeline_update_receiver,
                                        request_sender,
                                    )),
                                    timeline_subscriber_handler_task,
                                },
                            );
                            SignalToUI::set_ui_signal();
                        }
                        Err(error) => {
                            error!("Failed to create thread-focused timeline for room {room_id}, thread {thread_root_event_id}: {error}");
                            let mut all_joined_rooms = ALL_JOINED_ROOMS.lock().unwrap();
                            if let Some(room_info) = all_joined_rooms.get_mut(&room_id) {
                                room_info
                                    .pending_thread_timelines
                                    .remove(&thread_root_event_id);
                            }
                            enqueue_popup_notification(
                                format!("Failed to create thread-focused timeline. Please retry opening the thread again later.\n\nError: {error}"),
                                PopupKind::Error,
                                None,
                            );
                        }
                    }
                });
            }

            MatrixRequest::Knock {
                room_or_alias_id,
                reason,
                server_names,
            } => {
                let Some(client) = get_client() else { continue };
                let _knock_room_task = Handle::current().spawn(async move {
                    log!("Sending request to knock on room {room_or_alias_id}...");
                    match client
                        .knock(room_or_alias_id.clone(), reason, server_names)
                        .await
                    {
                        Ok(room) => {
                            let _ = room.display_name().await; // populate this room's display name cache
                            Cx::post_action(KnockResultAction::Knocked {
                                room_or_alias_id,
                                room,
                            });
                        }
                        Err(error) => Cx::post_action(KnockResultAction::Failed {
                            room_or_alias_id,
                            error,
                        }),
                    }
                });
            }

            MatrixRequest::InviteUser { room_id, user_id } => {
                let Some(client) = get_client() else { continue };
                let _invite_task = Handle::current().spawn(async move {
                    // We use `client.get_room()` here because the room might also be a space,
                    // not just a joined room.
                    if let Some(room) = client.get_room(&room_id) {
                        log!("Sending request to invite user {user_id} to room {room_id}...");
                        match room.invite_user_by_id(&user_id).await {
                            Ok(_) => Cx::post_action(InviteResultAction::Sent { room_id, user_id }),
                            Err(error) => Cx::post_action(InviteResultAction::Failed {
                                room_id,
                                user_id,
                                error,
                            }),
                        }
                    } else {
                        error!("Room/Space not found for invite user request {room_id}, {user_id}");
                        Cx::post_action(InviteResultAction::Failed {
                            room_id,
                            user_id,
                            error: matrix_sdk::Error::UnknownError(
                                "Room/Space not found in client's known list.".into(),
                            ),
                        })
                    }
                });
            }

            MatrixRequest::JoinRoom { room_id } => {
                let Some(client) = get_client() else { continue };
                let _join_room_task = Handle::current().spawn(async move {
                    log!("Sending request to join room {room_id}...");
                    let result_action = if let Some(room) = client.get_room(&room_id) {
                        match room.join().await {
                            Ok(()) => {
                                log!("Successfully joined known room {room_id}.");
                                JoinRoomResultAction::Joined { room_id }
                            }
                            Err(e) => {
                                error!("Error joining known room {room_id}: {e:?}");
                                JoinRoomResultAction::Failed { room_id, error: e }
                            }
                        }
                    } else {
                        match client.join_room_by_id(&room_id).await {
                            Ok(_room) => {
                                log!("Successfully joined new unknown room {room_id}.");
                                JoinRoomResultAction::Joined { room_id }
                            }
                            Err(e) => {
                                error!("Error joining new unknown room {room_id}: {e:?}");
                                JoinRoomResultAction::Failed { room_id, error: e }
                            }
                        }
                    };
                    Cx::post_action(result_action);
                });
            }

            MatrixRequest::JoinRoomByIdOrAlias {
                room_or_alias_id,
                server_names,
            } => {
                let Some(client) = get_client() else { continue };
                let _join_room_task = Handle::current().spawn(async move {
                    log!("Sending request to join room or alias {room_or_alias_id}...");
                    let known_room_id = OwnedRoomId::try_from(room_or_alias_id.to_string()).ok();
                    let result_action = if let Some(room_id) = known_room_id.as_ref() {
                        if let Some(room) = client.get_room(room_id) {
                            match room.join().await {
                                Ok(()) => {
                                    log!("Successfully joined known Matrix link room {room_id}.");
                                    MatrixLinkJoinResultAction::Joined {
                                        room_or_alias_id,
                                        server_names,
                                        room_id: room_id.clone(),
                                    }
                                }
                                Err(e) => {
                                    error!(
                                        "Error joining known Matrix link room {room_id}: {e:?}"
                                    );
                                    MatrixLinkJoinResultAction::Failed {
                                        room_or_alias_id,
                                        server_names,
                                        error: e,
                                    }
                                }
                            }
                        } else {
                            match client
                                .join_room_by_id_or_alias(&room_or_alias_id, &server_names)
                                .await
                            {
                                Ok(room) => {
                                    let joined_room_id = room.room_id().to_owned();
                                    log!(
                                        "Successfully joined Matrix link target {room_or_alias_id} as {joined_room_id}."
                                    );
                                    MatrixLinkJoinResultAction::Joined {
                                        room_or_alias_id,
                                        server_names,
                                        room_id: joined_room_id,
                                    }
                                }
                                Err(e) => {
                                    error!(
                                        "Error joining Matrix link target {room_or_alias_id}: {e:?}"
                                    );
                                    MatrixLinkJoinResultAction::Failed {
                                        room_or_alias_id,
                                        server_names,
                                        error: e,
                                    }
                                }
                            }
                        }
                    } else {
                        match client
                            .join_room_by_id_or_alias(&room_or_alias_id, &server_names)
                            .await
                        {
                            Ok(room) => {
                                let joined_room_id = room.room_id().to_owned();
                                log!(
                                    "Successfully joined Matrix link alias {room_or_alias_id} as {joined_room_id}."
                                );
                                MatrixLinkJoinResultAction::Joined {
                                    room_or_alias_id,
                                    server_names,
                                    room_id: joined_room_id,
                                }
                            }
                            Err(e) => {
                                error!("Error joining Matrix link alias {room_or_alias_id}: {e:?}");
                                MatrixLinkJoinResultAction::Failed {
                                    room_or_alias_id,
                                    server_names,
                                    error: e,
                                }
                            }
                        }
                    };
                    Cx::post_action(result_action);
                });
            }

            MatrixRequest::LeaveRoom { room_id } => {
                let Some(client) = get_client() else { continue };
                let _leave_room_task = Handle::current().spawn(async move {
                    log!("Sending request to leave room {room_id}...");
                    let result_action = if let Some(room) = client.get_room(&room_id) {
                        match room.leave().await {
                            Ok(()) => {
                                log!("Successfully left room {room_id}.");
                                LeaveRoomResultAction::Left { room_id }
                            }
                            Err(e) => {
                                error!("Error leaving room {room_id}: {e:?}");
                                LeaveRoomResultAction::Failed { room_id, error: e }
                            }
                        }
                    } else {
                        error!("BUG: client could not get room with ID {room_id}");
                        LeaveRoomResultAction::Failed {
                            room_id,
                            error: matrix_sdk::Error::UnknownError(
                                "Client couldn't locate room to leave it.".into(),
                            ),
                        }
                    };
                    Cx::post_action(result_action);
                });
            }

            MatrixRequest::GetRoomMembers {
                timeline_kind,
                memberships,
                local_only,
            } => {
                let Some((timeline, sender)) = get_timeline_and_sender(&timeline_kind) else {
                    log!("BUG: {timeline_kind} not found for get room members request");
                    continue;
                };

                let _get_members_task = Handle::current().spawn(async move {
                    let send_update = |members: Vec<matrix_sdk::room::RoomMember>, source: &str| {
                        log!("{} {} members for {timeline_kind}", source, members.len());
                        sender
                            .send(TimelineUpdate::RoomMembersListFetched { members })
                            .unwrap();
                        SignalToUI::set_ui_signal();
                    };

                    let room = timeline.room();
                    if local_only {
                        if let Ok(members) = room.members_no_sync(memberships).await {
                            send_update(members, "Got");
                        }
                    } else {
                        if let Ok(members) = room.members(memberships).await {
                            send_update(members, "Successfully fetched");
                        }
                    }
                });
            }

            MatrixRequest::GetRoomPreview {
                room_or_alias_id,
                via,
                response_mode,
            } => {
                let Some(client) = get_client() else { continue };
                let _fetch_task = Handle::current().spawn(async move {
                    let res = fetch_room_preview_with_avatar(&client, &room_or_alias_id, via).await;
                    match response_mode {
                        RoomPreviewResponseMode::Action => {
                            Cx::post_action(RoomPreviewAction::Fetched(res));
                        }
                        RoomPreviewResponseMode::RoomPreviewCache => match res {
                            Ok(fetched) => enqueue_room_preview_update(RoomPreviewUpdate {
                                room_or_alias_id,
                                fetched,
                            }),
                            Err(e) => {
                                log!("Failed to get room preview for {room_or_alias_id:?}: {e:?}")
                            }
                        },
                    }
                });
            }

            MatrixRequest::PreviewMatrixLinkTarget {
                timeline_kind,
                room_or_alias_id,
                via,
                event_id,
            } => {
                let Some(client) = get_client() else { continue };
                let preview_result_sender =
                    get_timeline_and_sender(&timeline_kind).map(|(_timeline, sender)| sender);
                let _fetch_task = Handle::current().spawn(async move {
                    let via_count = via.len();
                    let fetch_via = via.clone();
                    match fetch_room_preview_with_avatar(&client, &room_or_alias_id, fetch_via)
                        .await
                    {
                        Ok(fetched) => {
                            let room_name = fetched.room_name_id.to_string();
                            let alias = fetched
                                .canonical_alias
                                .as_ref()
                                .map(|alias| format!(" Alias: {alias}."))
                                .unwrap_or_default();
                            let topic = fetched
                                .topic
                                .as_deref()
                                .filter(|topic| !topic.trim().is_empty())
                                .map(|topic| format!(" Topic: {topic}."))
                                .unwrap_or_default();
                            let event_note = event_id
                                .as_ref()
                                .map(|event_id| {
                                    format!(
                                        " Event {event_id} context fetch is still not wired."
                                    )
                                })
                                .unwrap_or_default();
                            let event_source_json = if let Some(event_id) = event_id.as_ref()
                                && let Some(room) = client.get_room(fetched.room_name_id.room_id())
                            {
                                fetch_event_source_json(&room, event_id).await.ok()
                            } else {
                                None
                            };
                            let source_note = match (event_id.as_ref(), event_source_json.as_ref()) {
                                (Some(_), Some(json)) => format!(
                                    " Event source fetched through Room::load_or_fetch_event: {} chars.",
                                    json.chars().count()
                                ),
                                (Some(_), None) => {
                                    " Event source fetch unavailable from the current client room cache.".to_string()
                                }
                                (None, _) => String::new(),
                            };
                            let result_metadata = format!(
                                "{}{}",
                                matrix_link_preview_result_metadata_label(
                                    &fetched,
                                    event_id.as_ref(),
                                ),
                                source_note
                            );
                            enqueue_popup_notification(
                                format!(
                                    "Matrix link room preview: {room_name}. Members: {}.{alias}{topic}{event_note} {result_metadata}",
                                    fetched.num_joined_members
                                ),
                                PopupKind::Info,
                                Some(8.0),
                            );
                            if let Some(sender) = preview_result_sender.as_ref() {
                                let event_source_room_id = event_source_json
                                    .as_ref()
                                    .map(|_| fetched.room_name_id.room_id().clone());
                                let _ = sender.send(TimelineUpdate::MatrixLinkPreviewResult {
                                    room_or_alias_id,
                                    via,
                                    event_id,
                                    event_source_room_id,
                                    event_source_json,
                                    result: Ok(result_metadata),
                                });
                            }
                        }
                        Err(error) => {
                            let target = room_or_alias_id.to_string();
                            let error_text = error.to_string();
                            let failure_metadata = matrix_link_preview_failure_metadata_label(
                                &target,
                                via_count,
                                event_id.as_ref(),
                                &error_text,
                            );
                            enqueue_popup_notification(
                                format!(
                                    "Failed to resolve Matrix link preview for {target}: {error_text}. {failure_metadata}"
                                ),
                                PopupKind::Error,
                                Some(6.0),
                            );
                            if let Some(sender) = preview_result_sender.as_ref() {
                                let _ = sender.send(TimelineUpdate::MatrixLinkPreviewResult {
                                    room_or_alias_id,
                                    via,
                                    event_id,
                                    event_source_room_id: None,
                                    event_source_json: None,
                                    result: Err(error_text),
                                });
                            }
                        }
                    }
                    SignalToUI::set_ui_signal();
                });
            }

            MatrixRequest::GetSuccessorRoomDetails { tombstoned_room_id } => {
                let Some(client) = get_client() else { continue };
                let (sender, successor_room) = {
                    let all_joined_rooms = ALL_JOINED_ROOMS.lock().unwrap();
                    let Some(room_info) = all_joined_rooms.get(&tombstoned_room_id) else {
                        error!(
                            "BUG: tombstoned room {tombstoned_room_id} info not found for get successor room details request"
                        );
                        continue;
                    };
                    (
                        room_info.main_timeline.timeline_update_sender.clone(),
                        room_info.main_timeline.timeline.room().successor_room(),
                    )
                };
                spawn_fetch_successor_room_preview(
                    client,
                    successor_room,
                    tombstoned_room_id,
                    sender,
                );
            }

            MatrixRequest::OpenOrCreateDirectMessage {
                user_profile,
                allow_create,
            } => {
                let Some(client) = get_client() else { continue };
                let _create_dm_task = Handle::current().spawn(async move {
                    if let Some(room) = client.get_dm_room(&user_profile.user_id) {
                        log!("Found existing DM room: {}", room.room_id());
                        Cx::post_action(DirectMessageRoomAction::FoundExisting {
                            user_id: user_profile.user_id,
                            room_name_id: RoomNameId::from_room(&room).await,
                        });
                        return;
                    }
                    if !allow_create {
                        Cx::post_action(DirectMessageRoomAction::DidNotExist { user_profile });
                        return;
                    }
                    log!("Creating new DM room with {user_profile:?}...");
                    match client.create_dm(&user_profile.user_id).await {
                        Ok(room) => {
                            log!("Successfully created DM room: {}", room.room_id());
                            Cx::post_action(DirectMessageRoomAction::NewlyCreated {
                                user_profile,
                                room_name_id: RoomNameId::from_room(&room).await,
                            });
                        }
                        Err(error) => {
                            error!("Failed to create DM with {user_profile:?}: {error}");
                            Cx::post_action(DirectMessageRoomAction::FailedToCreate {
                                user_profile,
                                error,
                            });
                        }
                    }
                });
            }

            MatrixRequest::GetUserProfile {
                user_id,
                room_id,
                local_only,
            } => {
                let Some(client) = get_client() else { continue };
                let _fetch_task = Handle::current().spawn(async move {
                    // log!("Sending get user profile request: user: {user_id}, \
                    //     room: {room_id:?}, local_only: {local_only}...",
                    // );

                    let mut update = None;

                    if let Some(room_id) = room_id.as_ref() {
                        if let Some(room) = client.get_room(room_id) {
                            let member = if local_only {
                                room.get_member_no_sync(&user_id).await
                            } else {
                                room.get_member(&user_id).await
                            };
                            if let Ok(Some(room_member)) = member {
                                update = Some(UserProfileUpdate::Full {
                                    new_profile: UserProfile {
                                        username: room_member.display_name().map(|u| u.to_owned()),
                                        user_id: user_id.clone(),
                                        avatar_state: AvatarState::Known(room_member.avatar_url().map(|u| u.to_owned())),
                                    },
                                    room_id: room_id.to_owned(),
                                    room_member,
                                });
                            } else {
                                // log!("User profile request: user {user_id} was not a member of room {room_id}");
                            }
                        } else {
                            log!("User profile request: client could not get room with ID {room_id}");
                        }
                    }

                    if !local_only {
                        if update.is_none() {
                            if let Ok(response) = client.account().fetch_user_profile_of(&user_id).await {
                                update = Some(UserProfileUpdate::UserProfileOnly(
                                    UserProfile {
                                        username: response.get_static::<DisplayName>().ok().flatten(),
                                        user_id: user_id.clone(),
                                        avatar_state: response.get_static::<AvatarUrl>()
                                            .ok()
                                            .map_or(AvatarState::Unknown, AvatarState::Known),
                                    }
                                ));
                            } else {
                                log!("User profile request: client could not get user with ID {user_id}");
                            }
                        }

                        match update.as_mut() {
                            Some(UserProfileUpdate::Full { new_profile: UserProfile { username, .. }, .. }) if username.is_none() => {
                                if let Ok(response) = client.account().fetch_user_profile_of(&user_id).await {
                                    *username = response.get_static::<DisplayName>().ok().flatten();
                                }
                            }
                            _ => { }
                        }
                    }

                    if let Some(upd) = update {
                        // log!("Successfully completed get user profile request: user: {user_id}, room: {room_id:?}, local_only: {local_only}.");
                        enqueue_user_profile_update(upd);
                    } else {
                        log!("Failed to get user profile: user: {user_id}, room: {room_id:?}, local_only: {local_only}.");
                    }
                });
            }

            MatrixRequest::SearchUserDirectory { query, limit } => {
                let Some(client) = get_client() else {
                    Cx::post_action(UserDirectorySearchAction::Searched(Err(
                        "Failed to search Matrix user directory: client unavailable".to_string(),
                    )));
                    SignalToUI::set_ui_signal();
                    continue;
                };
                let _search_user_directory_task = Handle::current().spawn(async move {
                    let query = sanitize_user_directory_search_query(&query);
                    let result = if query.is_empty() {
                        Err("Failed to search Matrix user directory: query is empty".to_string())
                    } else {
                        match client.search_users(&query, limit).await {
                            Ok(response) => Ok(UserDirectorySearchResult {
                                query,
                                limited: response.limited,
                                results: response
                                    .results
                                    .into_iter()
                                    .map(|user| UserDirectorySearchEntry {
                                        user_id: user.user_id,
                                        display_name: user.display_name,
                                        avatar_url: user.avatar_url,
                                    })
                                    .collect(),
                            }),
                            Err(error) => {
                                let err_msg =
                                    format!("Failed to search Matrix user directory: {error}");
                                error!("{err_msg}");
                                Err(err_msg)
                            }
                        }
                    };
                    Cx::post_action(UserDirectorySearchAction::Searched(result));
                    SignalToUI::set_ui_signal();
                });
            }

            MatrixRequest::GetNumberUnreadMessages { timeline_kind } => {
                let Some((timeline, sender)) = get_timeline_and_sender(&timeline_kind) else {
                    log!("Skipping get number of unread messages request for {timeline_kind}");
                    continue;
                };

                let _get_unreads_task = Handle::current().spawn(async move {
                    match sender.send(TimelineUpdate::NewUnreadMessagesCount(
                        UnreadMessageCount::Known(timeline.room().num_unread_messages())
                    )) {
                        Ok(_) => SignalToUI::set_ui_signal(),
                        Err(e) => log!("Failed to send timeline update: {e:?} for GetNumberUnreadMessages request for {timeline_kind}"),
                    }
                    if let TimelineKind::MainRoom { room_id } = timeline_kind {
                        enqueue_rooms_list_update(RoomsListUpdate::UpdateNumUnreadMessages {
                            room_id,
                            is_marked_unread: timeline.room().is_marked_unread(),
                            unread_messages: UnreadMessageCount::Known(timeline.room().num_unread_messages()),
                            unread_mentions: timeline.room().num_unread_mentions(),
                        });
                    }
                });
            }

            MatrixRequest::SetUnreadFlag {
                room_id,
                mark_as_unread,
            } => {
                let Some(main_timeline) = get_room_timeline(&room_id) else {
                    log!("BUG: skipping set unread flag request for not-yet-known room {room_id}");
                    continue;
                };
                let _set_unread_task = Handle::current().spawn(async move {
                    let result = main_timeline.room().set_unread_flag(mark_as_unread).await;
                    match result {
                        Ok(_) => log!("Set unread flag to {} for room {}", mark_as_unread, room_id),
                        Err(e) => error!(
                            "Failed to set unread flag to {} for room {}: {:?}",
                            mark_as_unread, room_id, e
                        ),
                    }
                });
            }

            MatrixRequest::SetIsFavorite {
                room_id,
                is_favorite,
            } => {
                let Some(main_timeline) = get_room_timeline(&room_id) else {
                    log!(
                        "BUG: skipping set favorite flag request for not-yet-known room {room_id}"
                    );
                    continue;
                };
                let _set_favorite_task = Handle::current().spawn(async move {
                    let result = main_timeline
                        .room()
                        .set_is_favourite(is_favorite, None)
                        .await;
                    match result {
                        Ok(_) => log!("Set favorite to {} for room {}", is_favorite, room_id),
                        Err(e) => error!(
                            "Failed to set favorite to {} for room {}: {:?}",
                            is_favorite, room_id, e
                        ),
                    }
                });
            }

            MatrixRequest::SetIsLowPriority {
                room_id,
                is_low_priority,
            } => {
                let Some(main_timeline) = get_room_timeline(&room_id) else {
                    log!(
                        "BUG: skipping set low priority flag request for not-yet-known room {room_id}"
                    );
                    continue;
                };
                let _set_lp_task = Handle::current().spawn(async move {
                    let result = main_timeline
                        .room()
                        .set_is_low_priority(is_low_priority, None)
                        .await;
                    match result {
                        Ok(_) => log!(
                            "Set low priority to {} for room {}",
                            is_low_priority,
                            room_id
                        ),
                        Err(e) => error!(
                            "Failed to set low priority to {} for room {}: {:?}",
                            is_low_priority, room_id, e
                        ),
                    }
                });
            }

            MatrixRequest::SetRoomNotificationMode { room_id, mode } => {
                let timeline_kind = TimelineKind::MainRoom {
                    room_id: room_id.clone(),
                };
                let Some((main_timeline, sender)) = get_timeline_and_sender(&timeline_kind) else {
                    log!(
                        "BUG: skipping set room notification mode request for not-yet-known room {room_id}"
                    );
                    continue;
                };
                let _set_notification_mode_task = Handle::current().spawn(async move {
                    let notification_settings =
                        main_timeline.room().client().notification_settings().await;
                    let result = notification_settings
                        .set_room_notification_mode(&room_id, mode)
                        .await;
                    let update_result = match result {
                        Ok(()) => {
                            main_timeline
                                .room()
                                .update_cached_user_defined_notification_mode(mode);
                            log!("Set notification mode to {mode:?} for room {room_id}");
                            Ok(())
                        }
                        Err(error) => {
                            error!(
                                "Failed to set notification mode to {mode:?} for room {room_id}: {error:?}"
                            );
                            Err(format!("{error}"))
                        }
                    };
                    if sender
                        .send(TimelineUpdate::RoomNotificationModeSet {
                            mode,
                            result: update_result,
                        })
                        .is_err()
                    {
                        error!("Failed to send room notification mode write result to UI.");
                    }
                    SignalToUI::set_ui_signal();
                });
            }

            MatrixRequest::SetRoomName {
                timeline_kind,
                name,
            } => {
                let Some((timeline, sender)) = get_timeline_and_sender(&timeline_kind) else {
                    log!("BUG: {timeline_kind} not found for room name write request");
                    continue;
                };
                let _set_room_name_task = Handle::current().spawn(async move {
                    let result = timeline
                        .room()
                        .set_name(name.clone())
                        .await
                        .map(|_| ())
                        .map_err(|error| format!("{error}"));
                    if sender
                        .send(TimelineUpdate::RoomSettingsMutationResult {
                            field: RoomSettingsMutationField::Name,
                            value: name,
                            result,
                        })
                        .is_err()
                    {
                        error!("Failed to send room name write result to UI.");
                    }
                    SignalToUI::set_ui_signal();
                });
            }

            MatrixRequest::SetRoomTopic {
                timeline_kind,
                topic,
            } => {
                let Some((timeline, sender)) = get_timeline_and_sender(&timeline_kind) else {
                    log!("BUG: {timeline_kind} not found for room topic write request");
                    continue;
                };
                let _set_room_topic_task = Handle::current().spawn(async move {
                    let result = timeline
                        .room()
                        .set_room_topic(&topic)
                        .await
                        .map(|_| ())
                        .map_err(|error| format!("{error}"));
                    if sender
                        .send(TimelineUpdate::RoomSettingsMutationResult {
                            field: RoomSettingsMutationField::Topic,
                            value: topic,
                            result,
                        })
                        .is_err()
                    {
                        error!("Failed to send room topic write result to UI.");
                    }
                    SignalToUI::set_ui_signal();
                });
            }

            MatrixRequest::RemoveRoomAvatar { timeline_kind } => {
                let Some((timeline, sender)) = get_timeline_and_sender(&timeline_kind) else {
                    log!("BUG: {timeline_kind} not found for room avatar removal request");
                    continue;
                };
                let _remove_room_avatar_task = Handle::current().spawn(async move {
                    let result = timeline
                        .room()
                        .remove_avatar()
                        .await
                        .map(|_| ())
                        .map_err(|error| format!("{error}"));
                    if sender
                        .send(TimelineUpdate::RoomSettingsMutationResult {
                            field: RoomSettingsMutationField::Avatar,
                            value: "remove avatar".to_string(),
                            result,
                        })
                        .is_err()
                    {
                        error!("Failed to send room avatar removal result to UI.");
                    }
                    SignalToUI::set_ui_signal();
                });
            }

            MatrixRequest::UploadRoomAvatar {
                timeline_kind,
                file_path,
                mime_type,
            } => {
                let Some((timeline, sender)) = get_timeline_and_sender(&timeline_kind) else {
                    log!("BUG: {timeline_kind} not found for room avatar upload request");
                    continue;
                };
                let _upload_room_avatar_task = Handle::current().spawn(async move {
                    let filename = file_path
                        .file_name()
                        .and_then(|name| name.to_str())
                        .unwrap_or("selected image")
                        .to_string();
                    let value = format!("upload avatar: {filename}");
                    let result = match std::fs::read(&file_path) {
                        Ok(data) => timeline
                            .room()
                            .upload_avatar(&mime_type, data, None)
                            .await
                            .map(|_| ())
                            .map_err(|error| format!("{error}")),
                        Err(error) => Err(format!("Failed to read selected avatar file: {error}")),
                    };
                    if sender
                        .send(TimelineUpdate::RoomSettingsMutationResult {
                            field: RoomSettingsMutationField::Avatar,
                            value,
                            result,
                        })
                        .is_err()
                    {
                        error!("Failed to send room avatar upload result to UI.");
                    }
                    SignalToUI::set_ui_signal();
                });
            }

            MatrixRequest::SetRoomHistoryVisibility {
                timeline_kind,
                visibility,
            } => {
                let Some((timeline, sender)) = get_timeline_and_sender(&timeline_kind) else {
                    log!(
                        "BUG: {timeline_kind} not found for room history visibility write request"
                    );
                    continue;
                };
                let _set_room_history_visibility_task = Handle::current().spawn(async move {
                    let result = match parse_room_history_visibility(&visibility) {
                        Ok(history_visibility) => timeline
                            .room()
                            .send_state_event(RoomHistoryVisibilityEventContent::new(
                                history_visibility,
                            ))
                            .await
                            .map(|_| ())
                            .map_err(|error| format!("{error}")),
                        Err(error) => Err(error),
                    };
                    if sender
                        .send(TimelineUpdate::RoomSettingsMutationResult {
                            field: RoomSettingsMutationField::HistoryVisibility,
                            value: visibility,
                            result,
                        })
                        .is_err()
                    {
                        error!("Failed to send room history visibility write result to UI.");
                    }
                    SignalToUI::set_ui_signal();
                });
            }

            MatrixRequest::SetRoomJoinRule {
                timeline_kind,
                join_rule,
            } => {
                let Some((timeline, sender)) = get_timeline_and_sender(&timeline_kind) else {
                    log!("BUG: {timeline_kind} not found for room join rule write request");
                    continue;
                };
                let _set_room_join_rule_task = Handle::current().spawn(async move {
                    let result = match parse_room_join_rule(&join_rule) {
                        Ok(rule) => timeline
                            .room()
                            .send_state_event(RoomJoinRulesEventContent::new(rule))
                            .await
                            .map(|_| ())
                            .map_err(|error| format!("{error}")),
                        Err(error) => Err(error),
                    };
                    if sender
                        .send(TimelineUpdate::RoomSettingsMutationResult {
                            field: RoomSettingsMutationField::JoinRule,
                            value: join_rule,
                            result,
                        })
                        .is_err()
                    {
                        error!("Failed to send room join rule write result to UI.");
                    }
                    SignalToUI::set_ui_signal();
                });
            }

            MatrixRequest::SetRoomCanonicalAlias {
                timeline_kind,
                alias,
                alt_aliases,
            } => {
                let Some((timeline, sender)) = get_timeline_and_sender(&timeline_kind) else {
                    log!("BUG: {timeline_kind} not found for room canonical alias write request");
                    continue;
                };
                let _set_room_canonical_alias_task = Handle::current().spawn(async move {
                    let result = match OwnedRoomAliasId::try_from(alias.as_str()) {
                        Ok(parsed_alias) => {
                            let mut content = RoomCanonicalAliasEventContent::new();
                            content.alias = Some(parsed_alias);
                            content.alt_aliases = alt_aliases;
                            timeline
                                .room()
                                .send_state_event(content)
                                .await
                                .map(|_| ())
                                .map_err(|error| format!("{error}"))
                        }
                        Err(error) => Err(format!("Invalid room canonical alias: {error}")),
                    };
                    if sender
                        .send(TimelineUpdate::RoomSettingsMutationResult {
                            field: RoomSettingsMutationField::CanonicalAlias,
                            value: alias,
                            result,
                        })
                        .is_err()
                    {
                        error!("Failed to send room canonical alias write result to UI.");
                    }
                    SignalToUI::set_ui_signal();
                });
            }

            MatrixRequest::SetRoomTombstone {
                timeline_kind,
                replacement_room_id,
                reason,
            } => {
                let Some((timeline, sender)) = get_timeline_and_sender(&timeline_kind) else {
                    log!("BUG: {timeline_kind} not found for room tombstone write request");
                    continue;
                };
                let _set_room_tombstone_task = Handle::current().spawn(async move {
                    let replacement_value = replacement_room_id.to_string();
                    let result = timeline
                        .room()
                        .send_state_event(RoomTombstoneEventContent::new(
                            reason,
                            replacement_room_id,
                        ))
                        .await
                        .map(|_| ())
                        .map_err(|error| format!("{error}"));
                    if sender
                        .send(TimelineUpdate::RoomSettingsMutationResult {
                            field: RoomSettingsMutationField::Tombstone,
                            value: replacement_value,
                            result,
                        })
                        .is_err()
                    {
                        error!("Failed to send room tombstone write result to UI.");
                    }
                    SignalToUI::set_ui_signal();
                });
            }

            MatrixRequest::SetAvatar { avatar_url } => {
                let Some(client) = get_client() else { continue };
                let _set_avatar_task = Handle::current().spawn(async move {
                    let is_removing = avatar_url.is_none();
                    log!(
                        "Sending request to {} avatar...",
                        if is_removing { "remove" } else { "set" }
                    );
                    let result = client.account().set_avatar_url(avatar_url.as_deref()).await;
                    match result {
                        Ok(_) => {
                            log!(
                                "Successfully {} avatar.",
                                if is_removing { "removed" } else { "set" }
                            );
                            Cx::post_action(AccountDataAction::AvatarChanged(avatar_url));
                        }
                        Err(e) => {
                            let err_msg = format!(
                                "Failed to {} avatar: {e}",
                                if is_removing { "remove" } else { "set" }
                            );
                            Cx::post_action(AccountDataAction::AvatarChangeFailed(err_msg));
                        }
                    }
                });
            }

            MatrixRequest::UploadAvatar {
                file_path,
                mime_type,
            } => {
                let Some(client) = get_client() else { continue };
                let _upload_avatar_task = Handle::current().spawn(async move {
                    log!("Uploading account avatar from {file_path:?} ({mime_type})...");
                    let result: Result<OwnedMxcUri> = match tokio::fs::read(&file_path).await {
                        Ok(data) => client
                            .account()
                            .upload_avatar(&mime_type, data)
                            .await
                            .map_err(Into::into),
                        Err(error) => Err(error.into()),
                    };
                    match result {
                        Ok(avatar_url) => {
                            log!("Successfully uploaded account avatar: {avatar_url}");
                            Cx::post_action(AccountDataAction::AvatarChanged(Some(avatar_url)));
                        }
                        Err(error) => {
                            Cx::post_action(AccountDataAction::AvatarChangeFailed(format!(
                                "Failed to upload avatar: {error}"
                            )));
                        }
                    }
                });
            }

            MatrixRequest::SetDisplayName { new_display_name } => {
                let Some(client) = get_client() else { continue };
                let _set_display_name_task = Handle::current().spawn(async move {
                    let is_removing = new_display_name.is_none();
                    log!(
                        "Sending request to {} display name{}...",
                        if is_removing { "remove" } else { "set" },
                        new_display_name
                            .as_ref()
                            .map(|n| format!(" to '{n}'"))
                            .unwrap_or_default()
                    );
                    let result = client
                        .account()
                        .set_display_name(new_display_name.as_deref())
                        .await;
                    match result {
                        Ok(_) => {
                            log!(
                                "Successfully {} display name.",
                                if is_removing { "removed" } else { "set" }
                            );
                            Cx::post_action(AccountDataAction::DisplayNameChanged(
                                new_display_name,
                            ));
                        }
                        Err(e) => {
                            let err_msg = format!(
                                "Failed to {} display name: {e}",
                                if is_removing { "remove" } else { "set" }
                            );
                            Cx::post_action(AccountDataAction::DisplayNameChangeFailed(err_msg));
                        }
                    }
                });
            }

            MatrixRequest::GetOwnDevice => {
                let Some(client) = get_client() else { continue };
                let _get_own_device_task = Handle::current().spawn(async move {
                    let device = match client.encryption().get_own_device().await {
                        Ok(device) => device,
                        Err(e) => {
                            error!("Failed to get own device: {e:?}");
                            None
                        }
                    };
                    Cx::post_action(AccountDataAction::OwnDeviceFetched(device.map(Box::new)));
                });
            }

            MatrixRequest::GetDevices => {
                let Some(client) = get_client() else { continue };
                let _get_devices_task = Handle::current().spawn(async move {
                    let result = match client.devices().await {
                        Ok(response) => Ok(response
                            .devices
                            .into_iter()
                            .map(|device| AccountDeviceDirectoryEntry {
                                device_id: device.device_id.to_string(),
                                display_name: device.display_name,
                                last_seen_ip: device.last_seen_ip,
                                last_seen_ts_ms: device
                                    .last_seen_ts
                                    .map(|timestamp| timestamp.get().into()),
                            })
                            .collect()),
                        Err(e) => {
                            let err_msg = format!("Failed to fetch account devices: {e}");
                            error!("{err_msg}");
                            Err(err_msg)
                        }
                    };
                    Cx::post_action(AccountDataAction::OwnDevicesFetched(result));
                });
            }

            MatrixRequest::RenameDevice {
                device_id,
                display_name,
            } => {
                let Some(client) = get_client() else { continue };
                let _rename_device_task = Handle::current().spawn(async move {
                    let display_name = display_name.trim().to_string();
                    if display_name.is_empty() {
                        Cx::post_action(AccountDataAction::DeviceRenamed(Err(
                            "Failed to rename current device: display name is empty".to_string(),
                        )));
                        return;
                    }

                    log!("Sending request to rename current device {device_id} to '{display_name}'...");
                    let result = match client.rename_device(&device_id, &display_name).await {
                        Ok(_) => {
                            log!("Successfully renamed current device {device_id}.");
                            Ok(AccountDeviceRenameResult {
                                device_id,
                                display_name,
                            })
                        }
                        Err(error) => {
                            let err_msg =
                                format!("Failed to rename current device {device_id}: {error}");
                            error!("{err_msg}");
                            Err(err_msg)
                        }
                    };
                    Cx::post_action(AccountDataAction::DeviceRenamed(result));
                });
            }

            MatrixRequest::GenerateMatrixLink {
                room_id,
                event_id,
                use_matrix_scheme,
                join_on_click,
            } => {
                let Some(client) = get_client() else { continue };
                let _gen_link_task = Handle::current().spawn(async move {
                    if let Some(room) = client.get_room(&room_id) {
                        let result = if use_matrix_scheme {
                            if let Some(event_id) = event_id {
                                room.matrix_event_permalink(event_id)
                                    .await
                                    .map(MatrixLinkAction::MatrixUri)
                            } else {
                                room.matrix_permalink(join_on_click)
                                    .await
                                    .map(MatrixLinkAction::MatrixUri)
                            }
                        } else {
                            if let Some(event_id) = event_id {
                                room.matrix_to_event_permalink(event_id)
                                    .await
                                    .map(MatrixLinkAction::MatrixToUri)
                            } else {
                                room.matrix_to_permalink()
                                    .await
                                    .map(MatrixLinkAction::MatrixToUri)
                            }
                        };

                        match result {
                            Ok(action) => Cx::post_action(action),
                            Err(e) => Cx::post_action(MatrixLinkAction::Error(e.to_string())),
                        }
                    } else {
                        Cx::post_action(MatrixLinkAction::Error(format!(
                            "Room {room_id} not found"
                        )));
                    }
                });
            }

            MatrixRequest::IgnoreUser {
                ignore,
                room_member,
                room_id,
            } => {
                let Some(client) = get_client() else { continue };
                let _ignore_task = Handle::current().spawn(async move {
                    let user_id = room_member.user_id();
                    log!("Sending request to {}ignore user: {user_id}...", if ignore { "" } else { "un" });
                    let ignore_result = if ignore {
                        room_member.ignore().await
                    } else {
                        room_member.unignore().await
                    };

                    log!("{} user {user_id} {}",
                        if ignore { "Ignoring" } else { "Unignoring" },
                        if ignore_result.is_ok() { "succeeded." } else { "failed." },
                    );

                    if ignore_result.is_err() {
                        return;
                    }

                    // We need to re-acquire the `RoomMember` object now that its state
                    // has changed, i.e., the user has been (un)ignored.
                    // We then need to send an update to replace the cached `RoomMember`
                    // with the now-stale ignored state.
                    if let Some(room) = client.get_room(&room_id) {
                        if let Ok(Some(new_room_member)) = room.get_member(user_id).await {
                            log!("Enqueueing user profile update for user {user_id}, who went from {}ignored to {}ignored.",
                                if room_member.is_ignored() { "" } else { "un" },
                                if new_room_member.is_ignored() { "" } else { "un" },
                            );
                            enqueue_user_profile_update(UserProfileUpdate::RoomMemberOnly {
                                room_id: room_id.clone(),
                                room_member: new_room_member,
                            });
                        }
                    }

                    // After successfully (un)ignoring a user, all timelines are fully cleared by the Matrix SDK.
                    // Therefore, we need to re-fetch all timelines for all rooms,
                    // and currently the only way to actually accomplish this is via pagination.
                    // See: <https://github.com/matrix-org/matrix-rust-sdk/issues/1703#issuecomment-2250297923>
                    //
                    // Note that here we only proactively re-paginate the *current* room
                    // (the one being viewed by the user when this ignore request was issued),
                    // and all other rooms will be re-paginated in `handle_ignore_user_list_subscriber()`.`
                    submit_async_request(MatrixRequest::PaginateTimeline {
                        timeline_kind: TimelineKind::MainRoom { room_id },
                        num_events: 50,
                        direction: PaginationDirection::Backwards,
                    });
                });
            }

            MatrixRequest::SendTypingNotice { room_id, typing } => {
                let Some(main_room_timeline) = get_room_timeline(&room_id) else {
                    log!(
                        "BUG: skipping send typing notice request for not-yet-known room {room_id}"
                    );
                    continue;
                };
                let _typing_task = Handle::current().spawn(async move {
                    if let Err(e) = main_room_timeline.room().typing_notice(typing).await {
                        error!("Failed to send typing notice to room {room_id}: {e:?}");
                    }
                });
            }

            MatrixRequest::SubscribeToTypingNotices { room_id, subscribe } => {
                let (main_timeline, timeline_update_sender, mut typing_notice_receiver) = {
                    let mut all_joined_rooms = ALL_JOINED_ROOMS.lock().unwrap();
                    let Some(jrd) = all_joined_rooms.get_mut(&room_id) else {
                        log!(
                            "BUG: room info not found for subscribe to typing notices request, room {room_id}"
                        );
                        continue;
                    };
                    let (main_timeline, receiver) = if subscribe {
                        if jrd.typing_notice_subscriber.is_some() {
                            warning!(
                                "Note: room {room_id} is already subscribed to typing notices."
                            );
                            continue;
                        } else {
                            let main_timeline = jrd.main_timeline.timeline.clone();
                            let (drop_guard, receiver) =
                                main_timeline.room().subscribe_to_typing_notifications();
                            jrd.typing_notice_subscriber = Some(drop_guard);
                            (main_timeline, receiver)
                        }
                    } else {
                        jrd.typing_notice_subscriber.take();
                        continue;
                    };
                    // Here: we don't have an existing subscriber running, so we fall through and start one.
                    (
                        main_timeline,
                        jrd.main_timeline.timeline_update_sender.clone(),
                        receiver,
                    )
                };

                let _typing_notices_task = Handle::current().spawn(async move {
                    while let Ok(user_ids) = typing_notice_receiver.recv().await {
                        // log!("Received typing notifications for room {room_id}: {user_ids:?}");
                        let users = join_all(user_ids.into_iter().map(|user_id| {
                            let tl = main_timeline.clone();
                            async move {
                                tl.room().get_member_no_sync(&user_id).await
                                    .ok().flatten()
                                    .and_then(|m| m.display_name().map(|d| d.to_owned()))
                                    .unwrap_or_else(|| user_id.to_string())
                            }
                        })).await;
                        if let Err(e) = timeline_update_sender.send(TimelineUpdate::TypingUsers { users }) {
                            error!("Error: timeline update sender couldn't send the list of typing users: {e:?}");
                        }
                        SignalToUI::set_ui_signal();
                    }
                    // log!("Note: typing notifications recv loop has ended for room {}", room_id);
                });
            }

            MatrixRequest::SubscribeToOwnUserReadReceiptsChanged {
                timeline_kind,
                subscribe,
            } => {
                if !subscribe {
                    if let Some(task_handler) =
                        subscribers_own_user_read_receipts.remove(&timeline_kind)
                    {
                        task_handler.abort();
                    }
                    continue;
                }
                let Some((timeline, sender)) = get_timeline_and_sender(&timeline_kind) else {
                    log!(
                        "BUG: skipping subscribe to own user read receipts changed request for {timeline_kind}"
                    );
                    continue;
                };

                let timeline_kind_clone = timeline_kind.clone();
                let subscribe_own_read_receipt_task = Handle::current().spawn(async move {
                    let update_receiver = timeline.subscribe_own_user_read_receipts_changed().await;
                    pin_mut!(update_receiver);
                    if let Some(client_user_id) = current_user_id() {
                        if let Some((event_id, receipt)) = timeline.latest_user_read_receipt(&client_user_id).await {
                            log!("Received own user read receipt for {timeline_kind}: {receipt:?}, event ID: {event_id:?}");
                            if sender.send(TimelineUpdate::OwnUserReadReceipt(receipt)).is_err() {
                                error!("Failed to send own user read receipt to UI.");
                            }
                        }

                        while update_receiver.next().await.is_some() {
                            if let Some((_, receipt)) = timeline.latest_user_read_receipt(&client_user_id).await {
                                if sender.send(TimelineUpdate::OwnUserReadReceipt(receipt)).is_err() {
                                    error!("Failed to send own user read receipt to UI.");
                                }
                                // When read receipts change (from other devices), update unread count
                                let unread_count = timeline.room().num_unread_messages();
                                let unread_mentions = timeline.room().num_unread_mentions();
                                if sender.send(TimelineUpdate::NewUnreadMessagesCount(
                                    UnreadMessageCount::Known(unread_count)
                                )).is_err() {
                                    error!("Failed to send unread message count update to UI.");
                                }
                                if let TimelineKind::MainRoom { room_id } = &timeline_kind {
                                    // Update the rooms list with new unread counts
                                    enqueue_rooms_list_update(RoomsListUpdate::UpdateNumUnreadMessages {
                                        room_id: room_id.clone(),
                                        is_marked_unread: timeline.room().is_marked_unread(),
                                        unread_messages: UnreadMessageCount::Known(unread_count),
                                        unread_mentions,
                                    });
                                }
                            }
                        }
                    }
                });
                subscribers_own_user_read_receipts
                    .insert(timeline_kind_clone, subscribe_own_read_receipt_task);
            }

            MatrixRequest::SubscribeToPinnedEvents { room_id, subscribe } => {
                if !subscribe {
                    if let Some(task_handler) = subscribers_pinned_events.remove(&room_id) {
                        task_handler.abort();
                    }
                    continue;
                }
                let kind = TimelineKind::MainRoom {
                    room_id: room_id.clone(),
                };
                let Some((main_timeline, sender)) = get_timeline_and_sender(&kind) else {
                    log!(
                        "BUG: skipping subscribe to pinned events request for unknown room {room_id}"
                    );
                    continue;
                };
                let subscribe_pinned_events_task = Handle::current().spawn(async move {
                    // Send an initial update, as the stream may not update immediately.
                    let pinned_events = main_timeline.room().pinned_event_ids().unwrap_or_default();
                    match sender.send(TimelineUpdate::PinnedEvents(pinned_events)) {
                        Ok(()) => SignalToUI::set_ui_signal(),
                        Err(_) => log!("Failed to send initial pinned events update to UI."),
                    }
                    let update_receiver = main_timeline.room().pinned_event_ids_stream();
                    pin_mut!(update_receiver);
                    while let Some(pinned_events) = update_receiver.next().await {
                        match sender.send(TimelineUpdate::PinnedEvents(pinned_events)) {
                            Ok(()) => SignalToUI::set_ui_signal(),
                            Err(e) => log!("Failed to send pinned events update: {e:?}"),
                        }
                    }
                });
                subscribers_pinned_events.insert(room_id, subscribe_pinned_events_task);
            }

            MatrixRequest::SpawnSSOServer {
                brand,
                homeserver_url,
                identity_provider_id,
            } => {
                spawn_sso_server(
                    brand,
                    homeserver_url,
                    identity_provider_id,
                    login_sender.clone(),
                )
                .await;
            }

            MatrixRequest::FetchAvatar {
                mxc_uri,
                on_fetched,
            } => {
                let Some(client) = get_client() else { continue };
                Handle::current().spawn(async move {
                    // log!("Sending fetch avatar request for {mxc_uri:?}...");
                    let media_request = MediaRequestParameters {
                        source: MediaSource::Plain(mxc_uri.clone()),
                        format: AVATAR_THUMBNAIL_FORMAT.into(),
                    };
                    let res = client.media().get_media_content(&media_request, true).await;
                    // log!("Fetched avatar for {mxc_uri:?}, succeeded? {}", res.is_ok());
                    on_fetched(AvatarUpdate {
                        mxc_uri,
                        avatar_data: res.map(|v| v.into()),
                    });
                });
            }

            MatrixRequest::FetchMedia {
                media_request,
                on_fetched,
                destination,
                update_sender,
            } => {
                let Some(client) = get_client() else { continue };

                let _fetch_task = Handle::current().spawn(async move {
                    // log!("Sending fetch media request for {media_request:?}...");
                    let res = client.media().get_media_content(&media_request, true).await;
                    on_fetched(&destination, media_request, res, update_sender);
                });
            }

            MatrixRequest::SaveMedia {
                media_request,
                destination_path,
                open_after_save,
                update_sender,
            } => {
                let Some(client) = get_client() else { continue };
                let source_key = match &media_request.source {
                    MediaSource::Plain(mxc_uri) => Some(mxc_uri.as_str().to_owned()),
                    MediaSource::Encrypted(_) => None,
                };

                let _save_media_task = Handle::current().spawn(async move {
                    log!("Saving media to {destination_path:?}...");
                    let send_save_result =
                        |destination_path: &Path, result: std::result::Result<(), String>| {
                            if let (Some(update_sender), Some(source_key)) =
                                (update_sender.as_ref(), source_key.as_ref())
                            {
                                let _ = update_sender.send(TimelineUpdate::MediaSaveResult {
                                    source_key: source_key.clone(),
                                    destination_path: destination_path.to_path_buf(),
                                    result,
                                });
                            }
                        };
                    match client.media().get_media_content(&media_request, true).await {
                        Ok(data) => match tokio::fs::write(&destination_path, &data).await {
                            Ok(()) => {
                                let open_outcome = if open_after_save {
                                    match url::Url::from_file_path(&destination_path) {
                                        Ok(file_url) => match Uri::new(file_url.as_str()).open() {
                                            Ok(()) => SaveMediaOpenOutcome::Opened,
                                            Err(error) => {
                                                SaveMediaOpenOutcome::Failed(format!("{error:?}"))
                                            }
                                        },
                                        Err(()) => SaveMediaOpenOutcome::InvalidPath,
                                    }
                                } else {
                                    SaveMediaOpenOutcome::NotRequested
                                };
                                let message = save_media_success_popup_message(
                                    &destination_path,
                                    open_outcome,
                                );
                                enqueue_popup_notification(message, PopupKind::Success, Some(4.0));
                                send_save_result(&destination_path, Ok(()));
                            }
                            Err(error) => {
                                let error = error.to_string();
                                send_save_result(&destination_path, Err(error.clone()));
                                enqueue_popup_notification(
                                    format!("Failed to save media: {error}"),
                                    PopupKind::Error,
                                    Some(4.0),
                                );
                            }
                        },
                        Err(error) => {
                            let error = error.to_string();
                            send_save_result(&destination_path, Err(error.clone()));
                            enqueue_popup_notification(
                                format!("Failed to download media: {error}"),
                                PopupKind::Error,
                                Some(4.0),
                            );
                        }
                    }
                });
            }

            MatrixRequest::SendMessage {
                timeline_kind,
                message,
                replied_to,
                #[cfg(feature = "tsp")]
                sign_with_tsp,
            } => {
                // TODO: use this timeline `_sender` once we support sending-message status/operations in the UI.
                let Some((timeline, _sender)) = get_timeline_and_sender(&timeline_kind) else {
                    log!("BUG: {timeline_kind} not found for send message request");
                    continue;
                };

                // Spawn a new async task that will send the actual message.
                let _send_message_task = Handle::current().spawn(async move {
                    log!("Sending message to {timeline_kind}: {message:?}...");
                    let message = {
                        #[cfg(not(feature = "tsp"))] {
                            message
                        }

                        #[cfg(feature = "tsp")] {
                            let mut message = message;
                            if sign_with_tsp {
                                log!("Signing message with TSP...");
                                match serde_json::to_vec(&message) {
                                    Ok(message_bytes) => {
                                        log!("Serialized message to bytes, length {}", message_bytes.len());
                                        match crate::tsp::sign_anycast_with_default_vid(&message_bytes) {
                                            Ok(signed_msg) => {
                                                log!("Successfully signed message with TSP, length {}", signed_msg.len());
                                                use matrix_sdk::ruma::serde::Base64;
                                                message.tsp_signature = Some(Base64::new(signed_msg));
                                            }
                                            Err(e) => {
                                                error!("Failed to sign message with TSP: {e:?}");
                                                enqueue_popup_notification(
                                                    format!("Failed to sign message with TSP: {e}"),
                                                    PopupKind::Error,
                                                    None,
                                                );
                                                return;
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        error!("Failed to serialize message to bytes for TSP signing: {e:?}");
                                        enqueue_popup_notification(
                                            format!("Failed to serialize message for TSP signing: {e}"),
                                            PopupKind::Error,
                                            None,
                                        );
                                        return;
                                    }
                                }
                            }
                            message
                        }
                    };

                    if let Some(replied_to_info) = replied_to {
                        let reply_content = match timeline
                            .room()
                            .make_reply_event(message.into(), replied_to_info)
                            .await
                        {
                            Ok(content) => content,
                            Err(_e) => {
                                error!("Failed to build reply content to send to {timeline_kind}: {_e:?}");
                                enqueue_popup_notification(
                                    format!("Failed to send reply: {_e}"),
                                    PopupKind::Error,
                                    None,
                                );
                                return;
                            }
                        };
                        match timeline.send(reply_content.into()).await {
                            Ok(_send_handle) => log!("Sent reply message to {timeline_kind}."),
                            Err(_e) => {
                                error!("Failed to send reply message to {timeline_kind}: {_e:?}");
                                enqueue_popup_notification(format!("Failed to send reply: {_e}"), PopupKind::Error, None);
                            }
                        }
                    } else {
                        match timeline.send(message.into()).await {
                            Ok(_send_handle) => log!("Sent message to {timeline_kind}."),
                            Err(_e) => {
                                error!("Failed to send message to {timeline_kind}: {_e:?}");
                                enqueue_popup_notification(format!("Failed to send message: {_e}"), PopupKind::Error, None);
                            }
                        }
                    }
                    SignalToUI::set_ui_signal();
                });
            }

            MatrixRequest::SendAttachment {
                timeline_kind,
                file_path,
                mime_type,
                caption,
                mentions,
                in_reply_to,
            } => {
                let Some((timeline, sender)) = get_timeline_and_sender(&timeline_kind) else {
                    log!("BUG: {timeline_kind} not found for send attachment request");
                    continue;
                };

                let _send_attachment_task = Handle::current().spawn(async move {
                    let filename = file_path
                        .file_name()
                        .and_then(|name| name.to_str())
                        .unwrap_or("attachment")
                        .to_string();
                    log!(
                        "Sending attachment to {timeline_kind}: {filename} ({mime_type}) from {file_path:?}..."
                    );
                    let config = AttachmentConfig {
                        caption,
                        mentions,
                        in_reply_to,
                        ..Default::default()
                    };
                    match timeline
                        .send_attachment(file_path, mime_type, config)
                        .use_send_queue()
                        .await
                    {
                        Ok(()) => {
                            log!("Queued attachment send to {timeline_kind}: {filename}.");
                            if sender
                                .send(TimelineUpdate::AttachmentSendResult {
                                    filename,
                                    result: Ok(()),
                                })
                                .is_err()
                            {
                                error!("Failed to send attachment queued result to UI.");
                            }
                        }
                        Err(_e) => {
                            error!("Failed to send attachment to {timeline_kind}: {_e:?}");
                            let error = format!("{_e}");
                            enqueue_popup_notification(
                                format!("Failed to send attachment: {error}"),
                                PopupKind::Error,
                                None,
                            );
                            if sender
                                .send(TimelineUpdate::AttachmentSendResult {
                                    filename,
                                    result: Err(error),
                                })
                                .is_err()
                            {
                                error!("Failed to send attachment failure result to UI.");
                            }
                        }
                    }
                    SignalToUI::set_ui_signal();
                });
            }

            MatrixRequest::AbortLocalSend {
                timeline_kind,
                send_handle,
            } => {
                let update_sender =
                    get_timeline_and_sender(&timeline_kind).map(|(_timeline, sender)| sender);
                let _abort_local_send_task = Handle::current().spawn(async move {
                    log!("Aborting local echo send queue item in {timeline_kind}...");
                    let abort_result = match send_handle.abort().await {
                        Ok(true) => {
                            enqueue_popup_notification(
                                "Queued local send canceled.",
                                PopupKind::Success,
                                Some(4.0),
                            );
                            Ok(true)
                        }
                        Ok(false) => {
                            enqueue_popup_notification(
                                "Queued local send was already sent or no longer cancellable.",
                                PopupKind::Warning,
                                Some(5.0),
                            );
                            Ok(false)
                        }
                        Err(error) => {
                            error!("Failed to abort local send in {timeline_kind}: {error:?}");
                            let error = format!("{error}");
                            enqueue_popup_notification(
                                format!("Failed to cancel queued local send: {error}"),
                                PopupKind::Error,
                                Some(6.0),
                            );
                            Err(error)
                        }
                    };
                    if let Some(update_sender) = update_sender {
                        if update_sender
                            .send(TimelineUpdate::LocalSendAbortResult {
                                result: abort_result,
                            })
                            .is_err()
                        {
                            error!("Failed to send local send abort result to UI.");
                        }
                    }
                    SignalToUI::set_ui_signal();
                });
            }

            MatrixRequest::ReadReceipt {
                timeline_kind,
                event_id,
                receipt_type,
            } => {
                let Some(timeline) = get_timeline(&timeline_kind) else {
                    log!("BUG: {timeline_kind} not found when sending read receipt, {event_id}");
                    continue;
                };

                let _send_rr_task = Handle::current().spawn(async move {
                    match timeline.send_single_receipt(receipt_type.clone(), event_id.clone()).await {
                        Ok(sent) => log!("{} {receipt_type} read receipt to {timeline_kind} for event {event_id}", if sent { "Sent" } else { "Already sent" }),
                        Err(_e) => error!("Failed to send {receipt_type} read receipt to {timeline_kind} for event {event_id}; error: {_e:?}"),
                    }
                    if let TimelineKind::MainRoom { room_id } = timeline_kind {
                        // Also update the number of unread messages in the room.
                        enqueue_rooms_list_update(RoomsListUpdate::UpdateNumUnreadMessages {
                            room_id,
                            is_marked_unread: timeline.room().is_marked_unread(),
                            unread_messages: UnreadMessageCount::Known(timeline.room().num_unread_messages()),
                            unread_mentions: timeline.room().num_unread_mentions()
                        });
                    }
                });
            }

            MatrixRequest::GetRoomPowerLevels { timeline_kind } => {
                let Some((timeline, sender)) = get_timeline_and_sender(&timeline_kind) else {
                    log!("BUG: {timeline_kind} not found for room power levels request");
                    continue;
                };

                let Some(user_id) = current_user_id() else {
                    continue;
                };

                let _power_levels_task = Handle::current().spawn(async move {
                    match timeline.room().power_levels().await {
                        Ok(power_levels) => {
                            log!("Successfully fetched power levels for {timeline_kind}.");
                            if sender
                                .send(TimelineUpdate::UserPowerLevels(UserPowerLevels::from(
                                    &power_levels,
                                    &user_id,
                                )))
                                .is_err()
                            {
                                error!("Failed to send room power levels to UI.")
                            }
                            SignalToUI::set_ui_signal();
                        }
                        Err(e) => {
                            error!("Failed to fetch power levels for {timeline_kind}: {e:?}");
                        }
                    }
                });
            }

            MatrixRequest::GetRoomNotificationMode { timeline_kind } => {
                let Some((timeline, sender)) = get_timeline_and_sender(&timeline_kind) else {
                    log!("BUG: {timeline_kind} not found for room notification mode request");
                    continue;
                };

                let _notification_mode_task = Handle::current().spawn(async move {
                    let mode = timeline.room().notification_mode().await;
                    if sender
                        .send(TimelineUpdate::RoomNotificationMode(mode))
                        .is_err()
                    {
                        error!("Failed to send room notification mode to UI.")
                    }
                    SignalToUI::set_ui_signal();
                });
            }

            MatrixRequest::GetNotificationKeywordRules { timeline_kind } => {
                let Some((timeline, sender)) = get_timeline_and_sender(&timeline_kind) else {
                    log!("BUG: {timeline_kind} not found for notification keyword rules request");
                    continue;
                };

                let _notification_keyword_rules_task = Handle::current().spawn(async move {
                    let notification_settings =
                        timeline.room().client().notification_settings().await;
                    let has_enabled_keywords = notification_settings.contains_keyword_rules().await;
                    let mut enabled_keywords: Vec<String> = notification_settings
                        .enabled_keywords()
                        .await
                        .into_iter()
                        .collect();
                    enabled_keywords.sort();
                    if sender
                        .send(TimelineUpdate::NotificationKeywordRulesFetched(
                            NotificationKeywordRulesSummary {
                                has_enabled_keywords,
                                enabled_keywords,
                            },
                        ))
                        .is_err()
                    {
                        error!("Failed to send notification keyword rules to UI.")
                    }
                    SignalToUI::set_ui_signal();
                });
            }

            MatrixRequest::SetNotificationKeywordRule {
                timeline_kind,
                keyword,
                mutation,
            } => {
                let Some((timeline, sender)) = get_timeline_and_sender(&timeline_kind) else {
                    log!("BUG: {timeline_kind} not found for notification keyword rule mutation");
                    continue;
                };

                let _notification_keyword_mutation_task = Handle::current().spawn(async move {
                    let notification_settings =
                        timeline.room().client().notification_settings().await;
                    let result = match mutation {
                        NotificationKeywordMutation::Add => {
                            notification_settings.add_keyword(keyword.clone()).await
                        }
                        NotificationKeywordMutation::Remove => {
                            notification_settings.remove_keyword(&keyword).await
                        }
                    };
                    let update_result = result.map_err(|error| format!("{error}"));
                    if sender
                        .send(TimelineUpdate::NotificationKeywordRulesMutated {
                            keyword,
                            mutation,
                            result: update_result,
                        })
                        .is_err()
                    {
                        error!("Failed to send notification keyword mutation result to UI.")
                    }
                    SignalToUI::set_ui_signal();
                });
            }

            MatrixRequest::GetNotificationPusherStatus { timeline_kind } => {
                let Some((timeline, sender)) = get_timeline_and_sender(&timeline_kind) else {
                    log!("BUG: {timeline_kind} not found for notification pusher status request");
                    continue;
                };

                let _notification_pusher_status_task = Handle::current().spawn(async move {
                    let encrypted_event_to_device_push = timeline
                        .room()
                        .client()
                        .can_homeserver_push_encrypted_event_to_device()
                        .await
                        .map_err(|error| format!("{error:?}"));
                    if sender
                        .send(TimelineUpdate::NotificationPusherStatusFetched(
                            NotificationPusherStatusSummary {
                                encrypted_event_to_device_push,
                            },
                        ))
                        .is_err()
                    {
                        error!("Failed to send notification pusher status to UI.")
                    }
                    SignalToUI::set_ui_signal();
                });
            }

            MatrixRequest::GetDefaultRoomNotificationMode { timeline_kind } => {
                let Some((timeline, sender)) = get_timeline_and_sender(&timeline_kind) else {
                    log!("BUG: {timeline_kind} not found for notification default mode request");
                    continue;
                };

                let _notification_default_mode_task = Handle::current().spawn(async move {
                    let room = timeline.room();
                    let notification_settings = room.client().notification_settings().await;
                    let result = match room.latest_encryption_state().await {
                        Ok(encryption_state) => {
                            let is_encrypted = encryption_state.is_encrypted();
                            let active_members_count = room.active_members_count();
                            let is_one_to_one = active_members_count == 2;
                            let mode = notification_settings
                                .get_default_room_notification_mode(
                                    IsEncrypted::from(is_encrypted),
                                    IsOneToOne::from(is_one_to_one),
                                )
                                .await;
                            Ok(NotificationDefaultRoomModeSummary {
                                mode,
                                is_encrypted,
                                is_one_to_one,
                                active_members_count,
                            })
                        }
                        Err(error) => Err(format!("{error:?}")),
                    };
                    if sender
                        .send(TimelineUpdate::NotificationDefaultRoomModeFetched(result))
                        .is_err()
                    {
                        error!("Failed to send notification default mode to UI.")
                    }
                    SignalToUI::set_ui_signal();
                });
            }

            MatrixRequest::SetDefaultRoomNotificationMode {
                timeline_kind,
                mode,
            } => {
                let Some((timeline, sender)) = get_timeline_and_sender(&timeline_kind) else {
                    log!("BUG: {timeline_kind} not found for notification default mode mutation");
                    continue;
                };

                let _notification_default_mode_mutation_task =
                    Handle::current().spawn(async move {
                        let room = timeline.room();
                        let notification_settings = room.client().notification_settings().await;
                        let result = match room.latest_encryption_state().await {
                            Ok(encryption_state) => {
                                let is_encrypted = encryption_state.is_encrypted();
                                let active_members_count = room.active_members_count();
                                let is_one_to_one = active_members_count == 2;
                                let encrypted = IsEncrypted::from(is_encrypted);
                                let one_to_one = IsOneToOne::from(is_one_to_one);
                                match notification_settings
                                    .set_default_room_notification_mode(encrypted, one_to_one, mode)
                                    .await
                                {
                                    Ok(()) => {
                                        let mode = notification_settings
                                            .get_default_room_notification_mode(
                                                encrypted, one_to_one,
                                            )
                                            .await;
                                        Ok(NotificationDefaultRoomModeSummary {
                                            mode,
                                            is_encrypted,
                                            is_one_to_one,
                                            active_members_count,
                                        })
                                    }
                                    Err(error) => Err(format!("{error:?}")),
                                }
                            }
                            Err(error) => Err(format!("{error:?}")),
                        };
                        if sender
                            .send(TimelineUpdate::NotificationDefaultRoomModeMutated {
                                mode,
                                result,
                            })
                            .is_err()
                        {
                            error!("Failed to send notification default mode mutation to UI.")
                        }
                        SignalToUI::set_ui_signal();
                    });
            }

            MatrixRequest::ToggleReaction {
                timeline_kind,
                timeline_event_id,
                reaction,
            } => {
                let Some(timeline) = get_timeline(&timeline_kind) else {
                    log!("BUG: {timeline_kind} not found for toggle reaction request");
                    continue;
                };

                let _toggle_reaction_task = Handle::current().spawn(async move {
                    log!("Sending toggle reaction {reaction:?} to {timeline_kind}: ...");
                    match timeline
                        .toggle_reaction(&timeline_event_id, &reaction)
                        .await
                    {
                        Ok(_send_handle) => {
                            log!("Sent toggle reaction {reaction:?} to {timeline_kind}.");
                            SignalToUI::set_ui_signal();
                        }
                        Err(_e) => error!(
                            "Failed to send toggle reaction to {timeline_kind}; error: {_e:?}"
                        ),
                    }
                });
            }

            MatrixRequest::RedactMessage {
                timeline_kind,
                timeline_event_id,
                reason,
            } => {
                let Some(timeline) = get_timeline(&timeline_kind) else {
                    log!("BUG: {timeline_kind} not found for redact message request");
                    continue;
                };

                let _redact_task = Handle::current().spawn(async move {
                    match timeline.redact(&timeline_event_id, reason.as_deref()).await {
                        Ok(()) => log!("Successfully redacted message in {timeline_kind}."),
                        Err(e) => {
                            error!("Failed to redact message in {timeline_kind}; error: {e:?}");
                            enqueue_popup_notification(
                                format!("Failed to redact message. Error: {e}"),
                                PopupKind::Error,
                                None,
                            );
                        }
                    }
                });
            }

            MatrixRequest::ReportContent {
                timeline_kind,
                event_id,
                reason,
            } => {
                let Some((timeline, sender)) = get_timeline_and_sender(&timeline_kind) else {
                    log!("BUG: {timeline_kind} not found for report content request");
                    continue;
                };

                let _report_task = Handle::current().spawn(async move {
                    let result = timeline
                        .room()
                        .report_content(event_id.clone(), Some(reason))
                        .await
                        .map(|_| ())
                        .map_err(|e| format!("{e:?}"));
                    if sender
                        .send(TimelineUpdate::MessageReportResult { event_id, result })
                        .is_err()
                    {
                        error!("Failed to send message report result to UI.");
                    }
                    SignalToUI::set_ui_signal();
                });
            }

            MatrixRequest::PinEvent {
                timeline_kind,
                event_id,
                pin,
            } => {
                let Some((timeline, sender)) = get_timeline_and_sender(&timeline_kind) else {
                    log!("BUG: {timeline_kind} not found for pin event request");
                    continue;
                };

                let _pin_task = Handle::current().spawn(async move {
                    let room = timeline.room();
                    let result = if pin {
                        room.pin_event(&event_id).await
                    } else {
                        room.unpin_event(&event_id).await
                    };
                    match sender.send(TimelineUpdate::PinResult {
                        event_id,
                        pin,
                        result,
                    }) {
                        Ok(_) => SignalToUI::set_ui_signal(),
                        Err(_) => log!("Failed to send UI update for pin event."),
                    }
                });
            }

            MatrixRequest::GetUrlPreview {
                url,
                on_fetched,
                destination,
                update_sender,
            } => {
                // const MAX_LOG_RESPONSE_BODY_LENGTH: usize = 1000;
                // log!("Starting URL preview fetch for: {}", url);
                let _fetch_url_preview_task = Handle::current().spawn(async move {
                    let result: Result<LinkPreviewData, UrlPreviewError> = async {
                        // log!("Getting Matrix client for URL preview: {}", url);
                        let client = get_client().ok_or_else(|| {
                            // error!("Matrix client not available for URL preview: {}", url);
                            UrlPreviewError::ClientNotAvailable
                        })?;

                        let token = client.access_token().ok_or_else(|| {
                            // error!("Access token not available for URL preview: {}", url);
                            UrlPreviewError::AccessTokenNotAvailable
                        })?;
                        // Official Doc: https://spec.matrix.org/v1.11/client-server-api/#get_matrixclientv1mediapreview_url
                        // Element desktop is using /_matrix/media/v3/preview_url
                        let mut endpoint_url = client
                            .homeserver()
                            .join("/_matrix/client/v1/media/preview_url")
                            .map_err(UrlPreviewError::UrlParse)?;
                        endpoint_url
                            .query_pairs_mut()
                            .append_pair("url", url.as_str());
                        // log!("Fetching URL preview from endpoint: {} for URL: {}", endpoint_url, url);

                        let response = client
                            .http_client()
                            .get(endpoint_url.clone())
                            .bearer_auth(token)
                            .header("Content-Type", "application/json")
                            .send()
                            .await
                            .map_err(|e| {
                                // error!("HTTP request failed for URL preview {}: {}", url, e);
                                UrlPreviewError::Request(e)
                            })?;

                        let status = response.status();
                        // log!("URL preview response status for {}: {}", url, status);

                        if !status.is_success() && status.as_u16() != 429 {
                            // error!("URL preview request failed with status {} for URL: {}", status, url);
                            return Err(UrlPreviewError::HttpStatus(status.as_u16()));
                        }

                        let text = response.text().await.map_err(|e| {
                            // error!("Failed to read response text for URL preview {}: {}", url, e);
                            UrlPreviewError::Request(e)
                        })?;

                        // log!("URL preview response body length for {}: {} bytes", url, text.len());
                        // if text.len() > MAX_LOG_RESPONSE_BODY_LENGTH {
                        //     log!("URL preview response body preview for {}: {}...", url, &text[..MAX_LOG_RESPONSE_BODY_LENGTH]);
                        // } else {
                        //     log!("URL preview response body for {}: {}", url, text);
                        // }
                        // This request is rate limited, retry after a duration we get from the server.
                        if status.as_u16() == 429 {
                            let link_preview_429_res =
                                serde_json::from_str::<LinkPreviewRateLimitResponse>(&text)
                                    .map_err(|e| {
                                        // error!("Failed to parse as LinkPreviewRateLimitResponse for URL preview {}: {}", url, e);
                                        UrlPreviewError::Json(e)
                                    });
                            match link_preview_429_res {
                                Ok(link_preview_429_res) => {
                                    if let Some(retry_after) = link_preview_429_res.retry_after_ms {
                                        tokio::time::sleep(Duration::from_millis(
                                            retry_after.into(),
                                        ))
                                        .await;
                                        submit_async_request(MatrixRequest::GetUrlPreview {
                                            url: url.clone(),
                                            on_fetched,
                                            destination: destination.clone(),
                                            update_sender: update_sender.clone(),
                                        });
                                    }
                                }
                                Err(_e) => {
                                    // error!("Failed to parse as LinkPreviewRateLimitResponse for URL preview {}: {}", url, _e);
                                }
                            }
                            return Err(UrlPreviewError::HttpStatus(429));
                        }
                        serde_json::from_str::<LinkPreviewData>(&text)
                            .or_else(|_first_error| {
                                // log!("Failed to parse as LinkPreviewData, trying LinkPreviewDataNonNumeric for URL: {}", url);
                                serde_json::from_str::<LinkPreviewDataNonNumeric>(&text)
                                    .map(|non_numeric| non_numeric.into())
                            })
                            .map_err(|e| {
                                // error!("Failed to parse JSON response for URL preview {}: {}", url, e);
                                // error!("Response body that failed to parse: {}", text);
                                UrlPreviewError::Json(e)
                            })
                    }
                    .await;

                    // match &result {
                    //     Ok(preview_data) => {
                    //         log!("Successfully fetched URL preview for {}: title: {:?}, site_name: {:?}",
                    //              url, preview_data.title, preview_data.site_name);
                    //     }
                    //     Err(e) => {
                    //         error!("URL preview fetch failed for {}: {}", url, e);
                    //     }
                    // }

                    on_fetched(url, destination, result, update_sender);
                    SignalToUI::set_ui_signal();
                });
            }
        }
    }

    error!("matrix_worker_task task ended unexpectedly");
    bail!("matrix_worker_task task ended unexpectedly")
}
