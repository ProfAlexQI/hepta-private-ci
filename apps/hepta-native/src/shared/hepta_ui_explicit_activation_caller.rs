//! Operator-governed explicit activation caller for default-off UI material hosts.
//!
//! This source closes the caller seam without registering a product command. The
//! caller loads one sealed evidence bundle, validates candidate/digest/expiry and
//! nonce replay boundaries, then invokes a target exactly once. No startup or
//! lifecycle event can call it automatically.

pub const HEPTA_UI_EXPLICIT_CALLER_SOURCE_IMPLEMENTED: bool = true;
pub const HEPTA_UI_EXPLICIT_CALLER_REGISTERED: bool = false;
pub const HEPTA_UI_EXPLICIT_CALLER_AUTOMATIC: bool = false;
pub const HEPTA_UI_EXPLICIT_CALLER_RUNTIME_VALIDATED: bool = false;
pub const HEPTA_UI_EXPLICIT_CALLER_PRODUCTION_AUTHORITY: bool = false;
pub const HEPTA_UI_EXPLICIT_CALLER_EFFECT_AUTHORITY: bool = false;
pub const HEPTA_UI_EXPLICIT_CALLER_LIVE_ADAPTER_AUTHORITY: bool = false;
pub const HEPTA_UI_EXPLICIT_CALLER_OPERATOR_ACCEPTANCE: bool = false;
pub const HEPTA_UI_EXPLICIT_CALLER_PROMOTION: bool = false;
pub const HEPTA_UI_EXPLICIT_CALLER_RELEASE: bool = false;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HeptaUiActivationCommand {
    pub candidate_commit: String,
    pub candidate_tree: String,
    pub evidence_digest: String,
    pub operator_acceptance_digest: String,
    pub nonce: String,
    pub issued_at_unix: u64,
    pub expires_at_unix: u64,
    pub explicit: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HeptaUiSealedEvidence<E> {
    pub candidate_commit: String,
    pub candidate_tree: String,
    pub digest: String,
    pub operator_acceptance_digest: String,
    pub payload: E,
    pub grants_authority: bool,
}

pub trait HeptaUiSealedEvidenceLoader<E> {
    type Error;
    fn load_exact(&mut self, digest: &str) -> Result<HeptaUiSealedEvidence<E>, Self::Error>;
}

pub trait HeptaUiExplicitActivationTarget<E> {
    type Receipt;
    type Error;
    fn activate_explicit(&mut self, evidence: &E) -> Result<Self::Receipt, Self::Error>;
}

pub trait HeptaUiNonceLedger {
    fn claim(&mut self, nonce: &str, expires_at_unix: u64) -> bool;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HeptaUiExplicitCallerError<L, T> {
    CommandNotExplicit,
    InvalidCandidate,
    InvalidDigest,
    InvalidOperatorAcceptance,
    InvalidLifetime,
    Expired,
    Replay,
    EvidenceLoad(L),
    EvidenceCandidateDrift,
    EvidenceDigestDrift,
    OperatorAcceptanceDrift,
    AuthorityEscape,
    Target(T),
}

pub fn invoke_explicit<E, L, N, T>(
    command: &HeptaUiActivationCommand,
    now_unix: u64,
    loader: &mut L,
    nonce_ledger: &mut N,
    target: &mut T,
) -> Result<T::Receipt, HeptaUiExplicitCallerError<L::Error, T::Error>>
where
    L: HeptaUiSealedEvidenceLoader<E>,
    N: HeptaUiNonceLedger,
    T: HeptaUiExplicitActivationTarget<E>,
{
    if !command.explicit { return Err(HeptaUiExplicitCallerError::CommandNotExplicit); }
    if !git_object_id(&command.candidate_commit) || !git_object_id(&command.candidate_tree) { return Err(HeptaUiExplicitCallerError::InvalidCandidate); }
    if !sha256_hex(&command.evidence_digest) { return Err(HeptaUiExplicitCallerError::InvalidDigest); }
    if !sha256_hex(&command.operator_acceptance_digest) { return Err(HeptaUiExplicitCallerError::InvalidOperatorAcceptance); }
    if command.issued_at_unix == 0 || command.expires_at_unix <= command.issued_at_unix { return Err(HeptaUiExplicitCallerError::InvalidLifetime); }
    if now_unix < command.issued_at_unix || now_unix >= command.expires_at_unix { return Err(HeptaUiExplicitCallerError::Expired); }
    if command.nonce.is_empty() || command.nonce.len() > 128 || !nonce_ledger.claim(&command.nonce, command.expires_at_unix) { return Err(HeptaUiExplicitCallerError::Replay); }
    let evidence = loader.load_exact(&command.evidence_digest).map_err(HeptaUiExplicitCallerError::EvidenceLoad)?;
    if evidence.candidate_commit != command.candidate_commit || evidence.candidate_tree != command.candidate_tree { return Err(HeptaUiExplicitCallerError::EvidenceCandidateDrift); }
    if evidence.digest != command.evidence_digest { return Err(HeptaUiExplicitCallerError::EvidenceDigestDrift); }
    if evidence.operator_acceptance_digest != command.operator_acceptance_digest { return Err(HeptaUiExplicitCallerError::OperatorAcceptanceDrift); }
    if evidence.grants_authority { return Err(HeptaUiExplicitCallerError::AuthorityEscape); }
    target.activate_explicit(&evidence.payload).map_err(HeptaUiExplicitCallerError::Target)
}

fn git_object_id(value:&str)->bool{value.len()==40&&value.bytes().all(|b|b.is_ascii_hexdigit()&&!b.is_ascii_uppercase())}
fn sha256_hex(value:&str)->bool{value.len()==64&&value.bytes().all(|b|b.is_ascii_hexdigit()&&!b.is_ascii_uppercase())}

#[cfg(test)]
mod tests{
    use super::*;use std::collections::BTreeSet;
    #[derive(Default)]struct Ledger(BTreeSet<String>);impl HeptaUiNonceLedger for Ledger{fn claim(&mut self,n:&str,_:u64)->bool{self.0.insert(n.to_owned())}}
    struct Loader{e:HeptaUiSealedEvidence<u64>}impl HeptaUiSealedEvidenceLoader<u64> for Loader{type Error=();fn load_exact(&mut self,_:&str)->Result<HeptaUiSealedEvidence<u64>,()>{Ok(self.e.clone())}}
    #[derive(Default)]struct Target{calls:u8}impl HeptaUiExplicitActivationTarget<u64> for Target{type Receipt=u64;type Error=();fn activate_explicit(&mut self,e:&u64)->Result<u64,()>{self.calls+=1;Ok(*e)}}
    fn cmd()->HeptaUiActivationCommand{HeptaUiActivationCommand{candidate_commit:"a".repeat(40),candidate_tree:"b".repeat(40),evidence_digest:"c".repeat(64),operator_acceptance_digest:"d".repeat(64),nonce:"n1".into(),issued_at_unix:10,expires_at_unix:20,explicit:true}}
    fn loader()->Loader{Loader{e:HeptaUiSealedEvidence{candidate_commit:"a".repeat(40),candidate_tree:"b".repeat(40),digest:"c".repeat(64),operator_acceptance_digest:"d".repeat(64),payload:7,grants_authority:false}}}
    #[test]fn explicit_command_invokes_once(){let mut l=loader();let mut n=Ledger::default();let mut t=Target::default();assert_eq!(invoke_explicit(&cmd(),15,&mut l,&mut n,&mut t),Ok(7));assert_eq!(t.calls,1);assert_eq!(invoke_explicit(&cmd(),15,&mut l,&mut n,&mut t),Err(HeptaUiExplicitCallerError::Replay));}
    #[test]fn implicit_expired_or_authority_evidence_is_rejected(){let mut c=cmd();c.explicit=false;let mut l=loader();let mut n=Ledger::default();let mut t=Target::default();assert_eq!(invoke_explicit(&c,15,&mut l,&mut n,&mut t),Err(HeptaUiExplicitCallerError::CommandNotExplicit));let mut l=loader();l.e.grants_authority=true;let mut c=cmd();c.nonce="n2".into();assert_eq!(invoke_explicit(&c,15,&mut l,&mut n,&mut t),Err(HeptaUiExplicitCallerError::AuthorityEscape));}
}
