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

const SCHEMA: &str = "hepta_workgraph_normalized_bundle_v3";
const CODEGEN_DIRECTORY: &str = "codegen/workgraph-v3";
const BUNDLE_FILE: &str = "modules.bundle.gz";
const FAMILY_ALIAS_PREFIX: &str = "/*workgraph-family-alias:";
const CONTROL_PLANE_TESTS: &str = "src/work_graph_control_plane_tests.rs";
const EXPECTED_MODULE_COUNT: usize = 99;
const MAX_COMPRESSED_BUNDLE_BYTES: usize = 8 * 1024 * 1024;
const MAX_DECODED_BUNDLE_BYTES: u64 = 64 * 1024 * 1024;

fn main() {
    println!("cargo:rerun-if-changed={CODEGEN_DIRECTORY}");
    println!("cargo:rerun-if-changed={CONTROL_PLANE_TESTS}");
    let output_root = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR is required"))
        .join("hepta_workgraph_bundle");
    fs::create_dir_all(&output_root).expect("create WorkGraph bundle output directory");
    let bundle_path = Path::new(CODEGEN_DIRECTORY).join(BUNDLE_FILE);
    let compressed = fs::read(&bundle_path).expect("read WorkGraph module bundle");
    assert!(
        compressed.len() <= MAX_COMPRESSED_BUNDLE_BYTES,
        "compressed WorkGraph bundle exceeds byte limit"
    );
    let decoder = GzDecoder::new(compressed.as_slice());
    let mut payload = Vec::new();
    decoder
        .take(MAX_DECODED_BUNDLE_BYTES + 1)
        .read_to_end(&mut payload)
        .expect("decode WorkGraph module bundle");
    assert!(
        payload.len() as u64 <= MAX_DECODED_BUNDLE_BYTES,
        "decoded WorkGraph bundle exceeds byte limit"
    );
    let modules = decode_bundle(&payload);
    assert_eq!(
        modules.len(),
        EXPECTED_MODULE_COUNT,
        "unexpected WorkGraph module count"
    );
    let mut declarations = String::new();
    for module in modules {
        let family_aliases = decode_family_aliases(&module.source);
        let source_path = output_root.join(format!("{}.rs", module.name));
        fs::write(&source_path, &module.source).expect("write generated WorkGraph module");
        let escaped_path = source_path
            .to_string_lossy()
            .replace('\\', "\\\\")
            .replace('"', "\\\"");
        if matches!(
            module.name.as_str(),
            "work_graph_append_only_store_runtime_enablement_preview"
                | "work_graph_replay_readback_preview"
        ) {
            declarations.push_str("#[allow(clippy::manual_contains)]\n");
        }
        writeln!(
            &mut declarations,
            "#[path = \"{escaped_path}\"]\nmod {};",
            module.name
        )
        .expect("write generated WorkGraph declaration");
        for (legacy_module, variant_module) in family_aliases {
            writeln!(
                &mut declarations,
                "#[allow(unused_imports)]\npub(crate) use {}::{} as {};",
                module.name, variant_module, legacy_module
            )
            .expect("write generated WorkGraph family alias");
        }
    }
    fs::copy(
        CONTROL_PLANE_TESTS,
        output_root.join("work_graph_control_plane_tests.rs"),
    )
    .expect("copy WorkGraph control-plane tests");
    fs::write(output_root.join("modules.rs"), declarations)
        .expect("write generated WorkGraph module declarations");
}

struct BundledModule {
    name: String,
    source: Vec<u8>,
}

enum TemplateSegment {
    Fixed(Vec<u8>),
    Slot,
}

struct TemplateFamily {
    segments: Vec<TemplateSegment>,
    slot_count: usize,
}

