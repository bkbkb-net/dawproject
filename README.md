# dawproject

[![Crate](https://img.shields.io/crates/v/dawproject.svg)](https://crates.io/crates/dawproject)
[![API](https://docs.rs/dawproject/badge.svg)](https://docs.rs/dawproject)

Dawproject is a format for exchanging user data between digital audio workstations (DAWs).
This project is port of [DAWproject](https://github.com/bitwig/dawproject) in Rust.

## Usage

You can easily read and write `.dawproject` files.

```rust
use dawproject::prelude::*;

// read dawproject file
let mut reader = DawprojectReader::open("assets/tests/canon.dawproject").unwrap();
reader.read_dawproject().unwrap();
// use it wherever you want
let dawproject = reader.build_dawproject().unwrap();
// Write
let mut writer = DawprojectWriter::create("assets/tests/copied_canon.dawproject").unwrap();
writer.write_dawproject(&dawproject).unwrap();
```

## Documentations

- [DAWproject](https://github.com/bitwig/dawproject)
  - Bitwig DAWproject repository
- [DAWPROJECT XML Reference](https://htmlpreview.github.io/?https://github.com/bitwig/dawproject/blob/main/Reference.html)
  - Bitwig DawProject XML Reference
- [Project Document](https://docs.rs/dawproject)
  - This project document

## Build from source

The published crate supports Rust 1.88 and newer. This repository pins Rust
1.97.1 so code generation and `rustfmt` output remain reproducible.

```bash
cargo build
```

Normal builds use the generated Rust models committed under `src/generated/`.
They do not run `xsd-parser` and do not require either submodule.

The `xsd-parser` submodule is pinned to commit
`bf1323fca2d7c95de4c79fbae65a3183ae0d158a` and is only needed when
refreshing the generated models. The source DAWproject schema is pinned to
upstream `main` commit `ee4dcdde75940f30e14e55401a26955a58b8322b`:

```bash
git submodule update --init --recursive
cargo run --manifest-path tools/codegen/Cargo.toml --locked
```

Commit changes under `assets/` and `src/generated/` together with any
intentional schema or generator update.
