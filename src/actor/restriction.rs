use crate::{ID, actor::state::State};
use enumflags2::{BitFlags, bitflags};

#[bitflags]
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub enum Restriction {
    Presence = 1 << 0, // a general restriction for anything that requires "presence"
    Contact = 1 << 1,
    NotebookReceive = 1 << 2,
    NotebookUsage = 1 << 3,
    NotebookPassage = 1 << 4,
    PassiveLinks = 1 << 5, // this restriction disables passive links
}
pub type Restrictions = BitFlags<Restriction>;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Source {
    State(State),
    Manual(ID), // frontend maps strings to internal ids
}
