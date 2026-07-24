use std::str::Split;

use hepta_contracts::CapabilityId;
use hepta_contracts::CapabilityManifestRef;
use hepta_contracts::ContentHash;
use hepta_contracts::PreferenceId;
use hepta_contracts::PrincipalId;
use hepta_contracts::Revision;
use hepta_contracts::RevisionStamp;
use sha2::Digest;
use sha2::Sha256;

use super::AccumulatorData;
use super::EXPLICIT_PREFERENCE_REDUCER_VERSION;
use super::ExplicitPreferenceTarget;
use super::PreferenceReductionError;

const ACCUMULATOR_SCHEMA: &str = "hepta.intelligence.explicit-preference.accumulator.v1";
const STATE_HASH_DOMAIN: &str = "hepta.intelligence.explicit-preference.state.v1";
const TARGET_HASH_DOMAIN: &str = "hepta.intelligence.explicit-preference.target.v1";

pub(super) fn parse_data(payload: &str) -> Result<AccumulatorData, PreferenceReductionError> {
    let mut fields = payload.split('|');
    if next_field(&mut fields, "reducer")? != EXPLICIT_PREFERENCE_REDUCER_VERSION
        || next_field(&mut fields, "schema")? != ACCUMULATOR_SCHEMA
        || next_field(&mut fields, "target")? != "capability"
    {
        return Err(PreferenceReductionError::UnsupportedVersion);
    }
    let subject = PrincipalId::new(decode_text(next_field(&mut fields, "subject")?)?);
    let preference = PreferenceId::new(decode_text(next_field(&mut fields, "preference")?)?);
    let capability_id = CapabilityId::new(decode_text(next_field(&mut fields, "capability_id")?)?);
    let capability_revision = parse_u64(next_field(&mut fields, "capability_revision")?)?;
    let manifest_hash = ContentHash::new(decode_text(next_field(
        &mut fields,
        "capability_manifest_hash",
    )?)?);
    let catalog_revision = parse_u64(next_field(&mut fields, "catalog_revision")?)?;
    let catalog_hash = ContentHash::new(decode_text(next_field(&mut fields, "catalog_hash")?)?);
    let declared_target = ContentHash::new(decode_text(next_field(
        &mut fields,
        "target_binding_hash",
    )?)?);
    let data = AccumulatorData {
        subject,
        preference,
        target: ExplicitPreferenceTarget::Capability(CapabilityManifestRef::new(
            capability_id,
            Revision::new(capability_revision),
            manifest_hash,
            RevisionStamp::new(Revision::new(catalog_revision), catalog_hash),
        )),
        revision: Revision::new(parse_u64(next_field(&mut fields, "revision")?)?),
        accepted_count: parse_u64(next_field(&mut fields, "accepted")?)?,
        rejected_count: parse_u64(next_field(&mut fields, "rejected")?)?,
    };
    if fields.next().is_some() {
        return Err(PreferenceReductionError::MalformedPreviousPayload(
            "trailing field",
        ));
    }
    let computed_target = target_binding_hash(&data.target);
    if declared_target != computed_target {
        return Err(PreferenceReductionError::PayloadTargetBindingMismatch {
            declared: declared_target,
            computed: computed_target,
        });
    }
    if canonical_payload(&data, &computed_target) != payload {
        return Err(PreferenceReductionError::NonCanonicalPreviousPayload);
    }
    Ok(data)
}

pub(super) fn canonical_payload(data: &AccumulatorData, target_hash: &ContentHash) -> String {
    let capability = data.target.capability();
    format!(
        "reducer={EXPLICIT_PREFERENCE_REDUCER_VERSION}|schema={ACCUMULATOR_SCHEMA}|target=capability|subject={}|preference={}|capability_id={}|capability_revision={}|capability_manifest_hash={}|catalog_revision={}|catalog_hash={}|target_binding_hash={}|revision={}|accepted={}|rejected={}",
        encode_text(data.subject.as_str()),
        encode_text(data.preference.as_str()),
        encode_text(capability.id().as_str()),
        capability.revision().get(),
        encode_text(capability.manifest_hash().as_str()),
        capability.catalog().revision().get(),
        encode_text(capability.catalog().content_hash().as_str()),
        encode_text(target_hash.as_str()),
        data.revision.get(),
        data.accepted_count,
        data.rejected_count,
    )
}

