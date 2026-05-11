use crate::{ID, actor::state::State};
use enumflags2::{BitFlags, bitflags};

#[bitflags]
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub enum Modifier {
    NoPresence = 1 << 0,
    NoContact = 1 << 1,
    NoNotebookReceive = 1 << 2,
    NoNotebookUsage = 1 << 3,
    NoNotebookPassage = 1 << 4,
    DisablePassiveLinks = 1 << 5,
    WriteImmunity = 1 << 6,
    AntiPresenceImmunity = 1 << 7, // cannot be kidnapped, arrested, etc...
}
pub type Modifiers = BitFlags<Modifier>;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Source {
    State(State),
    Manual(ID),
}
