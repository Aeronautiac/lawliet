pub mod add_to_org;
pub mod change_org_leader;
pub mod create_org;
pub mod remove_from_org;
pub mod system_use_org_ability;
pub mod use_org_ability;

// you should be allowed to add and remove dead people to/from an org. these restrictions shall be
// applied through invite abilities and similar if necessary.
// when someone dies, they remain an org member

// org members who are not present should not be allowed to use abilities

// org additions
// org leadership
// org passives
// abilities that require votes and dont require votes
// leader only abilities
// member requirements
// leadership changes
//
// things like specific invite abilities SHOULD NOT be tested here, only the general org system

#[cfg(test)]
mod org_tests {
    use crate::{
        config::{actor::organization::OrganizationName, role::Role},
        engine::Engine,
        helpers::get_org,
        test_helpers::*,
    };

    #[test]
    fn basic_addition() {
        let mut eng = Engine::new();
        let p1 = add_player(&mut eng, 0, Role::Civilian, "p1");
        let o1 = add_org(&mut eng, 0, OrganizationName::NULL);

        let org = get_org(&eng, o1).unwrap();
        assert!(!org.has_member(p1));

        add_to_org(&mut eng, 0, o1, p1, false, true);

        let org = get_org(&eng, o1).unwrap();
        assert!(org.has_member(p1));
    }

    #[test]
    fn basic_removal() {
        let mut eng = Engine::new();
        let p1 = add_player(&mut eng, 0, Role::Civilian, "p1");
        let o1 = add_org(&mut eng, 0, OrganizationName::NULL);

        add_to_org(&mut eng, 0, o1, p1, false, true);
        remove_from_org(&mut eng, 0, o1, p1);

        let org = get_org(&eng, o1).unwrap();
        assert!(!org.has_member(p1));
    }

    // operations on dead people should be allowed. these restrictions are only applied through
    // invite abilities if applicable.
    #[test]
    fn add_dead() {}

    #[test]
    fn remove_dead() {}

    #[test]
    fn leader_no_old() {}

    #[test]
    fn change_og_status() {}

    #[test]
    fn already_member() {}

    #[test]
    fn leader_replace() {}

    // you should be allowed to replace the leader with a dead person
    #[test]
    fn leader_replace_dead() {}

    #[test]
    fn leader_only_ability() {}

    #[test]
    fn no_vote_ability() {}

    #[test]
    fn vote_ability() {}

    // they shouldnt be allowed to start votes and such if theyre not present
    #[test]
    fn dead_use_ability() {}

    #[test]
    fn members_have_effective_passives() {}
}
