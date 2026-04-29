use crate::ID;

#[derive(Debug)]
pub struct OwnershipStruct {
    pub owner: Option<ID>, // the actor which this item is owned by (if any)
    pub volatile: bool,    // determines whether or not the item is deleted when the owner changes
    // significantly (i.e., the role changes)
    pub transferrable: bool, // determines whether or not the ability will transfer on death (on
                             // transfer, the item will no longer be volatile)
}

impl OwnershipStruct {
    pub fn new(volatile: bool, transferrable: bool) -> Self {
        OwnershipStruct {
            owner: None,
            volatile,
            transferrable,
        }
    }

    pub fn set_owner(&mut self, id: ID) {
        self.owner = Some(id);
    }

    /// true if transferrable and the transfer was a success, false otherwise
    pub fn try_transfer(&mut self, id: ID) -> bool {
        if !self.transferrable {
            false
        } else {
            self.volatile = false;
            self.owner = Some(id);
            true
        }
    }
}
