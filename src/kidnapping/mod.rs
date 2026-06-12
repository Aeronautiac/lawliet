/*
* A kidnapping is a debuff + channel wrapper.
*
* A kidnapping can either be anonymous or public.
* An anonymous kidnapping does not reveal the kidnapper on release.
* A public kidnapping does.
*
* Kidnapped players may be released early by the kidnapper or a host.
*/

use crate::{ActorKey, ChannelKey, actor::ActorDisplay};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum KidnappingType {
    Anonymous,
    Public(ActorDisplay),
}

#[derive(Debug)]
pub struct Kidnapping {
    pub victim: ActorKey,
    pub channel_id: ChannelKey,
    pub kidnapping_type: KidnappingType,
    pub kidnapper: ActorKey, // this is typically an org, but in the future there may be individual
                             // kidnap abilities as well. think Mello.
}
