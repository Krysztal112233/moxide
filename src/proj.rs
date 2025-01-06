use std::{
    borrow::Borrow,
    fs,
    io::Write,
    path::{Path, PathBuf},
    string,
};

use clap::builder::Str;
use log::info;

use crate::{
    error::Result,
    manifest::Manifest,
    mkentry::{MarkdownEntry, MarkdownMeta},
};

pub(crate) struct MoxideProj {
    manifest: Manifest,
    base: PathBuf,
}

impl MoxideProj {
    pub(crate) fn try_new<P>(manifest: P) -> Result<MoxideProj>
    where
        P: AsRef<Path>,
    {
        let mut base = manifest.as_ref().to_path_buf();
        base.pop();

        let manifest: Manifest = fs::read_to_string(manifest)?.try_into()?;

        Ok(MoxideProj { manifest, base })
    }

    pub(crate) fn src(&self) -> PathBuf {
        let mut src = self.base.clone();
        src.push("src");
        src
    }

    pub(crate) fn output(&self) -> PathBuf {
        let mut output = self.base.clone();
        output.push("output");
        output
    }

    pub(crate) fn new_page<T>(&self, name: T) -> Result<PathBuf>
    where
        T: Into<String>,
    {
        let name: String = name.into();
        let encoded_name = urlencoding::encode(&name);

        info!("Encoded new page name `{name}` into `{encoded_name}`");

        let page_path = {
            let mut t = self.src();
            t.push(encoded_name.to_string());
            t
        };
        fs::create_dir_all(&page_path)?;
        info!("Created page in {}", page_path.to_str().unwrap());

        let index_md_path = {
            let mut index_path = page_path.clone();
            index_path.push("index.md");
            index_path
        };
        let mut index_md_file = fs::File::create_new(&index_md_path)?;
        let index_md_content = {
            MarkdownEntry::new(
                MarkdownMeta::default(),
                "Hello,World! This is the index markdown of your `page`/`bundle`/`...`!",
            )
            .into_document()
        }?;
        index_md_file.write_all(index_md_content.as_bytes())?;
        info!(
            "Created index markdown file at {}",
            index_md_path.to_str().unwrap()
        );

        Ok(page_path)
    }

    pub(crate) fn create_proj<T>(name: T) -> Result<MoxideProj>
    where
        T: Into<String>,
    {
        let name: String = name.into();
        let encoded_name = urlencoding::encode(&name);

        info!("Encoded new project name `{name}` into `{encoded_name}`");

        // Encoded to safe path
        let proj = PathBuf::from_iter([encoded_name.to_string()]);

        info!("Created new Moxide project at: {}", proj.to_str().unwrap());

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
        })?;
        manifest_file.write_all(manifest_content.as_bytes())?;

        Self::try_new(manifest_path)
    }
}
