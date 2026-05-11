/*
* SYSTEM ONLY
* Try to use an organization ability
*/

use indexmap::IndexSet;

use crate::{
    ID,
    ability::AbilityBehaviour,
    action::{
        Action, ActionActor, ActionContext, ActionError, ActionInterface, ActionResponse,
        ActionResult, ability::use_ability::UseAbility, poll::create_poll::CreatePoll,
    },
    actor::{modifier::Modifier, organization::OrgAbilityPolicy},
    config::role::Role,
    helpers::get_org,
    poll::{PollPolicy, PollVisibility, VoterPolicy},
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct SystemUseOrgAbilityResponse {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct SystemUseOrgAbility {
    pub org_id: ID,
    pub ability_id: ID,
    pub ability_args: AbilityBehaviour,
    pub dont_vote: bool,
}

impl ActionInterface for SystemUseOrgAbility {
    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        ctx: &mut ActionContext,
        actor: &ActionActor,
        version: crate::common::Version,
        mutate: bool,
    ) -> ActionResult {
        actor.require_system()?;

        if let Ok(org_data) = get_org(eng, self.org_id) {
            let ActionActor::Organization(org_info) = actor else {
                unreachable!();
            };
            let player_id = org_info.player_id;

            let org_ability = org_data.abilities.get(&self.ability_id).unwrap();
            if org_data.member_count(|id, _| {
                let member_data = eng.world.get_actor(id).unwrap();
                !member_data.has_modifier(Modifier::NoPresence)
            }) < org_ability.require_members
            {
                return Err(ActionError::NotEnoughMembers);
            }

            let available_roles: IndexSet<Role> = org_data
                .members
                .keys()
                .filter(|id| {
                    let actor_data = eng.world.get_actor(**id).unwrap();
                    !actor_data.has_modifier(Modifier::NoPresence)
                })
                .map(|id| eng.world.get_player(*id).unwrap().role)
                .collect();
            for role in &org_ability.require_roles {
                if !available_roles.contains(role) {
                    return Err(ActionError::RequiredRolesNotPresent);
                }
            }

            let ability_policies = org_ability.usage_policies;
            if ability_policies.contains(OrgAbilityPolicy::RequireLeader) {
                let Some(leadership_struct) = &org_data.leadership_struct else {
                    unreachable!(); // there must be a leadership struct if the ability requires a leader
                };
                if leadership_struct.leader != Some(player_id) {
                    return Err(ActionError::PlayerIsNotLeader);
                }
            }
            if !self.dont_vote && ability_policies.contains(OrgAbilityPolicy::RequireVote) {
                Action::CreatePoll(CreatePoll {
                    voter_policy: VoterPolicy::Present,
                    visibility: PollVisibility::Org(self.org_id),
                    update_policy: PollPolicy::Majority,
                    timeout_policy: PollPolicy::Majority,
                    payload: Box::new(Action::SystemUseOrgAbility(SystemUseOrgAbility {
                        org_id: self.org_id,
                        ability_id: self.ability_id,
                        ability_args: self.ability_args.clone(),
                        dont_vote: true,
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

        Ok(ActionResponse::SystemUseOrgAbility(
            SystemUseOrgAbilityResponse {},
        ))
    }
}
