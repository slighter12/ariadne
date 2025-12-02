-- Create relation_snapshots (聚合快照表)
CREATE TABLE IF NOT EXISTS "relation_snapshots" (
    "id" UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    "source_video_id" VARCHAR(255) NOT NULL,
    "source_start_time" INTEGER NOT NULL,
    "source_end_time" INTEGER NOT NULL,
    "target_video_id" VARCHAR(255) NOT NULL,
    "target_start_time" INTEGER NOT NULL,
    "target_end_time" INTEGER NOT NULL,
    "relation_type" relation_type_enum NOT NULL,
    "mark_count" INTEGER NOT NULL,
    "display_start_time" INTEGER NOT NULL,
    "display_end_time" INTEGER NOT NULL,
    "created_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    "updated_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS "idx_relation_snapshots_source_video_id" ON "relation_snapshots"("source_video_id");
CREATE INDEX IF NOT EXISTS "idx_relation_snapshots_target_video_id" ON "relation_snapshots"("target_video_id");