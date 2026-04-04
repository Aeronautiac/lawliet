use std::cmp::Ordering;

use crate::{ID, common::SequenceNumber, config::role::Role};

pub enum ActionActor {
    System,
    Player(crate::ID),
    Organization(crate::ID),
}

#[derive(PartialEq, Eq)]
pub enum Action {
    Kill {
        target_id: ID,
        killer_id: Option<ID>,
    },
    Revive {
        target_id: ID,
    },
    GiveRole {
        target_id: ID,
        role: Role,
    },
    WriteName {},
    ScheduleJob {},
    UseAbility {},
}

#[derive(PartialEq, Eq)]
pub struct ActionRequest {
    pub actor: crate::actor::Actor,
    pub timestamp: crate::Timestamp,
    pub payload: Action,
}
