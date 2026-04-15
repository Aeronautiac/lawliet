use enumflags2::{BitFlags, bitflags};

#[bitflags]
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub enum State {
    Dead = 1 << 0,
    Incarcerated = 1 << 1,
    Ipp = 1 << 2,
    Kidnapped = 1 << 3,
    Custody = 1 << 4,
}

pub type States = BitFlags<State>;
