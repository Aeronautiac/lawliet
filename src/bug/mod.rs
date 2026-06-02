use crate::common::{AbilityKey, ActorKey};

#[derive(Debug)]
pub struct Bug {
    pub target_id: ActorKey,
    pub ability_id: AbilityKey,
    pub enabled: bool,
}

impl Bug {
    pub fn new(target_id: ActorKey, ability_id: AbilityKey) -> Self {
        Bug {
            target_id,
            ability_id,
            enabled: true,
        }
    }
}
