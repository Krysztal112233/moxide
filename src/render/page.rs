use async_trait::async_trait;
use colored::Colorize;
use log::info;

use crate::error::Result;

use super::{MarkdownEntryContext, Render};

#[derive(Debug)]
pub(super) struct PageRender;

#[async_trait]
impl Render for PageRender {
    async fn render(&self, ctx: MarkdownEntryContext) -> Result<()> {
        info!(
            "Rendering content `{}` to `{}`",
            ctx.entry.meta.title.bold(),
            ctx.output.to_str().unwrap().bold().underline()
        );

        Ok(())
    }
}
