use crate::ID;

#[derive(Debug)]
pub struct Bug {
    pub target_id: ID,
    pub ability_id: ID,
    pub enabled: bool,
}

impl Bug {
    pub fn new(target_id: ID, ability_id: ID) -> Self {
        Bug {
            target_id,
            ability_id,
            enabled: true,
        }
    }
}
