use std::path::Path;

pub enum WasmKind {
    Wasm,
    Wasi,
}

pub struct Wasm {
    pub path: Path,
    pub kind: WasmKind,
}
