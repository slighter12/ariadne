-- Drop indexes first
DROP INDEX IF EXISTS "idx_relation_snapshots_source_video_id";
DROP INDEX IF EXISTS "idx_relation_snapshots_target_video_id";

-- Drop table
DROP TABLE IF EXISTS "relation_snapshots";