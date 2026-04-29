use crate::config::role::Role;

// command the frontend
pub enum Command {
    AnnounceDeath {
        true_name: String,
        death_message: String,
        role: Role,
        notebook_transferred: bool,
        ability_transferred: bool,
    },
}
