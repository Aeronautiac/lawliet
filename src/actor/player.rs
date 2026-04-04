#[derive(PartialEq, Eq)]
pub struct Player {
    eyes: u32,
}

impl Player {
    pub fn new() -> Self {
        Player { eyes: 2 }
    }
}
