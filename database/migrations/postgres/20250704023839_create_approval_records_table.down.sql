-- Remove status from relations table
DROP INDEX IF EXISTS idx_relations_status;
ALTER TABLE relations DROP COLUMN IF EXISTS status;

-- Drop indexes first
DROP INDEX IF EXISTS "idx_approval_records_relation_id";
DROP INDEX IF EXISTS "idx_approval_records_event_type";
DROP INDEX IF EXISTS "idx_approval_records_operator_id";
DROP INDEX IF EXISTS "idx_approval_records_to_status";

-- Drop trigger
DROP TRIGGER IF EXISTS "update_approval_records_updated_at" ON "approval_records";

-- Drop table
DROP TABLE IF EXISTS "approval_records";

-- Drop ENUM type
DROP TYPE IF EXISTS "relation_event_type"; 