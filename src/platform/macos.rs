pub fn get_disk_usage() -> (u64, u64, u64) {
    // MacOS root directory
    get_directory_usage("/")
}

pub fn format_kernel_version() -> String {
    format!("MacOS {}", get_kernel_version())
}
