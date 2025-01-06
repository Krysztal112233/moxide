use std::{
    fmt::Result,
    path::{Path, PathBuf},
    str::FromStr,
};

use itertools::Itertools;
use walkdir::WalkDir;

use crate::manifest::Manifest;

#[derive(Debug)]
pub(crate) struct MoxideBuilder {
    manifest: Manifest,

    output: PathBuf,
}

impl MoxideBuilder {
    pub(crate) fn new(manifest: Manifest) -> Self {
        Self {
            manifest,
            output: PathBuf::from_str("./output").unwrap(),
        }
    }

    pub(crate) fn output(&mut self, output: PathBuf) {
        self.output = output
    }

    // pub(crate) async fn build(&self) -> Result<()> {
    //     Ok(())
    // }

    pub(crate) fn to_prepared_render_tasks<P>(&self, input: P)
    where
        P: AsRef<Path>,
    {
        let collected = WalkDir::new(input)
            .max_depth(2)
            .into_iter()
            .flatten()
            .filter(|it| {
                it.file_type().is_file() && it.file_name().to_str().unwrap().eq("index.md")
            })
            .map(|it| it.into_path())
            .collect_vec();
    }
}
