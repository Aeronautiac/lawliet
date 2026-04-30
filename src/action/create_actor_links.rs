/*
* SYSTEM ACTION
* Create links from an actor to other actors based on the role config struct
*/

use crate::{
    ID,
    action::{ActionInterface, ActionResponse, get_actor, get_actor_mut, get_role_config},
    actor::{ActorLink, ActorType, Organization},
};

#[derive(PartialEq, Eq, Clone)]
pub struct CreateActorLinksResponse {}

#[derive(PartialEq, Eq, Clone)]
pub struct CreateActorLinks {
    pub actor_id: ID,
}

// for every link defined in config, go through every actor and create the link if possible

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
        get_actor(eng, self.actor_id)?;

        let mut links_to_create: Vec<ActorLink> = vec![];

        // player (role based) links
        let mut found_role = None;
        if let Some(player) = eng.world.get_player(self.actor_id) {
            found_role = Some(player.role);
        }
        if let Some(role) = found_role {
            let role_config = get_role_config(eng, role)?;
            let role_links = role_config.actor_links.clone();
            for link in role_links {
                for (id, actor) in eng.world.actors.iter() {
                    if let ActorType::Player(other_player) = &actor.actor_type
                        && other_player.role == link.role
                    {
                        links_to_create.push(ActorLink {
                            link_type: link.link_type,
                            link_dest: *id,
                        });
                    }
                }
            }
        }

        // TODO: Organization links (if added in the future)

        let target = get_actor_mut(eng, self.actor_id)?;
        if mutate {
            for link in links_to_create {
                target.add_link(link);
            }
        }

        Ok(ActionResponse::CreateActorLinks(
            CreateActorLinksResponse {},
        ))
    }
}
