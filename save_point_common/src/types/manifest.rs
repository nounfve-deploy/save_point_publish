#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct Manifest<F = ()> {
    pub dir_name: String,
    pub mod_time: DateTime<Utc>,

    #[serde(skip_serializing_if = "Description::is_empty")]
    #[serde(default)]
    pub desc: Description,

    /// sum hash for all file hashes
    #[serde(skip_serializing_if = "Blake3Hash::is_empty")]
    #[serde(default)]
    pub sum_blake3: Blake3Hash,

    #[serde(skip_serializing_if = "ExecInfo::is_empty")]
    #[serde(default)]
    pub exec: ExecInfo,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub ignore: Vec<String>,

    /// external sip
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub include: Vec<ExternalSip>,

    /// file list
    #[serde(skip_serializing)]
    #[serde(default)]
    pub files: F,
}

pub type Blake3Hash = ArrayString<{ 2 * blake3::OUT_LEN }>;
pub type FileListOrdered = BTreeMap<PathBuf, FileInfo>;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct FileInfo {
    pub path: PathBuf,
    #[serde(skip_serializing_if = "FileType::is_empty")]
    #[serde(default)]
    pub kind: FileType,
    #[serde(skip_serializing_if = "Blake3Hash::is_empty")]
    #[serde(default)]
    pub blake3: Blake3Hash,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum FileType {
    File,
    Dir,
    Extern(PathBuf),
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ExternalSip {
    pub url: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    #[serde(default)]
    pub mount: String,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ExecInfo {
    pub entry: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub args: Vec<String>,
}

impl ExecInfo {
    pub fn is_empty(&self) -> bool {
        self.entry.is_empty() && self.args.is_empty()
    }
}

impl Default for FileType {
    fn default() -> Self {
        FileType::File
    }
}

impl FileType {
    pub fn is_empty(&self) -> bool {
        if let FileType::File = self {
            true
        } else {
            false
        }
    }
}

impl Manifest {
    pub fn new<F: Default>(dir_name: String, files: F) -> Manifest<F> {
        Manifest {
            dir_name,
            mod_time: Utc::now(),
            files,
            ..Default::default()
        }
    }

    pub const MAIN: &str = ".sip.yaml";
    pub const FLIST: &str = ".sip.files.yaml";
    pub const BUILD: &str = ".sip.build.yaml";
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct SipBuild {
    #[serde(default)]
    pub desc_template: Description,
}

use std::{collections::BTreeMap, path::PathBuf};

use arrayvec::ArrayString;
use chrono::{DateTime, Utc};

use crate::types::description::Description;
