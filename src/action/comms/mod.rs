pub mod bug;
pub mod channel;
pub mod groupchat;
pub mod lounge;
pub mod update_contact_channels;

#[cfg(test)]
mod comms_tests {
    use indexmap::indexset;

    use crate::{
        action::{
            Action, ActionActor, ActionRequest, ActionResponse,
            comms::{
                channel::set_loggable::SetLoggable, groupchat::create_groupchat::CreateGroupchat,
                lounge::create_lounge::CreateLounge,
            },
        },
        actor::state::State,
        channel::{ChannelMember, ChannelPermission, SenderDisplay},
        command::Command,
        config::role::Role,
        engine::Engine,
        helpers::{get_channel, get_gc, get_player},
        lounge::LoungeVariant,
        test_helpers::*,
    };

    // ---- channel ----

    #[test]
    fn set_member_adds_player() {
        let mut eng = Engine::new();
        let p1 = add_player(&mut eng, 0, Role::Civilian, "p1");
        let ch = create_channel(&mut eng, 0, false);

        set_member(
            &mut eng,
            0,
            p1,
            ch,
            Some(ChannelMember {
                perms: ChannelPermission::Send | ChannelPermission::View,
                displays: indexset![SenderDisplay::Raw(p1)],
            }),
        )
        .unwrap();

        assert!(get_channel(&eng, ch).unwrap().get_member(p1).is_some());
    }

    #[test]
    fn set_member_removes_player() {
        let mut eng = Engine::new();
        let p1 = add_player(&mut eng, 0, Role::Civilian, "p1");
        let ch = create_channel(&mut eng, 0, false);

        set_member(
            &mut eng,
            0,
            p1,
            ch,
            Some(ChannelMember {
                perms: ChannelPermission::Send | ChannelPermission::View,
                displays: indexset![SenderDisplay::Raw(p1)],
            }),
        )
        .unwrap();
        set_member(&mut eng, 0, p1, ch, None).unwrap();

        assert!(get_channel(&eng, ch).unwrap().get_member(p1).is_none());
    }

    #[test]
    fn set_member_emits_update_channel_view() {
        let mut eng = Engine::new();
        let p1 = add_player(&mut eng, 0, Role::Civilian, "p1");
        let ch = create_channel(&mut eng, 0, false);

        let (_, ctx) = set_member(
            &mut eng,
            0,
            p1,
            ch,
            Some(ChannelMember {
                perms: ChannelPermission::Send | ChannelPermission::View,
                displays: indexset![SenderDisplay::Raw(p1)],
            }),
        )
        .unwrap();

        assert!(ctx.commands.iter().any(|p| {
            p.recipient == Some(p1)
                && matches!(&p.cmd, Command::UpdateChannelView { channel_id, .. } if *channel_id == ch)
        }));
    }

    #[test]
    fn set_member_removal_emits_remove_channel() {
        let mut eng = Engine::new();
        let p1 = add_player(&mut eng, 0, Role::Civilian, "p1");
        let ch = create_channel(&mut eng, 0, false);

        set_member(
            &mut eng,
            0,
            p1,
            ch,
            Some(ChannelMember {
                perms: ChannelPermission::Send | ChannelPermission::View,
                displays: indexset![SenderDisplay::Raw(p1)],
            }),
        )
        .unwrap();

        let (_, ctx) = set_member(&mut eng, 0, p1, ch, None).unwrap();

        assert!(ctx.commands.iter().any(|p| {
            p.recipient == Some(p1)
                && matches!(&p.cmd, Command::RemoveChannel { channel_id } if *channel_id == ch)
        }));
    }

    #[test]
    fn set_loggable_toggles_flag() {
        let mut eng = Engine::new();
        let ch = create_channel(&mut eng, 0, false);

        assert!(!get_channel(&eng, ch).unwrap().loggable);

        eng.execute(ActionRequest {
            actor: ActionActor::System,
            timestamp: 0,
            payload: Action::SetLoggable(SetLoggable {
                channel_id: ch,
                loggable: true,
            }),
        })
        .unwrap();

        assert!(get_channel(&eng, ch).unwrap().loggable);
    }

    #[test]
    fn send_message_valid() {
        let mut eng = Engine::new();
        let p1 = add_player(&mut eng, 0, Role::Civilian, "p1");
        let ch = create_channel(&mut eng, 0, false);

        set_member(
            &mut eng,
            0,
            p1,
            ch,
            Some(ChannelMember {
                perms: ChannelPermission::Send | ChannelPermission::View,
                displays: indexset![SenderDisplay::Raw(p1)],
            }),
        )
        .unwrap();

        let (_, ctx) = send_message(&mut eng, 0, p1, ch, SenderDisplay::Raw(p1), "hello").unwrap();

        assert!(ctx.commands.iter().any(|p| {
            matches!(&p.cmd, Command::AddMessage { channel_id, content, sender_display }
                if *channel_id == ch
                    && content == "hello"
                    && *sender_display == SenderDisplay::Raw(p1))
        }));
    }

