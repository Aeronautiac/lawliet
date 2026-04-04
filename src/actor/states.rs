use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct States: u32 {
        const NONE = 0;
        const DEAD = 1 << 0;
        const INCARCERATED = 1 << 1;
        const IPP = 1 << 2;
        const KIDNAPPED = 1 << 3;
        const CUSTODY = 1 << 4;
    }
}
