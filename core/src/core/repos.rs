use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

use crate::core::repo::TomlRepo;

/// this type is used to deserialize `.gitrepos` files.
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "kebab-case")]
pub struct TomlConfig {
    pub version: Option<String>,
    pub default_branch: Option<String>,
    pub default_remote: Option<String>,
    pub repos: Option<Vec<TomlRepo>>,
}

impl TomlConfig {
    /// deserialize config file (.gitrepos) with full file path
    pub fn load(path: impl AsRef<Path>) -> Option<Self> {
        if !path.as_ref().is_file() {
            return None;
        }

        // NOTE: mac not recognize "."
        let content = fs::read_to_string(path).unwrap().replace("\".\"", "\"\"");

        let Ok(mut toml_config) = toml::from_str::<TomlConfig>(&content) else {
            return None;
        };

        if let Some(item) = toml_config.repos.as_mut() {
            item.sort();
        }

        Some(toml_config)
    }

    // serialize config file .gitrepos
    pub fn serialize(&self) -> String {
        let toml = toml_edit::ser::to_item(self).unwrap();
        let mut out = String::new();

        out.push_str("# This file is automatically @generated by mgit.\n");
        out.push_str("# Editing it as you wish.\n");

        // version = "x.y.z"
        if let Some(item) = toml.get("version") {
            out.push_str(&format!("version = {}\n", item));
        }

        // default-branch = "your_branch"
        if let Some(item) = toml.get("default-branch") {
            out.push_str(&format!("default-branch = {}\n", item));
        }

        // default-remote = "your_remote"
        if let Some(item) = toml.get("default-remote") {
            out.push_str(&format!("default-remote = {}\n", item));
        }

        out.push('\n');

        // [[repos]]
        if let Some(repos) = toml.get("repos") {
            let list = repos.as_array().expect("repos must be an array");

            for entry in list {
                out.push_str("[[repos]]\n");
                let table = entry.as_inline_table().expect("repo must be table");

                // local = "your/local/path"
                if let Some(item) = table.get("local") {
                    out.push_str(&format!("local = {}\n", item));
                }

                // remote = "your://remote/url"
                if let Some(item) = table.get("remote") {
                    out.push_str(&format!("remote = {}\n", item));
                }

                // branch = "your_branch"
                if let Some(item) = table.get("branch") {
                    out.push_str(&format!("branch = {}\n", item));
                }

                // tag = "your_tag"
                if let Some(item) = table.get("tag") {
                    out.push_str(&format!("tag = {}\n", item));
                }

                // commit = "your_tag"
                if let Some(item) = table.get("commit") {
                    out.push_str(&format!("commit = {}\n", item));
                }

                // sparse = "[]"
                if let Some(item) = table.get("sparse") {
                    out.push_str(&format!("sparse = {}\n", item));
                }

                out.push('\n');
            }
        }

        out
    }
}
