#[allow(
    dead_code,
    unused_imports,
    clippy::large_enum_variant,
    clippy::redundant_field_names,
    clippy::single_match
)]
#[rustfmt::skip]
mod generated {
    include!(concat!(env!("OUT_DIR"), "/project_generated.rs"));
}

pub use generated::*;
