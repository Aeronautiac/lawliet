use crate::config::role::Role;

#[derive(PartialEq, Eq)]
pub struct Player {
    pub role: Role,
    pub true_name: String,
    pub eyes: u32,
}

impl Player {
    pub fn new(true_name: String, role: Role) -> Self {
        Player {
            role,
            true_name,
            eyes: 2,
        }
    }
}
