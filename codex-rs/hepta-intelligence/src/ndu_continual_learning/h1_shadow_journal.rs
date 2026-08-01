use std::collections::BTreeMap;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
#[cfg(unix)]
use std::os::unix::fs::OpenOptionsExt;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::path::PathBuf;

use fd_lock::RwLock;
use hepta_contracts::ContentHash;
use serde::Deserialize;
use serde::Serialize;

use super::NduH1ShadowConfig;
use super::NduH1ShadowController;
use super::NduH1ShadowError;
use super::NduH1ShadowReceipt;
use super::NduH1ShadowRequest;
use super::feasibility_verdict;
use super::stable_hash;

const JOURNAL_SCHEMA: &str = "hepta_ndu_h1_shadow_journal_v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct NduH1JournalRecord {
    schema: String,
    revision: u64,
    event_hash: String,
    previous_journal_hash: String,
    journal_hash: String,
    propensity_basis_points: u16,
    delayed_outcome_hash: Option<String>,
    arms: Vec<NduH1JournalArm>,
    production_authority_granted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct NduH1JournalArm {
    baseline: String,
    model_hash: String,
    config_hash: String,
    next_state_hash: String,
    task_value_basis_points: i32,
    learning_value_basis_points: i32,
    trust_basis_points: i32,
    memory_pollution_risk_basis_points: i32,
    resource_cost_basis_points: i32,
    uncertainty_basis_points: i32,
    safety: String,
    permission: String,
    budget: String,
    correctability: String,
    replay_receipt_hash: String,
}

impl NduH1JournalRecord {
    fn from_receipt(receipt: &NduH1ShadowReceipt) -> Self {
        Self {
            schema: JOURNAL_SCHEMA.to_owned(),
            revision: receipt.revision().get(),
            event_hash: receipt.event_hash().as_str().to_owned(),
            previous_journal_hash: receipt.previous_journal_hash().as_str().to_owned(),
            journal_hash: receipt.journal_hash().as_str().to_owned(),
            propensity_basis_points: receipt.propensity_basis_points(),
            delayed_outcome_hash: receipt
                .delayed_outcome_hash()
                .map(|hash| hash.as_str().to_owned()),
            arms: receipt
                .arm_results()
                .iter()
                .map(|result| {
                    let transition = result.transition();
                    let utility = transition.utility();
                    let feasibility = transition.feasibility();
                    NduH1JournalArm {
                        baseline: result.baseline().as_str().to_owned(),
                        model_hash: transition.model_hash().as_str().to_owned(),
                        config_hash: transition.config_hash().as_str().to_owned(),
                        next_state_hash: transition.next_state_hash().as_str().to_owned(),
                        task_value_basis_points: utility.task_value().basis_points(),
                        learning_value_basis_points: utility.learning_value().basis_points(),
                        trust_basis_points: utility.trust().basis_points(),
                        memory_pollution_risk_basis_points: utility
                            .memory_pollution_risk()
                            .basis_points(),
                        resource_cost_basis_points: utility.resource_cost().basis_points(),
                        uncertainty_basis_points: utility.uncertainty().basis_points(),
                        safety: feasibility_verdict(feasibility.safety()).to_owned(),
                        permission: feasibility_verdict(feasibility.permission()).to_owned(),
                        budget: feasibility_verdict(feasibility.budget()).to_owned(),
                        correctability: feasibility_verdict(feasibility.correctability())
                            .to_owned(),
                        replay_receipt_hash: result.replay_receipt_hash().as_str().to_owned(),
                    }
                })
                .collect(),
            production_authority_granted: receipt.production_authority_granted(),
        }
    }

    fn verify(&self, expected_revision: u64, expected_previous: &ContentHash) -> bool {
        if self.schema != JOURNAL_SCHEMA
            || self.revision != expected_revision
            || self.previous_journal_hash != expected_previous.as_str()
            || self.arms.len() != 4
            || self.production_authority_granted
        {
            return false;
        }
        let baselines = [
            "current_heuristic",
            "contextual_bandit",
            "frozen_gru_mlp",
            "ndu_shadow",
        ];
        for (arm, baseline) in self.arms.iter().zip(baselines) {
            if arm.baseline != baseline || !arm.verify(self) {
                return false;
            }
        }
        let mut parts = vec![
            JOURNAL_SCHEMA,
            self.previous_journal_hash.as_str(),
            self.event_hash.as_str(),
        ];
        parts.extend(self.arms.iter().map(|arm| arm.replay_receipt_hash.as_str()));
        stable_hash(&parts).as_str() == self.journal_hash
    }
}

