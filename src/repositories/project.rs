#[allow(
    dead_code,
    unused_imports,
    clippy::large_enum_variant,
    clippy::redundant_field_names,
    clippy::single_match
)]
#[rustfmt::skip]
mod generated {
    include!("../generated/project.rs");
}

pub use generated::*;

// `clip`, `lanes`, `note` and `scene` share one content model, so the generator
// emits a single `ClipTypeContent` for all four. These are the names each
// owner's content had before 0.12.0.
/// Content of [`LanesType`]; the same type as [`ClipTypeContent`].
pub type LanesTypeContent = ClipTypeContent;
/// Content of [`NoteType`]; the same type as [`ClipTypeContent`].
pub type NoteTypeContent = ClipTypeContent;
/// Content of [`SceneType`]; the same type as [`ClipTypeContent`].
pub type SceneTypeContent = ClipTypeContent;
