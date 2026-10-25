use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use xsd_parser::config::{GeneratorFlags, InterpreterFlags, OptimizerFlags, Schema};
use xsd_parser::{generate, Config};

const XSD_PARSER_REV: &str = "96299a06eb207b420c3ce03b6d07c41421f555b0";
const DAWPROJECT_REV: &str = "ee4dcdde75940f30e14e55401a26955a58b8322b";

fn main() {
    let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("codegen crate must be located at tools/codegen")
        .to_owned();

    verify_submodule_revision(
        &project_root,
        "tools/xsd-parser",
        XSD_PARSER_REV,
        "xsd-parser",
    );
    verify_submodule_revision(&project_root, "dawproject", DAWPROJECT_REV, "DAWproject");
    refresh_schema_snapshots(&project_root);
    generate_from_xsd(
        &project_root.join("assets/FixedMetaData.xsd"),
        &project_root.join("src/generated/metadata.rs"),
    );
    generate_from_xsd(
        &project_root.join("assets/FixedProject.xsd"),
        &project_root.join("src/generated/project.rs"),
    );
}

fn verify_submodule_revision(project_root: &Path, path: &str, expected: &str, name: &str) {
    let submodule_root = project_root.join(path);
    let output = Command::new("git")
        .arg("-C")
        .arg(&submodule_root)
        .args(["rev-parse", "HEAD"])
        .output()
        .unwrap_or_else(|error| panic!("failed to inspect {name} submodule: {error}"));

    assert!(
        output.status.success(),
        "failed to inspect {}; run `git submodule update --init --recursive` first",
        submodule_root.display()
    );

    let revision = String::from_utf8(output.stdout)
        .expect("git returned a non-UTF-8 revision")
        .trim()
        .to_owned();
    assert_eq!(
        revision, expected,
        "{name} submodule is at an unexpected revision; run `git submodule update --init --recursive`"
    );

    let status = Command::new("git")
        .arg("-C")
        .arg(&submodule_root)
        .args(["status", "--porcelain", "--untracked-files=no"])
        .output()
        .unwrap_or_else(|error| panic!("failed to inspect {name} submodule status: {error}"));
    assert!(
        status.status.success(),
        "failed to inspect tracked changes in {}",
        submodule_root.display()
    );
    assert!(
        status.stdout.is_empty(),
        "{name} submodule contains tracked changes; commit or discard them before generating"
    );
}

fn refresh_schema_snapshots(project_root: &Path) {
    let schema_root = project_root.join("dawproject");
    let metadata_path = schema_root.join("MetaData.xsd");
    let project_path = schema_root.join("Project.xsd");

    let metadata_xsd = fs::read_to_string(&metadata_path).unwrap_or_else(|error| {
        panic!(
            "failed to read {}: {error}\nrun `git submodule update --init --recursive` first",
            metadata_path.display()
        )
    });
    let project_xsd = fs::read_to_string(&project_path).unwrap_or_else(|error| {
        panic!(
            "failed to read {}: {error}\nrun `git submodule update --init --recursive` first",
            project_path.display()
        )
    });

    let fixed_project_xsd = replace_expected(
        &project_xsd,
        "<xs:attribute name=\"contentType\">",
        "<xs:attribute name=\"contentTypes\">",
        1,
    );
    let fixed_project_xsd =
        replace_expected(&fixed_project_xsd, "\"eqBandType\"", "\"eqBandKind\"", 2);
    let fixed_project_xsd = replace_expected(&fixed_project_xsd, "\"sendType\"", "\"sendKind\"", 2);

    write_if_changed(
        &project_root.join("assets/FixedMetaData.xsd"),
        &metadata_xsd,
    );
    write_if_changed(
        &project_root.join("assets/FixedProject.xsd"),
        &fixed_project_xsd,
    );
}

fn replace_expected(input: &str, from: &str, to: &str, expected_count: usize) -> String {
    let count = input.matches(from).count();
    assert_eq!(
        count, expected_count,
        "DAWproject schema patch `{from}` matched {count} time(s), expected {expected_count}; update the generator for the new schema"
    );
    input.replace(from, to)
}

fn generate_from_xsd(xsd_path: &Path, output_path: &Path) {
    let config = Config::default()
        .with_schema(Schema::File(xsd_path.into()))
        .with_interpreter_flags(InterpreterFlags::all())
        .with_optimizer_flags(OptimizerFlags::all())
        .with_generator_flags(GeneratorFlags::all())
        .with_serde_quick_xml();

    let code = generate(config).unwrap_or_else(|error| {
        panic!(
            "failed to generate code from {}: {error}",
            xsd_path.display()
        );
    });
    let code = post_process(&code.to_string());
    let header = format!(
        "// @generated from DAWproject {DAWPROJECT_REV} with xsd-parser {XSD_PARSER_REV}.\n\
         // Do not edit manually; run `cargo run --manifest-path tools/codegen/Cargo.toml --locked`.\n\n"
    );

    let content = format_generated(&(header + &code));
    write_if_changed(output_path, &content);
}

