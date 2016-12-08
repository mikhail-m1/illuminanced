# Ambient Light Sensor Daemon
Tested on ASUS UX305L Ubuntu 16.04 without additional divers.

## How to build & install
* install Rust: `curl https://sh.rustup.rs -sSf | sh`
* clone : `git clone https://github.com/mikhail-m1/illuminanced.git`
* build: `cd illuminanced; cargo build --release`
* install `sudo ./install.sh`

## How to Adjust

* choose how many light values do you need by `[general].light_steps`
* set defined points count by `[light].points_count`
* set each point by `illuminance_<n>` and `light_<n> where` illuminance from `in_illuminance_raw` (see below) and light in range [0..light_steps)

## How it works

Reads illuminance from `/sys/bus/acpi/devices/ACPI0008:00/iio:device0/in_illuminance_raw`, apply Kalman like filter, set backlight value base on defined points.
Unfortunately I cannot find a way how get events from [iio buffers](https://www.kernel.org/doc/htmldocs/iio/iiobuffer.html), for acpi-als driver, so now it polls.

## TODO
- support `<Fn> + a`
- keep fd's open & change user
- reread backlight before saving or D-bus integration
