use std::str::FromStr;

use url::Url;

use crate::wasm::{create_wasm_runtime, Wasm};

mod cmd;
mod common;
mod wasm;

#[tokio::main]
async fn main() {
    println!("Hello, wano!");

    let wasm = Wasm {
        name: "".to_string(),
        url: Url::from_str("").unwrap(),
        entry: None,
        args: None,
        envs: None,
    };

    let runtime = create_wasm_runtime(
        wasm.kind().unwrap(),
        None,
    ).unwrap();

    runtime.run(wasm).await;
}
