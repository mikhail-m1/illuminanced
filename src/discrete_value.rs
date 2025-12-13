#[derive(Debug)]
pub struct DiscreteValue {
    min: u32,
    _max: u32,
    step_size: f32,
    barrier: f32,
    last_level: f32,
}

impl DiscreteValue {
    pub fn new(min: u32, max: u32, steps_count: u32, barrier: f32) -> Self {
        DiscreteValue {
            min,
            _max: max,
            step_size: (max - min) as f32 / (steps_count - 1) as f32,
            barrier,
            last_level: 0.0,
        }
    }
    pub fn update(&mut self, level: f32) -> Option<u32> {
        let diff = level - self.last_level;
        trace!("brightness level change: {:.2} -> {:.2} (diff: {:.2}, barrier: {:.2})",
               self.last_level, level, diff, self.barrier);
        // Symmetric threshold: require crossing barrier in either direction
        if diff.abs() > self.barrier {
            self.last_level = level;
            let new_value = (level.floor() * self.step_size) as u32 + self.min;
            debug!("brightness update: level {:.2} -> hardware value {} (min={}, max={}, step_size={:.1})",
                   level, new_value, self.min, self._max, self.step_size);
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
        simplelog::TerminalMode::Stdout,
        simplelog::ColorChoice::Auto,
    );
    // DiscreteValue converts normalized brightness levels (0-10) to hardware values (10-100)
    // This prevents flickering by only updating when changes exceed a threshold.
    //
    // Parameters:
    // - min brightness = 10 (hardware minimum)
    // - max brightness = 100 (hardware maximum)
    // - 10 discrete steps (step_size = (100-10)/(10-1) = 10.0)
    // - barrier = 0.1 (threshold for detecting meaningful changes)
    //
    // Conversion formula: brightness = floor(level) * step_size + min
    // Example: level 1.11 -> floor(1.11) * 10 + 10 = 20
    let mut v = DiscreteValue::new(10, 100, 10, 0.1);

    // level=0.0: diff=0.0-0.0=0.0, abs(0.0) not > 0.1 barrier -> None
    assert_eq!(v.update(0.0), None);
    // level=1.09: diff=1.09-0.0=1.09, abs(1.09) > 0.1 barrier -> update
    // brightness = floor(1.09) * 10 + 10 = 1*10+10 = 20, last_level=1.09
    assert_eq!(v.update(1.09), Some(20));
    // level=1.11: diff=1.11-1.09=0.02, abs(0.02) not > 0.1 barrier -> None
    assert_eq!(v.update(1.11), None);
    // level=0.98: diff=0.98-1.09=-0.11, abs(-0.11) > 0.1 barrier -> update
    // brightness = floor(0.98) * 10 + 10 = 0*10+10 = 10, last_level=0.98
    assert_eq!(v.update(0.98), Some(10));
    // level=3.00: diff=3.00-0.98=2.02, abs(2.02) > 0.1 barrier -> update
    // brightness = floor(3.00) * 10 + 10 = 3*10+10 = 40, last_level=3.00
    assert_eq!(v.update(3.00), Some(40));
    // level=2.99: diff=2.99-3.00=-0.01, abs(-0.01) not > 0.1 barrier -> None
    assert_eq!(v.update(2.99), None);
    // level=3.01: diff=3.01-3.00=0.01, abs(0.01) not > 0.1 barrier -> None
    assert_eq!(v.update(3.01), None);
    // level=2.88: diff=2.88-3.00=-0.12, abs(-0.12) > 0.1 barrier -> update
    // brightness = floor(2.88) * 10 + 10 = 2*10+10 = 30, last_level=2.88
    assert_eq!(v.update(2.88), Some(30));
}
