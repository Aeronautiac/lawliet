use crate::{
    ID, Time,
    actor::modifier::Modifier,
    channel::{ChannelPermissions, SenderDisplay},
    config::role::Role,
};

#[derive(Clone)]
pub struct DeferredCommand {
    pub payload: CommandPayload,
    pub blocking_modifiers: Modifier, // when the target has none of these modifiers, they may
                                      // receive the command
}

#[derive(Clone)]
pub struct CommandPayload {
    pub timestamp: Time,
    pub recipient: ID,
    pub cmd: Command,
}

// command granularity: individual clients
// if a command has multiple recipients, the client of every recipient will receive a copy of that
// command (this is to save memory within lawliet)
//
// a host client receives every command
//
// problem:
// how to handle commands which are supposed to be sent out immediately, but are
// blocked by some condition and must be sent out later?
// solution:
// - rather than immediately sending out important commands, push them to a central payload buffer
// - on update, evaluate every command payload in the buffer in order of lowest time to highest time and commit any of them that are
// currently possible

// command the frontend
#[derive(Clone)]
pub enum Command {
    // add to the list of deaths
    Death {
        true_name: String,
        death_message: String,
        role: Role,
        notebook_transferred: bool,
        ability_transferred: bool,
    },

    // the frontend will filter out the messages itself for now as it has direct access. it is
    // likely more performant this way as it avoids sending massive message chunks to lawliet. if
    // necessary, lawliet can filter messages itself for more complex actions and return the
    // filtered messages to render. likely not necessary currently.
    RevealAutopsyMessages {
        target_id: ID,
        range: Time, // ms
        redact_names: bool,
    },

    // display a message in a channel
    ShowMessage {
        content: String,
        channel_id: ID,
        sender_display: SenderDisplay,
    },

    // update someone's view of a channel based on their permissions
    GiveChannel {
        channel_id: ID,
        target_id: ID,
        perms: ChannelPermissions,
    },

    // entirely remove someone's view of a channel
    RemoveChannel {
        channel_id: ID,
    },

    AnnounceKidnapping {
        target_id: ID,
        duration: Time,
    },

    AnnouncePseudocideRevival {
        target_id: ID,
    },
}
