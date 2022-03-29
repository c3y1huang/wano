use std::fs::{create_dir_all, remove_dir_all};
use std::path::PathBuf;

const PROJECT_NAME: &'static str = env!("CARGO_BIN_NAME");

pub fn project_dir() -> anyhow::Result<PathBuf> {
    create_dir(dirs::home_dir().unwrap().join(PROJECT_NAME), false)
}

pub fn download_dir() -> anyhow::Result<PathBuf> {
    create_dir(project_dir()?.join("downloads"), false)
}

pub fn wasm_path(wasm_name: &str) -> anyhow::Result<PathBuf> {
    Ok(download_dir()?.join(wasm_name))
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
