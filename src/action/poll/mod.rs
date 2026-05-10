pub mod add_vote;
pub mod create_poll;
pub mod poll_timeout;
pub mod remove_vote;
pub mod update_polls;

// these tests will largely just use polls for killing people as that is a very easy action to test
// the polls will all have different configurations and voting scenarios ranging from actors with
// vote amplification passives, dead voters, side effect based executions, etc...
#[cfg(test)]
mod poll_tests {
    use crate::{
        action::{Action, engine::null::Null, poll::create_poll::CreatePoll},
        actor::state::State,
        config::role::Role,
        engine::Engine,
        helpers::get_actor,
        passive::PassiveType,
        poll::{PollPolicy, PollVisibility, VoterPolicy},
        test_helpers::*,
    };

    #[test]
    fn vote_addition() {
        let mut eng = Engine::new();
        let p1 = add_player(&mut eng, 0, Role::Civilian, "p1");

        // will never resolve
        let poll_id = create_poll(
            &mut eng,
            0,
            CreatePoll {
                voter_policy: VoterPolicy::Present,
                visibility: PollVisibility::AllPresent,
                update_policy: PollPolicy::AlwaysInconclusive,
                timeout_policy: PollPolicy::AlwaysInconclusive,
                duration: None,
                payload: Box::new(Action::Null(Null {})),
            },
        );

        let poll_data = eng.world.get_poll(poll_id).unwrap();
        assert!(!poll_data.contains_voter(p1));

        add_vote(&mut eng, 0, poll_id, p1, true).unwrap();

        let poll_data = eng.world.get_poll(poll_id).unwrap();
        assert!(poll_data.contains_voter(p1));
    }

    #[test]
    fn vote_addition_already_voted() {
        let mut eng = Engine::new();
        let p1 = add_player(&mut eng, 0, Role::Civilian, "p1");

        // will never resolve
        let poll_id = create_poll(
            &mut eng,
            0,
            CreatePoll {
                voter_policy: VoterPolicy::Present,
                visibility: PollVisibility::AllPresent,
                update_policy: PollPolicy::AlwaysInconclusive,
                timeout_policy: PollPolicy::AlwaysInconclusive,
                duration: None,
                payload: Box::new(Action::Null(Null {})),
            },
        );

        add_vote(&mut eng, 0, poll_id, p1, true).unwrap();
        assert!(add_vote(&mut eng, 0, poll_id, p1, true).is_err());

        let poll_data = eng.world.get_poll(poll_id).unwrap();
        assert!(poll_data.contains_voter(p1));
    }

    #[test]
    fn vote_removal() {
        let mut eng = Engine::new();
        let p1 = add_player(&mut eng, 0, Role::Civilian, "p1");

        // will never resolve
        let poll_id = create_poll(
            &mut eng,
            0,
            CreatePoll {
                voter_policy: VoterPolicy::Present,
                visibility: PollVisibility::AllPresent,
                update_policy: PollPolicy::AlwaysInconclusive,
                timeout_policy: PollPolicy::AlwaysInconclusive,
                duration: None,
                payload: Box::new(Action::Null(Null {})),
            },
        );

        add_vote(&mut eng, 0, poll_id, p1, true).unwrap();
        remove_vote(&mut eng, 0, poll_id, p1).unwrap();

        let poll_data = eng.world.get_poll(poll_id).unwrap();
        assert!(!poll_data.contains_voter(p1));
    }

    #[test]
    fn vote_removal_hasnt_voted() {
        let mut eng = Engine::new();
        let p1 = add_player(&mut eng, 0, Role::Civilian, "p1");

        let poll_id = create_poll(
            &mut eng,
            0,
            CreatePoll {
                voter_policy: VoterPolicy::Present,
                visibility: PollVisibility::AllPresent,
                update_policy: PollPolicy::AlwaysInconclusive,
                timeout_policy: PollPolicy::AlwaysInconclusive,
                duration: None,
                payload: Box::new(Action::Null(Null {})),
            },
        );

        let poll_data = eng.world.get_poll(poll_id).unwrap();
        assert!(!poll_data.contains_voter(p1));

        assert!(remove_vote(&mut eng, 0, poll_id, p1).is_err());

        let poll_data = eng.world.get_poll(poll_id).unwrap();
        assert!(!poll_data.contains_voter(p1));
    }

