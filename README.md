# wasmlet

A development tool and runtime for managing and running WASI or non-WASI WASM modules.

## Functions

- Build a WASM module with metadata describing the execution configurations
- Push a WASM module with annotated metadata to an OCI registry
- Pull a WASM module from an OCI registry
- Run a WASM module adaptively for WASI or non-WASI module based on Deno
- Run a WASM module with a specific runtime
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
