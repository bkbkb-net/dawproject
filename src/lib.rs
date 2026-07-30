#![doc = include_str!("../README.md")]

mod models;
pub mod prelude;
mod read;
pub mod repositories;
pub mod utils;
mod write;

pub use zip;

/// This structure contains the metadata and project of DAWproject.
pub use models::{Dawproject, DawprojectWithZip};
pub use read::{DawprojectReadError, DawprojectReader};
/// Metadata of the `metadata.xml` file.
pub use repositories::metadata::MetaData;
/// Project of the `project.xml` file.
pub use repositories::project::ProjectType as Project;

// Re-export generated types for direct access
pub use repositories::project::{
    ApplicationType,
    TrackType,
    ChannelType,
    ClipType,
    ClipsType,
    TransportType,
    ProjectStructureElementType,
    ProjectStructureElementTypeContent,
    ProjectScenesElementType,
};
pub use write::{DawprojectWriteError, DawprojectWriter};
