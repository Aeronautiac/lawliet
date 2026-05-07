/*
* SYSTEM ACTION
* Destroy all volatile resources associated with an actor
*/

use crate::{
    ID,
    action::{ActionActor, ActionContext, ActionInterface, ActionResponse, ActionResult},
    helpers::{get_ability, get_actor, get_notebook, get_passive},
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct PurgeVolatilesResponse {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct PurgeVolatiles {
    pub actor_id: ID,
}

impl ActionInterface for PurgeVolatiles {
    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        ctx: &mut ActionContext,
        actor: &ActionActor,
        version: crate::common::Version,
        mutate: bool,
    ) -> ActionResult {
        actor.require_system()?;

        // if the actor struct contains a reference to an object that doesn't exist, there is
        // something wrong with the engine, and the engine should crash.
        let target_actor = get_actor(eng, self.actor_id)?;
        let mut remove_abilities: Vec<ID> = vec![];
        let mut remove_passives: Vec<ID> = vec![];
        let mut remove_notebooks: Vec<ID> = vec![];
        for id in target_actor.abilities.iter() {
            let ability = get_ability(eng, *id).unwrap();
            if ability.ownership_struct.volatile {
                remove_abilities.push(*id);
            }
        }
        for id in target_actor.passives.iter() {
            let passive = get_passive(eng, *id).unwrap();
            if passive.ownership_struct.volatile {
                remove_passives.push(*id);
            }
        }
        for id in target_actor.notebooks.iter() {
            let notebook = get_notebook(eng, *id).unwrap();
            if notebook.volatile {
                remove_notebooks.push(*id);
            }
        }

        if mutate {
            for id in remove_abilities {
                eng.world.remove_ability(id);
            }
            for id in remove_passives {
                eng.world.remove_passive(id);
            }
            for id in remove_notebooks {
                eng.world.remove_notebook(id);
            }
        }

        Ok(ActionResponse::PurgeVolatiles(PurgeVolatilesResponse {}))
    }
}
