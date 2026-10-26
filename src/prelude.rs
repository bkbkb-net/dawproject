pub use crate::models::bases::*;

/// Error returned by the parameter accessors on the generated `*ParameterType`s.
pub use crate::models::parameters::ParameterError;
/// This structure contains the metadata and project of DAWproject.
pub use crate::models::{Dawproject, DawprojectWithZip};
pub use crate::read::{DawprojectReadError, DawprojectReader};
/// Metadata of the `metadata.xml` file.
pub use crate::repositories::metadata::MetaData;
/// Project of the `project.xml` file.
pub use crate::repositories::project::ProjectType as Project;
/// The generated project types, e.g. `dawproject::prelude::ClipType`.
pub use crate::repositories::project::*;
pub use crate::repositories::*;
pub use crate::write::{DawprojectWriteError, DawprojectWriter};
