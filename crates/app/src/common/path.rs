use std::fs::{create_dir_all, remove_dir_all};
use std::path::PathBuf;
use crate::Wasm;

const PROJECT_NAME: &str = env!("CARGO_BIN_NAME");

pub fn project_dir() -> anyhow::Result<PathBuf> {
    create_dir(dirs::home_dir().unwrap().join(PROJECT_NAME), false)
}

pub fn wasm_dir(wasm: &Wasm) -> anyhow::Result<PathBuf> {
    create_dir(download_dir()?.join(&wasm.name), false)
}

pub fn wasm_path(wasm: &Wasm) -> anyhow::Result<PathBuf> {
    Ok(wasm_dir(wasm)?.join(&wasm.name))
}

pub fn download_dir() -> anyhow::Result<PathBuf> {
    create_dir(project_dir()?.join("downloads"), false)
}

fn create_dir(path: PathBuf, force: bool) -> anyhow::Result<PathBuf> {
    if path.exists() {
        if !force {
            return Ok(path);
        }
        remove_dir_all(&path)?;
    }

    create_dir_all(&path)?;
    Ok(path)
}
