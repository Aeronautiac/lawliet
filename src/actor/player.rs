use std::rc::Rc;

use crate::config::role::Role;

#[derive(PartialEq, Eq, Debug)]
pub struct Player {
    pub role: Role,
    pub true_name: Rc<str>,
    pub eyes: u32,
}

impl Player {
    pub fn new(name: &str, role: Role) -> Self {
        let true_name = Rc::from(name);
        Player {
            role,
            true_name,
            eyes: 2,
        }
    }
}
