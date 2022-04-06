use std::collections::HashMap;
use lazy_static::lazy_static;
use maplit::hashmap;

pub(crate) mod deno;
pub(crate) mod wasmer;

#[derive(PartialEq, Eq, Hash)]
pub(crate) enum WasmProviderKind {
    Deno,
    Wasmer,
}

pub(crate) struct WasmProviderCapability {
    web: bool,
    wasi: bool,
    emscripten: bool,
}

lazy_static! {
    pub(crate) static ref WASM_PROVIDER_CAPS: HashMap<WasmProviderKind, WasmProviderCapability> = hashmap! {
        WasmProviderKind::Deno => WasmProviderCapability {
            web: true,
            wasi: false,
            emscripten: false,
        },
        WasmProviderKind::Wasmer => WasmProviderCapability {
            web: false,
            wasi: true,
            emscripten: true,
        },
    };
}