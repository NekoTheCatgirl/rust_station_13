use std::env;

use clap::{CommandFactory, ValueEnum};
use clap_complete::{generate_to, Shell};
use clap_complete_fig::Fig;
use clap_complete_nushell::Nushell;

include!("src/args.rs");

fn main() -> std::io::Result<()> {
    // Generate the man page
    let outdir = std::path::PathBuf::from(env::var_os("OUT_DIR").expect("Unable to fetch outdir"));
    let cmd = ApplicationArguments::command();
    let man = clap_mangen::Man::new(cmd);
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;

    std::fs::write(outdir.join("rust_station_13.1"), buffer)?;
    // Generate shell autocompletes
    let outdir = env::var_os("OUT_DIR").expect("Unable to fetch outdir");
    let mut cmd = ApplicationArguments::command();
    for &shell in Shell::value_variants() {
        let path = generate_to(shell, &mut cmd, "rust_station_13", outdir.clone())?;

        println!("cargo:warning=completion file is generated: {path:?}");
    }
    let path = generate_to(Nushell, &mut cmd, "rust_station_13", outdir.clone())?;

    println!("cargo:warning=completion file is generated: {path:?}");

    let path = generate_to(Fig, &mut cmd, "rust_station_13", outdir.clone())?;

    println!("cargo:warning=completion file is generated: {path:?}");
    
    Ok(())
}