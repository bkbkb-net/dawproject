pub use crate::models::bases::*;

/// This structure contains the metadata and project of DAWproject.
pub use crate::models::{Dawproject, DawprojectWithZip};
pub use crate::read::{DawprojectReadError, DawprojectReader};
/// Metadata of the `metadata.xml` file.
pub use crate::repositories::metadata::MetaData;
/// Project of the `project.xml` file.
pub use crate::repositories::project::Project;

// Re-export all generated types for direct access
pub use crate::repositories::project::*;

pub use crate::write::{DawprojectWriteError, DawprojectWriter};
