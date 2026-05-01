use enumflags2::{BitFlags, bitflags};

use crate::{ID, actor::state::State};

#[bitflags]
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub enum Restriction {
    AbilitiesPhysical = 1 << 0,
    AbilitiesSupernatural = 1 << 1,
    Contact = 1 << 2,
    NotebookUsage = 1 << 3,
    NotebookPassage = 1 << 4,
    PassiveLinks = 1 << 5,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Source {
    State(State),
    Manual(ID), // frontend maps strings to internal ids
}

pub type Restrictions = BitFlags<Restriction>;
