pub mod add_ability;
pub mod add_link;
pub mod clear_volatile_links;
pub mod create_and_give_ability;
pub mod give_ability;
pub mod remove_link;
pub mod use_ability;

// test usages, transfers, volatility, presence restrictions, non-ownership, wrong data, world
// charge pools, actor charge pools, local charge pools, pool and limit links, no links
// test that abilities with local charge pools maintain their state throughout a chain of transfers
// test link weights

// will be using the gun ability for testing as it is extremely basic and easy to see if successful

#[cfg(test)]
mod ability_tests {
    use crate::{
        ability::{AbilityBehaviour, gun::Gun, pseudocide::Pseudocide},
        action::{
            ability::create_and_give_ability::CreateAndGiveAbility,
            chargepool::add_charge_pool::AddChargePool,
        },
        actor::state::State,
        chargepool::PoolLinkType,
        config::{ability::AbilityName, role::Role},
        engine::Engine,
        helpers::{get_ability, get_ability_mut, get_actor},
        test_helpers::*,
    };

    #[test]
    fn basic_usage() {
        let mut eng = Engine::new();
        let p1 = add_player(&mut eng, 0, Role::Civilian, "p1");

        let ability_id = quick_ability(
            &mut eng,
            0,
            CreateAndGiveAbility {
                actor_id: p1,
                ability_name: AbilityName::Gun,
                variant: 0,
                transferrable: false,
                volatile: false,
            },
        );

        let ability = get_ability(&eng, ability_id).unwrap();
        let initial_limit = ability.get_usage_limit(&eng).unwrap();
        assert!(initial_limit > 0);

        use_ability(
            &mut eng,
            0,
            p1,
            ability_id,
            AbilityBehaviour::Gun(Gun { target_id: p1 }),
        )
        .unwrap();

        let p1_actor = get_actor(&eng, p1).unwrap();
        assert!(p1_actor.has_state(State::Dead));

        let ability = get_ability(&eng, ability_id).unwrap();
        let new_limit = ability.get_usage_limit(&eng).unwrap();
        assert!(new_limit == initial_limit - 1);
    }

    #[test]
    fn use_unowned() {
        let mut eng = Engine::new();
        let p1 = add_player(&mut eng, 0, Role::Civilian, "p1");
        let p2 = add_player(&mut eng, 0, Role::Civilian, "p2");

        let ability_id = quick_ability(
            &mut eng,
            0,
            CreateAndGiveAbility {
                actor_id: p1,
                ability_name: AbilityName::Gun,
                variant: 0,
                transferrable: false,
                volatile: false,
            },
        );

        assert!(
            use_ability(
                &mut eng,
                0,
                p2,
                ability_id,
                AbilityBehaviour::Gun(Gun { target_id: p1 }),
            )
            .is_err()
        );
    }

    // return by death?
    #[test]
    fn usage_exhaustion() {
        let mut eng = Engine::new();
        let p1 = add_player(&mut eng, 0, Role::Civilian, "p1");

        let ability_id = quick_ability(
            &mut eng,
            0,
            CreateAndGiveAbility {
                actor_id: p1,
                ability_name: AbilityName::Gun,
                variant: 0,
                transferrable: false,
                volatile: false,
            },
        );

        let ability = get_ability(&eng, ability_id).unwrap();
        let initial_limit = ability.get_usage_limit(&eng).unwrap();
        for _ in 0..initial_limit {
            use_ability(
                &mut eng,
                0,
                p1,
                ability_id,
                AbilityBehaviour::Gun(Gun { target_id: p1 }),
            )
            .unwrap();
            quick_revive(&mut eng, 0, true, p1);
        }

        let ability = get_ability(&eng, ability_id).unwrap();
        assert!(ability.get_usage_limit(&eng).unwrap() == 0);
        assert!(
            use_ability(
                &mut eng,
                0,
                p1,
                ability_id,
                AbilityBehaviour::Gun(Gun { target_id: p1 }),
            )
            .is_err()
        );
    }

    #[test]
    fn usage_arg_mismatch() {
        let mut eng = Engine::new();
        let p1 = add_player(&mut eng, 0, Role::Civilian, "p1");

        let ability_id = quick_ability(
            &mut eng,
            0,
            CreateAndGiveAbility {
                actor_id: p1,
                ability_name: AbilityName::Gun,
                variant: 0,
                transferrable: false,
                volatile: false,
            },
        );

        let ability = get_ability(&eng, ability_id).unwrap();
        let initial_limit = ability.get_usage_limit(&eng).unwrap();
        assert!(
            use_ability(
                &mut eng,
                0,
                p1,
                ability_id,
                AbilityBehaviour::Pseudocide(Pseudocide {
                    target_id: p1,
                    true_name: "john porkington".into(),
                    death_message: "hlep".into(),
                    role: Role::Civilian,
                    notebook_transferred: false,
                    ability_transferred: false,
                })
            )
            .is_err()
        );

        let ability = get_ability(&eng, ability_id).unwrap();
        let limit = ability.get_usage_limit(&eng).unwrap();
        assert!(limit == initial_limit);
    }

