use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
    sync::OnceLock,
};

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};

#[derive(Debug, Deserialize, Default, Serialize)]
pub(crate) struct MarkdownMeta {
    pub(crate) title: String,
    pub(crate) date: DateTime<Local>,

    #[serde(default = "HashSet::default")]
    #[serde(skip_serializing_if = "HashSet::is_empty")]
    pub(crate) tag: HashSet<String>,

    #[serde(default = "default_renderer")]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub(crate) renderer: String,
}

fn default_renderer() -> String {
    "page".to_owned()
}

pub(crate) struct MarkdownEntryContext {
    pub(crate) index: PathBuf,

    pub(crate) output: PathBuf,

    pub(crate) entry: MarkdownEntry,
}

impl MarkdownEntryContext {
    pub(crate) fn try_new<P>(index: P, output: P) -> Result<MarkdownEntryContext>
    where
        P: AsRef<Path>,
    {
        Ok(MarkdownEntryContext {
            index: index.as_ref().to_path_buf(),
            output: output.as_ref().to_path_buf(),
            entry: MarkdownEntry::try_from(index.as_ref().to_path_buf())?,
        })
    }
}

pub(crate) struct MarkdownEntry {
    pub(crate) meta: MarkdownMeta,

    pub(crate) description: String,

    pub(crate) content: String,
}

static REGEX: OnceLock<regex::Regex> = OnceLock::new();

impl MarkdownEntry {
    pub(crate) fn new<T>(meta: MarkdownMeta, description: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            meta,
            description: description.into(),
            content: "".into(),
        }
    }

    pub(crate) fn with_content(content: &str) -> Result<MarkdownEntry> {
        let document = content.trim();

        if !document.starts_with("+++") {
            Err(Error::InvalidDataBlock)
        } else {
            let meta = Self::extract_meta(content)?;

            let document = Self::remove_meta(content);

            let description = document
                .split_once("<!-- more -->")
                .map(|(a, _)| a.trim())
                .unwrap_or(&document)
                .into();

            Ok(MarkdownEntry {
                meta,
                content: document,
                description,
            })
        }
    }

    pub(crate) fn into_document(self) -> Result<String> {
        let meta = toml::to_string_pretty(&self.meta)?.trim().to_owned();
        let description = format!("{}\n<!-- more -->", self.description);

        let result = [
            format!(
                "+++
{meta}
+++"
            ),
            description,
            self.content,
        ]
        .join("\n\n");

        Ok(result)
    }

    fn remove_meta(content: &str) -> String {
        let re = REGEX.get_or_init(|| regex::Regex::new(r"(?s)\+\+\+(.*?)\+\+\+").unwrap());
        re.replace(content, "").to_string()
    }

    fn extract_meta(content: &str) -> Result<MarkdownMeta> {
        let re = REGEX.get_or_init(|| regex::Regex::new(r"(?s)\+\+\+(.*?)\+\+\+").unwrap());

        let meta = re
            .captures(content)
            .map(|caps| caps[1].trim().to_string())
            .ok_or(Error::InvalidDataBlock)?;

        Ok(toml::from_str(&meta)?)
    }
}

impl TryFrom<PathBuf> for MarkdownEntry {
    type Error = Error;

    fn try_from(value: PathBuf) -> std::result::Result<Self, Self::Error> {
        MarkdownEntry::try_from(&value)
    }
}

impl TryFrom<&PathBuf> for MarkdownEntry {
    type Error = Error;

    fn try_from(value: &PathBuf) -> std::result::Result<Self, Self::Error> {
        let content = fs::read_to_string(value)?;

        MarkdownEntry::with_content(&content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_new() {
        let binding = toml::to_string_pretty(&MarkdownMeta {
            title: "test".to_owned(),
            renderer: "page".to_owned(),
            date: Local::now(),
            ..MarkdownMeta::default()
        })
        .unwrap();
        let meta = binding.trim();

        let test_input = format!(
            "+++
{meta}
+++"
        );
        MarkdownEntry::with_content(&test_input).unwrap();

        let test_input = format!(
            "
{test_input}

test1

<!-- more -->
"
        );

        assert_eq!(
            MarkdownEntry::with_content(&test_input)
                .unwrap()
                .description,
            "test1"
        )
    }
}
