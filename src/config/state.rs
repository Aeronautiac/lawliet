use std::collections::BTreeMap;

use crate::actor::{
    restriction::{Restriction, Restrictions},
    state::State,
};

pub type StateRestrictionMap = BTreeMap<State, Restrictions>;

pub fn default_state_restrictions() -> StateRestrictionMap {
    let mut map = BTreeMap::new();

    map.insert(State::Dead, Restrictions::all());
    map.insert(
        State::Incarcerated,
        Restriction::AbilitiesPhysical
            | Restriction::Contact
            | Restriction::NotebookPassage
            | Restriction::NotebookUsage,
    );
    map.insert(
        State::Kidnapped,
        Restriction::AbilitiesPhysical
            | Restriction::Contact
            | Restriction::NotebookUsage
            | Restriction::NotebookPassage,
    );
    map.insert(
        State::Custody,
        Restriction::NotebookPassage | Restriction::NotebookUsage,
    );

    map
}
