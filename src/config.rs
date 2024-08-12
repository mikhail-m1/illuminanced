use log::LevelFilter;
use std::str::FromStr;
use toml;
use ErrorCode;
use LightPoint;

pub struct Config {
    table: Option<toml::Table>,
}

impl Config {
    pub fn new(table: Option<toml::Table>) -> Self {
        Config { table: table }
    }

    pub fn log_to_syslog(&self) -> bool {
        self.get_str("daemonize", "log_to")
            .map_or(true, |v| v == "syslog")
    }

    pub fn log_filename(&self) -> &str {
        self.get_str("daemonize", "log_to")
            .unwrap_or("/var/log/illuminanced.log")
    }

    pub fn log_level(&self) -> LevelFilter {
        self.get_str("daemonize", "log_level")
            .and_then(|s| LevelFilter::from_str(s).ok())
            .unwrap_or(LevelFilter::Warn)
    }

    pub fn pid_filename(&self) -> &str {
        self.get_str("daemonize", "pid_file")
            .unwrap_or("/run/illuminanced.pid")
    }

    pub fn light_steps(&self) -> u32 {
        self.get_u32("general", "light_steps").unwrap_or(10)
    }

    pub fn min_backlight(&self) -> u32 {
        self.get_u32("general", "min_backlight").unwrap_or(70)
    }

    pub fn step_barrier(&self) -> f32 {
        self.get_f32("general", "step_barrier").unwrap_or(0.1)
    }

    pub fn check_period_in_seconds(&self) -> u64 {
        self.get_u32("general", "check_period_in_seconds")
            .unwrap_or(1) as u64
    }

    pub fn event_device_name(&self) -> &str {
        self.get_str("general", "event_device_name")
            .unwrap_or("/dev/input/event/*")
    }

    pub fn event_device_mask(&self) -> &str {
        self.get_str("general", "event_device_mask")
            .unwrap_or("Asus WMI hotkeys")
    }

    pub fn is_max_brightness_mode(&self) -> bool {
        self.get_bool("general", "enable_max_brightness_mode")
            .unwrap_or(true)
    }

    pub fn kalman_q(&self) -> f32 {
        self.get_f32("kalman", "q").unwrap_or(1.0)
    }

    pub fn kalman_r(&self) -> f32 {
        self.get_f32("kalman", "r").unwrap_or(20.0)
    }

    pub fn kalman_covariance(&self) -> f32 {
        self.get_f32("kalman", "covariance").unwrap_or(10.0)
    }

    pub fn max_backlight_filename(&self) -> &str {
        self.get_str("general", "max_backlight_file")
            .unwrap_or("/sys/class/backlight/intel_backlight/max_brightness")
    }

    pub fn backlight_filename(&self) -> &str {
        self.get_str("general", "backlight_file")
            .unwrap_or("/sys/class/backlight/intel_backlight/brightness")
    }

    pub fn illuminance_filename(&self) -> &str {
        self.get_str("general", "illuminance_file")
            .unwrap_or("/sys/bus/acpi/devices/ACPI0008:00/iio:device0/in_illuminance_raw")
    }

    pub fn light_points(&self) -> Result<Vec<LightPoint>, ErrorCode> {
        if self.table.is_none() {
            return Ok(self.default_ligth_points());
        }
        let count = self
            .get_u32("light", "points_count")
            .ok_or(ErrorCode::InvalidPointsInConfig)?;
        let points: Vec<_> = (0..count)
            .map(|i| {
                self.get_u32("light", &format!("illuminance_{}", i))
                    .and_then(|ill| {
                        self.get_u32("light", &format!("light_{}", i))
                            .map(|light| LightPoint {
                                illuminance: ill,
                                light: light,
                            })
                    })
            })
            .collect();

        if points.iter().any(|ref p| p.is_none()) {
            Err(ErrorCode::InvalidPointsInConfig)
        } else {
            Ok(points.into_iter().map(|x| x.unwrap()).collect())
        }
    }

    fn default_ligth_points(&self) -> Vec<LightPoint> {
        vec![LightPoint {
            illuminance: 700,
            light: self.light_steps() - 1,
        }]
    }

    fn get_table_val(&self, table_name: &str, name: &str) -> Option<&toml::Value> {
        if self.table.is_none() {
            return None;
        }
        let v = self
            .table
            .as_ref()
            .unwrap()
            .get(table_name)
            .and_then(|v| v.as_table())
            .and_then(|t| t.get(name));
        if v.is_none() {
            warn!("Cannot find `{}` in [{}]", name, table_name);
        }
        v
    }

    fn get_str(&self, table_name: &str, name: &str) -> Option<&str> {
        self.get_table_val(table_name, name)
            .and_then(|v| v.as_str())
    }

    fn get_u32(&self, table_name: &str, name: &str) -> Option<u32> {
        self.get_table_val(table_name, name)
            .and_then(|v| v.as_integer())
            .map(|i| i as u32)
    }

    fn get_f32(&self, table_name: &str, name: &str) -> Option<f32> {
        self.get_table_val(table_name, name)
            .and_then(|v| v.as_float())
            .map(|i| i as f32)
    }

    fn get_bool(&self, table_name: &str, name: &str) -> Option<bool> {
        self.get_table_val(table_name, name)
            .and_then(|v| v.as_bool())
            .map(|i| i as bool)
    }
}
