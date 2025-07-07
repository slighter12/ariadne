-- Create event type ENUM
CREATE TYPE "relation_event_type" AS ENUM ('create', 'approve', 'reject', 'auto_reject', 'update', 'delete');

-- Create approval/event log table
CREATE TABLE IF NOT EXISTS "approval_records" (
    "id" UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    "relation_id" UUID NOT NULL REFERENCES "relations"("id") ON DELETE CASCADE,
    "event_type" relation_event_type NOT NULL,
    "operator_id" UUID REFERENCES "users"("id"),
    "from_status" relation_status,
    "to_status" relation_status NOT NULL,
    "payload" JSONB,
    "created_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Create indexes for approval records
CREATE INDEX IF NOT EXISTS "idx_approval_records_relation_id" ON "approval_records"("relation_id");
CREATE INDEX IF NOT EXISTS "idx_approval_records_event_type" ON "approval_records"("event_type");
CREATE INDEX IF NOT EXISTS "idx_approval_records_operator_id" ON "approval_records"("operator_id");
CREATE INDEX IF NOT EXISTS "idx_approval_records_to_status" ON "approval_records"("to_status");

-- Create trigger for approval records updated_at
CREATE TRIGGER "update_approval_records_updated_at" 
    BEFORE UPDATE ON "approval_records" 
    FOR EACH ROW 
    EXECUTE FUNCTION update_updated_at_column();

-- Add status column to relations table for quick filtering
ALTER TABLE "relations" ADD COLUMN IF NOT EXISTS "status" VARCHAR(20) DEFAULT 'pending';
CREATE INDEX IF NOT EXISTS "idx_relations_status" ON "relations"("status"); 