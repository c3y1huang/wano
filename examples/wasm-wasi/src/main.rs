fn main() {
    println!("Hello from main!");
}

// only able to support limited data type like int or float. If wanting to use richer types, need to use wasm-bindgen for WASM interface types
// ref: https://github.com/bytecodealliance/wasmtime/blob/main/docs/wasm-rust.md
#[no_mangle]
pub extern "C" fn greet() {
    println!("Hello!");
}