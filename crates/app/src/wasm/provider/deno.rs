use crate::wasm::{Wasm, WasmRuntimeTrait};

pub struct DenoRuntime;

impl DenoRuntime {
    pub(crate) fn new() -> impl WasmRuntimeTrait {
        Self
    }
}

impl WasmRuntimeTrait for DenoRuntime {
    fn run_wasi(&self, wasm: Wasm) -> anyhow::Result<()> {
        todo!()
    }

    fn run_wasm_web(&self, wasm: Wasm) -> anyhow::Result<()> {
        todo!()
    }
}
