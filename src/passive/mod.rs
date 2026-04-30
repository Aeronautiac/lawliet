use crate::ownership::OwnershipStruct;

#[derive(PartialEq, Eq, Clone, Debug, Copy)]
pub enum ContactLogType {
    Full,
    Even,
    Odd,
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum PassiveType {
    Wanted,
    JuryDuty,
    ContactLogs(ContactLogType),
    OwnedNotebookBlock, // blocks usage of all notebooks originally owned by the actor
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Passive {
    pub ownership_struct: OwnershipStruct,
    pub passive_type: PassiveType,
}
