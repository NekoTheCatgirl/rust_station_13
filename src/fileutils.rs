use std::{fs::File, io::{Read, Write}, path::PathBuf};

use serde::{de::DeserializeOwned, Serialize};

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

pub fn write_client_config<Data: Serialize>(file_name: &str, data: Data) -> Result<(), Box<dyn std::error::Error>>
{
    let home = get_home_path();

    let mut file = File::create(home.join("client_config").join(file_name))?;

    let toml_data = toml::to_string_pretty(&data)?;

    file.write_all(toml_data.as_bytes())?;

    Ok(())
}

pub fn read_client_config<Data: DeserializeOwned>(file_name: &str) -> Result<Data, Box<dyn std::error::Error>> {
    let home = get_home_path();

    let mut file = File::open(home.join("client_config").join(file_name))?;

    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)?;

    let toml_data = String::from_utf8(buffer)?;

    Ok(toml::from_str(&toml_data)?)
}

pub fn write_server_config<Data: Serialize>(file_name: &str, data: Data) -> Result<(), Box<dyn std::error::Error>> {
    let home = get_home_path();

    let mut file = File::create(home.join("server_config").join(file_name))?;

    let toml_data = toml::to_string_pretty(&data)?;

    file.write_all(toml_data.as_bytes())?;

    Ok(())
}

pub fn read_server_config<Data: DeserializeOwned>(file_name: &str) -> Result<Data, Box<dyn std::error::Error>> {
    let home = get_home_path();

    let mut file = File::open(home.join("server_config").join(file_name))?;

    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)?;

    let toml_data = String::from_utf8(buffer)?;

    Ok(toml::from_str(&toml_data)?)
}
