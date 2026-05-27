use indexmap::{IndexSet, indexset};

use crate::ID;

// when the owner leaves a groupchat, the owner is set to None
//
// you should not be able to give gc owner to people who dont currently have access to the gc (even
// if they are a member, they must have contact permissions)
//
// similarly to lounges, players have caches of groupchats they are in

#[derive(Debug)]
pub struct Groupchat {
    pub channel_id: ID,
    pub owner: Option<ID>,
    pub members: IndexSet<ID>,
}

impl Groupchat {
    pub fn new(channel_id: ID) -> Self {
        Groupchat {
            channel_id,
            owner: None,
            members: indexset! {},
        }
    }

    pub fn add_member(&mut self, id: ID) {
        self.members.insert(id);
    }

    pub fn remove_member(&mut self, id: ID) {
        self.members.swap_remove(&id);
    }

    pub fn contains_member(&self, id: ID) -> bool {
        self.members.contains(&id)
    }

    pub fn set_owner(&mut self, owner: Option<ID>) {
        self.owner = owner;
    }
}
