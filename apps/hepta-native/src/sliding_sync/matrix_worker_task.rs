/// The entry point for the worker task that runs Matrix-related operations.
///
/// All this task does is wait for [`MatrixRequests`] from the main UI thread
/// and then executes them within an async runtime context.
async fn matrix_worker_task(
    mut request_receiver: MatrixRequestReceiver,
    login_sender: Sender<LoginRequest>,
) -> Result<()> {
    log!("Started matrix_worker_task.");

    // The async tasks that are spawned to subscribe to changes in our own user's read receipts for each timeline.
    let mut subscribers_own_user_read_receipts: HashMap<TimelineKind, JoinHandle<()>> =
        HashMap::new();
    // The async tasks that are spawned to subscribe to changes in the pinned events for each room.
    let mut subscribers_pinned_events: HashMap<OwnedRoomId, JoinHandle<()>> = HashMap::new();
    // The async tasks spawned to handle media downloads, keyed by MxcUri.
    // Here we intentionally use a `std` Mutex, not async, since it's cheaper under no contention.
    let download_tasks: Arc<Mutex<HashMap<OwnedMxcUri, ActiveDownload>>> =
        Arc::new(Mutex::new(HashMap::new()));

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
                    if send_timeline_update(&sender, TimelineUpdate::PaginationRunning(direction)).is_err() {
                        return;
                    }

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
                            let _ = send_timeline_update(&sender, TimelineUpdate::PaginationIdle {
                                fully_paginated,
                                direction,
                            });
                        }
                        Err(error) => {
                            error!("Error sending {direction} pagination request for {timeline_kind}: {error:?}");
                            let _ = send_timeline_update(&sender, TimelineUpdate::PaginationError {
                                error,
                                direction,
                            });
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
                    let _ = send_timeline_update(&sender, TimelineUpdate::MessageEdited {
                        timeline_event_item_id,
                        result,
                    });
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
                    if send_timeline_update(
                        &sender,
                        TimelineUpdate::EventDetailsFetched { event_id, result },
                    )
                    .is_err()
                    {
                        error!("Failed to send fetched event details to UI for {timeline_kind}");
                    }
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

                    if send_timeline_update(&sender, TimelineUpdate::ThreadSummaryDetailsFetched {
                        thread_root_event_id,
                        timeline_item_index,
                        num_replies,
                        latest_reply_preview_text,
                    }).is_err() {
                        error!("Failed to send fetched thread summary details to UI for {timeline_kind}");
                    }
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
                    let _ = send_timeline_update(&sender, TimelineUpdate::RoomMembersSynced);
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
                            if !room_info.pending_thread_timelines.remove(&thread_root_event_id) {
                                log!("Thread-focused timeline for room {room_id}, thread {thread_root_event_id} was closed during creation; discarding it.");
                                return;
                            }
                            log!("Successfully created thread-focused timeline for room {room_id}, thread {thread_root_event_id}.");
                            let thread_timeline = Arc::new(thread_timeline);
                            let (timeline_update_sender, timeline_update_receiver) = timeline_update_channel();
                            let (request_sender, request_receiver) = watch::channel(TimelineRequest {
                                backwards_paginate: Vec::new(),
                                is_timeline_open: true,
                                resync_generation: 0,
                            });
                            let timeline_subscriber_handler_task = Handle::current().spawn(
                                timeline_subscriber_handler(
                                    thread_timeline.clone(),
                                    timeline_update_sender.clone(),
                                    request_receiver,
                                    Some(thread_root_event_id.clone()),
                                )
                            );
                            room_info.thread_timelines.insert(
                                thread_root_event_id.clone(),
                                PerTimelineDetails {
                                    timeline: thread_timeline,
                                    thread_root_event_id: Some(thread_root_event_id),
                                    timeline_update_sender,
                                    timeline_singleton_endpoints: Some((
                                        timeline_update_receiver,
                                        request_sender,
                                    )),
                                    timeline_subscriber: TimelineSubscriber::Running(
                                        timeline_subscriber_handler_task,
                                    ),
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

            MatrixRequest::CloseThreadTimeline {
                room_id,
                thread_root_event_id,
            } => {
                let mut all_joined_rooms = ALL_JOINED_ROOMS.lock().unwrap();
                let Some(room_info) = all_joined_rooms.get_mut(&room_id) else {
                    continue;
                };
                // Remove it from the pending set to handle the rare case where we showed the
                // thread timeline but then quickly hid it before its backend task could finish being set up.
                room_info
                    .pending_thread_timelines
                    .remove(&thread_root_event_id);
                // Remove and drop the entry (see [`PerTimelineDetails::drop()`] to abort its async task.
                if room_info
                    .thread_timelines
                    .remove(&thread_root_event_id)
                    .is_some()
                {
                    log!(
                        "Closed thread timeline for room {room_id}, thread {thread_root_event_id}."
                    );
                }
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
                        if send_timeline_update(
                            &sender,
                            TimelineUpdate::RoomMembersListFetched { members },
                        )
                        .is_err()
                        {
                            warning!("Room member update receiver closed for {timeline_kind}");
                        }
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

            MatrixRequest::GetMatchingRooms {
                query,
                request_id,
                owner,
            } => {
                let Some(client) = get_client() else { continue };
                let _match_task = Handle::current().spawn(async move {
                    let items = rank_matching_rooms(&client, &query).await;
                    Cx::post_action(MentionMatches::new(request_id, owner, items));
                });
            }

            MatrixRequest::GetNumberUnreadMessages { timeline_kind } => {
                let Some((timeline, sender)) = get_timeline_and_sender(&timeline_kind) else {
                    log!("Skipping get number of unread messages request for {timeline_kind}");
                    continue;
                };

                let _get_unreads_task = Handle::current().spawn(async move {
                    if send_timeline_update(&sender, TimelineUpdate::NewUnreadMessagesCount(
                        UnreadMessageCount::Known(timeline.room().num_unread_messages())
                    )).is_err() {
                        log!("Failed to send timeline update for GetNumberUnreadMessages request for {timeline_kind}");
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

            MatrixRequest::RequestSelfVerification => {
                let Some(client) = get_client() else { continue };
                let _verify_task = Handle::current().spawn(
                    crate::verification::request_self_verification_handler(client),
                );
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
                                tl.room()
                                    .get_member_no_sync(&user_id)
                                    .await
                                    .ok()
                                    .flatten()
                                    .and_then(|m| m.display_name().map(|d| d.to_owned()))
                                    .unwrap_or_else(|| user_id.to_string())
                            }
                        }))
                        .await;
                        match send_timeline_update(
                            &timeline_update_sender,
                            TimelineUpdate::TypingUsers { users },
                        ) {
                            Ok(()) | Err(TimelineUpdateSendError::Backpressured) => {}
                            Err(
                                TimelineUpdateSendError::Closed
                                | TimelineUpdateSendError::DeliveryLost,
                            ) => {
                                log!("Typing update transport ended for room {room_id}");
                                break;
                            }
                        }
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
                            match send_timeline_update(&sender, TimelineUpdate::OwnUserReadReceipt(receipt)) {
                                Ok(()) | Err(TimelineUpdateSendError::Backpressured) => {}
                                Err(TimelineUpdateSendError::Closed | TimelineUpdateSendError::DeliveryLost) => {
                                    return;
                                }
                            }
                        }

                        while update_receiver.next().await.is_some() {
                            if let Some((_, receipt)) = timeline.latest_user_read_receipt(&client_user_id).await {
                                match send_timeline_update(&sender, TimelineUpdate::OwnUserReadReceipt(receipt)) {
                                    Ok(()) | Err(TimelineUpdateSendError::Backpressured) => {}
                                    Err(TimelineUpdateSendError::Closed | TimelineUpdateSendError::DeliveryLost) => {
                                        break;
                                    }
                                }
                                // When read receipts change (from other devices), update unread count
                                let unread_count = timeline.room().num_unread_messages();
                                let unread_mentions = timeline.room().num_unread_mentions();
                                match send_timeline_update(&sender, TimelineUpdate::NewUnreadMessagesCount(
                                    UnreadMessageCount::Known(unread_count)
                                )) {
                                    Ok(()) | Err(TimelineUpdateSendError::Backpressured) => {}
                                    Err(TimelineUpdateSendError::Closed | TimelineUpdateSendError::DeliveryLost) => {
                                        break;
                                    }
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
                    if let Err(error) =
                        send_timeline_update(&sender, TimelineUpdate::PinnedEvents(pinned_events))
                    {
                        log!("Failed to send initial pinned events update to UI: {error}");
                    }
                    let update_receiver = main_timeline.room().pinned_event_ids_stream();
                    pin_mut!(update_receiver);
                    while let Some(pinned_events) = update_receiver.next().await {
                        match send_timeline_update(
                            &sender,
                            TimelineUpdate::PinnedEvents(pinned_events),
                        ) {
                            Ok(()) | Err(TimelineUpdateSendError::Backpressured) => {}
                            Err(
                                TimelineUpdateSendError::Closed
                                | TimelineUpdateSendError::DeliveryLost,
                            ) => {
                                log!("Pinned-events update transport ended");
                                break;
                            }
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

            MatrixRequest::FetchRoomAvatar { room_name_id } => {
                let Some(client) = get_client() else { continue };
                let Some(room) = client.get_room(room_name_id.room_id()) else {
                    log!(
                        "Skipping avatar fetch for unknown room {}",
                        room_name_id.room_id()
                    );
                    continue;
                };
                spawn_fetch_room_avatar_inner(room, room_name_id);
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

            MatrixRequest::SendAttachment { upload_id, upload } => {
                let timeline_kind = upload.timeline_kind.clone();
                let Some((timeline, sender)) = get_timeline_and_sender(&timeline_kind) else {
                    log!("BUG: {timeline_kind} not found for send attachment request");
                    enqueue_popup_notification(
                        "Cannot upload file: timeline not available.",
                        PopupKind::Error,
                        None,
                    );
                    SignalToUI::set_ui_signal();
                    continue;
                };

                #[cfg(feature = "tsp")]
                if upload.sign_with_tsp {
                    let _ = send_timeline_update(
                        &sender,
                        TimelineUpdate::FileUploadError {
                            upload_id,
                            error: "TSP-signed attachment uploads are not supported yet."
                                .to_string(),
                            upload,
                            retryable: false,
                        },
                    );
                    continue;
                }

                let sender_clone = sender.clone();
                let progress_sender = sender.clone();
                let monitor_timeline_kind = timeline_kind.clone();
                let (abort_handle, abort_registration) =
                    futures_util::future::AbortHandle::new_pair();
                // Spawn a new async task to send the attachment.
                let _send_attachment_task = Handle::current().spawn(async move {
                    use matrix_sdk::attachment::{
                        AttachmentInfo,
                        BaseFileInfo, BaseImageInfo, BaseVideoInfo, BaseAudioInfo,
                    };
                    use matrix_sdk_ui::timeline::AttachmentConfig as TimelineAttachmentConfig;

                    let upload_future = async move {
                        if send_timeline_update(&sender_clone, TimelineUpdate::FileUploadStarted {
                            upload_id,
                            file_name: upload.file_data.file_name(),
                            in_reply_to: upload.in_reply_to.clone(),
                            abort_handle,
                        }).is_err() {
                            return;
                        }

                        let max_upload_size = match get_client() {
                            Some(client) => match client.load_or_fetch_max_upload_size().await {
                                Ok(max_upload_size) => Some(max_upload_size),
                                Err(e) => {
                                    warning!("Could not fetch homeserver max upload size for {timeline_kind}: {e:?}; continuing without a local size-limit check.");
                                    None
                                }
                            },
                            None => {
                                warning!("Could not fetch homeserver max upload size for {timeline_kind}: client unavailable; continuing without a local size-limit check.");
                                None
                            }
                        };
                        if let Some(max_upload_size) = max_upload_size {
                            let exceeds_max_upload_size = matrix_sdk::ruma::UInt::try_from(upload.file_data.size)
                                .map(|upload_size| upload_size > max_upload_size)
                                .unwrap_or(true);
                            if exceeds_max_upload_size {
                                let max_size: u64 = max_upload_size.into();
                                let error = format!(
                                    "file size of ({}) exceeds the homeserver's {} limit.",
                                    utils::format_decimal_file_size(upload.file_data.size),
                                    utils::format_decimal_file_size(max_size),
                                );
                                let _ = send_timeline_update(&sender_clone, TimelineUpdate::FileUploadError {
                                    upload_id,
                                    error,
                                    upload,
                                    retryable: false,
                                });
                                return;
                            }
                        }

                        let upload_for_error = upload.clone();
                        let AttachmentUpload {
                            file_data,
                            in_reply_to,
                            ..
                        } = upload;

                        log!(
                            "Sending attachment to {timeline_kind}: {} ({} bytes)...",
                            file_data.file_name(),
                            file_data.size,
                        );

                        // Parse MIME type, falling back to octet-stream for unknown types
                        let content_type: Mime = file_data.mime_type.parse()
                            .unwrap_or(mime::APPLICATION_OCTET_STREAM);

                        let image_dimensions: Option<(u32, u32)> = if content_type.type_() == mime::IMAGE {
                            crate::image_utils::read_image_dimensions(file_data.path())
                                .map(|(w, h)| (w as u32, h as u32))
                        } else {
                            None
                        };
                        let matrix_file_size = || matrix_sdk::ruma::UInt::try_from(file_data.size).ok();

                        // Create AttachmentInfo based on the MIME type
                        let info = match content_type.type_() {
                            mime::IMAGE => AttachmentInfo::Image(BaseImageInfo {
                                width: image_dimensions.map(|(width, _height)| width.into()),
                                height: image_dimensions.map(|(_width, height)| height.into()),
                                size: matrix_file_size(),
                                blurhash: None,
                                is_animated: None,
                            }),
                            mime::VIDEO => AttachmentInfo::Video(BaseVideoInfo {
                                // TODO: Extract actual dimensions and duration from video
                                width: None,
                                height: None,
                                duration: None,
                                size: matrix_file_size(),
                                blurhash: None,
                            }),
                            mime::AUDIO => AttachmentInfo::Audio(BaseAudioInfo {
                                // TODO: Extract actual duration from audio
                                duration: None,
                                size: matrix_file_size(),
                                waveform: None,
                            }),
                            _ => AttachmentInfo::File(BaseFileInfo {
                                size: matrix_file_size(),
                            }),
                        };

                        let send_request = timeline.send_attachment(
                            file_data.path().to_path_buf(),
                            content_type,
                            TimelineAttachmentConfig {
                                info: Some(info),
                                caption: file_data.caption.as_ref().map(TextMessageEventContent::plain),
                                in_reply_to,
                                ..Default::default()
                            },
                        );
                        let progress_subscriber = send_request.subscribe_to_send_progress();
                        // Spawn a task to handle progress updates
                        Handle::current().spawn(async move {
                            let mut subscriber = progress_subscriber;
                            loop {
                                let progress = subscriber.get();
                                let current: u64 = progress.current as u64;
                                let total: u64 = progress.total as u64;
                                if send_timeline_update(&progress_sender, TimelineUpdate::FileUploadUpdate {
                                    upload_id,
                                    current,
                                    total,
                                }).is_err() {
                                    break;
                                }
                                // Wait for next update
                                if subscriber.next().await.is_none() {
                                    break;
                                }
                            }
                        });

                        match send_request.await {
                            Ok(()) => {
                                log!("Successfully sent attachment to {timeline_kind}.");
                                let _ = send_timeline_update(&sender_clone, TimelineUpdate::FileUploadComplete {
                                    upload_id,
                                });
                            }
                            Err(e) => {
                                error!("Failed to send attachment to {timeline_kind}: {e:?}");
                                let _ = send_timeline_update(&sender_clone, TimelineUpdate::FileUploadError {
                                    upload_id,
                                    error: format!("{e}"),
                                    upload: upload_for_error,
                                    retryable: true,
                                });
                            }
                        }

                    };

                    match Abortable::new(upload_future, abort_registration).await {
                        Ok(()) => {}
                        Err(_) => {
                            log!("Attachment upload task {upload_id:?} for {monitor_timeline_kind} was aborted.");
                        }
                    }
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
                            if send_timeline_update(
                                &sender,
                                TimelineUpdate::UserPowerLevels(UserPowerLevels::from(
                                    &power_levels,
                                    &user_id,
                                )),
                            )
                            .is_err()
                            {
                                error!("Failed to send room power levels to UI.")
                            }
                        }
                        Err(e) => {
                            error!("Failed to fetch power levels for {timeline_kind}: {e:?}");
                        }
                    }
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
                    if send_timeline_update(
                        &sender,
                        TimelineUpdate::PinResult {
                            event_id,
                            pin,
                            result,
                        },
                    )
                    .is_err()
                    {
                        log!("Failed to send UI update for pin event.");
                    }
                });
            }

            MatrixRequest::GetUrlPreview {
                url,
                on_fetched,
                destination,
                update_sender,
            } => {
                let _fetch_url_preview_task = Handle::current().spawn(async move {
                    let result: Result<LinkPreviewData, UrlPreviewError> = async {
                        let client = get_client().ok_or(UrlPreviewError::ClientNotAvailable)?;
                        let request = get_media_preview::v1::Request::new(url);
                        let response = client
                            .send(request)
                            .await
                            .map_err(UrlPreviewError::Request)?;
                        match response.data {
                            Some(raw) => serde_json::from_str::<LinkPreviewData>(raw.get())
                                .map_err(UrlPreviewError::Json),
                            None => Ok(LinkPreviewData::default()),
                        }
                    }
                    .await;

                    on_fetched(destination, result, update_sender);
                    SignalToUI::set_ui_signal();
                });
            }

            MatrixRequest::DownloadMedia {
                media_source,
                filename,
                on_download_result,
            } => {
                use crate::shared::attachment_download::{
                    enqueue_already_downloading_notification, MediaDownloadResult,
                };

                // Note: in this code block, we always want to call `on_download_result` with any error.

                let Some(client) = get_client() else {
                    on_download_result(MediaDownloadResult::Failed(
                        "Matrix client is not available".to_string(),
                    ));
                    continue;
                };
                let mxc_uri = media_source_mxc(&media_source).clone();
                // Only allow a given MxcUri to be downloaded once at a time.
                if download_tasks.lock().unwrap().contains_key(&mxc_uri) {
                    enqueue_already_downloading_notification();
                    on_download_result(MediaDownloadResult::Cancelled);
                    continue;
                }
                let (abort_handle, abort_registration) =
                    futures_util::future::AbortHandle::new_pair();
                let download_tasks2 = download_tasks.clone();
                let mxc_uri2 = mxc_uri.clone();
                let download_future = async move {
                    let media_request = MediaRequestParameters {
                        source: media_source,
                        format: matrix_sdk::media::MediaFormat::File,
                    };
                    let res = match client.media().get_media_content(&media_request, true).await {
                        Ok(bytes) => {
                            log!(
                                "Downloaded attachment {filename:?} ({} bytes) to memory",
                                bytes.len()
                            );
                            Ok(bytes)
                        }
                        Err(e) => {
                            error!(
                                "Failed to fetch media content for attachment {filename:?}: {e}"
                            );
                            Err(e.to_string())
                        }
                    };
                    if let Some(active) = download_tasks2.lock().unwrap().remove(&mxc_uri2) {
                        (active.on_download_result)(match res {
                            Ok(bytes) => MediaDownloadResult::Downloaded(bytes),
                            Err(e) => MediaDownloadResult::Failed(e),
                        });
                    }
                };

                let download_tasks3 = download_tasks.clone();
                let mxc_uri3 = mxc_uri.clone();
                download_tasks.lock().unwrap().insert(
                    mxc_uri,
                    ActiveDownload {
                        abort_handle,
                        on_download_result,
                    },
                );
                Handle::current().spawn(async move {
                    if Abortable::new(download_future, abort_registration)
                        .await
                        .is_err()
                    {
                        if let Some(active) = download_tasks3.lock().unwrap().remove(&mxc_uri3) {
                            (active.on_download_result)(MediaDownloadResult::Cancelled);
                        }
                    }
                });
            }

            MatrixRequest::CancelDownload(mxc) => {
                if let Some(active) = download_tasks.lock().unwrap().get(&mxc) {
                    active.abort_handle.abort();
                }
            }
        }
    }

    error!("matrix_worker_task task ended unexpectedly");
    bail!("matrix_worker_task task ended unexpectedly")
}
