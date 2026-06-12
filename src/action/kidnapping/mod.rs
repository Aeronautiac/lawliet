pub mod create_kidnapping;
pub mod release_kidnapping;
pub mod update_kidnap_channels;

#[cfg(test)]
mod tests {
    use crate::{
        action::{Action, ActionActor, ActionError, ActionRequest, kidnapping::release_kidnapping::ReleaseKidnapping},
        actor::{ActorDisplay, state::State},
        channel::{ChannelPermission, ChannelPermissions},
        config::{actor::organization::OrganizationName, role::Role},
        engine::Engine,
        helpers::{get_actor, get_channel, get_kidnapping},
        kidnapping::KidnappingType,
        test_helpers::*,
    };

    // Basic creation: victim gets Kidnapped state and Send | View on the channel.
    #[test]
    fn create_sets_kidnapped_state_and_channel_perms() {
        let mut eng = Engine::new();
        let kidnapper = add_player(&mut eng, 0, Role::Civilian, "kidnapper");
        let victim = add_player(&mut eng, 0, Role::Civilian, "victim");

        let (_, ch) = create_kidnapping(&mut eng, 0, kidnapper, victim, KidnappingType::Anonymous);

        assert!(get_actor(&eng, victim).unwrap().has_state(State::Kidnapped));
        let member = get_channel(&eng, ch).unwrap().get_member(victim).cloned().unwrap();
        assert!(member.perms.contains(ChannelPermission::Send));
        assert!(member.perms.contains(ChannelPermission::View));
    }

    // Victim death drops their channel perms to EMPTY; the kidnapping itself persists.
    #[test]
    fn victim_perms_drop_on_death() {
        let mut eng = Engine::new();
        let kidnapper = add_player(&mut eng, 0, Role::Civilian, "kidnapper");
        let victim = add_player(&mut eng, 0, Role::Civilian, "victim");

        let (kid_id, ch) = create_kidnapping(&mut eng, 0, kidnapper, victim, KidnappingType::Anonymous);
        quick_kill(&mut eng, 1, false, false, false, victim);

        let member = get_channel(&eng, ch).unwrap().get_member(victim).cloned().unwrap();
        assert_eq!(member.perms, ChannelPermissions::EMPTY);
        assert!(get_kidnapping(&eng, kid_id).is_ok());
    }

    // Reviving the victim restores Send | View.
    #[test]
    fn victim_perms_restore_on_revive() {
        let mut eng = Engine::new();
        let kidnapper = add_player(&mut eng, 0, Role::Civilian, "kidnapper");
        let victim = add_player(&mut eng, 0, Role::Civilian, "victim");

        let (_, ch) = create_kidnapping(&mut eng, 0, kidnapper, victim, KidnappingType::Anonymous);
        quick_kill(&mut eng, 1, false, false, false, victim);
        quick_revive(&mut eng, 2, true, victim);

        let member = get_channel(&eng, ch).unwrap().get_member(victim).cloned().unwrap();
        assert!(member.perms.contains(ChannelPermission::Send));
        assert!(member.perms.contains(ChannelPermission::View));
    }

    // Killing the kidnapper leaves the kidnapping and victim perms fully intact.
    #[test]
    fn kidnapper_death_preserves_kidnapping() {
        let mut eng = Engine::new();
        let kidnapper = add_player(&mut eng, 0, Role::Civilian, "kidnapper");
        let victim = add_player(&mut eng, 0, Role::Civilian, "victim");

        let (kid_id, ch) = create_kidnapping(&mut eng, 0, kidnapper, victim, KidnappingType::Anonymous);
        quick_kill(&mut eng, 1, false, false, false, kidnapper);

        assert!(get_kidnapping(&eng, kid_id).is_ok());
        let member = get_channel(&eng, ch).unwrap().get_member(victim).cloned().unwrap();
        assert!(member.perms.contains(ChannelPermission::Send));
        assert!(member.perms.contains(ChannelPermission::View));
    }

    // When an org member (kidnapper side) dies, their channel perms drop to EMPTY.
    #[test]
    fn org_member_perms_drop_on_death() {
        let mut eng = Engine::new();
        let org = add_org(&mut eng, 0, OrganizationName::NULL);
        let member = add_player(&mut eng, 0, Role::Civilian, "member");
        let victim = add_player(&mut eng, 0, Role::Civilian, "victim");
        add_to_org(&mut eng, 0, org, member, false, true).unwrap();

        let (_, ch) = create_kidnapping(&mut eng, 0, org, victim, KidnappingType::Anonymous);

        let before = get_channel(&eng, ch).unwrap().get_member(member).cloned().unwrap();
        assert!(before.perms.contains(ChannelPermission::Send));

        quick_kill(&mut eng, 1, false, false, false, member);

        let after = get_channel(&eng, ch).unwrap().get_member(member).cloned().unwrap();
        assert_eq!(after.perms, ChannelPermissions::EMPTY);
    }

    // Anonymous kidnapping: org members appear as Mysterious.
    #[test]
    fn anon_org_member_displays_as_mysterious() {
        let mut eng = Engine::new();
        let org = add_org(&mut eng, 0, OrganizationName::NULL);
        let member = add_player(&mut eng, 0, Role::Civilian, "member");
        let victim = add_player(&mut eng, 0, Role::Civilian, "victim");
        add_to_org(&mut eng, 0, org, member, false, true).unwrap();

        let (_, ch) = create_kidnapping(&mut eng, 0, org, victim, KidnappingType::Anonymous);

        let m = get_channel(&eng, ch).unwrap().get_member(member).cloned().unwrap();
        assert!(m.displays.contains(&ActorDisplay::Mysterious));
        assert!(!m.displays.contains(&ActorDisplay::Raw(member)));
    }

