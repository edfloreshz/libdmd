use std::{
    fs::{DirBuilder},
    path::{Path, PathBuf},
};

use anyhow::{Result};
use serde::{Deserialize, Serialize};
use core::default::Default;
use crate::utils::config::{
    file::*,
};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DirectoryBuilder {
    pub(crate) name: String,
    pub(crate) path: PathBuf,
    pub(crate) recursive: bool,
    pub(crate) dirs: Vec<DirectoryBuilder>,
    pub(crate) files: Vec<FileBuilder>
}

impl DirectoryBuilder {
    pub fn new() -> Self {
        DirectoryBuilder::default()
    }
    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }
    pub fn path(mut self, path: PathBuf) -> Self {
        self.path = path;
        self
    }
    pub fn parent(&mut self, path: PathBuf) -> Self {
        let mut path = path;
        path.push(Path::new(self.name.as_str()));
        self.path = path.to_path_buf();
        self.clone()
    }
    pub fn recursive(mut self, recursive: bool) -> Self {
        self.recursive = recursive;
        self
    }
    pub fn dir(mut self, dir: DirectoryBuilder) -> Self {
        self.dirs.push(dir);
        self
    }
    /// Add a file to DirectoryBuilder.
    pub fn file(&mut self, mut file: FileBuilder) -> Self {
        file.parent(&mut self.path.clone());
        self.files.push(file);
        self.clone()
    }
    pub fn build(&self) -> Result<()> {
        if !self.path.exists() {
            DirBuilder::new()
                .recursive(self.recursive)
                .create(&self.path)?;
        }
        for dir in &self.dirs {
            if !dir.path.exists() {
                dir.build()?;
            }
        }
        for file in &self.files {
            file.build()?;
        }
        Ok(())
    }
}