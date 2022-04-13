use std::path::PathBuf;

use async_trait::async_trait;
use url::Url;

use crate::common::file::{copy_wasm, download_wasm};
use crate::common::path::{wasm_dir, wasm_path};
use crate::wasm::provider::deno::DenoRuntime;
pub use crate::wasm::provider::wasmer::get_wasm_kind;
use crate::wasm::provider::wasmer::WasmerRuntime;
use crate::wasm::provider::WasmProviderKind;

pub mod provider;

#[derive(Debug)]
pub struct Wasm {
    pub name: String,
    pub url: Url,
    pub entry: Option<String>,
    pub args: Option<Vec<String>>,
    pub envs: Option<(String, String)>,
}

#[async_trait(? Send)]
pub trait WasmRuntimeTrait {
    async fn run(&self, wasm: Wasm) -> anyhow::Result<()> {
        match wasm.kind().unwrap() {
            WasmKind::Wasi => self.run_wasi(wasm).await,
            WasmKind::Web => self.run_web(wasm).await,
            WasmKind::Emscripten => self.run_emscripten(wasm).await
        }
    }

    async fn run_wasi(&self, _wasm: Wasm) -> anyhow::Result<()> {
        unimplemented!("not supported")
    }
    async fn run_web(&self, _wasm: Wasm) -> anyhow::Result<()> {
        unimplemented!("not supported")
    }
    async fn run_emscripten(&self, _wasm: Wasm) -> anyhow::Result<()> {
        unimplemented!("not supported")
    }
}

pub enum WasmKind {
    Wasi,
    Web,
    Emscripten,
}

pub fn create_wasm_runtime(
    wasm_kind: WasmKind,
    wasm_provider: Option<WasmProviderKind>,
) -> anyhow::Result<Box<dyn WasmRuntimeTrait>> {
    if wasm_provider.is_none() {
        let runtime: Box<dyn WasmRuntimeTrait> = match wasm_kind {
            WasmKind::Wasi => Box::new(WasmerRuntime::new()),
            WasmKind::Web => Box::new(DenoRuntime::new()),
            WasmKind::Emscripten => Box::new(WasmerRuntime::new()),
        };

        return Ok(runtime);
    }


    let runtime: Box<dyn WasmRuntimeTrait> = match wasm_provider {
        Some(WasmProviderKind::Deno) => Box::new(DenoRuntime::new()),
        Some(WasmProviderKind::Wasmer) => Box::new(WasmerRuntime::new()),
        _ => unreachable!()
    };

    Ok(runtime)
}

impl Wasm {
    pub fn new(url: Url) -> anyhow::Result<Self> {
        let file_path = url.to_file_path().unwrap();
        let name = file_path
            .file_name()
            .unwrap()
            .to_string_lossy();

        let wasm = Self {
            name: name.to_string(),
            url,
            entry: None,
            args: None,
            envs: None,
        };

        Ok(wasm)
    }

    pub fn kind(&self) -> anyhow::Result<WasmKind> {
        get_wasm_kind(self)
    }

    pub fn dir(&self) -> anyhow::Result<PathBuf> {
        wasm_dir(self)
    }

    pub fn path(&self) -> anyhow::Result<PathBuf> {
        wasm_path(self)
    }

    pub async fn download(&self) -> anyhow::Result<PathBuf> {
        //TODO add container image
        let path = match self.url.scheme() {
            "http" | "https" => download_wasm(&self.url, wasm_path(self)?).await?,
            "file" => copy_wasm(&self.url, wasm_path(self)?).await?,
            x => unimplemented!("{} not supported", x),
        };

        Ok(path)
    }
}