    #[test]
    fn send_message_not_a_member() {
        let mut eng = Engine::new();
        let p1 = add_player(&mut eng, 0, Role::Civilian, "p1");
        let ch = create_channel(&mut eng, 0, false);

        assert!(send_message(&mut eng, 0, p1, ch, SenderDisplay::Raw(p1), "hello").is_err());
    }

    #[test]
    fn send_message_no_send_perm() {
        let mut eng = Engine::new();
        let p1 = add_player(&mut eng, 0, Role::Civilian, "p1");
        let ch = create_channel(&mut eng, 0, false);

        set_member(
            &mut eng,
            0,
            p1,
            ch,
            Some(ChannelMember {
                perms: ChannelPermission::View.into(),
                displays: indexset![SenderDisplay::Raw(p1)],
            }),
        )
        .unwrap();

        assert!(send_message(&mut eng, 0, p1, ch, SenderDisplay::Raw(p1), "hello").is_err());
    }

    #[test]
    fn send_message_display_not_owned() {
        let mut eng = Engine::new();
        let p1 = add_player(&mut eng, 0, Role::Civilian, "p1");
        let p2 = add_player(&mut eng, 0, Role::Civilian, "p2");
        let ch = create_channel(&mut eng, 0, false);

        set_member(
            &mut eng,
            0,
            p1,
            ch,
            Some(ChannelMember {
                perms: ChannelPermission::Send | ChannelPermission::View,
                displays: indexset![SenderDisplay::Raw(p1)],
            }),
        )
        .unwrap();

        // p1 tries to send as p2 which they do not own
        assert!(send_message(&mut eng, 0, p1, ch, SenderDisplay::Raw(p2), "hello").is_err());
    }

    #[test]
    fn set_member_add_does_not_emit_remove_channel() {
        let mut eng = Engine::new();
        let p1 = add_player(&mut eng, 0, Role::Civilian, "p1");
        let ch = create_channel(&mut eng, 0, false);

        let (_, ctx) = set_member(
            &mut eng,
            0,
            p1,
            ch,
            Some(ChannelMember {
                perms: ChannelPermission::Send | ChannelPermission::View,
                displays: indexset![SenderDisplay::Raw(p1)],
            }),
        )
        .unwrap();

        assert!(
            !ctx.commands
                .iter()
                .any(|p| matches!(&p.cmd, Command::RemoveChannel { .. }))
        );
    }

    #[test]
    fn set_member_remove_does_not_emit_update_channel_view() {
        let mut eng = Engine::new();
        let p1 = add_player(&mut eng, 0, Role::Civilian, "p1");
        let ch = create_channel(&mut eng, 0, false);

        set_member(
            &mut eng,
            0,
            p1,
            ch,
            Some(ChannelMember {
                perms: ChannelPermission::Send | ChannelPermission::View,
                displays: indexset![SenderDisplay::Raw(p1)],
            }),
        )
        .unwrap();

        let (_, ctx) = set_member(&mut eng, 0, p1, ch, None).unwrap();

        assert!(
            !ctx.commands
                .iter()
                .any(|p| matches!(&p.cmd, Command::UpdateChannelView { .. }))
        );
    }

    // ---- groupchat ----

    #[test]
    fn create_groupchat_emits_map_gc() {
        let mut eng = Engine::new();

        let (response, ctx) = eng
            .execute(ActionRequest {
                actor: ActionActor::System,
                timestamp: 0,
                payload: Action::CreateGroupchat(CreateGroupchat {}),
            })
            .unwrap();

        let ActionResponse::CreateGroupchat(data) = response else {
            unreachable!()
        };
        let channel_id = get_gc(&eng, data.id).unwrap().channel_id;

        assert!(ctx.commands.iter().any(|p| {
            p.recipient.is_none()
                && matches!(&p.cmd, Command::MapGc { gc_id, channel_id: cid }
                    if *gc_id == data.id && *cid == channel_id)
        }));
    }