    // Public kidnapping: org members appear as Raw(member_id).
    #[test]
    fn public_org_member_displays_as_raw() {
        let mut eng = Engine::new();
        let org = add_org(&mut eng, 0, OrganizationName::NULL);
        let member = add_player(&mut eng, 0, Role::Civilian, "member");
        let victim = add_player(&mut eng, 0, Role::Civilian, "victim");
        add_to_org(&mut eng, 0, org, member, false, true).unwrap();

        let (_, ch) = create_kidnapping(
            &mut eng, 0, org, victim,
            KidnappingType::Public(ActorDisplay::Raw(member)),
        );

        let m = get_channel(&eng, ch).unwrap().get_member(member).cloned().unwrap();
        assert!(m.displays.contains(&ActorDisplay::Raw(member)));
        assert!(!m.displays.contains(&ActorDisplay::Mysterious));
    }

    // Release removes Kidnapped state, destroys the channel, and removes the kidnapping record.
    #[test]
    fn release_cleans_up_state_and_channel() {
        let mut eng = Engine::new();
        let kidnapper = add_player(&mut eng, 0, Role::Civilian, "kidnapper");
        let victim = add_player(&mut eng, 0, Role::Civilian, "victim");

        let (kid_id, ch) = create_kidnapping(&mut eng, 0, kidnapper, victim, KidnappingType::Anonymous);
        release_kidnapping(&mut eng, 1, kid_id);

        assert!(!get_actor(&eng, victim).unwrap().has_state(State::Kidnapped));
        assert!(get_kidnapping(&eng, kid_id).is_err());
        assert!(get_channel(&eng, ch).is_err());
    }

    // A third-party player cannot release a kidnapping they did not perform.
    #[test]
    fn release_by_unrelated_player_is_rejected() {
        let mut eng = Engine::new();
        let kidnapper = add_player(&mut eng, 0, Role::Civilian, "kidnapper");
        let victim = add_player(&mut eng, 0, Role::Civilian, "victim");
        let other = add_player(&mut eng, 0, Role::Civilian, "other");

        let (kid_id, _) = create_kidnapping(&mut eng, 0, kidnapper, victim, KidnappingType::Anonymous);

        let result = eng.execute(ActionRequest {
            actor: ActionActor::Player(other),
            timestamp: 1,
            payload: Action::ReleaseKidnapping(ReleaseKidnapping { kidnapping_id: kid_id }),
        });
        assert!(matches!(result, Err(ActionError::InsufficientPermissions)));
    }

    // Dead players have NoPresence and cannot be kidnapped.
    #[test]
    fn cannot_kidnap_dead_player() {
        let mut eng = Engine::new();
        let kidnapper = add_player(&mut eng, 0, Role::Civilian, "kidnapper");
        let victim = add_player(&mut eng, 0, Role::Civilian, "victim");
        quick_kill(&mut eng, 0, false, false, false, victim);

        let result = eng.execute(ActionRequest {
            actor: ActionActor::System,
            timestamp: 1,
            payload: Action::CreateKidnapping(crate::action::kidnapping::create_kidnapping::CreateKidnapping {
                kidnapper_id: kidnapper,
                victim_id: victim,
                kidnapping_type: KidnappingType::Anonymous,
            }),
        });
        assert!(matches!(result, Err(ActionError::UserNotPresent)));
    }

    // Already-kidnapped players have NoPresence and cannot be kidnapped again.
    #[test]
    fn cannot_kidnap_already_kidnapped_player() {
        let mut eng = Engine::new();
        let kidnapper = add_player(&mut eng, 0, Role::Civilian, "kidnapper");
        let victim = add_player(&mut eng, 0, Role::Civilian, "victim");
        create_kidnapping(&mut eng, 0, kidnapper, victim, KidnappingType::Anonymous);

        let result = eng.execute(ActionRequest {
            actor: ActionActor::System,
            timestamp: 1,
            payload: Action::CreateKidnapping(crate::action::kidnapping::create_kidnapping::CreateKidnapping {
                kidnapper_id: kidnapper,
                victim_id: victim,
                kidnapping_type: KidnappingType::Anonymous,
            }),
        });
        assert!(matches!(result, Err(ActionError::UserNotPresent)));
    }

    // IPP players have StrengthenedPresence and cannot be kidnapped.
    #[test]
    fn cannot_kidnap_ipp_player() {
        let mut eng = Engine::new();
        let kidnapper = add_player(&mut eng, 0, Role::Civilian, "kidnapper");
        let victim = add_player(&mut eng, 0, Role::Civilian, "victim");
        add_state(&mut eng, 0, victim, State::Ipp);

        let result = eng.execute(ActionRequest {
            actor: ActionActor::System,
            timestamp: 1,
            payload: Action::CreateKidnapping(crate::action::kidnapping::create_kidnapping::CreateKidnapping {
                kidnapper_id: kidnapper,
                victim_id: victim,
                kidnapping_type: KidnappingType::Anonymous,
            }),
        });
        assert!(matches!(result, Err(ActionError::ActorHasStrengthenedPresence)));
    }
}
