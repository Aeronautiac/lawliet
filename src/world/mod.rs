pub struct World {
    pub blackout: bool,
    pub actors: Vec<crate::actor::Actor>,
    pub abilities: Vec<crate::ability::Ability>,
}

impl World {
    pub fn new() -> Self {
        World {
            blackout: false,
            actors: vec![],
            abilities: vec![],
        }
    }
}