    #[test]
    fn add_to_groupchat_system() {
        let mut eng = Engine::new();
        let p1 = add_player(&mut eng, 0, Role::Civilian, "p1");
        let p2 = add_player(&mut eng, 0, Role::Civilian, "p2");
        let gc = create_gc(&mut eng, 0);

        // without owner flag: member and cache updated, no GcOwnerStatus emitted
        let (_, ctx) = add_to_gc(&mut eng, 0, ActionActor::System, gc, p1, false).unwrap();
        assert!(get_gc(&eng, gc).unwrap().contains_member(p1));
        assert!(get_player(&eng, p1).unwrap().groupchats.contains(&gc));
        assert!(
            !ctx.commands
                .iter()
                .any(|p| matches!(&p.cmd, Command::GcOwnerStatus { .. }))
        );

        // with owner flag: GcOwnerStatus{owner: true} emitted to new owner
        let (_, ctx) = add_to_gc(&mut eng, 0, ActionActor::System, gc, p2, true).unwrap();
        assert!(ctx.commands.iter().any(|p| {
            p.recipient == Some(p2)
                && matches!(&p.cmd, Command::GcOwnerStatus { owner: true, gc_id } if *gc_id == gc)
        }));
    }

    #[test]
    fn add_to_groupchat_as_owner_player() {
        let mut eng = Engine::new();
        let p1 = add_player(&mut eng, 0, Role::Civilian, "p1");
        let p2 = add_player(&mut eng, 0, Role::Civilian, "p2");
        let gc = create_gc(&mut eng, 0);

        add_to_gc(&mut eng, 0, ActionActor::System, gc, p1, true).unwrap();
        add_to_gc(&mut eng, 0, ActionActor::Player(p1), gc, p2, false).unwrap();

        assert!(get_gc(&eng, gc).unwrap().contains_member(p2));
    }

    #[test]
    fn remove_from_groupchat_as_owner_player() {
        let mut eng = Engine::new();
        let p1 = add_player(&mut eng, 0, Role::Civilian, "p1");
        let p2 = add_player(&mut eng, 0, Role::Civilian, "p2");
        let gc = create_gc(&mut eng, 0);

        add_to_gc(&mut eng, 0, ActionActor::System, gc, p1, true).unwrap();
        add_to_gc(&mut eng, 0, ActionActor::System, gc, p2, false).unwrap();
        remove_from_gc(&mut eng, 0, ActionActor::Player(p1), gc, p2).unwrap();

        assert!(!get_gc(&eng, gc).unwrap().contains_member(p2));
    }

    #[test]
    fn add_to_groupchat_non_owner_player() {
        let mut eng = Engine::new();
        let p1 = add_player(&mut eng, 0, Role::Civilian, "p1");
        let p2 = add_player(&mut eng, 0, Role::Civilian, "p2");
        let gc = create_gc(&mut eng, 0);

        assert!(add_to_gc(&mut eng, 0, ActionActor::Player(p1), gc, p2, false).is_err());
    }

    #[test]
    fn add_to_groupchat_target_no_contact() {
        let mut eng = Engine::new();
        let p1 = add_player(&mut eng, 0, Role::Civilian, "p1");
        let gc = create_gc(&mut eng, 0);

        add_state(&mut eng, 0, p1, State::Dead);

        assert!(add_to_gc(&mut eng, 0, ActionActor::System, gc, p1, false).is_err());
    }

    #[test]
    fn remove_from_groupchat_not_member() {
        let mut eng = Engine::new();
        let p1 = add_player(&mut eng, 0, Role::Civilian, "p1");
        let gc = create_gc(&mut eng, 0);

        assert!(remove_from_gc(&mut eng, 0, ActionActor::System, gc, p1).is_err());
    }

    #[test]
    fn set_gc_owner_emits_status_cmds() {
        let mut eng = Engine::new();
        let p1 = add_player(&mut eng, 0, Role::Civilian, "p1");
        let p2 = add_player(&mut eng, 0, Role::Civilian, "p2");
        let gc = create_gc(&mut eng, 0);

        add_to_gc(&mut eng, 0, ActionActor::System, gc, p1, true).unwrap();
        add_to_gc(&mut eng, 0, ActionActor::System, gc, p2, false).unwrap();

        let (_, ctx) = set_gc_owner(&mut eng, 0, ActionActor::System, gc, Some(p2)).unwrap();

        assert!(ctx.commands.iter().any(|p| {
            p.recipient == Some(p1)
                && matches!(&p.cmd, Command::GcOwnerStatus { owner: false, gc_id } if *gc_id == gc)
        }));
        assert!(ctx.commands.iter().any(|p| {
            p.recipient == Some(p2)
                && matches!(&p.cmd, Command::GcOwnerStatus { owner: true, gc_id } if *gc_id == gc)
        }));
    }

    // ---- lounge ----

