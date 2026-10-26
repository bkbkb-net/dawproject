//! Locks the paths downstream crates use to name public API types.
//!
//! Every path below has shipped in a release, so removing one is a breaking
//! change. Re-exports may be *added* freely; this test only guards removal.

// Generated project types, reachable through the module path...
use dawproject::project::{ClipType, ProjectType, SendType, TrackType};
// ...and through the crate root.
use dawproject::{ClipType as RootClipType, SendType as RootSendType};
// ...and through the prelude.
use dawproject::prelude::{ClipType as PreludeClipType, TrackType as PreludeTrackType};

// Curated crate-root API.
use dawproject::{Dawproject, DawprojectWithZip, MetaData, Project};
use dawproject::{DawprojectReadError, DawprojectReader};
use dawproject::{DawprojectWriteError, DawprojectWriter};

// Metadata, under both the alias and the generated name.
use dawproject::metadata::{MetaData as MetaDataAliased, MetaDataType};

// Model traits, exposed through the prelude only.
use dawproject::prelude::{NameableTrait, ParameterTrait, PointTrait, ReferenceableTrait};

// The content of `clip`, `lanes`, `note` and `scene` is one generated type
// since 0.12.0; each owner's earlier name still resolves to it.
use dawproject::project::{ClipTypeContent, LanesTypeContent, NoteTypeContent, SceneTypeContent};
use dawproject::{
    LanesTypeContent as RootLanesTypeContent, SceneTypeContent as RootSceneTypeContent,
};

// The error type returned by the parameter accessors. Before 0.12.0 this was
// reachable but unnameable, so callers could not write `Result<_, _>` for it.
use dawproject::prelude::ParameterError as PreludeParameterError;
use dawproject::ParameterError;

#[test]
fn public_paths_resolve() {
    fn assert_same_type<T>(_: Option<T>, _: Option<T>) {}

    assert_same_type(None::<ClipType>, None::<RootClipType>);
    assert_same_type(None::<ClipType>, None::<PreludeClipType>);
    assert_same_type(None::<SendType>, None::<RootSendType>);
    assert_same_type(None::<TrackType>, None::<PreludeTrackType>);
    assert_same_type(None::<ProjectType>, None::<Project>);
    assert_same_type(None::<MetaDataType>, None::<MetaData>);
    assert_same_type(None::<MetaDataAliased>, None::<MetaData>);
    assert_same_type(None::<ParameterError>, None::<PreludeParameterError>);
    assert_same_type(None::<ClipTypeContent>, None::<LanesTypeContent>);
    assert_same_type(None::<ClipTypeContent>, None::<NoteTypeContent>);
    assert_same_type(None::<ClipTypeContent>, None::<SceneTypeContent>);
    assert_same_type(None::<LanesTypeContent>, None::<RootLanesTypeContent>);
    assert_same_type(None::<SceneTypeContent>, None::<RootSceneTypeContent>);

    let _: Option<Dawproject> = None;
    let _: Option<DawprojectWithZip<std::io::Cursor<Vec<u8>>>> = None;
    let _: Option<DawprojectReadError> = None;
    let _: Option<DawprojectWriteError> = None;
    let _: Option<DawprojectReader<std::io::Cursor<Vec<u8>>>> = None;
    let _: Option<DawprojectWriter<std::io::Cursor<Vec<u8>>>> = None;
}

/// `ParameterError` must be nameable, otherwise callers cannot spell out the
/// return type of `RealParameterType::set_value` and friends.
#[test]
fn parameter_error_is_nameable() {
    fn call(p: &mut dawproject::RealParameterType) -> Result<bool, ParameterError> {
        p.set_value(0.5)
    }

    let mut p = dawproject::RealParameterType {
        name: None,
        color: None,
        comment: None,
        id: None,
        parameter_id: None,
        max: Some("1".to_string()),
        min: Some("0".to_string()),
        unit: dawproject::UnitType::Normalized,
        value: None,
    };
    assert!(call(&mut p).is_ok());

    p.max = Some("0.1".to_string());
    assert!(matches!(
        call(&mut p),
        Err(ParameterError::SetMaxError(_, _))
    ));
}

/// The traits stay object-safe-ish in the sense that they can be used as bounds.
#[test]
fn model_traits_are_usable_as_bounds() {
    fn _named<T: NameableTrait>(_: &T) {}
    fn _referenceable<T: ReferenceableTrait>(_: &T) {}
    fn _parameter<T: ParameterTrait>(_: &T) {}
    fn _point<T: PointTrait>(_: &T) {}
}
