-- Create ENUM types
CREATE TYPE "relation_status" AS ENUM ('pending', 'approved', 'rejected');
CREATE TYPE "relation_type_enum" AS ENUM ('reference', 'remix', 'annotation', 'translation', 'reaction');

-- Create relations table
CREATE TABLE IF NOT EXISTS "relations" (
    "id" UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    "source_video_id" VARCHAR(255) NOT NULL,
    "source_start_time" INTEGER NOT NULL,
    "source_end_time" INTEGER NOT NULL,
    "target_video_id" VARCHAR(255) NOT NULL,
    "target_start_time" INTEGER NOT NULL,
    "target_end_time" INTEGER NOT NULL,
    "relation_type" relation_type_enum NOT NULL DEFAULT 'reference',
    "created_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    "updated_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    "status" relation_status NOT NULL DEFAULT 'pending',
    "user_id" UUID REFERENCES "users"("id")
);

-- Create indexes for better performance
CREATE INDEX IF NOT EXISTS "idx_relations_source_video_id" ON "relations"("source_video_id");
CREATE INDEX IF NOT EXISTS "idx_relations_target_video_id" ON "relations"("target_video_id");
CREATE INDEX IF NOT EXISTS "idx_relations_created_at" ON "relations"("created_at");
CREATE INDEX IF NOT EXISTS "idx_relations_status" ON "relations"("status");
CREATE INDEX IF NOT EXISTS "idx_relations_user_id" ON "relations"("user_id");

-- Create updated_at trigger function
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW."updated_at" = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Create trigger to automatically update updated_at
CREATE TRIGGER "update_relations_updated_at" 
    BEFORE UPDATE ON "relations" 
    FOR EACH ROW 
    EXECUTE FUNCTION update_updated_at_column(); 