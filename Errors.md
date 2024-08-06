# Error codes and explanations!
Oh no! Did you encounter an error while running the application? Thats not good. But we are here to help!
All error codes has a explanation!

When asked to reffer to the `log` file, it exists within the `logs` directory (located within the default folder for the game).
- Linux: `/home/{user}/.games/rust_station_13/logs`
- Windows: `C:\Users\{user}\Documents\My Games\Rust Station 13\logs`
- Mac: `/Users/{user}/.games/rust_station_13/logs`

## Codes:
- Err 3 -> Network error, reffer to the error message returned in the `log` file
- Err 4 -> Auth error, the user account could not be authenticated.
- Err 5 -> Protocol error, server and client missmatch.
- Err 6 -> Resource error, a problem occured while attempting to access game files. Check permissions.
- Err 7 -> Server error, a problem occured on the server and the connection was closed.
- Err 8 -> Client error, a problem occured on the client, post the `log` in a issue.
- Err 9 -> Config error, a problem with the config files were detected, either missing keys or invalid values. Regenerate the configs and try again.
- Err 10 -> Permission error, the application was denied the permissions to read/write to the required directories.
- Err 11 -> OOM error, the application ran out of memory (how?)
- Err 12 -> Corruption error, some data was corrupt, validate game files, remove save files, or submit the `log` in a issue.
- Err 13 -> Memory error, your system appears to not have enough ram to run this application. Minimum is 1GB