fn post_process(code: &str) -> String {
    assert!(
        code.contains("# [derive (Debug"),
        "xsd-parser's generated derive format changed; update the code generator post-processing"
    );

    let code = code
        .replace(
            "# [derive (Debug , Deserialize , Serialize)]",
            "#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]",
        )
        .replace(
            "# [derive (Debug , Default , Deserialize , Serialize)]",
            "#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]",
        )
        .replace(
            ":: xsd_parser_types :: xml :: Base64String",
            "crate :: xml_types :: Base64String",
        )
        .replace(
            ":: xsd_parser_types :: xml :: HexString",
            "crate :: xml_types :: HexString",
        );

    assert!(
        !code.contains("# [derive ("),
        "generated code contains an unhandled derive declaration"
    );
    assert!(
        !code.contains("xsd_parser_types"),
        "generated code still references xsd-parser-types; update the code generator post-processing"
    );

    add_skip_serializing_if(&rename_shared_content_types(&code))
}

/// Gives shared anonymous content types a stable name.
///
/// When several complex types have identical anonymous content (DAWproject's
/// `clip`, `lanes`, `note` and `scene` all allow the same choice of timeline
/// elements), xsd-parser generates one enum for all of them and names it after
/// the first owner plus a counter, e.g. `ClipContent55Type`. The counter follows
/// the schema's declaration order, so the public name would change whenever the
/// schema gains a type. Use the name xsd-parser gives the content type of a
/// single owner instead (`ClipTypeContent`).
fn rename_shared_content_types(code: &str) -> String {
    let mut renames = BTreeMap::new();
    for ident in idents(code) {
        if let Some(owner) = shared_content_owner(ident) {
            renames
                .entry(ident)
                .or_insert_with(|| format!("{owner}TypeContent"));
        }
    }

    let existing: BTreeSet<&str> = idents(code).collect();
    for (from, to) in &renames {
        assert!(
            !existing.contains(to.as_str()),
            "cannot rename shared content type `{from}`: `{to}` already exists"
        );
    }
    let targets: BTreeSet<&String> = renames.values().collect();
    assert_eq!(
        targets.len(),
        renames.len(),
        "several shared content types of one owner would get the same name: {renames:?}"
    );

    let mut result = String::with_capacity(code.len());
    let mut remaining = code;
    while let Some(start) = remaining.find(is_ident_char) {
        result.push_str(&remaining[..start]);
        let after_start = &remaining[start..];
        let end = after_start
            .find(|c: char| !is_ident_char(c))
            .unwrap_or(after_start.len());
        let ident = &after_start[..end];
        result.push_str(renames.get(ident).map_or(ident, String::as_str));
        remaining = &after_start[end..];
    }

    result.push_str(remaining);
    result
}

/// `ClipContent55Type` -> `Clip`.
fn shared_content_owner(ident: &str) -> Option<&str> {
    let numbered = ident.strip_suffix("Type")?;
    let unnumbered = numbered.trim_end_matches(|c: char| c.is_ascii_digit());
    if unnumbered.len() == numbered.len() {
        return None;
    }

    unnumbered
        .strip_suffix("Content")
        .filter(|owner| !owner.is_empty())
}

fn idents(code: &str) -> impl Iterator<Item = &str> {
    code.split(|c: char| !is_ident_char(c))
        .filter(|ident| !ident.is_empty())
}

fn is_ident_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

fn add_skip_serializing_if(code: &str) -> String {
    let target = "# [serde (default , ";
    let mut result = String::with_capacity(code.len() + 10_000);
    let mut remaining = code;

    while let Some(position) = remaining.find(target) {
        result.push_str(&remaining[..position]);
        let after_target = &remaining[position + target.len()..];
        let boundary = after_target
            .find("# [")
            .or_else(|| after_target.find("pub struct "))
            .or_else(|| after_target.find("pub enum "))
            .or_else(|| after_target.find("pub type "))
            .unwrap_or(after_target.len());
        let field = &after_target[..boundary];

        if field.contains(":: core :: option :: Option") {
            result.push_str("# [serde (default , skip_serializing_if = \"Option::is_none\" , ");
        } else if field.contains(":: std :: vec :: Vec") {
            result.push_str("# [serde (default , skip_serializing_if = \"Vec::is_empty\" , ");
        } else {
            result.push_str(target);
        }

        remaining = after_target;
    }

    result.push_str(remaining);
    result
}

fn write_if_changed(path: &Path, content: &str) {
    if fs::read_to_string(path).is_ok_and(|existing| existing == content) {
        return;
    }

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .unwrap_or_else(|error| panic!("failed to create {}: {error}", parent.display()));
    }
    fs::write(path, content)
        .unwrap_or_else(|error| panic!("failed to write {}: {error}", path.display()));
}

fn format_generated(content: &str) -> String {
    let mut child = Command::new("rustfmt")
        .args(["--emit", "stdout", "--edition", "2021"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap_or_else(|error| panic!("failed to start rustfmt: {error}"));

    child
        .stdin
        .take()
        .expect("rustfmt stdin was not piped")
        .write_all(content.as_bytes())
        .unwrap_or_else(|error| panic!("failed to send generated code to rustfmt: {error}"));

    let output = child
        .wait_with_output()
        .unwrap_or_else(|error| panic!("failed to wait for rustfmt: {error}"));
    assert!(
        output.status.success(),
        "rustfmt failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    String::from_utf8(output.stdout).expect("rustfmt returned non-UTF-8 output")
}
