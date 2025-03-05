# Ambient Light Sensor Daemon for Linux
A user-space daemon that automatically adjusts screen brightness based on light sensor readings.

## Testing Before Installation

First, identify the light sensor device path, try to run from a terminal `find /sys/bus -name in_illuminance_raw`. In some cases the device name can be significantly different, try to broaden the search: `find /sys/bus -name '*illuminance*'`. When you have found a device, check that it works, for example:
`sudo watch cat /sys/bus/acpi/devices/ACPI0008\:00/iio\:device0/in_illuminance_raw`. The readings should be changing (try closing the sensor or add more light). If the number is still the same it means the sensor driver doesn't work properly.

For ZBook 15 G6, Framework laptops, and maybe others, the sensor path is `/sys/bus/iio/devices/iio:device0/in_illuminance_raw`.

You need to put the device path to the config before starting the daemon.
On some laptops, the device name can be different after each reboot, you can use `*` in the device path in the config.

## Supported laptops

Works on ASUS Zenbooks with built-in driver acpi-als:
* UX303UB
* UX305LA
* UX305FA
* UX310UQ
* UX330UA

Also works on Framework laptops:
* Framework 13 AMD
* Framework 16 AMD

## Install deb package
Download the deb package from the last release.

To install and start run next commands after download:
```
sudo dpkg -i ~/Download/illuminanced_1.0-0.deb
# change config if device path is different
sudo systemctl enable illuminanced.service
sudo systemctl start illuminanced.service
```

You can check status by running `systemctl status illuminanced.service`

## Build & install
* install Rust: `curl https://sh.rustup.rs -sSf | sh`
* clone : `git clone https://github.com/mikhail-m1/illuminanced.git`
* build: `cd illuminanced; cargo build --release`
* change device path if needed
* install `sudo ./install.sh`

## Troubleshooting
If the service fails to start, try to run it in foreground mode to see what is wrong: `sudo /usr/local/sbin/illuminanced -d`.

## How to Adjust
* open a config file `/usr/local/etc/illuminanced.toml` (Default)
* choose how many light values do you need by `[general].light_steps`
* set defined points count by `[light].points_count`
* set each point by `illuminance_<n>` and `light_<n> where` illuminance from `in_illuminance_raw` (see below) and light in range `[0..light_steps)`

## How it works
The daemon reads illuminance from `/sys/bus/acpi/devices/ACPI0008:00/iio:device0/in_illuminance_raw`, applies Kalman-like filter, set backlight value based on defined points.
Unfortunately, I cannot find a way how to get events from [iio buffers](https://www.kernel.org/doc/htmldocs/iio/iiobuffer.html), for acpi-als driver, so the daemon check the value every second.

## `<Fn> + A`
My laptop has a special key to control brightness, which sends `KEY_ALS_TOGGLE		0x230	/* Ambient light sensor */` code. There is an open ticket about making it configurable, but I am not sure what is a good replacement, you can add your opinion.

Switches three modes:
- Auto adjust
- Disabled
- Max brightness (useful for movies, can be disabled by config file `/usr/local/etc/illuminanced.toml`)
