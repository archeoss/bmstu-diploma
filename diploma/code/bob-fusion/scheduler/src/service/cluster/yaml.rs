use super::{BobClusterError, BobName, DiskName, DiskPath, VDiskId};
use crate::prelude::*;
use bob_fusion_core::types::Uri;
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
    str::FromStr,
};

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct PhysDisksYamlRepr {
    name: DiskName,
    path: DiskPath,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct BobNodeYamlRepr {
    name: BobName,
    address: Uri,
    disks: Vec<PhysDisksYamlRepr>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct ReplicaYamlRepr {
    node: BobName,
    disk: DiskName,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct VirtDiskYamlRepr {
    id: VDiskId,
    replicas: Vec<ReplicaYamlRepr>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct BobClusterYamlRepr {
    nodes: Vec<BobNodeYamlRepr>,
    vdisks: Vec<VirtDiskYamlRepr>,
}

impl BobClusterYamlRepr {
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

impl FromStr for BobClusterYamlRepr {
    type Err = Report<BobClusterError>;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        serde_yaml::from_str(s).change_context(BobClusterError::ReadConfigError)
    }
}

#[cfg(test)]
mod tests {}
