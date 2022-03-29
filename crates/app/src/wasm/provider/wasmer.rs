use wasmer::{imports, Instance, Module, Store, Value};

use crate::wasm::{Wasm, WasmRuntimeTrait};

pub struct WasmerRuntime;

impl WasmerRuntime {
    pub(crate) fn new() -> impl WasmRuntimeTrait {
        Self
    }
}

impl WasmRuntimeTrait for WasmerRuntime {
    fn run_wasi(&self, wasm: Wasm) -> anyhow::Result<()> {
        let store = Store::default();
        let module = Module::from_file(&store, &wasm.path.unwrap())?;
        let import_object = imports! {};
        let instance = Instance::new(&module, &import_object)?;

        let add_one = instance.exports.get_function("add_one")?;
        let result = add_one.call(&[Value::I32(42)])?;

        Ok(())
    }

    fn run_wasm_web(&self, wasm: Wasm) -> anyhow::Result<()> {
        todo!()
    }
}
