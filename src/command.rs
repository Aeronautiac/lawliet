use indexmap::IndexSet;

use crate::{
    ID, Time,
    actor::{modifier::Modifiers, state::States},
    channel::{ChannelPermissions, SenderDisplay},
    common::{AttemptCount, ChargeCount, IterationCount},
    config::{ability::AbilityName, role::Role},
};

#[derive(Clone)]
pub struct DeferredCommand {
    pub payload: CommandPayload,
    pub blocking_modifiers: Modifiers, // when the target has none of these modifiers, they may
                                       // receive the command
}

// commands with no recipient are considered "system" commands and are used to talk directly to the
// host or the backend of a frontend
//
// the frontend server is expected to intercept certain commands if they wish to implement host controls
#[derive(Clone)]
pub struct CommandPayload {
    pub timestamp: Time,
    pub recipient: Option<ID>,
    pub cmd: Command,
}

// problem:
// how to handle commands which are supposed to be sent out to players immediately, but are
// blocked by some condition and must be sent out later?
// solution:
// - rather than immediately sending out important commands, push them to a central payload buffer
// - on update, evaluate every command payload in the buffer in order of lowest time to highest time and commit any of them that are
// currently possible

// command the frontend
#[derive(Clone)]
pub enum Command {
    ////////////////////////////////////////////////
    // WORLD //
    ///////////
    // if doing something like sending as a message in the news channel, ensure that it is placed
    // into the proper slot and treated as a historical event if it wasn't sent immediately.

    /////=<TARGETTED>=/////

    // notify a specific player of a death. this can be done in any way. it can be put into the news
    // channel display, a dedicated list, etc...
    // the server doesn't need to intercept this because hosts can directly view the event log and
    // view/modify actor states
    Death {
        true_name: String,
        death_message: String,
        role: Role,
        notebook_transferred: bool,
        ability_transferred: bool,
    },

    // display/announce kidnapping. can be handled similar to death.
    Kidnapping {
        target_id: ID,
        duration: Time,
    },

    // display/announce a pseudocide revival. can be handled similarly to death.
    PseudocideRevival {
        target_id: ID,
    },

    ////////////////////////////////////////////////
    // Actors //
    ////////////
    // Actors will often have their state modified. Views of that actor should reflect
    // their current state(s).
    // Furthermore, there needs to be a way to address the underlying actor object of a player or org from
    // the frontend.
    // Both organizations and players are actors.

    /////=<NO RECIPIENT>=/////

    // map a player to an actor id
    // client connections should be mapped to player ids
    // handling the mapping on the client would allow players to spoof actions for other players
    // (horrible vulnerability)
    MapPlayer {
        player_id: ID,
        actor_id: ID,
    },

    // same as above, just for orgs
    MapOrg {
        org_id: ID,
        actor_id: ID,
    },

    // all display instances of this actor must be updated
    // this is handled by the frontend
    // ensure that clients cannot see the state of other actors
    // a clean way to handle this is to construct a set of display "blueprint" type objects on the
    // frontend server, map them to actors, and then send them out to every client (when necessary)
    // who currently has permission to view that actor in some way
    // the reason this isnt handled entirely on the engine level is because its irrelevant. there
    // are no deception mechanics regarding state displays.
    ActorState {
        state: States,
        actor_id: ID,
    },

    /////=<TARGETTED>=/////

    // display a player as an org member
    // this includes dead players and such as they are still considered org members
    AddMember {
        player_id: ID,
        org_id: ID,
    },

    // remove from org member list
    RemoveMember {
        player_id: ID,
        org_id: ID,
    },

    ////////////////////////////////////////////////
    // COMMS //
    ///////////
    // A player who is added to a channel after messages have already been sent should be allowed to
    // see the messages which have been sent in that channel previously if they have view
    // permissions. This must be handled by the frontend.

    /////=<NO RECIPIENT>=/////

    // add a message to a channel
    AddMessage {
        content: String,
        channel_id: ID,
        sender_display: SenderDisplay,
    },

    // map a lounge id to a channel id
    MapLounge {
        lounge_id: ID,
        channel_id: ID,
    },

    // map a gc id to a channel id
    MapGc {
        gc_id: ID,
        channel_id: ID,
    },

    /////=<TARGETTED>=/////

    // entirely remove someone's view of a channel
    RemoveChannel {
        channel_id: ID,
    },

    // update the owner status of a gc for a player
    GcOwnerStatus {
        owner: bool,
        gc_id: ID,
    },

    // display a channel member
    ShowChannelMember {
        channel_id: ID,
        display: SenderDisplay,
        channel_perms: ChannelPermissions,
    },

    // remove a channel member display
    RemoveChannelMember {
        channel_id: ID,
        display: SenderDisplay,
    },

    // update a player's view of the channel based on their permissions
    UpdateChannelView {
        channel_id: ID,
        perms: ChannelPermissions,
        displays: IndexSet<SenderDisplay>,
    },

    ////////////////////////////////////////////////
    // NOTEBOOKS //
    ///////////////
    // Any notebook attempt should be shown to anybody who currently possesses the notebook.
    // The way this is handled doesn't matter.
    // This means that while one player may receive immediate feedback, other players should see
    // the previous attempts in the notebook.
    // This is not a command in of itself because the command would essentially be a null command
    // and would serve no purpose.
    // Note that messages sent in a notebook channel are handled by design. This specifically refers
    // to notebook usages which may be represented differently.
    //
    // Some modifiers block certain notebook actions. A frontend should take this into account.

    /////=<NO RECIPIENT>=/////

    // map a notebook id to its channel id
    // the state of the display for a given player depends on that player's permissions in the
    // notebook's channel
    MapNotebook {
        notebook_id: ID,
        channel_id: ID,
    },

    // a write failure is not actually a failure to use an action. it is just the lack of a correct
    // true name and leads to actual state modification. the player must be explicitly notified, and
    // the usage must be logged.
    NotebookWriteFailure {
        notebook_id: ID,
        attempts_remaining: AttemptCount,
        user_id: ID,
    },

    // similar case to write failure
    NotebookWriteSuccess {
        notebook_id: ID,
        attempts_remaining: AttemptCount,
        user_id: ID,
    },

    ////////////////////////////////////////////////
    // ABILITIES //
    ///////////////
    // Clients may display some specific abilities differently from general abilities, but the
    // engine will have no knowledge of this. For instance, the contact ability should not be
    // treated as a normal ability on the frontend, but the engine sees it as no different than any
    // other ability.
    //
    // The specific actor that an ability belongs to should be taken into consideration.
    // As an example; even though you do not directly own organization abilities if you are in that
    // organization, it should still be clearly displayed, but differentiated from standard
    // self-owned abilities.
    // For this reason, there will be an owner id in the ability view command. If it is the client's
    // id, it doesn't really matter. If it is the org's id, it does.

    /////=<TARGETTED>=/////

    // update the view of an ability to reflect its current state
    UpdateAbilityView {
        ability_name: AbilityName,
        usages_remaining: ChargeCount,
        iterations_to_reset: IterationCount,
        ability_id: ID,
        owner_id: ID,
    },

    // entirely hide an ability from a user
    RemoveAbility {
        ability_id: ID,
    },

    // tell the frontend to display autopsy messages for a specific user. the frontend server will do the
    // querying and filtering, and the clients will handle the display of that info.
    RevealAutopsyMessages {
        target_id: ID,
        range: Time, // ms
        redact_names: bool,
    },
}
