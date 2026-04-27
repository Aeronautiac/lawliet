pub struct DefaultConfig {
    pub death_message: String,
}

pub fn default_defaults() -> DefaultConfig {
    DefaultConfig {
        death_message: "They died from a sudden heart attack.".into(),
    }
}

