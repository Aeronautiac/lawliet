use std::rc::Rc;

use indexmap::{IndexMap, IndexSet, indexset};

use crate::{ID, channel::ChannelPermissions, config::{role::Role, world::WorldChannelName}};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct WorldChannelOverride {
    pub default_perms: ChannelPermissions,
    pub force_perms: ChannelPermissions,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Player {
    pub role: Role,
    pub true_name: Rc<str>,
    pub eyes: u32,
    pub lounges: IndexSet<ID>,
    pub groupchats: IndexSet<ID>,
    pub bugs: IndexSet<ID>,
    pub world_channel_overrides: IndexMap<WorldChannelName, WorldChannelOverride>,
}

impl Player {
    pub fn new(name: &str, role: Role) -> Self {
        let true_name = Rc::from(name);
        Player {
            role,
            true_name,
            eyes: 2,
            lounges: indexset![],
            groupchats: indexset![],
            bugs: indexset![],
            world_channel_overrides: IndexMap::new(),
        }
    }

    pub fn add_lounge(&mut self, id: ID) {
        self.lounges.insert(id);
    }

    pub fn remove_lounge(&mut self, id: ID) {
        self.lounges.swap_remove(&id);
    }

    pub fn add_groupchat(&mut self, id: ID) {
        self.groupchats.insert(id);
    }

    pub fn remove_groupchat(&mut self, id: ID) {
        self.groupchats.swap_remove(&id);
    }

    pub fn add_bug(&mut self, id: ID) {
        self.bugs.insert(id);
    }
}