impl NduH1JournalArm {
    fn verify(&self, record: &NduH1JournalRecord) -> bool {
        let expected_state = stable_hash(&[
            "hepta_ndu_state_v1",
            record.previous_journal_hash.as_str(),
            record.event_hash.as_str(),
            self.model_hash.as_str(),
            self.config_hash.as_str(),
            self.baseline.as_str(),
            &record.revision.to_string(),
            &self.task_value_basis_points.to_string(),
            &self.learning_value_basis_points.to_string(),
            &self.trust_basis_points.to_string(),
            &self.memory_pollution_risk_basis_points.to_string(),
            &self.resource_cost_basis_points.to_string(),
            &self.uncertainty_basis_points.to_string(),
        ]);
        if expected_state.as_str() != self.next_state_hash {
            return false;
        }
        let expected_receipt = stable_hash(&[
            "hepta_ndu_paired_replay_receipt_v1",
            self.baseline.as_str(),
            self.next_state_hash.as_str(),
            &self.task_value_basis_points.to_string(),
            &self.learning_value_basis_points.to_string(),
            &self.trust_basis_points.to_string(),
            &self.memory_pollution_risk_basis_points.to_string(),
            &self.resource_cost_basis_points.to_string(),
            &self.uncertainty_basis_points.to_string(),
            self.safety.as_str(),
            self.permission.as_str(),
            self.budget.as_str(),
            self.correctability.as_str(),
            &record.propensity_basis_points.to_string(),
            record.delayed_outcome_hash.as_deref().unwrap_or("none"),
        ]);
        expected_receipt.as_str() == self.replay_receipt_hash
            && [
                self.task_value_basis_points,
                self.learning_value_basis_points,
                self.trust_basis_points,
                self.memory_pollution_risk_basis_points,
                self.resource_cost_basis_points,
                self.uncertainty_basis_points,
            ]
            .into_iter()
            .all(|value| (-10_000..=10_000).contains(&value))
            && [
                self.safety.as_str(),
                self.permission.as_str(),
                self.budget.as_str(),
                self.correctability.as_str(),
            ]
            .into_iter()
            .all(|value| matches!(value, "satisfied" | "violated" | "unknown"))
    }
}

#[derive(Debug)]
pub struct NduH1Journal {
    path: PathBuf,
    file: RwLock<File>,
    journal_bytes: u64,
    initial_state_hash: ContentHash,
    head: ContentHash,
    event_journal_hashes: BTreeMap<String, ContentHash>,
    record_count: u64,
}

impl NduH1Journal {
    pub fn open(
        path: impl AsRef<Path>,
        initial_state_hash: ContentHash,
    ) -> Result<Self, NduH1JournalError> {
        let path = path.as_ref().to_path_buf();
        if path
            .symlink_metadata()
            .is_ok_and(|metadata| metadata.file_type().is_symlink())
        {
            return Err(NduH1JournalError::SymlinkRejected);
        }
        if path.parent().is_some_and(|parent| {
            parent
                .symlink_metadata()
                .is_ok_and(|metadata| metadata.file_type().is_symlink())
        }) {
            return Err(NduH1JournalError::SymlinkRejected);
        }
        let mut options = OpenOptions::new();
        options.create(true).read(true).append(true);
        #[cfg(unix)]
        options.mode(0o600);
        let file = options.open(&path).map_err(|_| NduH1JournalError::Io)?;
        #[cfg(unix)]
        if file
            .metadata()
            .map_err(|_| NduH1JournalError::Io)?
            .permissions()
            .mode()
            & 0o077
            != 0
        {
            return Err(NduH1JournalError::InsecurePermissions);
        }
        let mut journal = Self {
            path,
            journal_bytes: file.metadata().map_err(|_| NduH1JournalError::Io)?.len(),
            file: RwLock::new(file),
            head: initial_state_hash.clone(),
            initial_state_hash,
            event_journal_hashes: BTreeMap::new(),
            record_count: 0,
        };
        journal.recover()?;
        Ok(journal)
    }

    pub fn append(&mut self, receipt: &NduH1ShadowReceipt) -> Result<(), NduH1JournalError> {
        let record = NduH1JournalRecord::from_receipt(receipt);
        if !record.verify(self.record_count + 1, &self.head) {
            return Err(NduH1JournalError::ChainMismatch);
        }
        if self.event_journal_hashes.contains_key(&record.event_hash) {
            return Err(NduH1JournalError::DuplicateEvent);
        }
        let mut encoded = serde_json::to_vec(&record).map_err(|_| NduH1JournalError::Encoding)?;
        encoded.push(b'\n');
        let mut file = self
            .file
            .try_write()
            .map_err(|_| NduH1JournalError::ConcurrentWriter)?;
        if file.metadata().map_err(|_| NduH1JournalError::Io)?.len() != self.journal_bytes {
            return Err(NduH1JournalError::ConcurrentWriter);
        }
        file.write_all(&encoded)
            .map_err(|_| NduH1JournalError::Io)?;
        file.sync_data().map_err(|_| NduH1JournalError::Io)?;
        self.journal_bytes += encoded.len() as u64;
        self.head = ContentHash::new(record.journal_hash.clone());
        self.event_journal_hashes
            .insert(record.event_hash, ContentHash::new(record.journal_hash));
        self.record_count += 1;
        Ok(())
    }

