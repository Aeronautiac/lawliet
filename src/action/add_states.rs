use crate::{
    ID,
    action::{Action, ActionActor, ActionResponse, ActionResult, ResponseData, get_actor},
    actor::state::State,
    engine::Engine,
};

/*
* Add states and any associated restrictions found in engine config
*/

pub struct AddStateData {}

#[derive(PartialEq, Eq, Clone)]
pub struct AddStateArgs {
    pub actor_id: ID,
    pub state: State,
}

pub fn state_addition(actor_id: ID, state: State) -> Action {
    Action::AddState(AddStateArgs { actor_id, state })
}

pub fn add_state(eng: &mut Engine, actor: ActionActor, args: AddStateArgs) -> ActionResult {
    actor.require_system()?;

    let restrictions = eng
        .config
        .state_restrictions
        .get(&args.state)
        .cloned()
        .unwrap_or_default();

    let target = get_actor(eng, args.actor_id)?;
    target.add_state(args.state, restrictions);

    Ok(ActionResponse {
        commands: vec![],
        next_actions: vec![],
        data: ResponseData::AddState(AddStateData {}),
    })
}
