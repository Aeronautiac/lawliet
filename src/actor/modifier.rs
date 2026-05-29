use crate::{ID, actor::state::State};
use enumflags2::{BitFlags, bitflags};

#[bitflags]
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub enum Modifier {
    NoPresence = 1 << 0, // cannot do things that require presence, cannot receive things that
    // require you to be present
    NoContact = 1 << 1, // cannot be contacted, cannot contact
    NoNotebookReceive = 1 << 2,
    NoNotebookUsage = 1 << 3,
    NoNotebookPassage = 1 << 4,
    DisablePassiveLinks = 1 << 5, // any passive link to/from you is nullified and treated as if it
    // doesn't exist
    WriteImmunity = 1 << 6, // cannot have your name written in a notebook
    StrengthenedPresence = 1 << 7, // immune to a variety of things which would normally apply NoPresence (death being an exception)
}
pub type Modifiers = BitFlags<Modifier>;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Source {
    State(State),
    Manual(ID),
}
