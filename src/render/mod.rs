use std::{
    future::Future,
    ops::Deref,
    pin::Pin,
    sync::{Arc, OnceLock},
};

use async_trait::async_trait;
use bundle::BundleRender;
use colored::Colorize;
use log::{info, trace};
use page::PageRender;
use parking_lot::RwLock;

use crate::{
    error::{Error, Result},
    mkentry::MarkdownEntryContext,
};

mod bundle;
mod page;

#[async_trait]
pub(crate) trait Render: Send + Sync {
    async fn render(&self, ctx: MarkdownEntryContext) -> Result<()>;
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

    /// Convert [`MarkdownEntryContext`] into prepared render
    ///
    /// ## Usage
    ///
    pub(crate) fn to_prepared_render(
        ctx: MarkdownEntryContext,
    ) -> Result<Pin<Box<dyn Future<Output = Result<()>> + Send>>> {
        let rendering = async move {
            let renderer = &ctx.entry.meta.renderer;
            trace!(
                "Detected render `{}` for entry `{}`.",
                ctx.entry.meta.renderer.italic().underline(),
                ctx.entry.meta.title.bold()
            );

            let render = Self::new().fetch(renderer);

            match render {
                Some(render) => render.render(ctx).await,
                None => Err(Error::RenderNotFound(renderer.to_owned())),
            }
        };

        Ok(Box::pin(rendering))
    }
}
