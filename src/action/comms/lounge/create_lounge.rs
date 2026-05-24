/*
* SYSTEM ACTION
* Create a lounge and return the lounge id and channel id.
* Add the lounge to all involved player caches and add players to the channels.
* Creating a lounge will update a player's lounges immediately after (their channel permissions will
* be modified based on current state).
* Channel permissions are set to none in the creation stage. They are only applied after a lounge update.
*/

// need to think about if ALL lounges are updated after any game action (i.e., a lounge update
// action is included within the central update action)
// it is more efficient to update lounges only on state changes for a specific player

use indexmap::{IndexSet, indexset};
use smallvec::{SmallVec, smallvec};

use crate::{
    ID,
    action::{
        Action, ActionInterface, ActionResponse,
        comms::channel::{create_channel::CreateChannel, set_member::SetMember},
    },
    channel::{ChannelMember, ChannelPermission, ChannelPermissions, SenderDisplay},
    lounge::{Lounge, LoungeVariant},
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct CreateLoungeResponse {
    pub lounge_id: ID,
    pub channel_id: ID,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct CreateLounge {
    pub variant: LoungeVariant,
}

struct Participant {
    pub displays: IndexSet<SenderDisplay>,
    pub id: ID,
}

impl ActionInterface for CreateLounge {
    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        ctx: &mut crate::action::ActionContext,
        actor: &crate::action::ActionActor,
        version: crate::common::Version,
        mutate: bool,
    ) -> crate::action::ActionResult {
        actor.require_system()?;

        let channel_response = Action::CreateChannel(CreateChannel { loggable: true })
            .handle(eng, ctx, actor, version, mutate)?;
        let ActionResponse::CreateChannel(data) = channel_response else {
            unreachable!();
        };
        let channel_id = data.id;
        let lounge_id = if mutate {
            let lounge = Lounge {
                channel_id,
                variant: self.variant.clone(),
            };

            let mut participants: SmallVec<[Participant; 8]> = smallvec![];
            match &self.variant {
                LoungeVariant::Fake {
                    creator_id,
                    contacted_id,
                    contactor_id,
                } => {
                    participants.push(Participant {
                        id: *creator_id,
                        displays: indexset![
                            SenderDisplay::Raw(*contacted_id),
                            SenderDisplay::Raw(*contactor_id),
                        ],
                    });
                }
                LoungeVariant::Basic {
                    contacted_id,
                    contactor_id,
                } => {
                    participants.push(Participant {
                        id: *contactor_id,
                        displays: indexset![SenderDisplay::Raw(*contactor_id),],
                    });
                    participants.push(Participant {
                        id: *contacted_id,
                        displays: indexset![SenderDisplay::Raw(*contacted_id),],
                    });
                }
            };
            for participant in participants {
                Action::SetMember(SetMember {
                    channel_id,
                    player_id: participant.id,
                    settings: Some(ChannelMember {
                        perms: ChannelPermissions::EMPTY,
                        displays: participant.displays,
                    }),
                })
                .handle(eng, ctx, actor, version, mutate)?;

                // TODO:
                // update channel permissions of all participants
            }

            eng.world.add_lounge(lounge)
        } else {
            0
        };

        Ok(ActionResponse::CreateLounge(CreateLoungeResponse {
            lounge_id,
            channel_id,
        }))
    }
}
