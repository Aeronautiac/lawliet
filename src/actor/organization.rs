use crate::{ID, config::actor::organization::OrganizationName};
use enumflags2::{BitFlags, bitflags};
use indexmap::IndexSet;

// the leader can be allowed to use multiple transfer policies at once
#[bitflags]
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub enum LeadershipTransferPolicy {
    Choose = 1 << 0,
    Random = 1 << 1,
}
pub type LeadershipTransferPolicies = BitFlags<LeadershipTransferPolicy>;

#[derive(Hash, PartialEq, Eq, Debug)]
pub struct LeadershipStruct {
    pub leader: ID,
    pub transfer_policies: LeadershipTransferPolicies,
}

#[derive(Hash, PartialEq, Eq, Debug)]
pub struct OrgMember {
    pub id: ID,
    pub og: bool,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Organization {
    pub leadership_struct: Option<LeadershipStruct>,
    pub members: IndexSet<OrgMember>,
    pub blacklist: IndexSet<ID>,
    pub org_name: OrganizationName,
}

impl Organization {
    pub fn new(name: OrganizationName) -> Self {
        Organization {
            leadership_struct: None,
            members: IndexSet::new(),
            blacklist: IndexSet::new(),
            org_name: name,
        }
    }
}
