use indexmap::IndexMap;

use crate::{
    actor::modifier::{Modifier, Modifiers},
    channel::{ChannelPermission, ChannelPermissions},
    chargepool::PoolSpecifier,
};

#[derive(Hash, Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub enum WorldChargePoolName {
    Prosecution,
}

#[derive(Hash, Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub enum WorldChannelName {
    News,
    Courtroom,
    General,
}

pub struct WorldChannelConfig {
    pub default_perms: ChannelPermissions,
    pub send_blocking: Modifiers,
    pub view_blocking: Modifiers,
}

pub struct WorldConfig {
    pub charge_pools: IndexMap<WorldChargePoolName, PoolSpecifier>,
    pub world_channels: IndexMap<WorldChannelName, WorldChannelConfig>,
}

impl WorldConfig {
    pub fn new() -> Self {
        let mut pools = IndexMap::new();
        pools.insert(
            WorldChargePoolName::Prosecution,
            PoolSpecifier {
                charges: 2,
                reset_time: 1,
            },
        );

        let mut channels = IndexMap::new();
        channels.insert(
            WorldChannelName::News,
            WorldChannelConfig {
                default_perms: ChannelPermission::View.into(),
                send_blocking: Modifier::NoContact.into(),
                view_blocking: Modifier::NoPresence.into(),
            },
        );
        channels.insert(
            WorldChannelName::Courtroom,
            WorldChannelConfig {
                default_perms: ChannelPermission::View.into(),
                send_blocking: Modifier::NoContact.into(),
                view_blocking: Modifier::NoPresence.into(),
            },
        );
        channels.insert(
            WorldChannelName::General,
            WorldChannelConfig {
                default_perms: ChannelPermission::Send | ChannelPermission::View,
                send_blocking: Modifier::NoContact.into(),
                view_blocking: Modifier::NoContact.into(),
            },
        );

        WorldConfig {
            charge_pools: pools,
            world_channels: channels,
        }
    }
}
