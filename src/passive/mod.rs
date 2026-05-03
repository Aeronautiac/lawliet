use crate::ownership::OwnershipStruct;

#[derive(PartialEq, Eq, Clone, Debug, Copy)]
pub enum ContactLogType {
    Full,
    Even,
    Odd,
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum PassiveType {
    Wanted,       // you can be silently prosecuted regardless of your affiliations
    JuryDuty,     // all votes are multiplied by some factor stated in config
    VolatileEyes, // certain abilities will cause the player to lose eyes when certain conditions
    // are met. if the player loses both their eyes, they cannot use certain abilities anymore.
    ContactLogs(ContactLogType),
    OwnedNotebookBlock, // blocks usage of all notebooks originally owned by the actor
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Passive {
    pub ownership_struct: OwnershipStruct,
    pub passive_type: PassiveType,
}
