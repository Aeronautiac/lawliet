use indexmap::{IndexMap, IndexSet};

use crate::{
    ID, action::Action, common::PollWeight, engine::Engine, poll::policies::resolution::majority,
};
mod policies;

// polls have resolution policies which determine if the poll resolves or not
// a poll may resolve immediately when some threshold is reached, or it may
// resolve after the poll times out
//
// polls also have valid voter policies which decide if a vote is valid (i.e., the vote counts if it
// is already in the set, or whether or not the vote is even added to the set)
//
// polls can only run while their attached action is possible. if for any reason the action's
// validation pass rejects, the poll will cancel itself.
//
// some examples:
// - org polls typically resolve immediately when majority is reached, and if majority is not met by
// the timeout, the poll is inconclusive
// - courtroom polls will only resolve after timing out. which ever side gets the most votes wins.
// if the vote counts are equal, the poll is inconclusive, and the player walks free.
//
// this behaviour is implemented as such:
// - polls have two policies: update and timeout
// - policies may return inconclusive, success, or reject
// - if an update policy returns inconclusive, nothing happens
// - if an update policy returns reject or accept, the poll concludes
// - a poll will always conclude with the return of a timeout policy

#[derive(Clone, Copy)]
pub enum VoterPolicy {
    Present, // whether or not the voter is "present", i.e., they are not dead, imprisoned, or in
             // any other way incapacitated
}

#[derive(Clone, Copy)]
pub enum PolicyResult {
    Accept,
    Reject,
    Inconclusive,
}

#[derive(Clone, Copy)]
pub enum PollPolicy {
    AlwaysInconclusive,
    Majority,
    WinningVote, // this should only be used in timeout policies because it will resolve immediately
                 // in update
}

pub enum PollVisibility {
    Org(ID),     // everyone present within an org
    Channel(ID), // everyone present within a channel
    AllPresent,  // everyone present in the game (not kidnapped, dead, etc...)
}

pub struct Vote {
    pub weight: PollWeight,
    pub accept: bool,
}

pub struct VoteQuery {
    pub accept: PollWeight,
    pub reject: PollWeight,
    pub total: PollWeight,
}

pub struct Poll {
    pub payload: Action,
    pub visibility: PollVisibility,
    pub update_policy: PollPolicy,
    pub timeout_policy: PollPolicy,
    pub voter_policy: VoterPolicy,
    pub votes: IndexMap<ID, Vote>,
}

impl Poll {
    fn policy(&self, pol: PollPolicy, eng: &Engine) -> PolicyResult {
        match pol {
            PollPolicy::AlwaysInconclusive => PolicyResult::Inconclusive,
            PollPolicy::Majority => majority(self, eng),
            PollPolicy::WinningVote => PolicyResult::Accept,
        }
    }

    pub fn update_policy(&self, eng: &Engine) -> PolicyResult {
        self.policy(self.update_policy, eng)
    }

    pub fn timeout_policy(&self, eng: &Engine) -> PolicyResult {
        self.policy(self.timeout_policy, eng)
    }

    pub fn voter_policy(&self, eng: &Engine, voter_id: ID) -> bool {
        match self.voter_policy {
            _ => true,
        }
    }

    pub fn weights(&self, eng: &Engine) -> VoteQuery {
        let mut accept = 0;
        let mut reject = 0;
        for (id, vote) in &self.votes {
            if !self.voter_policy(eng, *id) {
                continue;
            }
            if vote.accept {
                accept += vote.weight;
            } else {
                reject += vote.weight;
            }
        }
        VoteQuery {
            accept,
            reject,
            total: accept + reject,
        }
    }
}
