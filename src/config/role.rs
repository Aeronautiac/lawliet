#[derive(PartialEq, Eq)]
pub enum Role {
    Kira,
    Kira2nd,
    L,
    Watari,
    BeyondBirthday,
    PrivateInvestigator,
    NewsAnchor,
    Civilian,
    RogueCivilian,
    Poser,
    ConArtist,
    WantedCivilian,
    Near,
    Mello,
}

pub struct RoleConfig {}

impl RoleConfig {
    pub fn new() -> Self {
        RoleConfig {}
    }
}
