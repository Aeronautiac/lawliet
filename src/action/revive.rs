use crate::ID;

// revive a player
#[derive(PartialEq, Eq, Clone)]
pub struct ReviveArgs {
    pub target_id: ID,
}
