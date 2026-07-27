use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

const SCHEMA: &str = "hepta_workgraph_codegen_v1";
const CODEGEN_DIRECTORY: &str = "codegen/workgraph-v1";

fn main() {
    println!("cargo:rerun-if-changed={CODEGEN_DIRECTORY}");
    let output_root = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR is required"))
        .join("hepta_workgraph_codegen");
    fs::create_dir_all(&output_root).expect("create WorkGraph codegen output directory");
    let mut manifests = fs::read_dir(CODEGEN_DIRECTORY)
        .expect("read WorkGraph codegen directory")
        .map(|entry| entry.expect("read WorkGraph codegen entry").path())
        .filter(|path| path.extension().is_some_and(|extension| extension == "tsv"))
        .filter(|path| path.file_name().is_some_and(|name| name != "registry.tsv"))
        .collect::<Vec<_>>();
    manifests.sort();
    for manifest in manifests {
        expand_manifest(&manifest, &output_root);
    }
}

fn expand_manifest(manifest_path: &Path, output_root: &Path) {
    let stem = manifest_path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .expect("WorkGraph manifest must have a UTF-8 stem");
    let template_path = manifest_path.with_file_name(format!("{stem}.tmpl"));
    let template = fs::read_to_string(&template_path).expect("read WorkGraph template");
    let manifest = fs::read_to_string(manifest_path).expect("read WorkGraph manifest");
    let mut lines = manifest.lines();
    assert_eq!(
        lines.next(),
        Some(SCHEMA),
        "invalid WorkGraph manifest schema"
    );
    for line in lines {
        if line.is_empty() {
            continue;
        }
        let fields = line.split('\t').collect::<Vec<_>>();
        assert!(fields.len() >= 3, "invalid WorkGraph manifest row");
        let module = fields[0];
        let placeholder_count = fields[2]
            .parse::<usize>()
            .expect("parse WorkGraph placeholder count");
        assert_eq!(
            fields.len(),
            placeholder_count + 3,
            "WorkGraph replacement count mismatch for {module}"
        );
        let mut expanded = template.clone();
        for (index, encoded) in fields[3..].iter().enumerate() {
            let placeholder = format!("@@HEPTA_WORKGRAPH_TOKEN_{index:04}@@");
            let replacement = decode_hex(encoded);
            expanded = expanded.replace(&placeholder, &replacement);
        }
        assert!(
            !expanded.contains("@@HEPTA_WORKGRAPH_TOKEN_"),
            "unexpanded WorkGraph placeholder for {module}"
        );
        let expanded = strip_leading_inner_attributes(&expanded);
        fs::write(output_root.join(format!("{module}.rs")), expanded)
            .expect("write generated WorkGraph module");
    }
}

fn strip_leading_inner_attributes(mut source: &str) -> &str {
    while source.starts_with("#![") {
        let Some(newline) = source.find('\n') else {
            return "";
        };
        source = &source[newline + 1..];
    }
    source
}

fn decode_hex(encoded: &str) -> String {
    assert!(encoded.len().is_multiple_of(2), "invalid WorkGraph hex");
    let bytes = encoded
        .as_bytes()
        .chunks_exact(2)
        .map(|pair| {
            let high = hex_nibble(pair[0]);
            let low = hex_nibble(pair[1]);
            (high << 4) | low
        })
        .collect::<Vec<_>>();
    String::from_utf8(bytes).expect("WorkGraph replacement must be UTF-8")
}

fn hex_nibble(value: u8) -> u8 {
    match value {
        b'0'..=b'9' => value - b'0',
        b'a'..=b'f' => value - b'a' + 10,
        _ => panic!("invalid WorkGraph hex digit"),
    }
}
