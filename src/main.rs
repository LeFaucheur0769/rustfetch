pub mod common;
pub mod config;
pub mod platform;

use config::load_config;

/*
    TODO:
    Error Detection in case function returns "Null"
    Test with other distros
    Add ASCII art
    Implement MacOS functions
*/

fn main() {
    let config = load_config();

    if config.display.os {
        common::display_os();
    }
    if config.display.kernel {
        common::display_kernel();
    }
    if config.display.cpu {
        common::display_cpu();
    }
    if config.display.ram {
        common::display_ram_usage();
    }
    if config.display.swap {
        common::display_swap_usage();
    }
    if config.display.uptime {
        common::display_uptime();
    }
    if config.display.battery {
        common::display_battery();
    }
    if config.display.power_draw {
        common::display_power_draw();
    }
    if config.display.disk {
        common::display_disk_usage();
    }
}
