> Note: wasmlet is under active development

# wasmlet

A development tool and runtime for managing and running WASI or non-WASI WASM modules adaptively.

## Functions

- build
  - build browser-wasm or wasi-wasm
- tag
  - add annotations/attributes to an existing wasm file
- push
  - push a wasm to an OCI registry
- pull
  - pull a wasm from an OCI registry and validate if this is a qualified wasm file supported by wasmlet
- validate
  - validate a wasm to see if it's qualified one supported by wasmlet
- list
  - list all wasm files at local
- run
  - run a local/remote wasm locally w/ capabilities allowed
- stop
  - stop a running wasm at local
- ps
  - list all running wasm processes at local
- check
    - check the env is supported for wasmlet
