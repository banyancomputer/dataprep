use crate::types::{
    shared::{CompressionScheme, EncryptionScheme, PartitionScheme},
    spider::SpiderMetadata,
};
use std::{path::PathBuf, sync::Arc};

/// this struct is used to build up the data processing steps for a file
#[derive(Debug, Clone)]
pub struct PackPlan {
    /// Describes how we will compress the file (contains compression algorithm info)
    pub compression: CompressionScheme,
    /// Describes how we will partition the file (contains partition algorithm info)
    pub partition: PartitionScheme,
    /// Describes how we will encrypt the file (contains keys)
    pub encryption: EncryptionScheme,
    /// describes what directory we will write packed files to
    pub writeout: PathBuf,
}

/// This struct is used to describe how a filesystem structure was processed. Either it was a duplicate/symlink/
/// directory and there isn't much to do, or else we need to go through compression, partition, and
/// encryption steps.
/// this takes in pre-grouped files (for processing together) or marked directories/simlinks.
#[derive(Debug, Clone)]
pub enum PackPipelinePlan {
    /// It was a directory, just create it
    Directory(Arc<SpiderMetadata>),
    /// it was a symlink, just create it (with destination)
    Symlink(Arc<SpiderMetadata>, PathBuf),
    /// it was a group of identical files, here's the metadata for how they were encrypted and compressed
    FileGroup(Vec<Arc<SpiderMetadata>>, PackPlan),
}
