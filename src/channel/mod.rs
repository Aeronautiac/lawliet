// channels are the primitive objects used to facilitate communication
//
// lounges use channels
// groups use channels
// general chat is a channel
// the news uses a channel
//
// if abilities like bug are to relay messages, or players are to read each others messages within a
// space, then those messages must be sent through channels
//
// death notes contain "private" channels within them as they still facilitate communiction between players
// any kind of log is NOT a channel as players are not allowed to speak in logs
//
// to keep memory usage low, channels will not store the messages sent through them. they are only
// used to determine what HAPPENS when a player sends a message through them
//
// messages themselves are stored in the yagami layer database and sent to lawliet for processing if
// required

use indexmap::IndexMap;

use crate::{ID, config::role::Role};
use enumflags2::{BitFlags, bitflags};

// frontend servers can maintain tables of visible messages for specific channels within
// a temporary database or even memory (database makes more sense)
// frontend clients can render those messages when necessary

// if a channel is not visible to you, you cannot read the messages in the channel
// if you have send perms in a channel, but you cannot see that channel, you can still speak there,
// but you wont see any messages there (including your own)

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub enum SenderDisplay {
    Raw(ID),
    Role(Role),
    Mysterious,
}

#[bitflags]
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub enum ChannelPermission {
    Send = 1 << 0,
    View = 1 << 1,
}
pub type ChannelPermissions = BitFlags<ChannelPermission>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChannelMember {
    pub perms: ChannelPermissions,
    pub displays: IndexMap<ID, SenderDisplay>,
}

#[derive(Debug)]
pub struct Channel {
    pub loggable: bool, // whether or not abilities like autopsy can use messages sent here
    pub members: IndexMap<ID, ChannelMember>, // the people in the channel and their permissions
}

impl Channel {
    pub fn new(loggable: bool) -> Self {
        Channel {
            loggable,
            members: IndexMap::new(),
        }
    }

    pub fn set_member(&mut self, id: ID, settings: Option<ChannelMember>) {
        if let Some(obj) = settings {
            self.members.insert(id, obj);
        } else {
            self.members.swap_remove(&id);
        }
    }

    pub fn get_member(&self, id: ID) -> Option<&ChannelMember> {
        self.members.get(&id)
    }

    pub fn set_loggable(&mut self, loggable: bool) {
        self.loggable = loggable;
    }
}
