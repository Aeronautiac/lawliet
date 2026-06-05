/*
* Prosecution lifecycle:
* - Custody period
* - Trial period
* - Voting period
*
* Prosecutions have:
* - Prosecutor
*   * have displays (same as senders, so just generalize the senderdisplay type to
*   userdisplay)
* - Defendant
*   * also have displays, though there's currently nothing in the game where the defendant is
*   anonymous (they will always be raw)
* - Optional Lawyer
*   * The defendant is shown an option to select a lawyer. No response before the custody period ends
*   means they get no lawyer by default.
* - An autonomous flag
*
* The custody/grace period happens on prosecution. It lasts until some time limit
* is reached or both the prosecutor and defendant have agreed to start the trial period.
* (the max duration is configurable)
*
* If the prosecution is not autonomous, then a host must approve every phase transition.
*
* The trial period is a time period where both sides present their case/defense. Both sides are given their
* own time where they are the only ones with speaking privileges in the specific trial channel. After this,
* there is a short debate period.
* A "side" in this case is the prosecutor side or the defendant side. The defendant side consists of
* both the lawyer and the defendant.
* When the trial period starts, the prosecutor is given a timed grace period before their presentation
* timer begins. As soon as they send their first message, the grace period ends. The same is true
* for the defense.
* After both presentation periods, the debate period immediately begins. At this point, any side can
* deliberately choose to shorten the timer to some set duration (say 15 minutes or so) by stating
* they are finished.
*
* After the trial period fully concludes, the voting period begins. A poll is added to the channel
* to determine the trial outcome anonymously.
*
* If the majority of the votes were guilty, the target is executed. Otherwise, they are set free.
* The vote lasts some set duration regardless of when the trial ended.
*
* If either the defendant or prosecutor gain the NoPresence modifier during the custody or trial
* period, the prosecution is immediately terminated. (The state of the lawyer is irrelevant beyond
* initial selection)
*
* If the defendant specifically dies during the voting period, the prosecution immediately terminates.
* Other modifiers dont matter. In lore, we can say they've been injected with a remote kill device.
* We can also just prevent kidnappings and such if needed.
*
* If visibiliy for a trial is lost (blackout can do this), the trial restarts when the disruption ends
*
* If general visibility of the poll for the trial is lost, the voting period is extended by the
* duration of the disturbance.
*
* Custody wiretaps you (a bug instance is created). If you pick a lawyer, you establish a private
* line of communcation with that lawyer until the voting period begins.
*
* You can only be prosecuted by one person at a time. If you are in custody, you cannot be prosecuted.
*/

// need to think about termination:
// we can "archive" using deferred commands
// example:
// - person imprisoned
// - prosecution begins
// - deferred visibility commands sent out to everyone
// - prosecution concludes
// - archival command sent to frontend server
// - engine deletes prosecution from memory
// - person released
// - person receives old channel visibility command
// - when something is archived, it cannot be interacted with and is meant to be explicitly labeled
// as archived

use crate::{ActorKey, ChannelKey, PollKey, common::JobID};

pub struct Lawyer {
    pub actor_id: ActorKey,
    pub channel_id: ChannelKey,
}

pub struct ProsecutionDefense {
    pub defendant: ActorKey,
    pub lawyer: Option<Lawyer>,
}

pub enum TrialSubphase {
    Grace,
    Presentation,
}

pub enum TrialPhase {
    Prosecutor(TrialSubphase),
    Defense(TrialSubphase),
    Debate,
}

pub enum ProsecutionPhase {
    // advancement:
    //   both true || timeout:
    //     if not autonomous:
    //       advance if host approval
    //     else:
    //       advance
    Custody {
        prosecutor_start: bool,
        defense_start: bool,
        timeout_job_id: JobID,
    },

    // advancement:
    //   debate phase complete:
    //     if not autonomous:
    //       advance with host approval
    //     else:
    //       advance
    Trial {
        phase: TrialPhase,
        channel_id: ChannelKey,
        timeout_job_id: JobID, // constantly switched up depending on subphase
    },

    Voting {
        // necessary to quickly find polls to extend the duration of
        poll_id: PollKey,
    },
}

// channels already have user displays
// it is probably only necessary to decide on displays on creation, meaning it isnt relevant to his
// specific struct, and further meaning that senderdisplay does not actually require a refactor
pub struct Prosecution {
    pub prosecutor: ActorKey,
    pub defense: ProsecutionDefense,
    pub phase: ProsecutionPhase,
    pub autonomous: bool,
}
