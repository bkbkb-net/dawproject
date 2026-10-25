#[allow(
    dead_code,
    unused_imports,
    clippy::large_enum_variant,
    clippy::redundant_field_names,
    clippy::single_match
)]
#[rustfmt::skip]
mod generated {
    include!("../generated/metadata.rs");
}

pub use generated::MetaDataType;
/// Preferred name for [`MetaDataType`], mirroring `project::{Project, ProjectType}`.
pub use generated::MetaDataType as MetaData;
