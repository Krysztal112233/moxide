use std::{
    ops::Deref,
    path::{Path, PathBuf},
    sync::{Arc, OnceLock},
};

use async_trait::async_trait;
use bundle::BundleRender;
use page::PageRender;
use parking_lot::RwLock;

use crate::error::Result;

mod bundle;
mod page;

#[async_trait]
pub(crate) trait Render {
    async fn render(&self, input: PathBuf, output: PathBuf) -> Result<()>;
}

#[derive(Default)]
pub(crate) struct RenderRegistry {
    map: im::HashMap<String, Arc<dyn Render>>,
}

unsafe impl Send for RenderRegistry {}
unsafe impl Sync for RenderRegistry {}

static mut REGISTRY: OnceLock<RwLock<RenderRegistry>> = OnceLock::new();

#[allow(unused)]
impl RenderRegistry {
    #[allow(static_mut_refs)]
    pub(crate) fn new() -> impl Deref<Target = RenderRegistry> {
        unsafe {
            REGISTRY
                .get_or_init(|| {
                    let mut registry = RenderRegistry::default();

                    // Default renderer
                    registry.map = registry
                        .map
                        .update("page".to_owned(), Arc::new(PageRender))
                        .update("bundle".to_owned(), Arc::new(BundleRender));

                    RwLock::new(registry)
                })
                .read()
        }
    }

    #[allow(static_mut_refs)]
    pub(crate) fn register(key: &str, render: Arc<dyn Render>) {
        Self::new();
        unsafe {
            REGISTRY
                .get_mut()
                .unwrap()
                .write()
                .map
                .insert(key.to_string(), render);
        }
    }

    pub(crate) fn fetch(&self, key: &str) -> Option<Arc<dyn Render>> {
        self.map.get(key).cloned()
    }

    /// Convert `index.md` into prepared render
    ///
    /// ## Usage
    ///
    pub(crate) fn to_prepared_render<P>(
        entry_path: P,
        output: P,
    ) -> Result<Box<dyn Fn() -> Result<()>>>
    where
        P: AsRef<Path>,
    {
        todo!()
    }
}
