use std::env;
use std::fmt::Write as _;
use std::fs;
use std::io::Cursor;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;

use flate2::read::GzDecoder;
use sha2::Digest;
use sha2::Sha256;

const SCHEMA: &str = "hepta_workgraph_module_bundle_v2";
const CODEGEN_DIRECTORY: &str = "codegen/workgraph-v2";
const BUNDLE_FILE: &str = "modules.bundle.gz";
const CONTROL_PLANE_TESTS: &str = "src/work_graph_control_plane_tests.rs";
const EXPECTED_MODULE_COUNT: usize = 711;

fn main() {
    println!("cargo:rerun-if-changed={CODEGEN_DIRECTORY}");
    println!("cargo:rerun-if-changed={CONTROL_PLANE_TESTS}");
    let output_root = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR is required"))
        .join("hepta_workgraph_bundle");
    fs::create_dir_all(&output_root).expect("create WorkGraph bundle output directory");
    let bundle_path = Path::new(CODEGEN_DIRECTORY).join(BUNDLE_FILE);
    let compressed = fs::read(&bundle_path).expect("read WorkGraph module bundle");
    let mut decoder = GzDecoder::new(compressed.as_slice());
    let mut payload = Vec::new();
    decoder
        .read_to_end(&mut payload)
        .expect("decode WorkGraph module bundle");
    let modules = decode_bundle(&payload);
    assert_eq!(
        modules.len(),
        EXPECTED_MODULE_COUNT,
        "unexpected WorkGraph module count"
    );
    let mut declarations = String::new();
    for module in modules {
        let source_path = output_root.join(format!("{}.rs", module.name));
        fs::write(&source_path, module.source).expect("write generated WorkGraph module");
        let escaped_path = source_path
            .to_string_lossy()
            .replace('\\', "\\\\")
            .replace('"', "\\\"");
        writeln!(
            &mut declarations,
            "#[path = \"{escaped_path}\"]\nmod {};",
            module.name
        )
        .expect("write generated WorkGraph declaration");
    }
    fs::copy(
        CONTROL_PLANE_TESTS,
        output_root.join("work_graph_control_plane_tests.rs"),
    )
    .expect("copy WorkGraph control-plane tests");
    fs::write(output_root.join("modules.rs"), declarations)
        .expect("write generated WorkGraph module declarations");
}

struct BundledModule<'a> {
    name: &'a str,
    source: &'a [u8],
}

fn decode_bundle(payload: &[u8]) -> Vec<BundledModule<'_>> {
    let mut cursor = Cursor::new(payload);
    let mut magic = vec![0; SCHEMA.len() + 1];
    cursor
        .read_exact(&mut magic)
        .expect("read WorkGraph bundle magic");
    assert_eq!(magic, format!("{SCHEMA}\0").as_bytes());
    let module_count = read_u32(&mut cursor) as usize;
    let mut modules = Vec::with_capacity(module_count);
    for _ in 0..module_count {
        let name_length = read_u32(&mut cursor) as usize;
        let source_length = read_u32(&mut cursor) as usize;
        let name_start = cursor.position() as usize;
        let name_end = name_start
            .checked_add(name_length)
            .expect("WorkGraph name length overflow");
        let digest_end = name_end
            .checked_add(32)
            .expect("WorkGraph digest length overflow");
        let source_end = digest_end
            .checked_add(source_length)
            .expect("WorkGraph source length overflow");
        assert!(source_end <= payload.len(), "truncated WorkGraph bundle");
        let name = std::str::from_utf8(&payload[name_start..name_end])
            .expect("WorkGraph module name must be UTF-8");
        assert!(
            valid_module_name(name),
            "invalid WorkGraph module name: {name}"
        );
        let expected_digest = &payload[name_end..digest_end];
        let source = &payload[digest_end..source_end];
        assert_eq!(
            Sha256::digest(source).as_slice(),
            expected_digest,
            "WorkGraph module SHA mismatch: {name}"
        );
        cursor.set_position(source_end as u64);
        modules.push(BundledModule { name, source });
    }
    assert_eq!(
        cursor.position() as usize,
        payload.len(),
        "trailing WorkGraph bundle bytes"
    );
    modules
}

fn read_u32(cursor: &mut Cursor<&[u8]>) -> u32 {
    let mut bytes = [0; 4];
    cursor
        .read_exact(&mut bytes)
        .expect("read WorkGraph bundle integer");
    u32::from_be_bytes(bytes)
}

fn valid_module_name(name: &str) -> bool {
    (name.starts_with("wg_") || name.starts_with("work_graph_"))
        && name
            .bytes()
            .all(|byte| byte == b'_' || byte.is_ascii_lowercase() || byte.is_ascii_digit())
}
