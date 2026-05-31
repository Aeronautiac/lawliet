/*
* SYSTEM ACTION
* Top-level engine initialization. Seeds the RNG and initializes world state.
*/

use crate::{
    action::{
        Action, ActionActor, ActionContext, ActionInterface, ActionResponse, ActionResult,
        world::{
            initialize_world::InitializeWorld,
            set_random_seed::SetRandomSeed,
        },
    },
    common::{Seed, Version},
    engine::Engine,
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct InitializeEngineResponse {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct InitializeEngine {
    pub seed: Seed,
}

impl ActionInterface for InitializeEngine {
    fn handle(
        &mut self,
        eng: &mut Engine,
        ctx: &mut ActionContext,
        actor: &ActionActor,
        version: Version,
        mutate: bool,
    ) -> ActionResult {
        actor.require_system()?;

        Action::SetRandomSeed(SetRandomSeed { seed: self.seed })
            .handle(eng, ctx, actor, version, mutate)?;

        Action::InitializeWorld(InitializeWorld {})
            .handle(eng, ctx, actor, version, mutate)?;

        Ok(ActionResponse::InitializeEngine(InitializeEngineResponse {}))
    }
}
