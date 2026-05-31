pub mod add_to_world_channels;
pub mod create_orgs;
pub mod initialize_world;
pub mod next_iteration;
pub mod set_world_channel_override;
pub mod update_world_channel_perms;

#[cfg(test)]
mod world_tests {
    use crate::{
        actor::{player::WorldChannelOverride, state::State},
        channel::{ChannelPermission, ChannelPermissions},
        config::{role::Role, world::WorldChannelName},
        engine::Engine,
        helpers::get_channel,
        test_helpers::*,
    };

    fn world_channel_perms(
        eng: &Engine,
        name: WorldChannelName,
        player_id: crate::ID,
    ) -> ChannelPermissions {
        let channel_id = *eng.world.world_channel_map.get(&name).unwrap();
        get_channel(eng, channel_id)
            .unwrap()
            .get_member(player_id)
            .unwrap()
            .perms
    }

    // ---- initialization ----

    #[test]
    fn init_creates_world_channels() {
        let mut eng = Engine::new();
        init_world(&mut eng);

        assert!(eng.world.world_channel_map.contains_key(&WorldChannelName::News));
        assert!(eng.world.world_channel_map.contains_key(&WorldChannelName::Courtroom));
        assert!(eng.world.world_channel_map.contains_key(&WorldChannelName::General));
    }

    #[test]
    fn world_channels_are_loggable() {
        let mut eng = Engine::new();
        init_world(&mut eng);

        for channel_id in eng.world.world_channel_map.values().copied().collect::<Vec<_>>() {
            assert!(get_channel(&eng, channel_id).unwrap().loggable);
        }
    }

    // ---- membership ----

    #[test]
    fn player_added_to_all_world_channels() {
        let mut eng = Engine::new();
        init_world(&mut eng);
        let p1 = add_player(&mut eng, 0, Role::Civilian, "p1");

        for channel_id in eng.world.world_channel_map.values().copied().collect::<Vec<_>>() {
            assert!(get_channel(&eng, channel_id).unwrap().get_member(p1).is_some());
        }
    }

    // ---- default permissions ----

    #[test]
    fn default_perms_no_modifiers() {
        let mut eng = Engine::new();
        init_world(&mut eng);
        let p1 = add_player(&mut eng, 0, Role::Civilian, "p1");

        let news_perms = world_channel_perms(&eng, WorldChannelName::News, p1);
        assert!(news_perms.contains(ChannelPermission::View));
        assert!(!news_perms.contains(ChannelPermission::Send));

        let court_perms = world_channel_perms(&eng, WorldChannelName::Courtroom, p1);
        assert!(court_perms.contains(ChannelPermission::View));
        assert!(!court_perms.contains(ChannelPermission::Send));

        let gen_perms = world_channel_perms(&eng, WorldChannelName::General, p1);
        assert!(gen_perms.contains(ChannelPermission::Send));
        assert!(gen_perms.contains(ChannelPermission::View));
    }

    #[test]
    fn no_contact_removes_send_from_general() {
        let mut eng = Engine::new();
        init_world(&mut eng);
        let p1 = add_player(&mut eng, 0, Role::Civilian, "p1");

        add_state(&mut eng, 0, p1, State::Dead);

        assert!(!world_channel_perms(&eng, WorldChannelName::General, p1)
            .contains(ChannelPermission::Send));
    }

    #[test]
    fn no_presence_removes_view_from_news_and_courtroom() {
        let mut eng = Engine::new();
        init_world(&mut eng);
        let p1 = add_player(&mut eng, 0, Role::Civilian, "p1");

        add_state(&mut eng, 0, p1, State::Dead);

        assert!(!world_channel_perms(&eng, WorldChannelName::News, p1)
            .contains(ChannelPermission::View));
        assert!(!world_channel_perms(&eng, WorldChannelName::Courtroom, p1)
            .contains(ChannelPermission::View));
    }

