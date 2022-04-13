use std::fs::{copy, File};
use std::io::Write;
use std::path::PathBuf;

use url::Url;

pub async fn download_wasm(url: &Url, path: PathBuf) -> anyhow::Result<PathBuf> {
    let response = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?
        .get(url.to_string())
        .send().await?;

    let mut dest = File::create(path.file_name().unwrap())?;
    let content = response.bytes().await?;
    dest.write_all(&content.to_vec())?;

    Ok(path)
}

pub async fn copy_wasm(url: &Url, path: PathBuf) -> anyhow::Result<PathBuf> {
    copy(url.path(), &path)?;

    Ok(path)
}

pub async fn download_wasm_image(_url: &Url, _path: PathBuf) -> anyhow::Result<PathBuf> {
    todo!()
}