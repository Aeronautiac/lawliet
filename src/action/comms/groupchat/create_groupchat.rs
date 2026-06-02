/*
* SYSTEM ACTION
* Create a group chat
*/

use crate::{
    action::{
        Action, ActionInterface, ActionResponse, comms::channel::create_channel::CreateChannel,
    },
    command::Command,
    common::GroupchatKey,
    groupchat::Groupchat,
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct CreateGroupchatResponse {
    pub id: GroupchatKey,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct CreateGroupchat {}

impl ActionInterface for CreateGroupchat {
    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        ctx: &mut crate::action::ActionContext,
        actor: &crate::action::ActionActor,
        version: crate::common::Version,
        mutate: bool,
    ) -> crate::action::ActionResult {
        actor.admin_or_system()?;

        let channel_response = Action::CreateChannel(CreateChannel { loggable: true })
            .handle(eng, ctx, actor, version, mutate)?;
        let ActionResponse::CreateChannel(data) = channel_response else {
            unreachable!();
        };
        let channel_id = data.id;

        let id = if mutate {
            eng.world.add_groupchat(Groupchat::new(channel_id))
        } else {
            GroupchatKey::default()
        };

        ctx.push_cmd(
            Command::MapGc {
                gc_id: id,
                channel_id,
            },
            None,
            eng.time,
        );

        Ok(ActionResponse::CreateGroupchat(CreateGroupchatResponse {
            id,
        }))
    }
}
