use hepta_contracts::PreferenceId;
use hepta_contracts::PrincipalId;
use sqlx::Row;

use super::EvidencedTransitionUse;
use super::PREFERENCE_ROW_SCHEMA_VERSION;
use super::PreferenceCasError;
use super::PreferenceGenesisWire;
use super::PreferenceKey;
use super::PreferenceStateDocument;
use super::PreferenceTransitionRowWire;
use super::map_durable_error;
use crate::durable::DurableDatabase;
use crate::durable::DurableStorageError;

pub(super) fn decode_document_row(
    database: &DurableDatabase,
    row: sqlx::sqlite::SqliteRow,
    row_kind: &str,
) -> Result<PreferenceStateDocument, PreferenceCasError> {
    decode_keyed_document_row(database, row, row_kind).map(|(_, document)| document)
}

pub(super) fn decode_keyed_document_row(
    database: &DurableDatabase,
    row: sqlx::sqlite::SqliteRow,
    row_kind: &str,
) -> Result<(PreferenceKey, PreferenceStateDocument), PreferenceCasError> {
    let preference_id: String = row.try_get("preference_id").map_err(|error| {
        map_durable_error(DurableStorageError::persistence(
            "decode preference identity column",
            error,
        ))
    })?;
    let subject_id: String = row.try_get("subject_id").map_err(|error| {
        map_durable_error(DurableStorageError::persistence(
            "decode preference subject column",
            error,
        ))
    })?;
    let payload_json: String = row.try_get("payload_json").map_err(|error| {
        map_durable_error(DurableStorageError::persistence(
            "decode preference payload column",
            error,
        ))
    })?;
    let storage_hash: String = row.try_get("storage_hash").map_err(|error| {
        map_durable_error(DurableStorageError::persistence(
            "decode preference storage hash column",
            error,
        ))
    })?;
    let wire: PreferenceGenesisWire = database
        .decode_canonical_row(&payload_json, &storage_hash, row_kind)
        .map_err(map_durable_error)?;
    if wire.schema_version != PREFERENCE_ROW_SCHEMA_VERSION {
        return Err(PreferenceCasError::Corrupt {
            detail: format!(
                "unsupported {row_kind} schema version {}",
                wire.schema_version
            ),
        });
    }
    if wire.preference_id != preference_id || wire.subject_id != subject_id {
        return Err(PreferenceCasError::Corrupt {
            detail: format!("{row_kind} indexed identity disagrees with canonical payload"),
        });
    }
    Ok((
        PreferenceKey::new(
            PreferenceId::new(preference_id),
            PrincipalId::new(subject_id),
        ),
        wire.document.into_contract(),
    ))
}

pub(super) fn decode_transition_row(
    database: &DurableDatabase,
    row: sqlx::sqlite::SqliteRow,
) -> Result<EvidencedTransitionUse, PreferenceCasError> {
    let transition_id: String = row.try_get("transition_id").map_err(|error| {
        map_durable_error(DurableStorageError::persistence(
            "decode transition identity column",
            error,
        ))
    })?;
    let evidence_id: String = row.try_get("evidence_id").map_err(|error| {
        map_durable_error(DurableStorageError::persistence(
            "decode evidence identity column",
            error,
        ))
    })?;
    let receipt_id: String = row.try_get("receipt_id").map_err(|error| {
        map_durable_error(DurableStorageError::persistence(
            "decode transition receipt column",
            error,
        ))
    })?;
    let preference_id: String = row.try_get("preference_id").map_err(|error| {
        map_durable_error(DurableStorageError::persistence(
            "decode transition preference column",
            error,
        ))
    })?;
    let subject_id: String = row.try_get("subject_id").map_err(|error| {
        map_durable_error(DurableStorageError::persistence(
            "decode transition subject column",
            error,
        ))
    })?;
    let payload_json: String = row.try_get("payload_json").map_err(|error| {
        map_durable_error(DurableStorageError::persistence(
            "decode transition payload column",
            error,
        ))
    })?;
    let storage_hash: String = row.try_get("storage_hash").map_err(|error| {
        map_durable_error(DurableStorageError::persistence(
            "decode transition storage hash column",
            error,
        ))
    })?;
    let wire: PreferenceTransitionRowWire = database
        .decode_canonical_row(&payload_json, &storage_hash, "preference transition")
        .map_err(map_durable_error)?;
    if wire.schema_version != PREFERENCE_ROW_SCHEMA_VERSION {
        return Err(PreferenceCasError::Corrupt {
            detail: format!(
                "unsupported preference transition schema version {}",
                wire.schema_version
            ),
        });
    }
    let transition = wire.transition.into_contract().map_err(map_durable_error)?;
    let document = wire.document.into_contract();
    if document.state() != transition.committed_next() {
        return Err(PreferenceCasError::Corrupt {
            detail: format!(
                "preference transition {} document does not match committed next state",
                transition.id()
            ),
        });
    }
    if transition.id().as_str() != transition_id
        || transition.evidence().id().as_str() != evidence_id
        || transition.caused_by().id().as_str() != receipt_id
        || transition.preference().as_str() != preference_id
        || transition.subject().as_str() != subject_id
    {
        return Err(PreferenceCasError::Corrupt {
            detail: format!(
                "preference transition {} indexed bindings disagree with canonical payload",
                transition.id()
            ),
        });
    }
    Ok(EvidencedTransitionUse {
        transition,
        document,
    })
}
