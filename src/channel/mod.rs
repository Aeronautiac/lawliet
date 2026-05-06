// channels are the primitive objects used to facilitate communication
// lounges use channels
// group use channels
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

use indexmap::IndexSet;

use crate::{ID, Time};

// messages are ephemeral within lawliet. they are just a delivery mechanism.
pub struct Message {
    pub content: String, // this may be empty in certain cases (when content is irrelevant)
    pub sent_at: Time,
    pub sent_by: ID,
    pub channel_id: ID,
}

// channels have members
// this only contains the id for now, but it may contain other stuff later
#[derive(Eq, Hash, PartialEq)]
pub struct ChannelMember {
    pub id: ID,
}

pub struct Channel {
    pub loggable: bool, // whether or not abilities like autopsy can use messages sent here
    pub members: IndexSet<ChannelMember>, // the people in the channel
}

impl Channel {
    pub fn new(loggable: bool) -> Self {
        Channel {
            loggable,
            members: IndexSet::new(),
        }
    }

    pub fn add_member(&mut self, id: ID) {
        let member = ChannelMember { id };
        self.members.insert(member);
    }

    pub fn remove_member(&mut self, id: ID) {
        let member = ChannelMember { id };
        self.members.swap_remove(&member);
    }

    pub fn has_member(&self, id: ID) {
        let member = ChannelMember { id };
        self.members.contains(&member);
    }

    pub fn set_loggable(&mut self, loggable: bool) {
        self.loggable = loggable;
    }
}