pub(super) fn target_binding_hash(target: &ExplicitPreferenceTarget) -> ContentHash {
    let capability = target.capability();
    let mut hash = FramedHash::new(TARGET_HASH_DOMAIN);
    hash.text("target.kind", "capability");
    hash.text("capability.id", capability.id().as_str());
    hash.number("capability.revision", capability.revision().get());
    hash.text(
        "capability.manifest_hash",
        capability.manifest_hash().as_str(),
    );
    hash.number("catalog.revision", capability.catalog().revision().get());
    hash.text(
        "catalog.content_hash",
        capability.catalog().content_hash().as_str(),
    );
    hash.finish()
}

pub(super) fn state_hash(payload: &str) -> ContentHash {
    let mut hash = FramedHash::new(STATE_HASH_DOMAIN);
    hash.text("reducer.version", EXPLICIT_PREFERENCE_REDUCER_VERSION);
    hash.text("canonical.payload", payload);
    hash.finish()
}

fn next_field<'a>(
    fields: &mut Split<'a, char>,
    expected: &'static str,
) -> Result<&'a str, PreferenceReductionError> {
    let field = fields
        .next()
        .ok_or(PreferenceReductionError::MalformedPreviousPayload(expected))?;
    let (name, value) = field
        .split_once('=')
        .ok_or(PreferenceReductionError::MalformedPreviousPayload(expected))?;
    if name != expected {
        return Err(PreferenceReductionError::MalformedPreviousPayload(expected));
    }
    Ok(value)
}

fn parse_u64(value: &str) -> Result<u64, PreferenceReductionError> {
    let parsed = value
        .parse::<u64>()
        .map_err(|_| PreferenceReductionError::MalformedPreviousPayload("integer"))?;
    if parsed.to_string() != value {
        return Err(PreferenceReductionError::NonCanonicalPreviousPayload);
    }
    Ok(parsed)
}

fn encode_text(value: &str) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut encoded = String::with_capacity(value.len() * 2);
    for byte in value.bytes() {
        encoded.push(HEX[(byte >> 4) as usize] as char);
        encoded.push(HEX[(byte & 0x0f) as usize] as char);
    }
    encoded
}

fn decode_text(value: &str) -> Result<String, PreferenceReductionError> {
    if !value.len().is_multiple_of(2) {
        return Err(PreferenceReductionError::MalformedPreviousPayload(
            "hex text",
        ));
    }
    let mut decoded = Vec::with_capacity(value.len() / 2);
    for pair in value.as_bytes().chunks_exact(2) {
        decoded.push((hex_nibble(pair[0])? << 4) | hex_nibble(pair[1])?);
    }
    String::from_utf8(decoded)
        .map_err(|_| PreferenceReductionError::MalformedPreviousPayload("utf-8 text"))
}

fn hex_nibble(value: u8) -> Result<u8, PreferenceReductionError> {
    match value {
        b'0'..=b'9' => Ok(value - b'0'),
        b'a'..=b'f' => Ok(value - b'a' + 10),
        _ => Err(PreferenceReductionError::NonCanonicalPreviousPayload),
    }
}

struct FramedHash(Sha256);

impl FramedHash {
    fn new(domain: &str) -> Self {
        let mut hash = Self(Sha256::new());
        hash.bytes("domain", domain.as_bytes());
        hash
    }

    fn text(&mut self, name: &str, value: &str) {
        self.bytes(name, value.as_bytes());
    }

    fn number(&mut self, name: &str, value: u64) {
        self.bytes(name, &value.to_be_bytes());
    }

    fn bytes(&mut self, name: &str, value: &[u8]) {
        self.0.update((name.len() as u64).to_be_bytes());
        self.0.update(name.as_bytes());
        self.0.update((value.len() as u64).to_be_bytes());
        self.0.update(value);
    }

    fn finish(self) -> ContentHash {
        ContentHash::new(format!("sha256:{:x}", self.0.finalize()))
    }
}
