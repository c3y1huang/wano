use std::path::PathBuf;

use anyhow::anyhow;
use async_trait::async_trait;
use url::Url;

use crate::common::path::wasm_path;
use crate::wasm::provider::deno::DenoRuntime;
pub(crate) use crate::wasm::provider::wasmer::get_wasm_kind;
use crate::wasm::provider::wasmer::WasmerRuntime;
use crate::wasm::provider::WasmProviderKind;

pub(crate) mod provider;

#[derive(Debug)]
pub(crate) struct Wasm {
    pub(crate) name: String,
    pub(crate) url: Option<Url>,
    pub(crate) path: Option<PathBuf>,
    pub(crate) entry: Option<String>,
    pub(crate) args: Option<Vec<String>>,
    pub(crate) envs: Option<(String, String)>,
}

#[async_trait(? Send)]
pub(crate) trait WasmRuntimeTrait {
    async fn run_wasi(&self, wasm: Wasm) -> anyhow::Result<()>;
    async fn run_wasm_web(&self, wasm: Wasm) -> anyhow::Result<()>;
    async fn run_wasm_emscripten(&self, wasm: Wasm) -> anyhow::Result<()>;
}

pub(crate) enum WasmKind {
    Wasi,
    Web,
    Emscripten,
}

pub(crate) fn create_wasm_runtime(
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
        let path = wasm_path(&name)?;

        let wasm = Self {
            name: name.to_string(),
            url: Some(url),
            path: Some(path),
            entry: None,
            args: None,
            envs: None
        };

        wasm.download()?;
        Ok(wasm)
    }

    pub fn kind(&self) -> anyhow::Result<WasmKind> {
        get_wasm_kind(self)
    }

    fn download(&self) -> anyhow::Result<PathBuf> {
        if self.url.is_none() {
            return Err(anyhow!(""));
        }

        //TODO extended downloader
        match self.url.as_ref().unwrap().scheme() {
            "http" | "https" => {}
            "file" => {}
            x => unimplemented!("{} not supported", x),
        }

        Ok(PathBuf::new())
    }
}
