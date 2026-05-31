/*
* SYSTEM ACTION
* Set the engine RNG seed.
*/

use crate::{
    action::{ActionInterface, ActionResponse},
    common::Seed,
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct SetRandomSeedResponse {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct SetRandomSeed {
    pub seed: Seed,
}

impl ActionInterface for SetRandomSeed {
    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        _ctx: &mut crate::action::ActionContext,
        actor: &crate::action::ActionActor,
        _version: crate::common::Version,
        mutate: bool,
    ) -> crate::action::ActionResult {
        actor.require_system()?;
        if mutate {
            eng.rng_state = self.seed;
        }
        Ok(ActionResponse::SetRandomSeed(SetRandomSeedResponse {}))
    }
}
