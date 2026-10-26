# Changelog

## [0.12.0](https://github.com/bkbkb-net/dawproject/compare/v0.11.0...v0.12.0) - 2026-10-26

### Security

- Bumped `zip` from 8.6.0 to 9.1.2, addressing `RUSTSEC-2026-0367`,
  `RUSTSEC-2026-0368`, and `RUSTSEC-2026-0369`. `DawprojectReader::extract`
  calls `ZipArchive::extract`, which in 8.x let a crafted archive create
  symbolic links pointing outside the destination directory, apply Unix
  permissions through such links, and keep SUID/SGID bits.

### Breaking Changes

- `zip` is part of the public API (`dawproject::zip`, `DawprojectWithZip::zip`,
  `DawprojectReader::{into_inner, by_name}`, `DawprojectWriter::raw_copy_file`,
  and the `ZipError` variants), so code that uses these needs `zip` 9 as well.
  Among other changes, `ZipFile::name` and `ZipArchive::file_names` now return
  `ZipResult<Cow<str>>`.
- Updated `xsd-parser` from `bf1323fc` to `96299a06`. `clip`, `lanes`, `note`,
  and `scene` share one content model, and the generator now emits a single
  `ClipTypeContent` for all four instead of an identical enum per owner.
  `LanesTypeContent`, `NoteTypeContent`, and `SceneTypeContent` remain as
  aliases of it, so every published path still resolves and converting
  between them is no longer needed. Its `Lanes`, `Notes`, and `Clips` variants
  now hold a `Box`: construct them with `Box::new(..)`; code that only reads
  through a binding usually compiles unchanged thanks to auto-deref.

### Fixed

- `ParameterError` is now re-exported from the crate root and the prelude. It is
  the error type of `IntegerParameterType::set_value`,
  `RealParameterType::{get_max, get_min, get_value, set_value}`, but it lived in
  the private `models` module, so callers could not name it in their own
  signatures. `#![warn(unnameable_types)]` now guards against a regression.

### Added

- The generated project types are re-exported at the crate root and in the
  prelude, so `dawproject::ClipType` works in addition to the existing
  `dawproject::project::ClipType`. Every previously published path is kept.
- `metadata::MetaDataType` is exported under its generated name as well as the
  `MetaData` alias, mirroring `project::{Project, ProjectType}`.
- `tests/public_api.rs` pins the public paths so they cannot be dropped
  accidentally.

### Changed

- Bumped `quick-xml` from 0.41.0 to 0.42.0. It stays internal to the crate.
  The serializer now writes `\r`, `\n`, and `\t` in attribute values as
  character references, so they survive a round trip, and the deserializer
  rejects documents nested more than 128 levels deep. The bundled
  `canon.dawproject` is written byte for byte as with 0.11.0.
- Bumped `thiserror` to 2.0.21.
- The code generator names a shared content type after its first owner
  (`ClipTypeContent`) instead of keeping xsd-parser's counter-based name
  (`ClipContent55Type`), which would change whenever the schema gains a type.
- Updated the code generator's lockfile, and the pinned development and CI
  toolchain from Rust 1.97.1 to 1.99.1. The minimum supported Rust version
  stays 1.88.

## [0.11.0](https://github.com/bkbkb-net/dawproject/compare/0.10.3...0.11.0) - 2026-08-09

### Breaking Changes

- Updated the generated DAWproject model to the current `xsd-parser` master
  API. Abstract XSD types such as `NameableType`, `ReferenceableType`,
  `ParameterType`, `LaneType`, `TimelineType`, `PluginType`, and `PointType`
  are now enums backed by generated `*Dyn` field structs. Existing code that
  directly constructs or pattern-matches these generated types may need to be
  updated.
- Updated the DAWproject schema from `5818a716` to upstream `main` commit
  `ee4dcdde`. Generated `SendType` now includes the optional `Enable` parameter,
  and `ClipType` includes the optional `enable` attribute. Adding these public
  struct fields may require updates to code that uses struct literals.
- Replaced the public `xsd-parser-types` Base64/hex string wrappers referenced
  by generated aliases with equivalent `dawproject::xml_types` wrappers.

### Changed

- Moved XSD-to-Rust generation out of the package build script. Generated
  models are now committed under `src/generated/`, so normal builds and
  crates.io packages do not require `xsd-parser`.
