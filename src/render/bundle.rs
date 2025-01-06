use std::path::PathBuf;

use async_trait::async_trait;

use crate::error::Result;

use super::Render;

#[derive(Debug)]
pub(super)  struct BundleRender;

#[async_trait]
impl Render for BundleRender {
    async fn render(&self, input: PathBuf, output: PathBuf) -> Result<()> {
        todo!()
    }
}
