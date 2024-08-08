use std::{env, fs, path::PathBuf};

use clap::{CommandFactory, ValueEnum};
use clap_complete::{generate_to, Shell};
use clap_complete_fig::Fig;
use clap_complete_nushell::Nushell;
use walkdir::WalkDir;

include!("src/args.rs");

fn main() -> std::io::Result<()> {
    // Get the build profile (debug or release)
    let profile = env::var("PROFILE").expect("Unable to fetch the current profile");

    // Determine the target directory
    let target_dir = env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target".to_string());
    let out_dir = PathBuf::from(target_dir).join(profile);

    // Define the source and dest paths
    let source_dir = PathBuf::from("assets");
    let dest_dir = out_dir.join("assets");

    // Copy all assets to the output directory
    if source_dir.exists() {
        fs::create_dir_all(&dest_dir)?;

        WalkDir::new(&source_dir)
            .into_iter()
            .filter_map(|entry| entry.ok()) // Filter out error paths
            .filter_map(|entry| {
                if entry.path_is_symlink() {
                    None
                } else {
                    Some(entry)
                }
            }) // Filter out symlinks
            .for_each(|entry| {
                let entry_path = entry.path();

                // Generate a relative path (only for nested files)
                let rel_path = entry_path
                    .strip_prefix(&source_dir)
                    .expect("Couldnt strip the prefix");

                if entry_path.is_file() {
                    let target_path = dest_dir.join(rel_path);
                    fs::create_dir_all(
                        target_path
                            .clone()
                            .parent()
                            .expect("Unable to get the parent of the path"),
                    )
                    .expect("Unable to make directories for the assets");
                    fs::copy(entry_path, target_path).expect("Unable to move assets");
                }
            });

        println!("cargo:note=Assets copied to: {:?}", dest_dir);
    } else {
        panic!("Assets dir was expected, but not found.")
    }

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

        println!("cargo:note=completion file is generated: {path:?}");
    }
    let path = generate_to(Nushell, &mut cmd, "rust_station_13", outdir.clone())?;

    println!("cargo:note=completion file is generated: {path:?}");

    let path = generate_to(Fig, &mut cmd, "rust_station_13", outdir.clone())?;

    println!("cargo:note=completion file is generated: {path:?}");

    Ok(())
}
