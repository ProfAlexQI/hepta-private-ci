-- Bind every post-v4 queue admission ambiguity to the exact common
-- Operation kernel identity. Existing rows remain NULL and therefore
-- require explicit operator/provider reconciliation.
ALTER TABLE automation_dispatch_outcomes ADD COLUMN operation_id TEXT;
ALTER TABLE automation_dispatch_outcomes ADD COLUMN operation_binding_sha256 TEXT;
ALTER TABLE automation_dispatch_outcomes ADD COLUMN operation_sequence INTEGER;

CREATE UNIQUE INDEX automation_dispatch_operation_id_idx
    ON automation_dispatch_outcomes(operation_id)
    WHERE operation_id IS NOT NULL;

CREATE TRIGGER automation_dispatch_operation_insert_guard
BEFORE INSERT ON automation_dispatch_outcomes
WHEN ((NEW.operation_id IS NULL) != (NEW.operation_binding_sha256 IS NULL))
  OR ((NEW.operation_id IS NULL) != (NEW.operation_sequence IS NULL))
  OR (NEW.operation_sequence IS NOT NULL AND NEW.operation_sequence <= 0)
  OR (NEW.operation_binding_sha256 IS NOT NULL
      AND (length(NEW.operation_binding_sha256) != 64
           OR NEW.operation_binding_sha256 GLOB '*[^0-9a-f]*'))
BEGIN
    SELECT RAISE(ABORT, 'invalid automation operation identity');
END;

CREATE TRIGGER automation_dispatch_operation_update_guard
BEFORE UPDATE OF operation_id, operation_binding_sha256, operation_sequence
ON automation_dispatch_outcomes
WHEN OLD.operation_id IS NOT NEW.operation_id
  OR OLD.operation_binding_sha256 IS NOT NEW.operation_binding_sha256
  OR OLD.operation_sequence IS NOT NEW.operation_sequence
BEGIN
    SELECT RAISE(ABORT, 'automation operation identity is immutable');
END;

DROP TRIGGER automation_meta_no_update;
UPDATE automation_meta SET schema_version = 4 WHERE singleton = 1;
CREATE TRIGGER automation_meta_no_update
BEFORE UPDATE ON automation_meta
BEGIN
    SELECT RAISE(ABORT, 'automation owner metadata is immutable');
END;
