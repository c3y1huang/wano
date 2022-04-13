use anyhow::anyhow;
use async_trait::async_trait;
use maplit::hashmap;
use wasmer::{imports, Instance, Module, Store};
use wasmer_emscripten::{EmEnv, EmscriptenGlobals, generate_emscripten_env};
use wasmer_wasi::WasiState;

use crate::wasm::{Wasm, WasmKind, WasmRuntimeTrait};

pub struct WasmerRuntime;

impl WasmerRuntime {
    pub fn new() -> impl WasmRuntimeTrait {
        Self
    }
}

#[async_trait(? Send)]
impl WasmRuntimeTrait for WasmerRuntime {
    async fn run_wasi(&self, wasm: Wasm) -> anyhow::Result<()> {
        let store = Store::default();
        let module = Module::from_file(&store, &wasm.path().unwrap())?;

        let mut wasm_state_builder = WasiState::new(env!("CARGO_PKG_NAME"));
        let wasi_env_builder = wasm_state_builder.args(wasm.args.clone().unwrap_or_default());

        for (key, value) in &wasm.envs {
            wasi_env_builder.env(&key, &value);
        }

        let mut wasi_env = wasi_env_builder.finalize()?;

        let import_object = wasi_env.import_object(&module)?;
        let instance = Instance::new(&module, &import_object)?;

        let entry = wasm.entry.unwrap_or("_start".to_string());
        let main = instance.exports.get_function(&entry)?;
        let _ = main.call(&[])?;

        Ok(())
    }

    async fn run_web(&self, _wasm: Wasm) -> anyhow::Result<()> {
        unimplemented!("Wasmer doesn't support WASM backed by Web API ABI")
    }

    async fn run_emscripten(&self, wasm: Wasm) -> anyhow::Result<()> {
        let store = Store::default();
        let module = Module::from_file(&store, &wasm.path().unwrap())?;

        let mut em_globals = EmscriptenGlobals::new(&store, &module)
            .map_err(|err| anyhow!("{}", err))?;
        let em_env = EmEnv::new(&em_globals.data, hashmap! {});

        let import_object = generate_emscripten_env(&store, &mut em_globals, &em_env);
        let instance = Instance::new(&module, &import_object)?;

        let entry = wasm.entry.expect("valid entry");
        let main = instance.exports.get_function(&entry)?;
        let _ = main.call(&[])?;

        Ok(())
    }
}

pub fn get_wasm_kind(wasm: &Wasm) -> anyhow::Result<WasmKind> {
    let store = Store::default();
    let module = Module::from_file(&store, &wasm.path().unwrap())?;
    let import_object = imports! {};
    let instance = Instance::new(&module, &import_object)?;

    let kind = if instance.exports.contains("_start") {
        WasmKind::Wasi
    } else {
        WasmKind::Emscripten
    };

    Ok(kind)
}