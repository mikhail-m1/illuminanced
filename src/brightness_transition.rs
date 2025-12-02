#[derive(Debug)]
pub enum BrightnessTransition {
    Instant,
    Linear,
}

const INSTANT_STR: &str = "INSTANT";
const LINEAR_STR: &str = "LINEAR";

impl BrightnessTransition {
    pub fn from_str(value: &str) -> Option<Self> {
        match value.to_uppercase().as_str() {
            INSTANT_STR => Some(Self::Instant),
            LINEAR_STR => Some(Self::Linear),
            _ => None,
        }
    }
}
