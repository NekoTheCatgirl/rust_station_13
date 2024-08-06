use std::path::PathBuf;

/// Returns the current users home directory path (setup for linux, windows and mac)
pub fn get_home_path() -> PathBuf {
    #[cfg(target_os = "linux")]
    {
        PathBuf::from(format!("/home/{}/.games/rust_station_13/", whoami::username()))
    }
    #[cfg(target_os = "windows")]
    {
        PathBuf::from(format!("C:\\Users\\{}\\Documents\\My Games\\rust_station_13\\", whoami::username()))
    }
    #[cfg(target_os = "macos")]
    {
        PathBuf::from(format!("/Users/{}/.games/rust_station_13/", whoami::username()))
    }
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        // If its not any of the supported operating systems, panic.
        panic!("Unsupported operating system");
    }
}