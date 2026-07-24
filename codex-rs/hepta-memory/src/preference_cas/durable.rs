mod codec;

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::path::Path;

use hepta_contracts::PreferenceId;
use hepta_contracts::PreferenceTransition;
use hepta_contracts::PrincipalId;
use hepta_contracts::Revision;
use serde::Deserialize;
use serde::Serialize;
use sqlx::Sqlite;
use sqlx::Transaction;

use super::EvidenceUse;
use super::EvidencedTransitionUse;
use super::PreferenceCasError;
use super::PreferenceDocumentCommitOutcome;
use super::PreferenceGenesisOutcome;
use super::PreferenceKey;
use super::PreferenceStateDocument;
use super::ReceiptUse;
use super::evidence_reuse_error;
use super::map_durable_error;
use super::transition_reuse_error;
use crate::contract_codec::PreferenceStateDocumentWire;
use crate::contract_codec::PreferenceTransitionWire;
use crate::durable::DurableDatabase;
use crate::durable::DurableIntegrityKey;
use crate::durable::DurableStorageError;
use codec::decode_document_row;
use codec::decode_keyed_document_row;
use codec::decode_transition_row;

const PREFERENCE_ROW_SCHEMA_VERSION: u32 = 1;

/// Recoverable SQLite-WAL implementation of V2 preference document CAS.
///
/// Genesis and transitions are immutable log rows. The current document is a
/// materialized head projection updated in the same transaction. Every open
/// replays the immutable rows, verifies their canonical-row integrity and
/// contract invariants, and requires the replayed state to exactly match the
/// persisted head projection before the store can serve reads or writes.
#[derive(Clone)]
pub struct DurablePreferenceStore {
    database: DurableDatabase,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PreferenceGenesisWire {
    schema_version: u32,
    preference_id: String,
    subject_id: String,
    document: PreferenceStateDocumentWire,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PreferenceTransitionRowWire {
    schema_version: u32,
    transition: PreferenceTransitionWire,
    document: PreferenceStateDocumentWire,
}

impl DurablePreferenceStore {
    /// Exclusively reserves a new path and bootstraps an empty V2 database.
    ///
    /// This refuses every pre-existing filesystem entry. Initialization
    /// failure leaves the reserved artifact in place so a retry cannot adopt
    /// uncertain storage.
    pub async fn bootstrap_new(path: impl AsRef<Path>) -> Result<Self, PreferenceCasError> {
        let database = DurableDatabase::bootstrap_new(path)
            .await
            .map_err(map_durable_error)?;
        Self::recover(database).await
    }

    /// Bootstraps a database whose canonical rows require an external key.
    pub async fn bootstrap_new_keyed(
        path: impl AsRef<Path>,
        key: DurableIntegrityKey,
    ) -> Result<Self, PreferenceCasError> {
        let database = DurableDatabase::bootstrap_new_keyed(path, key)
            .await
            .map_err(map_durable_error)?;
        Self::recover(database).await
    }

    /// Opens an initialized V2 database without creating or migrating it.
    pub async fn open_existing(path: impl AsRef<Path>) -> Result<Self, PreferenceCasError> {
        let database = DurableDatabase::open_existing(path)
            .await
            .map_err(map_durable_error)?;
        Self::recover(database).await
    }

    /// Opens a keyed database and rejects a wrong or missing integrity key.
    pub async fn open_existing_keyed(
        path: impl AsRef<Path>,
        key: DurableIntegrityKey,
    ) -> Result<Self, PreferenceCasError> {
        let database = DurableDatabase::open_existing_keyed(path, key)
            .await
            .map_err(map_durable_error)?;
        Self::recover(database).await
    }

    async fn recover(database: DurableDatabase) -> Result<Self, PreferenceCasError> {
        let store = Self { database };
        store.verify_recovery().await?;
        Ok(store)
    }

    /// Returns the SQLite database path backing this store.
    pub fn path(&self) -> &Path {
        self.database.path()
    }

    /// Initializes one immutable revision-zero document.
    pub async fn get_or_init_genesis(
        &self,
        preference: PreferenceId,
        subject: PrincipalId,
        document: PreferenceStateDocument,
    ) -> Result<PreferenceGenesisOutcome, PreferenceCasError> {
        self.validate_database_identity()?;
        if document.state().revision() != Revision::new(0) {
            return Err(PreferenceCasError::NonZeroGenesis {
                attempted: document.state().revision(),
            });
        }
        let key = PreferenceKey::new(preference, subject);
        let wire = PreferenceGenesisWire {
            schema_version: PREFERENCE_ROW_SCHEMA_VERSION,
            preference_id: key.preference.as_str().to_owned(),
            subject_id: key.subject.as_str().to_owned(),
            document: (&document).into(),
        };
        let row = self
            .database
            .encode_canonical_row(&wire)
            .map_err(map_durable_error)?;

        let mut transaction = self.database.pool().begin().await.map_err(|error| {
            map_durable_error(DurableStorageError::persistence(
                "begin preference genesis transaction",
                error,
            ))
        })?;
        DurableDatabase::acquire_write_serialization(&mut transaction)
            .await
            .map_err(map_durable_error)?;

        let existing_genesis = fetch_document_row(
            &mut transaction,
            "hepta_v2_preference_genesis",
            key.preference.as_str(),
            key.subject.as_str(),
        )
        .await?;
        let existing_head = fetch_document_row(
            &mut transaction,
            "hepta_v2_preference_heads",
            key.preference.as_str(),
            key.subject.as_str(),
        )
        .await?;
        match (existing_genesis, existing_head) {
            (Some(genesis), Some(head)) => {
                let existing = decode_document_row(&self.database, genesis, "preference genesis")?;
                decode_document_row(&self.database, head, "preference head")?;
                let outcome = if existing == document {
                    Ok(PreferenceGenesisOutcome::AlreadyInitialized)
                } else {
                    Err(PreferenceCasError::GenesisConflict {
                        existing: Box::new(existing),
                        attempted: Box::new(document),
                    })
                };
                return self.rollback_with_identity(transaction, outcome).await;
            }
            (Some(_), None) => {
                return self
                    .rollback_with_identity(
                        transaction,
                        Err(PreferenceCasError::Corrupt {
                            detail: format!(
                                "preference genesis {} for subject {} has no head projection",
                                key.preference, key.subject
                            ),
                        }),
                    )
                    .await;
            }
            (None, Some(_)) => {
                return self
                    .rollback_with_identity(
                        transaction,
                        Err(PreferenceCasError::Corrupt {
                            detail: format!(
                                "preference head {} for subject {} has no immutable genesis",
                                key.preference, key.subject
                            ),
                        }),
                    )
                    .await;
            }
            (None, None) => {}
        }

        insert_document_row(
            &mut transaction,
            "hepta_v2_preference_genesis",
            &key,
            &row.payload_json,
            &row.storage_hash,
        )
        .await?;
        insert_document_row(
            &mut transaction,
            "hepta_v2_preference_heads",
            &key,
            &row.payload_json,
            &row.storage_hash,
        )
        .await?;
        transaction.commit().await.map_err(|error| {
            map_durable_error(DurableStorageError::persistence(
                "commit preference genesis transaction",
                error,
            ))
        })?;
        self.validate_database_identity()?;
        Ok(PreferenceGenesisOutcome::Initialized)
    }

    /// Reads the exact recovered V2 document for one key.
    pub async fn read_document(
        &self,
        preference: &PreferenceId,
        subject: &PrincipalId,
    ) -> Result<Option<PreferenceStateDocument>, PreferenceCasError> {
        self.validate_database_identity()?;
        let mut transaction = self.database.pool().begin().await.map_err(|error| {
            map_durable_error(DurableStorageError::persistence(
                "begin preference read snapshot",
                error,
            ))
        })?;
        let head = fetch_document_row(
            &mut transaction,
            "hepta_v2_preference_heads",
            preference.as_str(),
            subject.as_str(),
        )
        .await?;
        let genesis = fetch_document_row(
            &mut transaction,
            "hepta_v2_preference_genesis",
            preference.as_str(),
            subject.as_str(),
        )
        .await?;
        let result = match (genesis, head) {
            (Some(genesis), Some(head)) => {
                decode_document_row(&self.database, genesis, "preference genesis")?;
                decode_document_row(&self.database, head, "preference head").map(Some)
            }
            (Some(_), None) => Err(PreferenceCasError::Corrupt {
                detail: format!(
                    "preference genesis {preference} for subject {subject} has no head projection"
                ),
            }),
            (None, Some(_)) => Err(PreferenceCasError::Corrupt {
                detail: format!(
                    "preference head {preference} for subject {subject} has no immutable genesis"
                ),
            }),
            (None, None) => Ok(None),
        };
        transaction.commit().await.map_err(|error| {
            map_durable_error(DurableStorageError::persistence(
                "close preference read snapshot",
                error,
            ))
        })?;
        self.validate_database_identity()?;
        result
    }

    /// Atomically commits a caller-supplied document under exact evidence CAS.
    pub async fn commit_evidenced(
        &self,
        transition: PreferenceTransition,
        document: PreferenceStateDocument,
    ) -> Result<PreferenceDocumentCommitOutcome, PreferenceCasError> {
        self.validate_database_identity()?;
        let mut transaction = self.database.pool().begin().await.map_err(|error| {
            map_durable_error(DurableStorageError::persistence(
                "begin preference CAS transaction",
                error,
            ))
        })?;
        DurableDatabase::acquire_write_serialization(&mut transaction)
            .await
            .map_err(map_durable_error)?;

        if let Some(existing) =
            fetch_transition_by(&mut transaction, "transition_id", transition.id().as_str()).await?
        {
            let existing = decode_transition_row(&self.database, existing)?;
            let outcome = if existing.transition == transition && existing.document == document {
                Ok(PreferenceDocumentCommitOutcome::AlreadyCommitted {
                    document: existing.document,
                })
            } else {
                Err(transition_reuse_error(&existing, &transition))
            };
            return self.rollback_with_identity(transaction, outcome).await;
        }
        if document.state() != transition.committed_next() {
            return self
                .rollback_with_identity(
                    transaction,
                    Err(PreferenceCasError::CommittedDocumentStateMismatch {
                        expected: transition.committed_next().clone(),
                        attempted: document.state().clone(),
                    }),
                )
                .await;
        }
        if let Some(existing) = fetch_transition_by(
            &mut transaction,
            "evidence_id",
            transition.evidence().id().as_str(),
        )
        .await?
        {
            let existing = decode_transition_row(&self.database, existing)?;
            let evidence_use = EvidenceUse {
                evidence: existing.transition.evidence().clone(),
                transition: existing.transition.id().clone(),
            };
            return self
                .rollback_with_identity(
                    transaction,
                    Err(evidence_reuse_error(&evidence_use, &transition)),
                )
                .await;
        }
        if let Some(existing) = fetch_transition_by(
            &mut transaction,
            "receipt_id",
            transition.caused_by().id().as_str(),
        )
        .await?
        {
            let existing = decode_transition_row(&self.database, existing)?;
            let receipt_use = ReceiptUse {
                receipt: existing.transition.caused_by().clone(),
                transition: existing.transition.id().clone(),
            };
            return self
                .rollback_with_identity(
                    transaction,
                    Err(PreferenceCasError::ReceiptReuseConflict {
                        receipt: transition.caused_by().id().clone(),
                        existing_receipt: Box::new(receipt_use.receipt),
                        attempted_receipt: Box::new(transition.caused_by().clone()),
                        existing_transition: receipt_use.transition,
                        attempted_transition: transition.id().clone(),
                    }),
                )
                .await;
        }

        let key = PreferenceKey::from_transition(&transition);
        let Some(current_row) = fetch_document_row(
            &mut transaction,
            "hepta_v2_preference_heads",
            key.preference.as_str(),
            key.subject.as_str(),
        )
        .await?
        else {
            let genesis_exists = fetch_document_row(
                &mut transaction,
                "hepta_v2_preference_genesis",
                key.preference.as_str(),
                key.subject.as_str(),
            )
            .await?
            .is_some();
            let error = if genesis_exists {
                PreferenceCasError::Corrupt {
                    detail: format!(
                        "preference genesis {} for subject {} has no head projection",
                        key.preference, key.subject
                    ),
                }
            } else {
                PreferenceCasError::PreferenceDocumentNotInitialized {
                    preference: key.preference,
                    subject: key.subject,
                }
            };
            return self.rollback_with_identity(transaction, Err(error)).await;
        };
        let current = decode_document_row(&self.database, current_row, "preference head")?;
        if current.state() != transition.cas_expected_previous() {
            return self
                .rollback_with_identity(
                    transaction,
                    Err(PreferenceCasError::StateConflict {
                        preference: key.preference,
                        subject: key.subject,
                        expected: transition.cas_expected_previous().clone(),
                        actual: current.state().clone(),
                    }),
                )
                .await;
        }
        if current.reducer_version() != document.reducer_version() {
            return self
                .rollback_with_identity(
                    transaction,
                    Err(PreferenceCasError::ReducerVersionConflict {
                        existing: current.reducer_version().to_owned(),
                        attempted: document.reducer_version().to_owned(),
                    }),
                )
                .await;
        }

        let transition_wire = PreferenceTransitionRowWire {
            schema_version: PREFERENCE_ROW_SCHEMA_VERSION,
            transition: (&transition).into(),
            document: (&document).into(),
        };
        let transition_row = self
            .database
            .encode_canonical_row(&transition_wire)
            .map_err(map_durable_error)?;
        sqlx::query(
            "INSERT INTO hepta_v2_preference_transitions (
                transition_id,
                evidence_id,
                receipt_id,
                preference_id,
                subject_id,
                payload_json,
                storage_hash
             ) VALUES (?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(transition.id().as_str())
        .bind(transition.evidence().id().as_str())
        .bind(transition.caused_by().id().as_str())
        .bind(key.preference.as_str())
        .bind(key.subject.as_str())
        .bind(&transition_row.payload_json)
        .bind(&transition_row.storage_hash)
        .execute(&mut *transaction)
        .await
        .map_err(|error| {
            map_durable_error(DurableStorageError::persistence(
                "append durable preference transition",
                error,
            ))
        })?;

        let head_wire = PreferenceGenesisWire {
            schema_version: PREFERENCE_ROW_SCHEMA_VERSION,
            preference_id: key.preference.as_str().to_owned(),
            subject_id: key.subject.as_str().to_owned(),
            document: (&document).into(),
        };
        let head_row = self
            .database
            .encode_canonical_row(&head_wire)
            .map_err(map_durable_error)?;
        let updated = sqlx::query(
            "UPDATE hepta_v2_preference_heads
             SET payload_json = ?, storage_hash = ?
             WHERE preference_id = ? AND subject_id = ?",
        )
        .bind(&head_row.payload_json)
        .bind(&head_row.storage_hash)
        .bind(key.preference.as_str())
        .bind(key.subject.as_str())
        .execute(&mut *transaction)
        .await
        .map_err(|error| {
            map_durable_error(DurableStorageError::persistence(
                "update durable preference head",
                error,
            ))
        })?
        .rows_affected();
        if updated != 1 {
            return self
                .rollback_with_identity(
                    transaction,
                    Err(PreferenceCasError::Corrupt {
                        detail: format!(
                            "preference head update affected {updated} rows for {} and {}",
                            key.preference, key.subject
                        ),
                    }),
                )
                .await;
        }

        transaction.commit().await.map_err(|error| {
            map_durable_error(DurableStorageError::persistence(
                "commit preference CAS transaction",
                error,
            ))
        })?;
        self.validate_database_identity()?;
        Ok(PreferenceDocumentCommitOutcome::Committed { document })
    }

    async fn verify_recovery(&self) -> Result<(), PreferenceCasError> {
        self.validate_database_identity()?;
        let mut transaction = self.database.pool().begin().await.map_err(|error| {
            map_durable_error(DurableStorageError::persistence(
                "begin preference recovery snapshot",
                error,
            ))
        })?;
        let genesis_rows = sqlx::query(
            "SELECT preference_id, subject_id, payload_json, storage_hash
             FROM hepta_v2_preference_genesis
             ORDER BY preference_id, subject_id",
        )
        .fetch_all(&mut *transaction)
        .await
        .map_err(|error| {
            map_durable_error(DurableStorageError::persistence(
                "scan preference genesis during recovery",
                error,
            ))
        })?;
        let mut replayed = BTreeMap::new();
        for row in genesis_rows {
            let (key, document) =
                decode_keyed_document_row(&self.database, row, "preference genesis")?;
            if document.state().revision() != Revision::new(0) {
                return Err(PreferenceCasError::Corrupt {
                    detail: format!(
                        "preference genesis {} for subject {} has nonzero revision {}",
                        key.preference,
                        key.subject,
                        document.state().revision()
                    ),
                });
            }
            if replayed.insert(key.clone(), document).is_some() {
                return Err(PreferenceCasError::Corrupt {
                    detail: format!(
                        "duplicate preference genesis {} for subject {}",
                        key.preference, key.subject
                    ),
                });
            }
        }

        let transition_rows = sqlx::query(
            "SELECT
                sequence,
                transition_id,
                evidence_id,
                receipt_id,
                preference_id,
                subject_id,
                payload_json,
                storage_hash
             FROM hepta_v2_preference_transitions
             ORDER BY sequence",
        )
        .fetch_all(&mut *transaction)
        .await
        .map_err(|error| {
            map_durable_error(DurableStorageError::persistence(
                "replay preference transitions",
                error,
            ))
        })?;
        let mut transition_ids = BTreeSet::new();
        let mut evidence_ids = BTreeSet::new();
        let mut receipt_ids = BTreeSet::new();
        for row in transition_rows {
            let stored = decode_transition_row(&self.database, row)?;
            if !transition_ids.insert(stored.transition.id().clone())
                || !evidence_ids.insert(stored.transition.evidence().id().clone())
                || !receipt_ids.insert(stored.transition.caused_by().id().clone())
            {
                return Err(PreferenceCasError::Corrupt {
                    detail: format!(
                        "duplicate identity while replaying preference transition {}",
                        stored.transition.id()
                    ),
                });
            }
            let key = PreferenceKey::from_transition(&stored.transition);
            let Some(current) = replayed.get(&key) else {
                return Err(PreferenceCasError::Corrupt {
                    detail: format!(
                        "preference transition {} has no genesis for {} and {}",
                        stored.transition.id(),
                        key.preference,
                        key.subject
                    ),
                });
            };
            if current.state() != stored.transition.cas_expected_previous() {
                return Err(PreferenceCasError::Corrupt {
                    detail: format!(
                        "preference transition {} does not follow the replayed CAS head",
                        stored.transition.id()
                    ),
                });
            }
            if stored.document.state() != stored.transition.committed_next() {
                return Err(PreferenceCasError::Corrupt {
                    detail: format!(
                        "preference transition {} WAL document does not match its committed next state",
                        stored.transition.id()
                    ),
                });
            }
            if current.reducer_version() != stored.document.reducer_version() {
                return Err(PreferenceCasError::Corrupt {
                    detail: format!(
                        "preference transition {} changes reducer version during replay",
                        stored.transition.id()
                    ),
                });
            }
            replayed.insert(key, stored.document);
        }

        let head_rows = sqlx::query(
            "SELECT preference_id, subject_id, payload_json, storage_hash
             FROM hepta_v2_preference_heads
             ORDER BY preference_id, subject_id",
        )
        .fetch_all(&mut *transaction)
        .await
        .map_err(|error| {
            map_durable_error(DurableStorageError::persistence(
                "scan preference heads during recovery",
                error,
            ))
        })?;
        let mut heads = BTreeMap::new();
        for row in head_rows {
            let (key, document) =
                decode_keyed_document_row(&self.database, row, "preference head")?;
            if heads.insert(key.clone(), document).is_some() {
                return Err(PreferenceCasError::Corrupt {
                    detail: format!(
                        "duplicate preference head {} for subject {}",
                        key.preference, key.subject
                    ),
                });
            }
        }
        if heads != replayed {
            return Err(PreferenceCasError::Corrupt {
                detail: "preference head projection does not match immutable WAL replay".into(),
            });
        }
        transaction.commit().await.map_err(|error| {
            map_durable_error(DurableStorageError::persistence(
                "close preference recovery snapshot",
                error,
            ))
        })?;
        self.validate_database_identity()?;
        Ok(())
    }

    fn validate_database_identity(&self) -> Result<(), PreferenceCasError> {
        self.database.validate_identity().map_err(map_durable_error)
    }

    async fn rollback_with_identity<T>(
        &self,
        transaction: Transaction<'_, Sqlite>,
        outcome: Result<T, PreferenceCasError>,
    ) -> Result<T, PreferenceCasError> {
        transaction.rollback().await.map_err(|error| {
            map_durable_error(DurableStorageError::persistence(
                "rollback preference transaction",
                error,
            ))
        })?;
        self.validate_database_identity()?;
        outcome
    }
}

async fn fetch_document_row(
    transaction: &mut Transaction<'_, Sqlite>,
    table: &str,
    preference_id: &str,
    subject_id: &str,
) -> Result<Option<sqlx::sqlite::SqliteRow>, PreferenceCasError> {
    let query = match table {
        "hepta_v2_preference_genesis" => {
            "SELECT preference_id, subject_id, payload_json, storage_hash
             FROM hepta_v2_preference_genesis
             WHERE preference_id = ? AND subject_id = ?"
        }
        "hepta_v2_preference_heads" => {
            "SELECT preference_id, subject_id, payload_json, storage_hash
             FROM hepta_v2_preference_heads
             WHERE preference_id = ? AND subject_id = ?"
        }
        _ => {
            return Err(PreferenceCasError::Corrupt {
                detail: format!("unsupported durable preference table {table}"),
            });
        }
    };
    sqlx::query(query)
        .bind(preference_id)
        .bind(subject_id)
        .fetch_optional(&mut **transaction)
        .await
        .map_err(|error| {
            map_durable_error(DurableStorageError::persistence(
                "read preference document row",
                error,
            ))
        })
}

async fn insert_document_row(
    transaction: &mut Transaction<'_, Sqlite>,
    table: &str,
    key: &PreferenceKey,
    payload_json: &str,
    storage_hash: &str,
) -> Result<(), PreferenceCasError> {
    let query = match table {
        "hepta_v2_preference_genesis" => {
            "INSERT INTO hepta_v2_preference_genesis (
                preference_id, subject_id, payload_json, storage_hash
             ) VALUES (?, ?, ?, ?)"
        }
        "hepta_v2_preference_heads" => {
            "INSERT INTO hepta_v2_preference_heads (
                preference_id, subject_id, payload_json, storage_hash
             ) VALUES (?, ?, ?, ?)"
        }
        _ => {
            return Err(PreferenceCasError::Corrupt {
                detail: format!("unsupported durable preference table {table}"),
            });
        }
    };
    sqlx::query(query)
        .bind(key.preference.as_str())
        .bind(key.subject.as_str())
        .bind(payload_json)
        .bind(storage_hash)
        .execute(&mut **transaction)
        .await
        .map_err(|error| {
            map_durable_error(DurableStorageError::persistence(
                "insert preference document row",
                error,
            ))
        })?;
    Ok(())
}

async fn fetch_transition_by(
    transaction: &mut Transaction<'_, Sqlite>,
    column: &str,
    value: &str,
) -> Result<Option<sqlx::sqlite::SqliteRow>, PreferenceCasError> {
    let query = match column {
        "transition_id" => {
            "SELECT
                sequence, transition_id, evidence_id, receipt_id,
                preference_id, subject_id, payload_json, storage_hash
             FROM hepta_v2_preference_transitions
             WHERE transition_id = ?"
        }
        "evidence_id" => {
            "SELECT
                sequence, transition_id, evidence_id, receipt_id,
                preference_id, subject_id, payload_json, storage_hash
             FROM hepta_v2_preference_transitions
             WHERE evidence_id = ?"
        }
        "receipt_id" => {
            "SELECT
                sequence, transition_id, evidence_id, receipt_id,
                preference_id, subject_id, payload_json, storage_hash
             FROM hepta_v2_preference_transitions
             WHERE receipt_id = ?"
        }
        _ => {
            return Err(PreferenceCasError::Corrupt {
                detail: format!("unsupported preference transition index {column}"),
            });
        }
    };
    sqlx::query(query)
        .bind(value)
        .fetch_optional(&mut **transaction)
        .await
        .map_err(|error| {
            map_durable_error(DurableStorageError::persistence(
                "read indexed preference transition",
                error,
            ))
        })
}
