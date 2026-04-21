use crate::config::role::Role;

// command the frontend
pub enum Command {
    AnnounceDeath {
        true_name: String,
        death_message: String,
        role: Role,
        had_notebook: bool,
    },
}
