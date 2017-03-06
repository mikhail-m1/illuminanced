use glob::glob;
use std::os::unix::io::AsRawFd;
use std::{mem, fs, io};
use std::io::Read;
use libc;
use std;

pub struct SwitchMonitor {
    fd: Option<fs::File>,
    state: State,
}

#[derive(Debug,Clone,Copy)]
pub enum State {
    Auto,
    Maximum,
    Off
}

impl SwitchMonitor {
    pub fn new(dev_mask: &str, dev_name: &str) -> Self {
        match glob(dev_mask) {
            Err(e) => error!("Cannot glob({}): {}", dev_mask, e),
            Ok(dir) => {
                for opt_item in dir {
                    match opt_item {
                        Err(e) => error!("Cannot get path from glob: {}", e),
                        Ok(item) => {
                            match fs::File::open(&item) {
                                Err(e) => error!("Cannot open {}: {}", item.to_string_lossy(), e),
                                Ok(fd) => {
                                    let mut buffer = [0u8; 256];
                                    let rc = unsafe {
                                        libc::ioctl(fd.as_raw_fd(), 0x8_100_45_06, &mut buffer)
                                    };
                                    if rc == -1 {
                                        error!("Cannot get device name for {}: errno {}",
                                               item.to_string_lossy(),
                                               io::Error::last_os_error());
                                    } else {
                                        let name = String::from_utf8_lossy(&buffer[0..rc as usize]);
                                        debug!("found input device {:?} `{}`", item, name);
                                        if name.starts_with(dev_name) {
                                            debug!("use {:?} `{}`", item, name);
                                            return SwitchMonitor { fd: Some(fd), state: State::Auto };
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        SwitchMonitor { fd: None, state: State::Auto }
    }

    pub fn wait_state_update(&mut self, timeout: u64) -> (State, bool) {
        if self.fd.is_none() {
            std::thread::sleep(std::time::Duration::from_secs(timeout));
            return (self.state, false);
        }
        let mut fd = self.fd.as_ref().unwrap();

        let rc = unsafe {
            use std::ptr::null_mut;

            let mut set: libc::fd_set = mem::zeroed();
            let mut timeval: libc::timeval = mem::zeroed();
            timeval.tv_sec = timeout as i64;
            libc::FD_SET(fd.as_raw_fd(), &mut set);
            libc::select(fd.as_raw_fd() + 1,
                         &mut set,
                         null_mut(),
                         null_mut(),
                         &mut timeval)
        };

        if rc == -1 {
            error!("Cannot select on event fd: {}", io::Error::last_os_error());
        }
        if rc != 1 {
            return (self.state, false);
        }

        #[repr(C)]
        #[derive(Debug)]
        struct Event {
            sec: i64,
            usec: i64,
            event_type: u16,
            code: u16,
            value: i32,
        }

        const SIZE: usize = 24;
        assert_eq!(SIZE, mem::size_of::<Event>());
        let event = unsafe {
            let mut event: Event = mem::zeroed();
            let mut u: &mut [u8; SIZE] = mem::transmute(&mut event);
            if let Err(e) = fd.read_exact(u) {
                error!("Cannot read from event device: {}", e);
                return (self.state, false);
            }
            event
        };
        debug!("input event received: {:?}", event);
        if event.event_type == 1 /*KEY*/ && event.code == 0x230/*KEY_ALS_TOGGLE*/ && event.value == 1 {
            self.state = match self.state {
                State::Auto => State::Off,
                State::Off => State::Maximum,
                State::Maximum => State::Auto,
            };
            return (self.state, true)
        }
        (self.state, false)
    }
}
