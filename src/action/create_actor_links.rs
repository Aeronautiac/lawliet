/*
* SYSTEM ACTION
* Create links between all actors
*/

use crate::{
    ID,
    action::{ActionInterface, ActionResponse, get_actor_mut, get_role_config},
    actor::{ActorLink, ActorLinkType, ActorType},
};

#[derive(PartialEq, Eq, Clone)]
pub struct CreateActorLinksResponse {}

#[derive(PartialEq, Eq, Clone)]
pub struct CreateActorLinks {}

struct LinkDescriptor {
    pub from_dest: ID,
    pub to_dest: ID,
    pub link_type: ActorLinkType,
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

        let mut links_to_create: Vec<LinkDescriptor> = vec![];
        for (id, _) in eng.world.actors.iter() {
            // player (role based) links
            let mut found_role = None;
            if let Some(player) = eng.world.get_player(*id) {
                found_role = Some(player.role);
            }
            if let Some(role) = found_role {
                let role_config = get_role_config(eng, role)?;
                let role_links = role_config.actor_links.clone();
                for link in role_links {
                    for (id_other, other_actor) in eng.world.actors.iter() {
                        if *id == *id_other {
                            continue;
                        }
                        if let ActorType::Player(other_player) = &other_actor.actor_type
                            && other_player.role == link.role
                        {
                            links_to_create.push(LinkDescriptor {
                                from_dest: *id,
                                to_dest: *id_other,
                                link_type: link.link_type,
                            });
                        }
                    }
                }
            }
            // TODO:
            // Org links (if added in the future)
        }

        if mutate {
            for link in links_to_create {
                let target = get_actor_mut(eng, link.from_dest)?;
                target.add_link(ActorLink {
                    link_type: link.link_type,
                    link_dest: link.to_dest,
                });
            }
        }

        Ok(ActionResponse::CreateActorLinks(
            CreateActorLinksResponse {},
        ))
    }
}