    #[test]
    fn state_removal_restores_perms() {
        let mut eng = Engine::new();
        init_world(&mut eng);
        let p1 = add_player(&mut eng, 0, Role::Civilian, "p1");

        add_state(&mut eng, 0, p1, State::Dead);
        remove_state(&mut eng, 0, p1, State::Dead);

        let gen_perms = world_channel_perms(&eng, WorldChannelName::General, p1);
        assert!(gen_perms.contains(ChannelPermission::Send));
        assert!(gen_perms.contains(ChannelPermission::View));
    }

    // ---- overrides ----

    #[test]
    fn default_override_replaces_world_default() {
        let mut eng = Engine::new();
        init_world(&mut eng);
        let p1 = add_player(&mut eng, 0, Role::Civilian, "p1");

        set_world_channel_override(&mut eng, 0, p1, WorldChannelName::News, Some(WorldChannelOverride {
            default_perms: ChannelPermission::Send | ChannelPermission::View,
            force_perms: ChannelPermissions::EMPTY,
        })).unwrap();

        assert!(world_channel_perms(&eng, WorldChannelName::News, p1)
            .contains(ChannelPermission::Send));
    }

    #[test]
    fn default_override_still_blocked_by_modifiers() {
        let mut eng = Engine::new();
        init_world(&mut eng);
        let p1 = add_player(&mut eng, 0, Role::Civilian, "p1");

        set_world_channel_override(&mut eng, 0, p1, WorldChannelName::News, Some(WorldChannelOverride {
            default_perms: ChannelPermission::Send | ChannelPermission::View,
            force_perms: ChannelPermissions::EMPTY,
        })).unwrap();
        add_state(&mut eng, 0, p1, State::Dead);

        assert!(world_channel_perms(&eng, WorldChannelName::News, p1).is_empty());
    }

    #[test]
    fn force_override_bypasses_blocking() {
        let mut eng = Engine::new();
        init_world(&mut eng);
        let p1 = add_player(&mut eng, 0, Role::Civilian, "p1");

        set_world_channel_override(&mut eng, 0, p1, WorldChannelName::News, Some(WorldChannelOverride {
            default_perms: ChannelPermissions::EMPTY,
            force_perms: ChannelPermission::Send | ChannelPermission::View,
        })).unwrap();
        add_state(&mut eng, 0, p1, State::Dead);

        let perms = world_channel_perms(&eng, WorldChannelName::News, p1);
        assert!(perms.contains(ChannelPermission::Send));
        assert!(perms.contains(ChannelPermission::View));
    }

    #[test]
    fn clearing_override_reverts_to_world_default() {
        let mut eng = Engine::new();
        init_world(&mut eng);
        let p1 = add_player(&mut eng, 0, Role::Civilian, "p1");

        set_world_channel_override(&mut eng, 0, p1, WorldChannelName::News, Some(WorldChannelOverride {
            default_perms: ChannelPermission::Send | ChannelPermission::View,
            force_perms: ChannelPermissions::EMPTY,
        })).unwrap();
        set_world_channel_override(&mut eng, 0, p1, WorldChannelName::News, None).unwrap();

        let perms = world_channel_perms(&eng, WorldChannelName::News, p1);
        assert!(perms.contains(ChannelPermission::View));
        assert!(!perms.contains(ChannelPermission::Send));
    }

    // clearing a force override while blocking modifiers are active exposes the blocked state
    #[test]
    fn clearing_force_override_exposes_blocking_state() {
        let mut eng = Engine::new();
        init_world(&mut eng);
        let p1 = add_player(&mut eng, 0, Role::Civilian, "p1");

        set_world_channel_override(&mut eng, 0, p1, WorldChannelName::News, Some(WorldChannelOverride {
            default_perms: ChannelPermissions::EMPTY,
            force_perms: ChannelPermission::Send | ChannelPermission::View,
        })).unwrap();
        add_state(&mut eng, 0, p1, State::Dead);

        // force is active, blocking state has no effect
        assert!(!world_channel_perms(&eng, WorldChannelName::News, p1).is_empty());

        // clearing the override exposes the blocking state
        set_world_channel_override(&mut eng, 0, p1, WorldChannelName::News, None).unwrap();
        assert!(world_channel_perms(&eng, WorldChannelName::News, p1).is_empty());
    }
}
