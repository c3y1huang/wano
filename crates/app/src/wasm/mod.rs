use std::path::PathBuf;

use anyhow::anyhow;
use url::Url;

use crate::common::path::wasm_path;
use crate::wasm::provider::deno::DenoRuntime;
use crate::wasm::provider::wasmer::WasmerRuntime;
use crate::wasm::provider::WasmProviderKind;

pub(crate) mod provider;

#[derive(Debug)]
pub(crate) struct Wasm {
    pub(crate) name: String,
    pub(crate) url: Option<Url>,
    pub(crate) path: Option<PathBuf>,
}

pub(crate) trait WasmRuntimeTrait {
    fn run_wasi(&self, wasm: Wasm) -> anyhow::Result<()>;
    fn run_wasm_web(&self, wasm: Wasm) -> anyhow::Result<()>;
}

pub(crate) enum WasmKind {
    Wasi,
    WasmWeb,
}

pub(crate) fn create_wasm_runtime(
    wasm_provider: WasmProviderKind,
) -> anyhow::Result<Box<dyn WasmRuntimeTrait>> {
    let runtime: Box<dyn WasmRuntimeTrait> = match wasm_provider {
        WasmProviderKind::Deno => Box::new(DenoRuntime::new()),
        WasmProviderKind::Wasmer => Box::new(WasmerRuntime::new()),
    };

    Ok(runtime)
}

impl Wasm {
    fn new(url: Url) -> anyhow::Result<Self> {
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
        };

        wasm.download()?;
        Ok(wasm)
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
