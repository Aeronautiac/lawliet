use crate::{
    ID,
    action::{
        ActionActor, ActionError, ActionResponse, ActionResult, Command, ResponseData,
        add_states::state_addition, get_actor,
    },
    actor::state::State,
    engine::Engine,
};

/*
* kill a player and handle side effects
*/

#[derive(PartialEq, Eq, Clone)]
pub struct KillData {}

#[derive(PartialEq, Eq, Clone)]
pub struct KillArgs {
    pub target_id: ID,
    pub killer_id: Option<ID>,
}

pub fn kill(eng: &mut Engine, actor: ActionActor, args: KillArgs) -> ActionResult {
    actor.require_system()?;

    let target = get_actor(eng, args.target_id)?;
    let mut next_actions = vec![state_addition(args.target_id, State::Dead)];

    if target.states.contains(State::Dead) {
        return Err(ActionError::ActorIsDead);
    }

    // handle stuff like ability transfers, notebook transfers, etc...
    if let Some(killer_id) = args.killer_id {
        let killer = get_actor(eng, killer_id)?;
    }

    Ok(ActionResponse {
        commands: vec![Command::AnnounceDeath {}],
        next_actions,
        data: ResponseData::Kill(KillData {}),
    })
}
