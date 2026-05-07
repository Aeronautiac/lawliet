use crate::{Time, config::ability::AbilityIdentifier};

pub struct DefaultConfig {
    pub death_message: String,
    pub life_link_death_message: String,
    pub pseudocide_duration: Time,                   // milliseconds
    pub universal_abilities: Vec<AbilityIdentifier>, // the abilities that everyone gets regardless
    // of role
    pub notebook_successes_per_day: u16,
    pub notebook_failures_per_day: u16,
    pub org_vote_time: Time,
}

pub fn default_defaults() -> DefaultConfig {
    DefaultConfig {
        death_message: "They died from a sudden heart attack.".into(),
        life_link_death_message: "They died to a sudden heart attack.".into(),
        pseudocide_duration: 24 * 60 * 60 * 1000, // 24 hrs
        universal_abilities: vec![AbilityIdentifier {
            name: crate::config::ability::AbilityName::Contact,
            variant: 0,
        }],
        notebook_successes_per_day: 1,
        notebook_failures_per_day: 3,
        org_vote_time: 6 * 60 * 60 * 1000, // 6 hrs
    }
}
