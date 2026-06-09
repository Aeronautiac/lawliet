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
    pub debate_default_timeout: Time,
    pub debate_shortened_timeout: Time,
    pub custody_timeout: Time,
    pub trial_vote_duration: Time,
    pub presentation_grace_timeout: Time,
    pub presentation_timeout: Time,
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
        org_vote_time: 6 * 60 * 60 * 1000,          // 6 hrs
        presentation_grace_timeout: 60 * 60 * 1000, // 1 hr
        presentation_timeout: 30 * 60 * 1000,       // 30 min
        debate_default_timeout: 60 * 60 * 1000,     // 1 hr
        debate_shortened_timeout: 15 * 60 * 1000,   // 15 minutes
        custody_timeout: 4 * 60 * 60 * 1000,        // 4 hrs
        trial_vote_duration: 6 * 60 * 60 * 1000,    // 6 hrs
    }
}
