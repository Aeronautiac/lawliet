/*
* PLAYER ONLY
* Try to use an organization ability
* This action wraps UseAbility
*/

use crate::{
    ID,
    ability::AbilityBehaviour,
    action::{
        Action, ActionActor, ActionContext, ActionError, ActionInterface, ActionResponse,
        ActionResult, ability::use_ability::UseAbility, poll::create_poll::CreatePoll,
    },
    actor::organization::OrgAbilityPolicy,
    helpers::get_org,
    poll::{PollPolicy, PollVisibility, VoterPolicy},
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct UseOrgAbilityResponse {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct UseOrgAbility {
    pub org_id: ID,
    pub ability_id: ID,
    pub ability_args: AbilityBehaviour,
}

impl ActionInterface for UseOrgAbility {
    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        ctx: &mut ActionContext,
        actor: &ActionActor,
        version: crate::common::Version,
        mutate: bool,
    ) -> ActionResult {
        actor.player_only()?;

        if let Ok(org_data) = get_org(eng, self.org_id) {
            let ActionActor::Organization(org_info) = actor else {
                unreachable!();
            };
            let player_id = org_info.player_id;
            let ability_policies = org_data.abilities.get(&self.ability_id).unwrap();
            if ability_policies.contains(OrgAbilityPolicy::RequireLeader) {
                let Some(leadership_struct) = &org_data.leadership_struct else {
                    unreachable!(); // there must be a leadership struct if the ability requires a leader
                };
                if leadership_struct.leader != player_id {
                    return Err(ActionError::PlayerIsNotLeader);
                }
            }
            if ability_policies.contains(OrgAbilityPolicy::RequireVote) {
                Action::CreatePoll(CreatePoll {
                    voter_policy: VoterPolicy::Present,
                    visibility: PollVisibility::Org(self.org_id),
                    update_policy: PollPolicy::Majority,
                    timeout_policy: PollPolicy::Majority,
                    payload: Box::new(Action::UseAbility(UseAbility {
                        ability_id: self.ability_id,
                        ability_args: self.ability_args.clone(),
                    })),
                    duration: Some(eng.config.defaults.org_vote_time),
                })
                .handle(eng, ctx, actor, version, mutate)?;
            } else {
                Action::UseAbility(UseAbility {
                    ability_id: self.ability_id,
                    ability_args: self.ability_args.clone(),
                })
                .handle(eng, ctx, actor, version, mutate)?;
            }
        }

        Ok(ActionResponse::UseOrgAbility(UseOrgAbilityResponse {}))
    }
}
