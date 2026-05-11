use crate::{
    ID,
    ability::{AbilityInterface, AbilityResponse},
    action::{
        Action, ActionActor, ActionContext, ActionInterface,
        actor::player::{kill::Kill, revive::Revive, schedule_revive::ScheduleRevive},
    },
    config::{ability::AbilityName, role::Role},
};

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone)]
pub struct PseudocideResponse {}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone)]
pub struct Pseudocide {
    pub target_id: ID,
    pub true_name: String,
    pub death_message: String,
    pub role: Role,
    pub notebook_transferred: bool,
    pub ability_transferred: bool,
}

impl AbilityInterface for Pseudocide {
    fn ability_name(&self) -> crate::config::ability::AbilityName {
        AbilityName::Pseudocide
    }

    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        ctx: &mut ActionContext,
        actor: &crate::action::ActionActor,
        ability: ID,
        version: u8,
        mutate: bool,
    ) -> super::AbilityResult {
        Action::Kill(Kill {
            allow_link_chaining: false,
            sever_links: false,
            silent: true,
            set_books_dormant: true,
            death_message: None,
            killer_id: None,
            target_id: self.target_id,
        })
        .handle(eng, ctx, &ActionActor::System, version, mutate)?;

        ctx.commands.push(crate::command::Command::AnnounceDeath {
            true_name: self.true_name.to_lowercase(),
            death_message: self.death_message.clone(),
            role: self.role,
            notebook_transferred: self.notebook_transferred,
            ability_transferred: self.ability_transferred,
        });

        Action::ScheduleRevive(ScheduleRevive {
            timestamp: eng.time + eng.config.defaults.pseudocide_duration,
            revive: Revive {
                ignore_links: true,
                target_id: self.target_id,
            },
        })
        .handle(eng, ctx, &ActionActor::System, version, mutate)?;

        Ok(AbilityResponse::Pseudocide(PseudocideResponse {}))
    }
}
