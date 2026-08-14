use super::*;
use pretty_assertions::assert_eq;

fn digest(byte: char) -> String {
    std::iter::repeat_n(byte, 64).collect()
}

fn valid_identity() -> AttemptIdentityV8 {
    AttemptIdentityV8 {
        attempt_nonce: digest('1'),
        barrier_generation: 7,
        candidate_head: CANDIDATE_HEAD.to_string(),
        candidate_tree: CANDIDATE_TREE.to_string(),
        driver_manifest_sha256: digest('2'),
        profile_manifest_sha256: digest('3'),
        parameter_manifest_sha256: digest('4'),
        machine_id_sha256: digest('5'),
        runner_snapshot_sha256: digest('6'),
        restore_plan_sha256: digest('7'),
    }
}

#[test]
fn exact_identity_validates_and_hashes_deterministically() {
    let identity = valid_identity();

    identity.validate().expect("exact identity validates");
    assert_eq!(
        identity.sha256().expect("valid digest"),
        identity.sha256().expect("valid repeated digest")
    );
    assert_eq!(identity.sha256().expect("valid digest").len(), 64);
}

#[test]
fn rejects_zero_generation_and_non_exact_candidate() {
    let mut identity = valid_identity();
    identity.barrier_generation = 0;
    assert!(identity.validate().is_err());

    let mut identity = valid_identity();
    identity.candidate_head = "0".repeat(40);
    assert!(identity.validate().is_err());

    let mut identity = valid_identity();
    identity.candidate_tree = "0".repeat(40);
    assert!(identity.validate().is_err());
}

#[test]
fn rejects_malformed_nonce_and_every_digest_binding() {
    let malformed = [
        String::new(),
        "a".repeat(63),
        "A".repeat(64),
        "g".repeat(64),
        "0".repeat(64),
    ];
    for value in malformed {
        let mut identity = valid_identity();
        identity.attempt_nonce = value;
        assert!(identity.validate().is_err());
    }

    for field in 0..6 {
        let mut identity = valid_identity();
        let value = if field % 2 == 0 {
            "A".repeat(64)
        } else {
            "0".repeat(64)
        };
        match field {
            0 => identity.driver_manifest_sha256 = value,
            1 => identity.profile_manifest_sha256 = value,
            2 => identity.parameter_manifest_sha256 = value,
            3 => identity.machine_id_sha256 = value,
            4 => identity.runner_snapshot_sha256 = value,
            5 => identity.restore_plan_sha256 = value,
            _ => unreachable!(),
        }
        assert!(identity.validate().is_err());
    }
}

#[test]
fn every_binding_changes_the_attempt_digest() {
    let original = valid_identity();
    let expected = original.sha256().expect("valid digest");

    let mut changes = Vec::new();
    let mut identity = original.clone();
    identity.attempt_nonce = digest('8');
    changes.push(identity);
    let mut identity = original.clone();
    identity.barrier_generation += 1;
    changes.push(identity);
    let mut identity = original.clone();
    identity.driver_manifest_sha256 = digest('8');
    changes.push(identity);
    let mut identity = original.clone();
    identity.profile_manifest_sha256 = digest('8');
    changes.push(identity);
    let mut identity = original.clone();
    identity.parameter_manifest_sha256 = digest('8');
    changes.push(identity);
    let mut identity = original.clone();
    identity.machine_id_sha256 = digest('8');
    changes.push(identity);
    let mut identity = original.clone();
    identity.runner_snapshot_sha256 = digest('8');
    changes.push(identity);
    let mut identity = original;
    identity.restore_plan_sha256 = digest('8');
    changes.push(identity);

    assert!(
        changes
            .into_iter()
            .all(|identity| identity.sha256().expect("valid digest") != expected)
    );
}

#[test]
fn serde_rejects_unknown_identity_fields() {
    let mut value = serde_json::to_value(valid_identity()).expect("serialize identity");
    value
        .as_object_mut()
        .expect("identity object")
        .insert("unbound".to_string(), serde_json::json!(true));

    assert!(serde_json::from_value::<AttemptIdentityV8>(value).is_err());
}
