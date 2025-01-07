use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};

use chrono::Utc;
use colored::Colorize;
use itertools::Itertools;
use log::trace;

use crate::{
    error::Result,
    manifest::Manifest,
    mkentry::{MarkdownEntry, MarkdownEntryContext, MarkdownMeta},
    render::RenderRegistry,
};

pub(crate) struct MoxideProj {
    manifest: Manifest,
    base: PathBuf,

    output: Option<PathBuf>,
}

impl MoxideProj {
    pub(crate) fn try_new<P>(manifest: P) -> Result<MoxideProj>
    where
        P: AsRef<Path>,
    {
        let mut base = manifest.as_ref().to_path_buf();
        base.pop();

        let manifest: Manifest = fs::read_to_string(manifest)?.try_into()?;

        Ok(MoxideProj {
            manifest,
            base,
            output: None,
        })
    }

    pub(crate) fn path_src(&self) -> PathBuf {
        let mut src = self.base.clone();
        src.push("src");
        src
    }

    pub(crate) fn set_output<P>(&mut self, output: P)
    where
        P: AsRef<Path>,
    {
        self.output = Some(output.as_ref().to_path_buf())
    }

    pub(crate) fn path_output(&self) -> PathBuf {
        self.output.clone().unwrap_or_else(|| {
            let mut output = self.base.clone();
            output.push("output");
            output
        })
    }

    pub(crate) fn create_page<T>(&self, name: T) -> Result<PathBuf>
    where
        T: Into<String>,
    {
        let name: String = name.into();
        let encoded_name = urlencoding::encode(&name);

        trace!(
            "Encoded new page name {} into {}",
            name.bold(),
            encoded_name.bold()
        );

        let page_path = {
            let mut t = self.path_src();
            t.push(encoded_name.to_string());
            t
        };
        fs::create_dir_all(&page_path)?;
        trace!(
            "Try creating page at {}",
            page_path.to_str().unwrap().bold().underline()
        );

        let index_md_path = {
            let mut index_path = page_path.clone();
            index_path.push("index.md");
            index_path
        };
        let mut index_md_file = fs::File::create_new(&index_md_path)?;
        let index_md_content = {
            MarkdownEntry::new(
                MarkdownMeta {
                    title: name,
                    date: Utc::now().into(),
                    ..MarkdownMeta::default()
                },
                "Hello,World! This is the index markdown of your `page`/`bundle`/`...`!",
            )
            .into_document()
        }?;
        index_md_file.write_all(index_md_content.as_bytes())?;
        trace!(
            "Created index markdown file at {}",
            index_md_path.to_str().unwrap().bold().underline()
        );

        Ok(page_path)
    }

    pub(crate) fn create_proj<T>(name: T) -> Result<MoxideProj>
    where
        T: Into<String>,
    {
        let name: String = name.into();
        let encoded_name = urlencoding::encode(&name);

        trace!(
            "Encoded new project name {} into {}",
            name.bold(),
            encoded_name.bold()
        );

        // Encoded to safe path
        let proj = PathBuf::from_iter([encoded_name.to_string()]);

        trace!(
            "Created new Moxide project at: {}",
            proj.to_str().unwrap().bold().underline()
        );

        fs::create_dir_all(&proj)?;

        let manifest_path = {
            let mut t = proj.clone();
            t.push("manifest.toml");
            t
        };
        let mut manifest_file = fs::File::create_new(&manifest_path)?;
        let manifest_content = toml::to_string_pretty(&Manifest {
            site: name.clone(),
            description: "Hello,World!".to_owned(),
            theme: "".to_owned(),
            renders: Vec::new(),
        })?;
        manifest_file.write_all(manifest_content.as_bytes())?;

        Self::try_new(manifest_path)
    }
}

impl MoxideProj {
    pub(crate) async fn build(&self) -> Result<()> {
        if fs::exists(self.path_output())? {
            trace!("Output directory exist, deleting it.");
            fs::remove_dir_all(self.path_output())?;
        }

        trace!("Created Moxide build output directory.");
        fs::create_dir_all(self.path_output())?;

        trace!(
            "Moxide project build output: {}",
            self.path_output().to_str().unwrap().bold().underline()
        );

        let output = walkdir::WalkDir::new(self.path_src())
            .max_depth(2)
            .into_iter()
            .flatten()
            .filter(|it| it.file_type().is_file())
            .filter(|it| it.file_name() == "index.md")
            .map(|it| it.into_path())
            .inspect(|it| {
                trace!(
                    "Walked index markdown: {}",
                    it.to_str().unwrap().bold().underline()
                )
            })
            .flat_map(|index| {
                MarkdownEntryContext::try_new(&index, &index).map(|it| {
                    let mut output = self.path_output();
                    let date = urlencoding::encode(&it.entry.meta.date.to_string()).to_string();
                    output.extend(["contents", &date]);
                    MarkdownEntryContext { output, ..it }
                })
            })
            .inspect(|it| {
                trace!(
                    "Parsed markdown entry for `{}` with date {}",
                    it.entry.meta.title.italic().bold(),
                    it.entry.meta.date.to_string().italic().bold()
                )
            })
            .flat_map(RenderRegistry::to_prepared_render)
            .collect_vec();

        let _ = futures::future::join_all(output).await;

        Ok(())
    }
}