- Added `xsd-parser` as a code-generation-only submodule pinned to commit
  `bf1323fca2d7c95de4c79fbae65a3183ae0d158a`, plus a standalone generator
  under `tools/codegen`.
- Replaced the generated models' dependency on `xsd-parser-types` with local
  XML Schema string wrapper types.
- Updated the model helper traits to delegate through the new generated enum
  variants while preserving the existing getter APIs.
- Declared Rust 1.88 as the minimum supported Rust version and pinned the
  repository development toolchain to Rust 1.97.1 for reproducible generated
  formatting.
- Updated GitHub workflows to `actions/checkout@v7` and added a dedicated
  Rust 1.88 compatibility job.

### Security

- Removed the vulnerable `quick-xml 0.38.4` dependency pulled in by the
  released `xsd-parser`. The published crate now uses only `quick-xml 0.41.0`,
  addressing `RUSTSEC-2026-0194` and `RUSTSEC-2026-0195`.
- Removed `anyhow` from the published dependency graph, addressing the
  `RUSTSEC-2026-0190` audit warning. The code-generation-only lockfile uses
  the corrected `anyhow 1.0.104`.

### Added

- Added a CI job that regenerates the checked-in models and fails when they
  are out of date, and validates the standalone generator with Clippy.
- Added security auditing for the standalone code generator's lockfile.
- Added regression tests for the current DAWproject schema fields, generated
  enum trait delegation, and local XML Schema string wrappers.

## [0.10.3](https://github.com/hifa-lang/dawproject/compare/0.10.2...0.10.3)

### Changed

- Bumped `quick-xml` from 0.40.1 to 0.41.0. This is the runtime XML
  serialization dependency; 0.41.0 adds namespace-resolver configuration
  and attribute-parsing performance work with no serde-facing breaking
  changes, so the generated serde attributes remain valid.

## [0.10.2](https://github.com/hifa-lang/dawproject/compare/0.10.1...0.10.2)

### Fixed

- `build.rs` no longer writes into the crate source tree. Generated Rust code
  is emitted to `OUT_DIR`, and the `assets/Fixed*.xsd` snapshots are only
  refreshed from the upstream submodule in a dev checkout. This unblocks
  builds in read-only environments such as the `~/.cargo/registry` cache,
  sandboxes, and CI with frozen sources.
- Stale `src/generated/*.rs` files are removed from the package; the modules
  now `include!` from `OUT_DIR`.

### Changed

- `Cargo.toml` excludes the `dawproject/` submodule from the published
  tarball.
- Bumped `quick-xml` from 0.39.2 to 0.40.1.
- Bumped `actions/checkout` from v4 to v6 in CI / publish / security
  workflows.

## [0.10.0](https://github.com/hifa-lang/dawproject/compare/0.9.0...0.10.0)

### Breaking Changes

- Replaced `yaserde` (hifa_yaserde) with `quick-xml` + `serde` for XML serialization/deserialization.
- Replaced `hifa-xml-schema-derive` with `xsd-parser` for XSD-to-Rust code generation.
- Generated structs are now **flat** — inherited XSD fields are inlined directly (no more `.base.base` nesting).
- Field paths changed: e.g. `metadata.content.title` → `metadata.title`, `project.content.version` → `project.version`.
- Trait APIs simplified: `NameableTrait::get_name()` now returns directly without intermediate `get_nameable()`.

### Added

- Build-time XSD→Rust code generation via `xsd-parser` with `serde` + `quick-xml` backend.
- Generated code lives in `src/generated/` for IDE browsability.
- `skip_serializing_if` on `Option`/`Vec` fields for clean round-trip XML serialization.
- `Clone`, `PartialEq` derives on all generated types.

### Changed

- Updated `zip` dependency from 2.6.1 to 8.4.
- Updated `quick-xml` to 0.38 (aligned with xsd-parser).
- Updated `thiserror` to 2.0.18.

### Removed

- Removed `hifa_yaserde`, `hifa_yaserde_derive`, `hifa-xml-schema`, `hifa-xml-schema-derive`, `xml-rs` dependencies.

## [0.9.0](https://github.com/hifa-lang/dawproject/compare/0.8.2...0.9.0) - 2025-04-24

- Update dependencies.
