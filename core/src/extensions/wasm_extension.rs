use async_trait::async_trait;
use deno_core::{Extension, JsRuntime, RuntimeOptions, ModuleLoader, ModuleSource, ModuleSourceCode, ResolutionKind, ModuleSpecifier, ModuleLoadResponse};
use deno_error::JsErrorBox;
use futures::executor::block_on;
use std::rc::Rc;
use std::path::PathBuf;
use anyhow::Result;

use super::{Extension as CognitoExtension, ExtensionManifest};

// Define a simple op for our extension
#[deno_core::op2]
#[string]
fn op_hello_world() -> String {
    "Hello from WASM extension!".to_string()
}

// Create the extension
deno_core::extension!(
    cognito_wasm_extension,
    ops = [op_hello_world],
);

pub struct WasmExtension {
    manifest: ExtensionManifest,
    extension_path: PathBuf,
}

// Make WasmExtension Send + Sync
unsafe impl Send for WasmExtension {}
unsafe impl Sync for WasmExtension {}

impl WasmExtension {
    pub fn new(_id: uuid::Uuid, manifest: ExtensionManifest, extension_path: PathBuf) -> Self {
        Self {
            manifest,
            extension_path,
        }
    }

    fn create_deno_extension() -> Extension {
        // Create a deno extension with our custom ops
        cognito_wasm_extension::init()
    }
}

#[async_trait]
impl CognitoExtension for WasmExtension {
    fn manifest(&self) -> &ExtensionManifest {
        &self.manifest
    }

    async fn initialize(&mut self) -> Result<()> {
        // Run the initialization synchronously in a blocking task
        let extension_path = self.extension_path.clone();
        let entry_file = self.manifest.entry_file.clone();
        
        tokio::task::spawn_blocking(move || -> Result<()> {
            // Create the runtime synchronously
            let module_loader = Rc::new(ExtensionModuleLoader {
                extension_path: extension_path.clone(),
            });

            // Create the deno extension
            let deno_ext = WasmExtension::create_deno_extension();

            // Create runtime options
            let options = RuntimeOptions {
                module_loader: Some(module_loader),
                extensions: vec![deno_ext],
                ..Default::default()
            };

            // Create the JS runtime
            let mut runtime = JsRuntime::new(options);
            
            // Try to load and execute the entry file
            let entry_path = extension_path.join(&entry_file);
            if entry_path.exists() {
                let module_specifier = ModuleSpecifier::from_file_path(&entry_path)
                    .map_err(|e| anyhow::anyhow!("Failed to create module specifier: {:?}", e))?;
                
                // Load and execute the module
                let mod_id = block_on(runtime.load_main_es_module(&module_specifier))
                    .map_err(|e| anyhow::anyhow!("Failed to load module: {:?}", e))?;
                
                let result = runtime.mod_evaluate(mod_id);
                block_on(runtime.run_event_loop(Default::default()))
                    .map_err(|e| anyhow::anyhow!("Failed to run event loop: {:?}", e))?;
                
                // We need to handle the result properly
                // In a synchronous context, we can block on the future
                block_on(result)
                    .map_err(|e| anyhow::anyhow!("Failed to evaluate module: {:?}", e))?;
            }
            
            Ok(())
        }).await??;

        Ok(())
    }
}

// Custom module loader for extensions
struct ExtensionModuleLoader {
    extension_path: PathBuf,
}

impl ModuleLoader for ExtensionModuleLoader {
    fn resolve(
        &self,
        specifier: &str,
        referrer: &str,
        _kind: ResolutionKind,
    ) -> Result<ModuleSpecifier, JsErrorBox> {
        // For now, we'll use a simple resolution strategy
        // In the future, we might want to implement more sophisticated resolution
        if specifier.starts_with("./") || specifier.starts_with("../") {
            let referrer_url = url::Url::parse(referrer)
                .map_err(|e| JsErrorBox::generic(format!("Invalid referrer URL: {}", e)))?;
            let resolved = referrer_url.join(specifier)
                .map_err(|e| JsErrorBox::generic(format!("Failed to resolve specifier: {}", e)))?;
            Ok(ModuleSpecifier::from(resolved))
        } else {
            // Try to resolve as a file in the extension directory
            let base_path = &self.extension_path;
            let resolved_path = base_path.join(specifier);
            ModuleSpecifier::from_file_path(resolved_path)
                .map_err(|_| JsErrorBox::generic("Failed to resolve module"))
        }
    }

    fn load(
        &self,
        module_specifier: &ModuleSpecifier,
        _maybe_referrer: Option<&ModuleSpecifier>,
        _is_dyn_import: bool,
        _requested_module_type: deno_core::RequestedModuleType,
    ) -> ModuleLoadResponse {
        let module_specifier = module_specifier.clone();
        let extension_path = self.extension_path.clone();

        ModuleLoadResponse::Async(Box::pin(async move {
            // Convert the module specifier back to a file path
            let path = module_specifier
                .to_file_path()
                .map_err(|_| JsErrorBox::generic("Invalid file path"))?;

            // Check if the file exists and is within the extension directory
            if !path.starts_with(&extension_path) {
                return Err(JsErrorBox::generic("Module loading restricted to extension directory"));
            }

            // Read the file content
            let code = tokio::fs::read_to_string(&path).await
                .map_err(|e| JsErrorBox::generic(format!("Failed to read file: {}", e)))?;
            
            let module_type = if path.extension().and_then(|ext| ext.to_str()) == Some("json") {
                deno_core::ModuleType::Json
            } else {
                deno_core::ModuleType::JavaScript
            };

            Ok(ModuleSource::new(
                module_type,
                ModuleSourceCode::String(code.into()),
                &module_specifier,
                None,
            ))
        }))
    }
}