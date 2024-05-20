mod yaml;
use crate::prelude::*;
use bob_fusion_core::types::Uri;
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
    str::FromStr,
};

#[derive(Error, Debug)]
pub enum BobClusterError {
    #[error("todo")]
    OpenConfigError,
    #[error("todo")]
    ReadConfigError,
}

#[derive(Debug, Clone, Serialize, Deserialize, Ord, Eq, PartialEq, PartialOrd)]
pub struct VDiskId(u64);
#[derive(Debug, Clone, Serialize, Deserialize, Ord, Eq, PartialEq, PartialOrd)]
pub struct BobName(String);
#[derive(Debug, Clone, Serialize, Deserialize, Ord, Eq, PartialEq, PartialOrd)]
pub struct DiskName(String);
#[derive(Debug, Clone, Serialize, Deserialize, Ord, Eq, PartialEq, PartialOrd)]
pub struct DiskPath(PathBuf);

pub type PhysDisks = BTreeMap<DiskName, DiskPath>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BobNode {
    name: BobName,
    address: Uri,
    disks: PhysDisks,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtDisks(BTreeMap<VDiskId, BTreeMap<BobName, DiskName>>);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BobCluster {
    nodes: BTreeMap<BobName, BobNode>,
    vdisks: VirtDisks,
}

impl BobCluster {
    /// .
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    pub fn from_config_path(path: &Path) -> Result<Self, BobClusterError> {
        let file = std::fs::File::open(path).change_context(BobClusterError::OpenConfigError)?;

        serde_yaml::from_reader(file).change_context(BobClusterError::ReadConfigError)
    }
}

impl FromStr for BobCluster {
    type Err = Report<BobClusterError>;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        serde_yaml::from_str(s).change_context(BobClusterError::ReadConfigError)
    }
}
