pub mod dirchecks;
pub mod dirgen;
pub mod dirrem;

pub fn prestart_networkcheck() -> Result<(), ()> {
    if let Err(_) = reqwest::blocking::get("https://www.google.com") {
        return Err(())
    }
    Ok(())
}