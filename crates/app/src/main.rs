use crate::wasm::provider::WasmProviderKind;
use crate::wasm::{create_wasm_runtime, Wasm};

mod cmd;
mod common;
mod wasm;

#[tokio::main]
async fn main() {
    println!("Hello, wano!");

    let wasm = Wasm {
        name: "".to_string(),
        url: None,
        path: Default::default(),
    };
    let runtime = create_wasm_runtime(WasmProviderKind::Deno).expect("valid runtime");
    runtime.run_wasi(wasm).expect("run successfully");
}
