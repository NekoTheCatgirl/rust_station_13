//! The purpose of this is for servers to be able to host their own assets.

use axum::{http::StatusCode, routing::get, serve, Router};
use log::*;
use serde::{Deserialize, Serialize};
use std::{
    env,
    fs::{self, File},
    io,
    net::SocketAddr,
    path::{Path, PathBuf},
};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use walkdir::WalkDir;

#[derive(Serialize, Deserialize, Clone)]
pub struct AssetEntry {
    path: String,
    checksum: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Manifest {
    assets: Vec<AssetEntry>,
}

pub async fn start_fileserver() -> io::Result<()> {
    generate_manifest_and_checksums("./assets")?;

    // Spawn the fileserver thread
    tokio::spawn(async move {
        info!("Starting fileserver");

        let asset_dir = PathBuf::from("./assets");

        let app = Router::new()
            .nest_service("/assets", ServeDir::new(asset_dir))
            .nest_service("/clients", ServeDir::new("./binaries"))
            .route("/", get(|| async { "This is the file server." }));

        let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
        info!("Listening on {addr}");

        let listener = TcpListener::bind(addr).await.unwrap();
        serve(listener, app).await.unwrap();
    });

    Ok(())
}

fn generate_sha256(file_path: &Path) -> io::Result<String> {
    let bytes = fs::read(file_path)?;
    let hash = sha256::digest(&bytes);
    info!(
        "Generated hash for {} -> {hash}",
        file_path.to_string_lossy().to_string()
    );
    Ok(hash)
}

fn generate_manifest_and_checksums(asset_dir: &str) -> io::Result<()> {
    let base_path = Path::new(asset_dir);
    info!("Generating/Regenerating manifest and checksums (safety precausion)");

    let mut manifest = Manifest { assets: Vec::new() };

    info!("Processing all assets, this may take a few minutes...");

    for entry in WalkDir::new(asset_dir)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .filter(|entry| entry.file_name() != "manifest.json")
    {
        let path = entry.path();

        let checksum = generate_sha256(&path)?;

        manifest.assets.push(AssetEntry {
            path: path
                .strip_prefix(base_path)
                .unwrap()
                .to_string_lossy()
                .to_string(),
            checksum,
        });
    }

    info!("Writing new manifest");

    let manifest_path = Path::new(asset_dir).join("manifest.json");
    let manifest_file = File::create(manifest_path)?;
    serde_json::to_writer_pretty(manifest_file, &manifest)?;

    info!("Manifest generation complete.");

    Ok(())
}
