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
        entry: None,
        args: None,
        envs: None
    };

    let runtime = create_wasm_runtime(
        wasm.kind().expect("valid wasm kind"),
        None,
    ).expect("valid runtime");

    runtime.run_wasi(wasm).await.expect("run successfully");
}
