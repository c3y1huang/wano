use crate::data::Wasm;

pub trait WasmRuntime {
    fn run();
}

pub struct WasmRuntimeFactory;

struct DefaultWasiRuntime;
struct DefaultWasmRuntime;

impl WasmRuntimeFactory {
    pub fn new(wasm: Wasm) -> impl WasmRuntime {
        todo!()
    }
}

impl WasmRuntime for DefaultWasiRuntime {
    fn run() {
        todo!()
    }
}

impl WasmRuntime for DefaultWasmRuntime {
    fn run() {
        todo!()
    }
}