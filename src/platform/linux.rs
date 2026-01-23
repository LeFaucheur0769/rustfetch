use crate::common::{get_trimmed, get_value_from_file};
use nix::sys::statvfs::*;
use std::path::Path;

/// Converts KiB figures into GB, MB or unchanged based on its size.
/// Returns a formatted String based on the conversion that has happened
/// 
/// For example: 14_648.4 KiB will return "15 MB"
fn convert_to_bytes(memory_kib: f64) -> String {
    if memory_kib >= ( 1024.0 * 1024.0 ){
        // If memory is more than 1GB, transform it to GB
        // 1 GB = (1024 * 1024) KiB
        let memory_gb = round_to_two_decimal(memory_kib / (1024.0 * 1024.0));
        format!("{} GB", memory_gb)
    } else if memory_kib >= 1024.0 {
        // Same as before but with MB
        // 1 MB = 1024 KiB
        let memory_mb = round_to_two_decimal(memory_kib / 1024.0);
        format!("{} MB", memory_mb)
    } else {
        // Else, just return it in KiB
        format!("{} KiB", memory_kib)
    }
}

/// Gets the percentage as a rounded value considering a part and a total
fn get_percentage_from_part(part: f64, total: f64) -> f64 {
    (part / total * 100.0).floor()
}  

/// Extracts the first whitespace-separated numeric value from a string.
/// 
/// For example, "1234567 kB" returns 1234567.0
fn extract_numeric_value(input: String) -> f64 {
    return (input.split_whitespace().next().unwrap())
        .parse::<f64>()
        .unwrap();
}

/// Rounds an f64 variable to two decimal points
fn round_to_two_decimal(input: f64) -> f64 {
    (input * 100.0).floor() / 100.0
}

/// Returns total and used memory in GB alongside the percentage of usage
fn get_memory_usage(total_input: &str, available_input: &str) -> (String, String, f64) {
    // TODO: Check if memory total and usage is equal or more than 1GB, if not display the value in MB
    let total_string = get_value_from_file(Path::new("/proc/meminfo"), total_input, ":");
    let total_kib = extract_numeric_value(total_string);
    let total = convert_to_bytes(total_kib);

    let available_string = get_value_from_file(Path::new("/proc/meminfo"), available_input, ":");
    let available_kib = extract_numeric_value(available_string);

    let used_kib = round_to_two_decimal(total_kib - available_kib);
    let used = convert_to_bytes(used_kib);

    let percentage = get_percentage_from_part(used_kib, total_kib);

    (total, used, percentage)
}

/// Returns total and used RAM in GB alongside the percentage of usage -> (total_gb, used, percentage)
pub fn get_ram_usage() -> (String, String, f64) {
    get_memory_usage("MemTotal", "MemAvailable")
}

/// Returns total and used RAM in GB alongside the percentage of usage -> (total_gb, used, percentage)
pub fn get_swap_usage() -> (String, String, f64) {
    get_memory_usage("SwapTotal", "SwapFree")
}

/// Gets device uptime in HHh MMm SSs format
pub fn get_uptime() -> String {
    let content = get_trimmed(Path::new("/proc/uptime"));
    if let Some((uptime_string, _)) = content.split_once(" ") {
        let uptime = (uptime_string.parse::<f64>().unwrap()).floor();

        // Transform the uptime (Which is in seconds) to hours, minutes and the remainder in seconds
        let hours = (uptime / 3600.0).floor();
        let minutes = ((uptime % 3600.0) / 60.0).floor();
        let seconds = uptime % 60.0;

        if hours < 1.0 {
            return format!("{:02}m {:02}s", minutes.to_string(), seconds.to_string());
        };
        return format!(
            "{:02}h {:02}m {:02}s",
            hours.to_string(),
            minutes.to_string(),
            seconds.to_string()
        );
    }
    String::from("Null")
}

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

/// Gets disk (root) usage and returns in GB and percentage (floored)
pub fn get_disk_usage() -> (u64, u64, f64) {
    let stats = statvfs("/").unwrap();
    let block_size = stats.block_size();
    let total = stats.blocks() * block_size;
    let free = stats.blocks_available() * block_size;
    let used = total - free;

    let percentage = get_percentage_from_part(used as f64, total as f64);

    (total / 1_000_000_000, used / 1_000_000_000, percentage)
}
