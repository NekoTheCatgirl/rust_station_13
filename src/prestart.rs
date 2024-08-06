pub mod dirchecks;
pub mod dirgen;
pub mod dirrem;

pub fn prestart_networkcheck() -> Result<(), ()> {
    if reqwest::blocking::get("https://www.google.com").is_err() {
        return Err(())
    }
    Ok(())
}