    #[test]
    fn vote_removal_doesnt_pass_voter_policy() {
        let mut eng = Engine::new();
        let p1 = add_player(&mut eng, 0, Role::Civilian, "p1");

        let poll_id = create_poll(
            &mut eng,
            0,
            CreatePoll {
                voter_policy: VoterPolicy::Present,
                visibility: PollVisibility::AllPresent,
                update_policy: PollPolicy::AlwaysInconclusive,
                timeout_policy: PollPolicy::AlwaysInconclusive,
                duration: None,
                payload: Box::new(Action::Null(Null {})),
            },
        );

        add_vote(&mut eng, 0, poll_id, p1, true).unwrap();
        quick_kill(&mut eng, 0, true, true, false, p1);
        assert!(remove_vote(&mut eng, 0, poll_id, p1).is_err());
    }

    #[test]
    fn vote_addition_doesnt_pass_voter_policy() {
        let mut eng = Engine::new();
        let p1 = add_player(&mut eng, 0, Role::Civilian, "p1");

        let poll_id = create_poll(
            &mut eng,
            0,
            CreatePoll {
                voter_policy: VoterPolicy::Present,
                visibility: PollVisibility::AllPresent,
                update_policy: PollPolicy::AlwaysInconclusive,
                timeout_policy: PollPolicy::AlwaysInconclusive,
                duration: None,
                payload: Box::new(Action::Null(Null {})),
            },
        );

        quick_kill(&mut eng, 0, true, true, false, p1);
        assert!(add_vote(&mut eng, 0, poll_id, p1, true).is_err());
    }

    #[test]
    fn present_majority_update_majority_timeout() {
        let mut eng = Engine::new();
        let p1 = add_player(&mut eng, 0, Role::Civilian, "p1");
        let p2 = add_player(&mut eng, 0, Role::Civilian, "p2");

        let poll_id = create_poll(
            &mut eng,
            0,
            CreatePoll {
                voter_policy: VoterPolicy::Present,
                visibility: PollVisibility::AllPresent,
                update_policy: PollPolicy::Majority,
                timeout_policy: PollPolicy::Majority,
                duration: Some(20),
                payload: Box::new(default_kill(p2)),
            },
        );

        let p2_actor = get_actor(&eng, p2).unwrap();
        assert!(!p2_actor.has_state(State::Dead));

        add_vote(&mut eng, 1, poll_id, p1, true).unwrap();

        let p2_actor = get_actor(&eng, p2).unwrap();
        assert!(!p2_actor.has_state(State::Dead));

        null_action(&mut eng, 20);

        let p2_actor = get_actor(&eng, p2).unwrap();
        assert!(!p2_actor.has_state(State::Dead));
    }

    // TODO:
    // test case where the poll timeout is scheduled at the same time that an event comes in. the
    // timeout should occur first due to the job scheduler.

    #[test]
    fn present_majority_update_no_timeout() {
        let mut eng = Engine::new();
        let p1 = add_player(&mut eng, 0, Role::Civilian, "p1");
        let p2 = add_player(&mut eng, 0, Role::Civilian, "p2");

        let poll_id = create_poll(
            &mut eng,
            0,
            CreatePoll {
                voter_policy: VoterPolicy::Present,
                visibility: PollVisibility::AllPresent,
                update_policy: PollPolicy::Majority,
                timeout_policy: PollPolicy::AlwaysInconclusive,
                duration: None,
                payload: Box::new(default_kill(p2)),
            },
        );

        let p2_actor = get_actor(&eng, p2).unwrap();
        assert!(!p2_actor.has_state(State::Dead));

        add_vote(&mut eng, 1, poll_id, p1, true).unwrap();

        let p2_actor = get_actor(&eng, p2).unwrap();
        assert!(!p2_actor.has_state(State::Dead));

        add_vote(&mut eng, 21, poll_id, p2, true).unwrap();

        let p2_actor = get_actor(&eng, p2).unwrap();
        assert!(p2_actor.has_state(State::Dead));
    }

