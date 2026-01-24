use crate::common::*;
use crate::sysinfo::*;
use std::path::Path;

/// Gets battery status as a tuple (Capacity, Status) if available
pub fn get_battery() -> (String, String) {
    let capacity = get_trimmed(Path::new("/sys/class/power_supply/BAT0/capacity"));
    let status = get_trimmed(Path::new("/sys/class/power_supply/BAT0/status"));
    (capacity, status)
}

/// Gets current power draw and returns it as Watts - Only available on battery-powered devices
pub fn get_power_draw() -> i32 {
    let power_draw_mw = get_trimmed(Path::new("/sys/class/power_supply/BAT0/power_now"))
        .parse::<i32>()
        .unwrap_or(0);
    power_draw_mw / 1_000_000 // power_now contains the value in microwatts, we transform it in watts
}

pub fn get_disk_usage() -> (u64, u64, u64) {
    // Linux root directory
    get_directory_usage("/")
}

pub fn format_kernel_version() -> String {
    format!("Linux {}", get_kernel_version())
}