    #[test]
    fn create_basic_lounge_participants_in_channel() {
        let mut eng = Engine::new();
        let p1 = add_player(&mut eng, 0, Role::Civilian, "p1");
        let p2 = add_player(&mut eng, 0, Role::Civilian, "p2");

        let (_, ch) = create_lounge(
            &mut eng,
            0,
            LoungeVariant::Basic {
                contactor_id: p1,
                contacted_id: p2,
            },
        );

        let channel = get_channel(&eng, ch).unwrap();
        assert!(channel.get_member(p1).is_some());
        assert!(channel.get_member(p2).is_some());
    }

    #[test]
    fn create_lounge_updates_player_caches() {
        let mut eng = Engine::new();
        let p1 = add_player(&mut eng, 0, Role::Civilian, "p1");
        let p2 = add_player(&mut eng, 0, Role::Civilian, "p2");

        let (lounge_id, _) = create_lounge(
            &mut eng,
            0,
            LoungeVariant::Basic {
                contactor_id: p1,
                contacted_id: p2,
            },
        );

        assert!(get_player(&eng, p1).unwrap().lounges.contains(&lounge_id));
        assert!(get_player(&eng, p2).unwrap().lounges.contains(&lounge_id));
    }

    #[test]
    fn create_lounge_emits_map_lounge() {
        let mut eng = Engine::new();
        let p1 = add_player(&mut eng, 0, Role::Civilian, "p1");
        let p2 = add_player(&mut eng, 0, Role::Civilian, "p2");

        let (response, ctx) = eng
            .execute(ActionRequest {
                actor: ActionActor::System,
                timestamp: 0,
                payload: Action::CreateLounge(CreateLounge {
                    variant: LoungeVariant::Basic {
                        contactor_id: p1,
                        contacted_id: p2,
                    },
                }),
            })
            .unwrap();

        let ActionResponse::CreateLounge(data) = response else {
            unreachable!()
        };

        assert!(ctx.commands.iter().any(|p| {
            p.recipient.is_none()
                && matches!(&p.cmd, Command::MapLounge { lounge_id, channel_id }
                    if *lounge_id == data.lounge_id && *channel_id == data.channel_id)
        }));
    }

    #[test]
    fn leave_lounge_removes_from_channel() {
        let mut eng = Engine::new();
        let p1 = add_player(&mut eng, 0, Role::Civilian, "p1");
        let p2 = add_player(&mut eng, 0, Role::Civilian, "p2");

        let (lounge_id, ch) = create_lounge(
            &mut eng,
            0,
            LoungeVariant::Basic {
                contactor_id: p1,
                contacted_id: p2,
            },
        );

        leave_lounge(&mut eng, 0, p1, lounge_id).unwrap();

        assert!(get_channel(&eng, ch).unwrap().get_member(p1).is_none());
    }

    #[test]
    fn leave_lounge_updates_player_cache() {
        let mut eng = Engine::new();
        let p1 = add_player(&mut eng, 0, Role::Civilian, "p1");
        let p2 = add_player(&mut eng, 0, Role::Civilian, "p2");

        let (lounge_id, _) = create_lounge(
            &mut eng,
            0,
            LoungeVariant::Basic {
                contactor_id: p1,
                contacted_id: p2,
            },
        );

        leave_lounge(&mut eng, 0, p1, lounge_id).unwrap();

        assert!(!get_player(&eng, p1).unwrap().lounges.contains(&lounge_id));
    }

    // ---- update_contact_channels ----

    #[test]
    fn no_contact_clears_lounge_perms() {
        let mut eng = Engine::new();
        let p1 = add_player(&mut eng, 0, Role::Civilian, "p1");
        let p2 = add_player(&mut eng, 0, Role::Civilian, "p2");

        let (_, ch) = create_lounge(
            &mut eng,
            0,
            LoungeVariant::Basic {
                contactor_id: p1,
                contacted_id: p2,
            },
        );

        add_state(&mut eng, 0, p1, State::Dead);

        assert!(
            get_channel(&eng, ch)
                .unwrap()
                .get_member(p1)
                .unwrap()
                .perms
                .is_empty()
        );
    }

    #[test]
    fn no_contact_cleared_restores_lounge_perms() {
        let mut eng = Engine::new();
        let p1 = add_player(&mut eng, 0, Role::Civilian, "p1");
        let p2 = add_player(&mut eng, 0, Role::Civilian, "p2");

        let (_, ch) = create_lounge(
            &mut eng,
            0,
            LoungeVariant::Basic {
                contactor_id: p1,
                contacted_id: p2,
            },
        );

        add_state(&mut eng, 0, p1, State::Dead);
        remove_state(&mut eng, 0, p1, State::Dead);

        let member = get_channel(&eng, ch)
            .unwrap()
            .get_member(p1)
            .unwrap()
            .clone();
        assert!(member.perms.contains(ChannelPermission::Send));
        assert!(member.perms.contains(ChannelPermission::View));
    }
}
