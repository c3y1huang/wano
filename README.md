> This project is under active development 🚧!

# WANO

A WASM development tool and runtime based on different WASM providers.

## Functions

- build (x, this should be handled by other language specific tool instead)
  - build browser-wasm or wasi-wasm
- tag
  - add annotations/attributes to a wasm file
- validate
  - validate a wasm to see if it's qualified one supported by wano
- push
  - push a wasm to an OCI registry
- pull
  - pull a wasm from an OCI registry and validate if this is a qualified wasm file supported by wano
- list
  - list all wasm files at local
- run
  - run a local/remote wasm w/ capabilities allowed
- stop
  - stop a running wasm at local
- ps
  - list all running wasm processes at local

