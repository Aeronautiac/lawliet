use std::rc::Rc;

use indexmap::{IndexSet, indexset};

use crate::{ID, config::role::Role};

#[derive(PartialEq, Eq, Debug)]
pub struct Player {
    pub role: Role,
    pub true_name: Rc<str>,
    pub eyes: u32,
    pub lounges: IndexSet<ID>,
    pub groupchats: IndexSet<ID>,
    pub bugs: IndexSet<ID>,
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
