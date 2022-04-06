use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;

use async_trait::async_trait;
use deno_runtime::deno_core::FsModuleLoader;
use deno_runtime::permissions::Permissions;

use crate::wasm::{Wasm, WasmRuntimeTrait};

pub struct DenoRuntime;

impl DenoRuntime {
    pub(crate) fn new() -> impl WasmRuntimeTrait {
        Self
    }
}

#[async_trait(? Send)]
impl WasmRuntimeTrait for DenoRuntime {
    async fn run_wasi(&self, wasm: Wasm) -> anyhow::Result<()> {
        unimplemented!("Deno support WASI, but can't be reused because it is unable to resolve w/o using deno-cli which is not a shared lib")
    }

    async fn run_wasm_web(&self, wasm: Wasm) -> anyhow::Result<()> {
        let module_loader = Rc::new(FsModuleLoader);

        let create_web_worker_cb = Arc::new(|_| {
            unimplemented!("not support creating web workers")
        });
        let web_worker_preload_module_cb = Arc::new(|_| {
            unimplemented!("not support creating web workers")
        });

        let options = deno_runtime::worker::WorkerOptions {
            bootstrap: deno_runtime::BootstrapOptions {
                apply_source_maps: false,
                args: vec![],
                cpu_count: 1,
                debug_flag: false,
                enable_testing_features: false,
                location: None,
                no_color: false,
                is_tty: false,
                runtime_version: "wano".to_string(),
                ts_version: "wano".to_string(),
                unstable: false,
            },
            extensions: vec![],
            unsafely_ignore_certificate_errors: None,
            root_cert_store: None,
            user_agent: "wano".to_string(),
            seed: None,
            js_error_create_fn: None,
            web_worker_preload_module_cb,
            create_web_worker_cb,
            maybe_inspector_server: None,
            should_break_on_first_statement: false,
            module_loader,
            get_error_class_fn: None,
            origin_storage_dir: None,
            blob_store: deno_runtime::deno_web::BlobStore::default(),
            broadcast_channel: deno_runtime::deno_broadcast_channel::InMemoryBroadcastChannel::default(),
            shared_array_buffer_store: None,
            compiled_wasm_module_store: None,
        };

        let js_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/wasm/provider/deno_main.js");
        let main_module = deno_runtime::deno_core::resolve_path(&js_path.to_string_lossy())?;
        let permissions = Permissions::allow_all();

        let mut worker = deno_runtime::worker::MainWorker::bootstrap_from_options(
            main_module.clone(),
            permissions,
            options,
        );
        worker.execute_main_module(&main_module).await?;
        worker.run_event_loop(false).await
    }

    async fn run_wasm_emscripten(&self, wasm: Wasm) -> anyhow::Result<()> {
        unimplemented!("Deno doesn't support import objects of emscripten ABI")
    }
}
