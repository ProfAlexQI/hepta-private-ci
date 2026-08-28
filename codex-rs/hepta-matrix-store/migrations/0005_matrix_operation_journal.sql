CREATE TABLE matrix_operations (
    event_id TEXT PRIMARY KEY,
    operation_id TEXT NOT NULL UNIQUE,
    idempotency_key TEXT NOT NULL UNIQUE,
    binding_sha256 TEXT NOT NULL CHECK (
        length(binding_sha256) = 64 AND binding_sha256 NOT GLOB '*[^0-9a-f]*'
    ),
    authority_epoch INTEGER NOT NULL CHECK (authority_epoch > 0),
    owner_epoch INTEGER NOT NULL CHECK (owner_epoch > 0),
    generation INTEGER NOT NULL CHECK (generation > 0),
    fencing_token_sha256 TEXT NOT NULL CHECK (
        length(fencing_token_sha256) = 64
        AND fencing_token_sha256 NOT GLOB '*[^0-9a-f]*'
    ),
    command_sha256 TEXT NOT NULL CHECK (
        length(command_sha256) = 64 AND command_sha256 NOT GLOB '*[^0-9a-f]*'
    ),
    command_bytes INTEGER NOT NULL CHECK (command_bytes > 0 AND command_bytes <= 1048576),
    sequence INTEGER NOT NULL CHECK (sequence > 0),
    phase TEXT NOT NULL CHECK (
        phase IN (
            'outbox_pending',
            'delivery_claimed',
            'acknowledged',
            'indeterminate',
            'reconciled_applied',
            'reconciled_not_applied',
            'quarantined'
        )
    ),
    destination_receipt_sha256 TEXT CHECK (
        destination_receipt_sha256 IS NULL OR (
            length(destination_receipt_sha256) = 64
            AND destination_receipt_sha256 NOT GLOB '*[^0-9a-f]*'
        )
    ),
    updated_at_ms INTEGER NOT NULL CHECK (updated_at_ms >= 0),
    FOREIGN KEY (event_id) REFERENCES inbox_events(event_id) ON DELETE RESTRICT,
    CHECK (
        (phase IN ('acknowledged', 'reconciled_applied')
            AND destination_receipt_sha256 IS NOT NULL)
        OR
        (phase NOT IN ('acknowledged', 'reconciled_applied')
            AND destination_receipt_sha256 IS NULL)
    )
) STRICT;

CREATE INDEX matrix_operations_phase_order
ON matrix_operations(phase, updated_at_ms, event_id);

CREATE TRIGGER matrix_operations_immutable_binding
BEFORE UPDATE ON matrix_operations
WHEN NEW.event_id != OLD.event_id
  OR NEW.operation_id != OLD.operation_id
  OR NEW.idempotency_key != OLD.idempotency_key
  OR NEW.binding_sha256 != OLD.binding_sha256
  OR NEW.authority_epoch != OLD.authority_epoch
  OR NEW.owner_epoch != OLD.owner_epoch
  OR NEW.generation != OLD.generation
  OR NEW.fencing_token_sha256 != OLD.fencing_token_sha256
  OR NEW.command_sha256 != OLD.command_sha256
  OR NEW.command_bytes != OLD.command_bytes
  OR NEW.sequence != OLD.sequence
BEGIN
    SELECT RAISE(ABORT, 'Matrix operation binding is immutable');
END;

CREATE TRIGGER matrix_operations_no_delete
BEFORE DELETE ON matrix_operations BEGIN
    SELECT RAISE(ABORT, 'Matrix operation journal is append-preserving');
END;
