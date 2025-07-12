use std::{collections::HashSet, path::Path, sync::Arc};

use async_trait::async_trait;
use uuid::Uuid;
use wasmtime::{
    Engine, Store,
    component::{Component, Linker},
};
use wasmtime_wasi::{
    ResourceTable,
    p2::{IoView, WasiCtxBuilder, WasiView},
};

use crate::events::AppEvent;

use super::{
    ExtensionManifest,
    extension::{Extension, ExtensionApi},
};

pub struct WasmExtension {
    manifest: ExtensionManifest,
    engine: Engine,
    component: Component,
    extension_api: Option<Arc<ExtensionApi>>,
}

struct ExtensionState {
    wasi: wasmtime_wasi::p2::WasiCtx,
    resource_table: ResourceTable,
    extension_api: Arc<ExtensionApi>,
}

impl IoView for ExtensionState {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.resource_table
    }
}

impl WasiView for ExtensionState {
    fn ctx(&mut self) -> &mut wasmtime_wasi::p2::WasiCtx {
        &mut self.wasi
    }
}

impl WasmExtension {
    pub fn load(manifest: ExtensionManifest, wasm_path: &Path) -> anyhow::Result<Self> {
        let engine = Engine::default();
        let component = Component::from_file(&engine, wasm_path)?;

        Ok(Self {
            manifest,
            engine,
            component,
            extension_api: None,
        })
    }

    fn get_extension_api(&self) -> anyhow::Result<&Arc<ExtensionApi>> {
        if let Some(api) = &self.extension_api {
            return Ok(api);
        }

        anyhow::bail!("Extension not initialized")
    }

    fn create_store(&self, api: Arc<ExtensionApi>) -> Store<ExtensionState> {
        let mut wasi_builder = WasiCtxBuilder::new();

        if self
            .manifest
            .permissions
            .contains(&super::Permission::FileSystem)
        {
            wasi_builder.inherit_stdio();
        }

        let wasi = wasi_builder.build();

        let state = ExtensionState {
            wasi,
            resource_table: ResourceTable::new(),
            extension_api: api,
        };

        Store::new(&self.engine, state)
    }

    fn create_linker(&self) -> anyhow::Result<Linker<ExtensionState>> {
        let mut linker = Linker::new(&self.engine);

        wasmtime_wasi::p2::add_to_linker_async(&mut linker)?;

        self.add_extension_api_to_linker(&mut linker)?;

        Ok(linker)
    }

    fn add_extension_api_to_linker(
        &self,
        linker: &mut Linker<ExtensionState>,
    ) -> anyhow::Result<()> {
        todo!()
    }
}

#[async_trait]
impl Extension for WasmExtension {
    fn publish_event(&self, event: AppEvent) -> anyhow::Result<()> {
        self.get_extension_api()?.event_bus.publish(event)?;

        Ok(())
    }

    fn get_items_ids(&self) -> anyhow::Result<&HashSet<Uuid>> {
        Ok(&self.get_extension_api()?.item_ids)
    }

    fn manifest(&self) -> &ExtensionManifest {
        &self.manifest
    }

    async fn initialize(&mut self) -> anyhow::Result<()> {
        self.extension_api = Some(Arc::new(ExtensionApi::default()));

        let mut store = self.create_store(self.get_extension_api()?.to_owned());
        let linker = self.create_linker()?;

        let instance = linker.instantiate(&mut store, &self.component)?;

        if let Ok(init_func) = instance.get_typed_func::<(), ()>(&mut store, "init") {
            init_func.call(&mut store, ())?;
        }

        Ok(())
    }
}
