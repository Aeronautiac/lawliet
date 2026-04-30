/*
* SYSTEM ACTION
* Sever every link to an actor ID
*/

use crate::{
    ID,
    action::{ActionInterface, ActionResponse},
};

#[derive(PartialEq, Eq, Clone)]
pub struct SeverLinksResponse {}

#[derive(PartialEq, Eq, Clone)]
pub struct SeverLinks {
    pub actor_id: ID,
}

impl ActionInterface for SeverLinks {
    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        ctx: &mut super::ActionContext,
        actor: &super::ActionActor,
        version: crate::common::Version,
        mutate: bool,
    ) -> super::ActionResult {
        actor.require_system()?;

        for (_, actor) in eng.world.actors.iter_mut() {
            let links = actor.actor_links.clone();
            for link in links {
                if link.link_dest == self.actor_id {
                    actor.sever_link(link);
                }
            }
        }

        Ok(ActionResponse::SeverLinks(SeverLinksResponse {}))
    }
}
