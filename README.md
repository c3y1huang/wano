# wasmlet

A development tool and runtime for managing and running (WASI) WASM modules.

## Functions

- Build (WASI) WASM module with metadata describing the execution configurations
- Push WASM module with annotated metadata to an OCI registry
- Pull WASM module from an OCI registry
- Run WASM module adaptively for WASI or non-WASI module based on Deno
- Run WASM module with a specific runtime
- List all managed WASM modules
- List all running WASM modules

## Commands

- wasmlet build
- wasmlet push
- wasmlet pull
- wasmlet list
- wasmlet run
- wasmlet stop
- waslmlet ps
