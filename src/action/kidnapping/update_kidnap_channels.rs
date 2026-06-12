/*
* SYSTEM ACTION
* Update channel membership for every active kidnapping based on current world state.
*
* Called from AddState, RemoveState, AddToOrg, RemoveFromOrg.
*
* Victim:
*   alive + currently kidnapped → Send | View, Raw display
*   otherwise                   → EMPTY perms
*
* Kidnapper (org only):
*   each member present  → Send | View, Mysterious (anon) or Raw (public)
*   each member absent   → EMPTY perms
*
* TODO: commands & optimizations
*/

use indexmap::indexset;

use crate::{
    action::{
        Action, ActionActor, ActionContext, ActionInterface, ActionResponse, ActionResult,
        comms::channel::set_member::SetMember,
    },
    actor::{ActorDisplay, ActorType, modifier::Modifier, state::State},
    channel::{ChannelMember, ChannelPermission, ChannelPermissions},
    common::{ActorKey, ChannelKey, Version},
    engine::Engine,
    helpers::get_actor,
    kidnapping::KidnappingType,
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct UpdateKidnapChannelsResponse {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct UpdateKidnapChannels {}

struct KidnappingData {
    victim: ActorKey,
    channel_id: ChannelKey,
    kidnapping_type: KidnappingType,
    kidnapper: ActorKey,
}

struct MemberUpdate {
    player_id: ActorKey,
    channel_id: ChannelKey,
    settings: ChannelMember,
}

impl ActionInterface for UpdateKidnapChannels {
    fn handle(
        &mut self,
        eng: &mut Engine,
        ctx: &mut ActionContext,
        actor: &ActionActor,
        version: Version,
        mutate: bool,
    ) -> ActionResult {
        actor.admin_or_system()?;

        let kidnappings: Vec<KidnappingData> = eng
            .world
            .kidnappings
            .values()
            .map(|k| KidnappingData {
                victim: k.victim,
                channel_id: k.channel_id,
                kidnapping_type: k.kidnapping_type,
                kidnapper: k.kidnapper,
            })
            .collect();

        let mut updates: Vec<MemberUpdate> = vec![];

        for kd in kidnappings {
            let victim_actor = get_actor(eng, kd.victim).expect("kidnapping victim must be valid");
            let victim_perms = if !victim_actor.has_state(State::Dead)
                && victim_actor.has_state(State::Kidnapped)
            {
                ChannelPermission::Send | ChannelPermission::View
            } else {
                ChannelPermissions::EMPTY
            };
            updates.push(MemberUpdate {
                player_id: kd.victim,
                channel_id: kd.channel_id,
                settings: ChannelMember {
                    perms: victim_perms,
                    displays: indexset![ActorDisplay::Raw(kd.victim)],
                },
            });

            let org_members: Vec<ActorKey> = {
                let kidnapper_actor =
                    get_actor(eng, kd.kidnapper).expect("kidnapper must be valid");
                if let ActorType::Org(org) = &kidnapper_actor.actor_type {
                    org.members.keys().copied().collect()
                } else {
                    vec![]
                }
            };

            for member_id in org_members {
                let member_actor = get_actor(eng, member_id).expect("org member must be valid");
                let perms = if !member_actor.has_modifier(Modifier::NoPresence) {
                    ChannelPermission::Send | ChannelPermission::View
                } else {
                    ChannelPermissions::EMPTY
                };
                let display = match kd.kidnapping_type {
                    KidnappingType::Anonymous => ActorDisplay::Mysterious,
                    KidnappingType::Public(_) => ActorDisplay::Raw(member_id),
                };
                updates.push(MemberUpdate {
                    player_id: member_id,
                    channel_id: kd.channel_id,
                    settings: ChannelMember {
                        perms,
                        displays: indexset![display],
                    },
                });
            }
        }

        for update in updates {
            Action::SetMember(SetMember {
                player_id: update.player_id,
                channel_id: update.channel_id,
                settings: Some(update.settings),
            })
            .handle(eng, ctx, &ActionActor::System, version, mutate)?;
        }

        Ok(ActionResponse::UpdateKidnapChannels(
            UpdateKidnapChannelsResponse {},
        ))
    }
}