    pub fn event_journal_hash(&self, event_hash: &ContentHash) -> Option<&ContentHash> {
        self.event_journal_hashes.get(event_hash.as_str())
    }

    pub fn head(&self) -> &ContentHash {
        &self.head
    }

    pub const fn record_count(&self) -> u64 {
        self.record_count
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    fn recover(&mut self) -> Result<(), NduH1JournalError> {
        let reader_file = File::open(&self.path).map_err(|_| NduH1JournalError::Io)?;
        let mut expected_previous = self.initial_state_hash.clone();
        for (index, line) in BufReader::new(reader_file).lines().enumerate() {
            let line = line.map_err(|_| NduH1JournalError::Io)?;
            if line.is_empty() {
                return Err(NduH1JournalError::CorruptRecord);
            }
            let record: NduH1JournalRecord =
                serde_json::from_str(&line).map_err(|_| NduH1JournalError::CorruptRecord)?;
            if !record.verify(index as u64 + 1, &expected_previous) {
                return Err(NduH1JournalError::ChainMismatch);
            }
            if self.event_journal_hashes.contains_key(&record.event_hash) {
                return Err(NduH1JournalError::DuplicateEvent);
            }
            expected_previous = ContentHash::new(record.journal_hash.clone());
            self.event_journal_hashes
                .insert(record.event_hash, ContentHash::new(record.journal_hash));
            self.record_count += 1;
        }
        self.head = expected_previous;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NduH1JournalError {
    Io,
    Encoding,
    CorruptRecord,
    ChainMismatch,
    DuplicateEvent,
    SymlinkRejected,
    InsecurePermissions,
    ConcurrentWriter,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NduH1ShadowServiceResult {
    Recorded(NduH1ShadowReceipt),
    AlreadyObserved { journal_hash: ContentHash },
}

#[derive(Debug)]
pub struct NduH1ShadowService {
    config: NduH1ShadowConfig,
    controller: NduH1ShadowController,
    journal: NduH1Journal,
}

impl NduH1ShadowService {
    pub fn open(
        config: NduH1ShadowConfig,
        journal_path: impl AsRef<Path>,
    ) -> Result<Self, NduH1ShadowServiceError> {
        let journal = NduH1Journal::open(journal_path, config.initial_state_hash().clone())
            .map_err(NduH1ShadowServiceError::Journal)?;
        let controller = NduH1ShadowController::resume(
            config.clone(),
            journal.head().clone(),
            journal.record_count(),
        )
        .map_err(NduH1ShadowServiceError::Controller)?;
        Ok(Self {
            config,
            controller,
            journal,
        })
    }

    pub fn observe(
        &mut self,
        request: NduH1ShadowRequest,
    ) -> Result<NduH1ShadowServiceResult, NduH1ShadowServiceError> {
        let event_hash = request.event_hash().clone();
        if let Some(journal_hash) = self.journal.event_journal_hash(&event_hash) {
            return Ok(NduH1ShadowServiceResult::AlreadyObserved {
                journal_hash: journal_hash.clone(),
            });
        }
        let receipt = self
            .controller
            .observe(request)
            .map_err(NduH1ShadowServiceError::Controller)?;
        if let Err(error) = self.journal.append(&receipt) {
            self.controller = NduH1ShadowController::resume(
                self.config.clone(),
                self.journal.head().clone(),
                self.journal.record_count(),
            )
            .map_err(NduH1ShadowServiceError::Controller)?;
            return Err(NduH1ShadowServiceError::Journal(error));
        }
        Ok(NduH1ShadowServiceResult::Recorded(receipt))
    }

    pub fn journal(&self) -> &NduH1Journal {
        &self.journal
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NduH1ShadowServiceError {
    Controller(NduH1ShadowError),
    Journal(NduH1JournalError),
}

#[cfg(test)]
mod tests {
    use hepta_contracts::Revision;
    use tempfile::tempdir;

    use super::*;
    use crate::HardFeasibilityMask;
    use crate::HardFeasibilityVerdict;
    use crate::NduShadowObservation;
    use crate::NduUtilityEventRef;

    fn hash(value: &str) -> ContentHash {
        ContentHash::new(value)
    }

    fn config() -> NduH1ShadowConfig {
        NduH1ShadowConfig::new(
            hash("tenant"),
            hash("consent"),
            hash("revocation"),
            hash("model"),
            hash("config"),
            hash("initial"),
            10,
            true,
        )
    }

    fn request(event: &str) -> NduH1ShadowRequest {
        NduH1ShadowRequest::new(
            hash("tenant"),
            hash("consent"),
            hash("revocation"),
            NduShadowObservation::new(
                NduUtilityEventRef::new(hash(event), hash("receipt"), hash("subject"), None),
                1_000,
                500,
                750,
                100,
                300,
                200,
                5_000,
                None,
            ),
            HardFeasibilityMask::new(
                HardFeasibilityVerdict::Satisfied,
                HardFeasibilityVerdict::Satisfied,
                HardFeasibilityVerdict::Satisfied,
                HardFeasibilityVerdict::Satisfied,
            ),
        )
    }

    #[test]
    fn journal_recovers_chain_and_replays_idempotently() {
        let directory = tempdir().unwrap();
        let journal_path = directory.path().join("ndu-h1.jsonl");
        let mut service = NduH1ShadowService::open(config(), &journal_path).unwrap();
        let recorded = service.observe(request("event-1")).unwrap();
        let receipt = match recorded {
            NduH1ShadowServiceResult::Recorded(receipt) => receipt,
            NduH1ShadowServiceResult::AlreadyObserved { .. } => panic!("unexpected replay"),
        };
        assert_eq!(receipt.revision(), Revision::new(1));
        drop(service);

        let mut reopened = NduH1ShadowService::open(config(), &journal_path).unwrap();
        assert_eq!(reopened.journal().record_count(), 1);
        assert!(matches!(
            reopened.observe(request("event-1")).unwrap(),
            NduH1ShadowServiceResult::AlreadyObserved { .. }
        ));
        let second = reopened.observe(request("event-2")).unwrap();
        assert!(matches!(second, NduH1ShadowServiceResult::Recorded(_)));
        assert_eq!(reopened.journal().record_count(), 2);
    }

    #[test]
    fn journal_rejects_corruption_and_symlink() {
        let directory = tempdir().unwrap();
        let journal_path = directory.path().join("ndu-h1.jsonl");
        std::fs::write(&journal_path, b"not-json\n").unwrap();
        #[cfg(unix)]
        std::fs::set_permissions(&journal_path, std::fs::Permissions::from_mode(0o600)).unwrap();
        assert!(matches!(
            NduH1Journal::open(&journal_path, hash("initial")),
            Err(NduH1JournalError::CorruptRecord)
        ));

        #[cfg(unix)]
        {
            let target = directory.path().join("target");
            let link = directory.path().join("link");
            std::fs::write(&target, b"").unwrap();
            std::os::unix::fs::symlink(&target, &link).unwrap();
            assert!(matches!(
                NduH1Journal::open(&link, hash("initial")),
                Err(NduH1JournalError::SymlinkRejected)
            ));

            let linked_parent = directory.path().join("linked-parent");
            let real_parent = directory.path().join("real-parent");
            std::fs::create_dir(&real_parent).unwrap();
            std::os::unix::fs::symlink(&real_parent, &linked_parent).unwrap();
            assert!(matches!(
                NduH1Journal::open(linked_parent.join("journal"), hash("initial")),
                Err(NduH1JournalError::SymlinkRejected)
            ));
        }
    }

    #[cfg(unix)]
    #[test]
    fn journal_requires_private_file_mode() {
        let directory = tempdir().unwrap();
        let journal_path = directory.path().join("ndu-h1.jsonl");
        let journal = NduH1Journal::open(&journal_path, hash("initial")).unwrap();
        assert_eq!(
            std::fs::metadata(journal.path())
                .unwrap()
                .permissions()
                .mode()
                & 0o777,
            0o600
        );
        drop(journal);
        std::fs::set_permissions(&journal_path, std::fs::Permissions::from_mode(0o644)).unwrap();
        assert!(matches!(
            NduH1Journal::open(&journal_path, hash("initial")),
            Err(NduH1JournalError::InsecurePermissions)
        ));
    }

    #[test]
    fn stale_concurrent_writer_fails_closed() {
        let directory = tempdir().unwrap();
        let journal_path = directory.path().join("ndu-h1.jsonl");
        let mut first = NduH1ShadowService::open(config(), &journal_path).unwrap();
        let mut stale = NduH1ShadowService::open(config(), &journal_path).unwrap();

        first.observe(request("event-1")).unwrap();
        assert_eq!(
            stale.observe(request("event-2")),
            Err(NduH1ShadowServiceError::Journal(
                NduH1JournalError::ConcurrentWriter
            ))
        );
    }
}
