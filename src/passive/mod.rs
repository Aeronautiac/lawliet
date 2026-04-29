use crate::ownership::OwnershipStruct;

#[derive(Debug)]
pub enum ContactLogType {
    Full,
    Even,
    Odd,
}

#[derive(Debug)]
pub enum PassiveType {
    Wanted,
    JuryDuty,
    ContactLogs(ContactLogType),
}

#[derive(Debug)]
pub struct Passive {
    pub ownership_struct: OwnershipStruct,
    pub passive_type: PassiveType,
}
