use log::error;

/// Logs the error code and the description, and returns it as a i32 for use with exit(code)
///
/// # Error codes:
/// 3  -> Network Error
/// 4  -> Auth Error
/// 5  -> Protocol Error
/// 6  -> Resource Error
/// 7  -> Server Error
/// 8  -> Client Error
/// 9  -> Config Error
/// 10 -> Permission Error
/// 11 -> OOM Error
/// 12 -> Corrupt Error
/// 13 -> Memory Error
pub fn log_error_code(code: u16) -> i32 {
    match code {
        3  => error!("Error {code}. Network error, unable to establish or maintain a stable connection. Check your router or ISP."),
        4  => error!("Error {code}. Auth error, unable to authenticate user account."),
        5  => error!("Error {code}. Protocol error, server and client version missmatch."),
        6  => error!("Error {code}. Resource error, a problem occured while attempting to access game files."),
        7  => error!("Error {code}. Server error, a problem on the server occured and the connection was closed."),
        8  => error!("Error {code}. Client error, a problem occured on the client."),
        9  => error!("Error {code}. Config error, a problem with the configuration files were detected, either missing keys or invalid values, regenerate the configs and try again."),
        10 => error!("Error {code}. Permission error, the application was denied the permission to read/write to the required directories."),
        11 => error!("Error {code}. OOM error, the game ran out of memory (how?)"),
        12 => error!("Error {code}. Corrupt error, some data was corrupt, validate game files, remove save files, or create a new issue with a full detailed log attached."),
        13 => error!("Error {code}. Memory error, your system appears to not have enough ram to run this game. Minimum ram is 1GB"),
        _  => error!("Error {code}. Unexpected error, this is unintended behaviour and should be reported!"),
    }
    code as i32
}
