/*
* SYSTEM ACTION
* Create links from an actor to other actors based on the role config struct
*/

use crate::{
    ID,
    action::{ActionInterface, ActionResponse, get_actor, get_actor_mut},
    actor::{ActorType, Organization},
};

#[derive(PartialEq, Eq, Clone)]
pub struct CreateActorLinksResponse {}

#[derive(PartialEq, Eq, Clone)]
pub struct CreateActorLinks {
    pub actor_id: ID,
}

impl ActionInterface for CreateActorLinks {
    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        ctx: &mut super::ActionContext,
        actor: &super::ActionActor,
        version: crate::common::Version,
        mutate: bool,
    ) -> super::ActionResult {
        actor.require_system()?;

        let target = get_actor_mut(eng, self.actor_id)?;
        if mutate {
            for (id, actor) in eng.world.actors.iter() {
                match &actor.actor_type {
                    ActorType::Player(player) => {}
                    ActorType::Org(org) => {}
                    _ => {}
                }
            }
        }

        Ok(ActionResponse::CreateActorLinks(
            CreateActorLinksResponse {},
        ))
    }
}
