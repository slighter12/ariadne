-- Drop indexes first
DROP INDEX IF EXISTS "idx_relations_source_video_id";
DROP INDEX IF EXISTS "idx_relations_target_video_id";
DROP INDEX IF EXISTS "idx_relations_created_at";
DROP INDEX IF EXISTS "idx_relations_status";
DROP INDEX IF EXISTS "idx_relations_user_id";

-- Drop trigger and function
DROP TRIGGER IF EXISTS "update_relations_updated_at" ON "relations";
DROP FUNCTION IF EXISTS update_updated_at_column();

-- Drop table
DROP TABLE IF EXISTS "relations";

-- Drop ENUM types
DROP TYPE IF EXISTS "relation_status";
DROP TYPE IF EXISTS "relation_type_enum";