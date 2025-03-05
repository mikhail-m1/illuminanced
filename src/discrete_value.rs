
#[derive(Debug)]
pub struct DiscreteValue {
    min: u32,
    _max: u32,
    step_size: f32,
    barrier: f32,
    last_level: u32,
}

impl DiscreteValue {
    pub fn new(min: u32, max: u32, steps_count: u32, barrier: f32) -> Self {
        DiscreteValue {
            min,
            _max: max,
            step_size: (max - min) as f32 / (steps_count - 1) as f32,
            barrier,
            last_level: 0,
        }
    }
    pub fn update(&mut self, level: f32) -> Option<u32> {
        let diff = level - self.last_level as f32;
        debug!("step diff {}", diff);
        if diff > 1.0 + self.barrier || diff < -self.barrier {
            self.last_level = level as u32;
            let new_value = (level.floor() * self.step_size) as u32 + self.min;
            Some(new_value)
        } else {
            None
        }
    }
}

#[test]
fn discrete_value_change() {
    use simplelog::{Config as LoggerConfig, LevelFilter, TermLogger};
    let _ = TermLogger::init(
        LevelFilter::Debug,
        LoggerConfig::default(),
        TerminalMode::Stdout,
        ColorChoice::Auto,
    );
    let mut v = DiscreteValue::new(10, 100, 10, 0.1);
    assert_eq!(v.update(0.0), None);
    assert_eq!(v.update(1.09), None);
    assert_eq!(v.update(1.11), Some(20));
    assert_eq!(v.update(3.00), Some(40));
    assert_eq!(v.update(2.99), None);
    assert_eq!(v.update(3.01), None);
    assert_eq!(v.update(2.88), Some(30));
}
