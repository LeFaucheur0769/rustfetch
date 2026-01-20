use std::path::Path;
mod platform;

/*
    TODO:
    Error Detection in case function returns "Null"
    Test with other distros
    Finish translating to rust
*/



fn main() {
        println!(
            "OS: {} ({})",
            platform::get_value_from_file(Path::new("/etc/os-release"), "PRETTY_NAME", "="),
            std::env::consts::ARCH // CPU Architecture the program was compiled for
        );
        println!(
            "Kernel: Linux {}",
            platform::get_trimmed(Path::new("/proc/sys/kernel/osrelease"))
        );
        println!(
            "CPU: {}",
            platform::get_value_from_file(Path::new("/proc/cpuinfo"), "model name", ":")
        );
        let (total, used, percentage) = platform::get_memory_usage("MemTotal", "MemAvailable");
        println!("RAM: {}GB / {}GB ({}% used)", used, total, percentage);
        let (total, used, percentage) = platform::get_memory_usage("SwapTotal", "SwapFree");
        println!("Swap: {}GB / {}GB ({}% used)", used, total, percentage);
        println!("Uptime: {}", platform::get_uptime());

        let (capacity, status) = platform::get_battery();
        if capacity != String::from("Null") && status != String::from("Null") {
            println!("Battery: {}% ({})", capacity, status);
        }
}