    #[test]
    fn basic_transfer() {
        let mut eng = Engine::new();
        let p1 = add_player(&mut eng, 0, Role::Civilian, "p1");
        let p2 = add_player(&mut eng, 0, Role::Civilian, "p2");

        let a1 = quick_ability(
            &mut eng,
            0,
            CreateAndGiveAbility {
                actor_id: p1,
                ability_name: AbilityName::Gun,
                variant: 0,
                transferrable: false,
                volatile: false,
            },
        );

        let a2 = quick_ability(
            &mut eng,
            0,
            CreateAndGiveAbility {
                actor_id: p2,
                ability_name: AbilityName::Gun,
                variant: 0,
                transferrable: true,
                volatile: false,
            },
        );

        let a1_data = get_ability(&eng, a1).unwrap();
        assert!(a1_data.ownership_struct.owner == Some(p1));
        let a2_data = get_ability(&eng, a2).unwrap();
        assert!(a2_data.ownership_struct.owner == Some(p2));

        use_ability(
            &mut eng,
            0,
            p1,
            a1,
            AbilityBehaviour::Gun(Gun { target_id: p2 }),
        )
        .unwrap();

        let a1_data = get_ability(&eng, a1).unwrap();
        assert!(a1_data.ownership_struct.owner == Some(p1));
        let a2_data = get_ability(&eng, a2).unwrap();
        assert!(a2_data.ownership_struct.owner == Some(p1));

        let p1_data = get_actor(&eng, p1).unwrap();
        assert!(p1_data.abilities.contains(&a1));
        assert!(p1_data.abilities.contains(&a2));
        let p2_data = get_actor(&eng, p2).unwrap();
        assert!(!p2_data.abilities.contains(&a1));
        assert!(!p2_data.abilities.contains(&a2));
    }

    #[test]
    fn transfer_local_state_persistence() {
        let mut eng = Engine::new();
        let p1 = add_player(&mut eng, 0, Role::Civilian, "p1");
        let p2 = add_player(&mut eng, 0, Role::Civilian, "p2");

        let a1 = quick_ability(
            &mut eng,
            0,
            CreateAndGiveAbility {
                actor_id: p1,
                ability_name: AbilityName::Gun,
                variant: 0,
                transferrable: false,
                volatile: false,
            },
        );

        let a2 = quick_ability(
            &mut eng,
            0,
            CreateAndGiveAbility {
                actor_id: p2,
                ability_name: AbilityName::Gun,
                variant: 0,
                transferrable: true,
                volatile: false,
            },
        );

        let ability = get_ability(&eng, a2).unwrap();
        let initial_limit = ability.get_usage_limit(&eng).unwrap();

        use_ability(
            &mut eng,
            0,
            p2,
            a2,
            AbilityBehaviour::Gun(Gun { target_id: p2 }),
        )
        .unwrap();
        quick_revive(&mut eng, 0, false, p2);

        let _ = use_ability(
            &mut eng,
            0,
            p1,
            a1,
            AbilityBehaviour::Gun(Gun { target_id: p2 }),
        );

        let ability = get_ability(&eng, a2).unwrap();
        let limit = ability.get_usage_limit(&eng).unwrap();
        assert!(limit == initial_limit - 1);
    }

    // verify infinite usage behaviour
    #[test]
    fn no_links() {}

    // test multiple abilities interacting with the same global pool
    // use the charges for one ability and see if it carries over to the other
    // create new abilities after running out of charges as well
    #[test]
    fn global_links() {}

    // test multiple abilities interacting with the same actor pool
    // use the charges for one ability and see if it carries over to another
    #[test]
    fn actor_links() {}

    #[test]
    fn local_links() {}

    #[test]
    fn local_link_volatility() {}

    #[test]
    fn limit_link() {}

    #[test]
    fn pool_link() {}

    #[test]
    fn pool_and_limit_links() {}

    // see if multiple abilities linking to the same pool with different weights is properly handled
    #[test]
    fn link_weights() {}

    // TODO:
    // Verify that ability state is properly dealt with on iteration progression (when these
    // mechanics are added)
}
