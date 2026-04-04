use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Restrictions: u32 {
        const NONE = 0;
        const NO_ABILITIES_PHYSICAL = 1 << 0;
        const NO_ABILITIES_SUPERNATURAL = 1 << 1;
        const NO_CONTACT = 1 << 2;
        const NO_NOTEBOOK_USAGE = 1 << 3;
        const NO_NOTEBOOK_PASSAGE = 1 << 4;
        const NO_ALIVE = 1 << 5; // if you have this, you're dead and you cant do anything
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Source {
    State(crate::ID),
    Manual,
}
