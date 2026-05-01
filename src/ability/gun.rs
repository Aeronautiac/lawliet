use crate::{
    ID,
    ability::{AbilityInterface, AbilityResponse},
    action::{Action, ActionContext, ActionInterface, actor_id, kill::Kill},
    config::ability::AbilityName,
};

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone)]
pub struct GunResponse {}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone)]
pub struct Gun {
    pub target_id: ID,
}

impl AbilityInterface for Gun {
    fn ability_name(&self) -> crate::config::ability::AbilityName {
        AbilityName::Gun
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
        let id = actor_id(actor);

        Action::Kill(Kill {
            allow_link_chaining: true,
            sever_links: true,
            silent: false,
            death_message: Some("They were found dead with 3 gunshot wounds to the back of the head. Their death was ruled a suicide.".into()),
            killer_id: id,
            target_id: self.target_id,
        })
        .handle(eng, ctx, actor, version, mutate)?;

        Ok(AbilityResponse::Gun(GunResponse {}))
    }
}
