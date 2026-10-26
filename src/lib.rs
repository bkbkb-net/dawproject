#![doc = include_str!("../README.md")]
// Every type that appears in the public API must be nameable by downstream
// crates. See `tests/public_api.rs` for the paths that are guaranteed.
#![warn(unnameable_types)]

mod models;
pub mod prelude;
mod read;
mod repositories;
pub mod utils;
mod write;
pub mod xml_types;

pub use zip;

/// Error returned by the parameter accessors on the generated `*ParameterType`s.
pub use models::parameters::ParameterError;
/// This structure contains the metadata and project of DAWproject.
pub use models::{Dawproject, DawprojectWithZip};
pub use read::{DawprojectReadError, DawprojectReader};
/// Metadata of the `metadata.xml` file.
pub use repositories::metadata::MetaData;
/// Project of the `project.xml` file.
pub use repositories::project::ProjectType as Project;
/// The generated project types again at the crate root, e.g. `dawproject::ClipType`.
pub use repositories::project::*;
/// The `metadata` and `project` modules, e.g. `dawproject::project::ClipType`.
pub use repositories::*;
pub use write::{DawprojectWriteError, DawprojectWriter};
