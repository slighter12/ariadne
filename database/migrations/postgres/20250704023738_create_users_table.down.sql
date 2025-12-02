-- Drop indexes first
DROP INDEX IF EXISTS "idx_users_email";
DROP INDEX IF EXISTS "idx_users_google_id";

-- Drop trigger
DROP TRIGGER IF EXISTS update_users_updated_at ON "users";

-- Drop table
DROP TABLE IF EXISTS "users";