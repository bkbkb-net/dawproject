# Changelog

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