fn decode_bundle(payload: &[u8]) -> Vec<BundledModule> {
    let mut cursor = Cursor::new(payload);
    let mut magic = vec![0; SCHEMA.len() + 1];
    cursor
        .read_exact(&mut magic)
        .expect("read WorkGraph bundle magic");
    assert_eq!(magic, format!("{SCHEMA}\0").as_bytes());
    let family_count = read_u32(&mut cursor) as usize;
    assert!(
        family_count > 0 && family_count <= EXPECTED_MODULE_COUNT,
        "unexpected WorkGraph template family count"
    );
    let families = (0..family_count)
        .map(|_| decode_family(&mut cursor))
        .collect::<Vec<_>>();
    let module_count = read_u32(&mut cursor) as usize;
    assert_eq!(
        module_count, EXPECTED_MODULE_COUNT,
        "unexpected WorkGraph module count"
    );
    let mut modules = Vec::with_capacity(module_count);
    for _ in 0..module_count {
        let name_length = read_u32(&mut cursor) as usize;
        let family_index = read_u32(&mut cursor) as usize;
        let replacement_count = read_u32(&mut cursor) as usize;
        let source_length = read_u32(&mut cursor) as usize;
        let name_bytes = read_bytes(&mut cursor, name_length);
        let name = std::str::from_utf8(&name_bytes).expect("WorkGraph module name must be UTF-8");
        assert!(
            valid_module_name(name),
            "invalid WorkGraph module name: {name}"
        );
        let expected_digest = read_bytes(&mut cursor, 32);
        let family = families
            .get(family_index)
            .expect("invalid WorkGraph template family index");
        assert_eq!(
            replacement_count, family.slot_count,
            "WorkGraph replacement count drifted: {name}"
        );
        let replacements = (0..replacement_count)
            .map(|_| {
                let length = read_u32(&mut cursor) as usize;
                read_bytes(&mut cursor, length)
            })
            .collect::<Vec<_>>();
        let source = expand_family(family, &replacements);
        assert_eq!(
            source.len(),
            source_length,
            "WorkGraph source size drifted: {name}"
        );
        assert_eq!(
            Sha256::digest(&source).as_slice(),
            expected_digest,
            "WorkGraph module SHA mismatch: {name}"
        );
        modules.push(BundledModule {
            name: name.to_owned(),
            source,
        });
    }
    assert_eq!(
        cursor.position() as usize,
        payload.len(),
        "trailing WorkGraph bundle bytes"
    );
    modules
}

fn decode_family(cursor: &mut Cursor<&[u8]>) -> TemplateFamily {
    let segment_count = read_u32(cursor) as usize;
    let slot_count = read_u32(cursor) as usize;
    let mut actual_slot_count = 0;
    let segments = (0..segment_count)
        .map(|_| match read_bytes(cursor, 1)[0] {
            0 => {
                let length = read_u32(cursor) as usize;
                TemplateSegment::Fixed(read_bytes(cursor, length))
            }
            1 => {
                actual_slot_count += 1;
                TemplateSegment::Slot
            }
            _ => panic!("invalid WorkGraph template segment kind"),
        })
        .collect::<Vec<_>>();
    assert_eq!(
        actual_slot_count, slot_count,
        "WorkGraph template slot count drifted"
    );
    TemplateFamily {
        segments,
        slot_count,
    }
}

fn expand_family(family: &TemplateFamily, replacements: &[Vec<u8>]) -> Vec<u8> {
    let mut replacement_index = 0;
    let mut source = Vec::new();
    for segment in &family.segments {
        match segment {
            TemplateSegment::Fixed(bytes) => source.extend_from_slice(bytes),
            TemplateSegment::Slot => {
                source.extend_from_slice(
                    replacements
                        .get(replacement_index)
                        .expect("missing WorkGraph replacement"),
                );
                replacement_index += 1;
            }
        }
    }
    assert_eq!(
        replacement_index,
        replacements.len(),
        "unused WorkGraph replacements"
    );
    source
}

fn read_u32(cursor: &mut Cursor<&[u8]>) -> u32 {
    let mut bytes = [0; 4];
    cursor
        .read_exact(&mut bytes)
        .expect("read WorkGraph bundle integer");
    u32::from_be_bytes(bytes)
}

fn read_bytes(cursor: &mut Cursor<&[u8]>, length: usize) -> Vec<u8> {
    let mut bytes = vec![0; length];
    cursor
        .read_exact(&mut bytes)
        .expect("read WorkGraph bundle bytes");
    bytes
}

fn decode_family_aliases(source: &[u8]) -> Vec<(String, String)> {
    let source = std::str::from_utf8(source).expect("WorkGraph module source must be UTF-8");
    let mut remaining = source;
    let mut aliases = Vec::new();
    while let Some(offset) = remaining.find(FAMILY_ALIAS_PREFIX) {
        let marker = &remaining[offset + FAMILY_ALIAS_PREFIX.len()..];
        let end = marker
            .find("*/")
            .expect("unterminated WorkGraph family alias");
        let (legacy_module, variant_module) = marker[..end]
            .split_once(':')
            .expect("invalid WorkGraph family alias");
        assert!(valid_module_name(legacy_module));
        assert!(valid_module_name(variant_module));
        aliases.push((legacy_module.to_owned(), variant_module.to_owned()));
        remaining = &marker[end + 2..];
    }
    aliases
}

fn valid_module_name(name: &str) -> bool {
    (name.starts_with("wg_") || name.starts_with("work_graph_"))
        && name
            .bytes()
            .all(|byte| byte == b'_' || byte.is_ascii_lowercase() || byte.is_ascii_digit())
}
