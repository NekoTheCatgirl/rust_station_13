pub mod dirchecks;
pub mod dirgen;
pub mod dirrem;

pub fn prestart_networkcheck() -> Result<(), ()> {
    if ureq::get("https://www.google.com").call().is_err() {
        return Err(())
    }
    Ok(())
}
