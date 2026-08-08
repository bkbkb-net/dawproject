# Changelog

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
