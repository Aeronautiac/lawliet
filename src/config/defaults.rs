use crate::Timestamp;

pub struct DefaultConfig {
    pub death_message: String,
    pub pseudocide_duration: Timestamp, // milliseconds
}

pub fn default_defaults() -> DefaultConfig {
    DefaultConfig {
        death_message: "They died from a sudden heart attack.".into(),
        pseudocide_duration: 24 * 60 * 60 * 1000, // 24 hrs
    }
}
