use hepta_core::*;

#[path = "memory_recall_contracts/formation.rs"]
mod formation;
#[path = "memory_recall_contracts/taxonomy.rs"]
mod taxonomy;
#[path = "memory_recall_contracts/temporal.rs"]
mod temporal;

fn stable_receipt_hash(parts: &[&str]) -> String {
    const OFFSET: u64 = 0xcbf29ce484222325;
    const PRIME: u64 = 0x100000001b3;

    let mut hash = OFFSET;
    for part in parts {
        for byte in part.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(PRIME);
        }
        hash ^= 0xff;
        hash = hash.wrapping_mul(PRIME);
    }
    format!("{hash:016x}")
}

fn stable_receipt_hash_is_valid(value: &str) -> bool {
    value.len() == 16 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}
