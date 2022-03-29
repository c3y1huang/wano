pub(crate) mod deno;
pub(crate) mod wasmer;

pub(crate) enum WasmProviderKind {
    Deno,
    Wasmer,
}

pub(crate) struct WasmProviderCapability {
    web: bool,
    wasi: bool,
}

impl WasmProviderKind {
    pub fn get_capabilities(&self) -> WasmProviderCapability {
        match self {
            WasmProviderKind::Deno => WasmProviderCapability {
                web: true,
                wasi: true,
            },

            WasmProviderKind::Wasmer => WasmProviderCapability {
                web: false,
                wasi: true,
            },
        }
    }
}