    #[test]
    fn present_majority_update_winning_timeout() {
        let mut eng = Engine::new();
        let p1 = add_player(&mut eng, 0, Role::Civilian, "p1");
        let p2 = add_player(&mut eng, 0, Role::Civilian, "p2");
        let p3 = add_player(&mut eng, 0, Role::Civilian, "p3");
        let p4 = add_player(&mut eng, 0, Role::Civilian, "p4");

        let poll_id = create_poll(
            &mut eng,
            0,
            CreatePoll {
                voter_policy: VoterPolicy::Present,
                visibility: PollVisibility::AllPresent,
                update_policy: PollPolicy::Majority,
                timeout_policy: PollPolicy::WinningVote,
                duration: Some(10),
                payload: Box::new(default_kill(p4)),
            },
        );

        add_vote(&mut eng, 1, poll_id, p1, true).unwrap();

        let p4_actor = get_actor(&eng, p4).unwrap();
        assert!(!p4_actor.has_state(State::Dead));

        add_vote(&mut eng, 9, poll_id, p2, true).unwrap();

        let p4_actor = get_actor(&eng, p4).unwrap();
        assert!(!p4_actor.has_state(State::Dead));

        add_vote(&mut eng, 9, poll_id, p3, false).unwrap();

        let p4_actor = get_actor(&eng, p4).unwrap();
        assert!(!p4_actor.has_state(State::Dead));

        // now winning vote should evaluate on timeout. since 2 > 1, it should kill p4.
        null_action(&mut eng, 20);

        let p4_actor = get_actor(&eng, p4).unwrap();
        assert!(p4_actor.has_state(State::Dead));
    }

    #[test]
    fn present_no_update_winning_timeout() {
        let mut eng = Engine::new();
        let p1 = add_player(&mut eng, 0, Role::Civilian, "p1");

        let poll_id = create_poll(
            &mut eng,
            0,
            CreatePoll {
                voter_policy: VoterPolicy::Present,
                visibility: PollVisibility::AllPresent,
                update_policy: PollPolicy::AlwaysInconclusive,
                timeout_policy: PollPolicy::WinningVote,
                duration: Some(10),
                payload: Box::new(default_kill(p1)),
            },
        );

        add_vote(&mut eng, 1, poll_id, p1, true).unwrap();

        let p1_actor = get_actor(&eng, p1).unwrap();
        assert!(!p1_actor.has_state(State::Dead));

        null_action(&mut eng, 20);

        let p1_actor = get_actor(&eng, p1).unwrap();
        assert!(p1_actor.has_state(State::Dead));
    }

    #[test]
    fn present_no_update_majority_timeout() {
        let mut eng = Engine::new();
        let p1 = add_player(&mut eng, 0, Role::Civilian, "p1");
        let p2 = add_player(&mut eng, 0, Role::Civilian, "p2");
        let p3 = add_player(&mut eng, 0, Role::Civilian, "p3");

        let poll_id = create_poll(
            &mut eng,
            0,
            CreatePoll {
                voter_policy: VoterPolicy::Present,
                visibility: PollVisibility::AllPresent,
                update_policy: PollPolicy::AlwaysInconclusive,
                timeout_policy: PollPolicy::Majority,
                duration: Some(10),
                payload: Box::new(default_kill(p1)),
            },
        );

        add_vote(&mut eng, 1, poll_id, p1, true).unwrap();
        add_vote(&mut eng, 1, poll_id, p2, true).unwrap();
        add_vote(&mut eng, 1, poll_id, p3, true).unwrap();

        let p1_actor = get_actor(&eng, p1).unwrap();
        assert!(!p1_actor.has_state(State::Dead));

        null_action(&mut eng, 20);

        let p1_actor = get_actor(&eng, p1).unwrap();
        assert!(p1_actor.has_state(State::Dead));
    }

    #[test]
    fn present_timeout_majority_failure() {
        let mut eng = Engine::new();
        let p1 = add_player(&mut eng, 0, Role::Civilian, "p1");
        let p2 = add_player(&mut eng, 0, Role::Civilian, "p2");
        let p3 = add_player(&mut eng, 0, Role::Civilian, "p3");

        quick_passive(
            &mut eng,
            0,
            p3,
            PassiveType::VoteAmplication { multiplier: 2 },
            false,
        );

        let poll_id = create_poll(
            &mut eng,
            0,
            CreatePoll {
                voter_policy: VoterPolicy::Present,
                visibility: PollVisibility::AllPresent,
                update_policy: PollPolicy::Majority,
                timeout_policy: PollPolicy::Majority,
                duration: Some(5),
                payload: Box::new(default_kill(p1)),
            },
        );

        add_vote(&mut eng, 0, poll_id, p1, true).unwrap();
        add_vote(&mut eng, 0, poll_id, p2, true).unwrap();
        add_vote(&mut eng, 0, poll_id, p3, false).unwrap();
        null_action(&mut eng, 10);

        // it should be a 50/50 split
        let p1_actor = get_actor(&eng, p1).unwrap();
        assert!(!p1_actor.has_state(State::Dead));
    }

