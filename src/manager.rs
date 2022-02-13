use crate::data::Wasm;
use crate::result::EmptyResult;
use crate::result::Result;
use crate::registry::OciRegistry;

pub struct WasmManager{
    registry: Box<dyn OciRegistry>,
}

pub trait WasmManagerFunctions {
    fn push(wasm: Wasm) -> EmptyResult;
    fn pull(wasm: Wasm) -> Result<Wasm>;
    fn run(wasm: Wasm) -> EmptyResult;
}