# Ambient Light Sensor Daemon For Linux
It's user mode daemon for change brightness base on light sensor value, designed for Asus Zenbooks, but may be used for other vendors after some tune.

## How to test befor install
Run from a terminal `sudo watch cat /sys/bus/acpi/devices/ACPI0008\:00/iio\:device0/in_illuminance_raw` and check that number is changing (try close the sensor or add more light). If the number is still the same it means the sensor driver doesn't work. If you see file not found error try to find correct path for `in_illuminance_raw` inside `/sys/bus/acpi/devices/`.

For ZBook 15 G6, and may be others, sensor path is `/sys/bus/iio/devices/iio:device0/in_illuminance_raw`. So try `sudo watch cat /sys/bus/iio/devices/iio:device0/in_illuminance_raw` and chenge the config if it works for you.

## Supported laptops

Works on ASUS Zenbooks with build in driver acpi-als:
* UX303UB
* UX305LA
* UX305FA
* UX310UQ
* UX330UA

On Dell Inspiron 13 7353, need to change driver path and brightness levels.

Some times works (base on responses)
* UX303UA
* UX305CA with [als driver](https://github.com/danieleds/als)
* UX430UQ Ubuntu with build in driver acpi-als, an extra ACPI call to enable the sensor `(_SB.PCI1.LPCB.EC0.ALSC)`
* UX410UQ

Doesn't work on Zenbooks driver issue:
* UX303LN
* UX305UA
* UX31A
* UX32LN

Something wrong with Arch Linux may be related with syslog, a pull request is appreciated

Please fill [a response form](https://drive.google.com/open?id=1mjr_R3nXBFAeObI7zB7BPD_EpSvTTpOf_H67x-HE2qo), it may helps other users

Keyboard back light is not adjust because my laptop doesn't have it. Want to help? Create an issue.

## How to build & install
* install Rust: `curl https://sh.rustup.rs -sSf | sh`
* clone : `git clone https://github.com/mikhail-m1/illuminanced.git`
* build: `cd illuminanced; cargo build --release`
* install `sudo ./install.sh`

## How to Adjust
* open a config file `/usr/local/etc/illuminanced.toml` (Default)
* choose how many light values do you need by `[general].light_steps`
* set defined points count by `[light].points_count`
* set each point by `illuminance_<n>` and `light_<n> where` illuminance from `in_illuminance_raw` (see below) and light in range `[0..light_steps)`

## How it works
Reads illuminance from `/sys/bus/acpi/devices/ACPI0008:00/iio:device0/in_illuminance_raw`, apply Kalman like filter, set back light value base on defined points.
Unfortunately I cannot find a way how get events from [iio buffers](https://www.kernel.org/doc/htmldocs/iio/iiobuffer.html), for acpi-als driver, so now it polls.

## `<Fn> + A`
Switches three modes:
- Auto adjust
- Disabled
- Max brightness (useful for movies, can be disabled by config file `/usr/local/etc/illuminanced.toml`)

## Contribution
Any feedback are welcome