    #[test]
    fn vote_amplification() {
        let mut eng = Engine::new();
        let p1 = add_player(&mut eng, 0, Role::Civilian, "p1");
        let p2 = add_player(&mut eng, 0, Role::Civilian, "p2");

        quick_passive(
            &mut eng,
            0,
            p2,
            PassiveType::VoteAmplication { multiplier: 10 },
            false,
        );

        let poll_id = create_poll(
            &mut eng,
            0,
            CreatePoll {
                voter_policy: VoterPolicy::Present,
                visibility: PollVisibility::AllPresent,
                update_policy: PollPolicy::Majority,
                timeout_policy: PollPolicy::Majority,
                duration: Some(5),
                payload: Box::new(default_kill(p1)),
            },
        );

        add_vote(&mut eng, 0, poll_id, p1, false).unwrap();
        add_vote(&mut eng, 0, poll_id, p2, true).unwrap();
        null_action(&mut eng, 10);

        let p1_actor = get_actor(&eng, p1).unwrap();
        assert!(p1_actor.has_state(State::Dead));
    }

    // test the scenario where a death allows a vote to cross threshold
    #[test]
    fn voter_death_majority_update() {
        let mut eng = Engine::new();
        let p1 = add_player(&mut eng, 0, Role::Civilian, "p1");
        let p2 = add_player(&mut eng, 0, Role::Civilian, "p2");

        let poll_id = create_poll(
            &mut eng,
            0,
            CreatePoll {
                voter_policy: VoterPolicy::Present,
                visibility: PollVisibility::AllPresent,
                update_policy: PollPolicy::Majority,
                timeout_policy: PollPolicy::Majority,
                duration: None,
                payload: Box::new(default_kill(p1)),
            },
        );

        add_vote(&mut eng, 0, poll_id, p1, true).unwrap();
        quick_kill(&mut eng, 0, true, true, false, p2);

        let p1_actor = get_actor(&eng, p1).unwrap();
        assert!(p1_actor.has_state(State::Dead));
    }

    #[test]
    fn voter_death_winning_vote_timeout() {
        let mut eng = Engine::new();
        let p1 = add_player(&mut eng, 0, Role::Civilian, "p1");
        let p2 = add_player(&mut eng, 0, Role::Civilian, "p2");

        quick_passive(
            &mut eng,
            0,
            p2,
            PassiveType::VoteAmplication { multiplier: 10 },
            false,
        );

        let poll_id = create_poll(
            &mut eng,
            0,
            CreatePoll {
                voter_policy: VoterPolicy::Present,
                visibility: PollVisibility::AllPresent,
                update_policy: PollPolicy::AlwaysInconclusive,
                timeout_policy: PollPolicy::WinningVote,
                duration: Some(5),
                payload: Box::new(default_kill(p1)),
            },
        );

        add_vote(&mut eng, 0, poll_id, p1, true).unwrap();
        add_vote(&mut eng, 0, poll_id, p2, false).unwrap();
        quick_kill(&mut eng, 0, true, true, false, p2);
        null_action(&mut eng, 10);

        let p1_actor = get_actor(&eng, p1).unwrap();
        assert!(p1_actor.has_state(State::Dead));
    }

    #[test]
    fn action_becomes_invalid() {
        let mut eng = Engine::new();
        let p1 = add_player(&mut eng, 0, Role::Civilian, "p1");
        let p2 = add_player(&mut eng, 0, Role::Civilian, "p2");

        let poll_id = create_poll(
            &mut eng,
            0,
            CreatePoll {
                voter_policy: VoterPolicy::Present,
                visibility: PollVisibility::AllPresent,
                update_policy: PollPolicy::Majority,
                timeout_policy: PollPolicy::AlwaysInconclusive,
                duration: None,
                payload: Box::new(default_kill(p2)),
            },
        );

        quick_kill(&mut eng, 0, true, true, false, p2);
        assert!(add_vote(&mut eng, 0, poll_id, p1, true).is_err());
    }

    // TODO:
    // visibility based tests (orgs, channels, etc...)
    // do it when channels are implemented
}
