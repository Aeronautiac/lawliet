use std::collections::BTreeSet;

use crate::{ID, config::organization::OrganizationName};

#[derive(PartialEq, Eq, Debug)]
pub struct OrgMember {}

#[derive(PartialEq, Eq, Debug)]
pub struct Organization {
    pub members: BTreeSet<OrgMember>,
    pub blacklist: BTreeSet<ID>,
    pub org_name: OrganizationName,
}

impl Organization {
    pub fn new(name: OrganizationName) -> Self {
        Organization {
            members: BTreeSet::new(),
            blacklist: BTreeSet::new(),
            org_name: name,
        }
    }
}
