use std::collections::BTreeSet;

use crate::AgentId;
use crate::AuthorityAction;
use crate::AuthorityGrant;
use crate::RuntimeAuthorityProfile;

use super::RuntimeProfileBinding;

const AGENT_ID: &str = "018f4f72-5f8f-7cc1-8f55-df9fb3aa2c12";

fn agent_id() -> AgentId {
    AgentId::parse(AGENT_ID).unwrap_or_else(|error| panic!("test AgentId must parse: {error}"))
}

#[test]
fn every_closed_profile_has_one_stable_seven_service_identity() {
    let profiles = [
        AuthorityGrant::snapshot_read_only(agent_id(), 1),
        AuthorityGrant::agent_local(agent_id(), 1),
        AuthorityGrant::qualification_cognitive_write(agent_id(), 1),
    ];
    let mut digests = BTreeSet::new();
    for authority in profiles {
        let authority = authority.unwrap_or_else(|error| panic!("authority must build: {error}"));
        let binding = RuntimeProfileBinding::for_authority(&authority)
            .unwrap_or_else(|error| panic!("profile binding must build: {error}"));
        assert_eq!(binding.service_rows().len(), 7);
        assert_eq!(binding.authority_grant_sha256(), &authority.digest());
        assert!(digests.insert(binding.profile_sha256().clone()));
    }
    assert_eq!(digests.len(), 3);
}

#[test]
fn profile_digest_binds_generation_and_profile() {
    let generation_one = AuthorityGrant::agent_local(agent_id(), 1)
        .unwrap_or_else(|error| panic!("authority must build: {error}"));
    let generation_two = AuthorityGrant::agent_local(agent_id(), 2)
        .unwrap_or_else(|error| panic!("authority must build: {error}"));
    let qualification = AuthorityGrant::qualification_cognitive_write(agent_id(), 1)
        .unwrap_or_else(|error| panic!("authority must build: {error}"));

    let first = RuntimeProfileBinding::for_authority(&generation_one)
        .unwrap_or_else(|error| panic!("profile must build: {error}"));
    let second = RuntimeProfileBinding::for_authority(&generation_two)
        .unwrap_or_else(|error| panic!("profile must build: {error}"));
    let third = RuntimeProfileBinding::for_authority(&qualification)
        .unwrap_or_else(|error| panic!("profile must build: {error}"));

    assert_ne!(first.profile_sha256(), second.profile_sha256());
    assert_ne!(first.profile_sha256(), third.profile_sha256());
    assert_eq!(first.profile(), RuntimeAuthorityProfile::AgentLocal);
    assert_eq!(third.profile(), RuntimeAuthorityProfile::QualificationCognitiveWrite);
}

#[test]
fn local_profile_rows_keep_all_external_actions_outside_the_profile() {
    let authority = AuthorityGrant::agent_local(agent_id(), 7)
        .unwrap_or_else(|error| panic!("authority must build: {error}"));
    let actions = authority.actions().collect::<BTreeSet<_>>();
    for action in [
        AuthorityAction::InvokeModel,
        AuthorityAction::DispatchProvider,
        AuthorityAction::ExternalEffect,
        AuthorityAction::MutateFleet,
        AuthorityAction::AcceptOperator,
        AuthorityAction::PromoteRelease,
    ] {
        assert!(!actions.contains(&action));
    }
    let binding = RuntimeProfileBinding::for_authority(&authority)
        .unwrap_or_else(|error| panic!("profile must build: {error}"));
    assert!(binding
        .service_rows()
        .contains(&"provider_effect_adapter|dormant_boundary|disabled|not_started|false"));
}
