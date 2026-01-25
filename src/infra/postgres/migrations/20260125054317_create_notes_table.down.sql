-- Add down migration script here
DROP TRIGGER IF EXISTS trg_set_updated_at ON notes;
DROP FUNCTION IF EXISTS set_updated_at();
DROP TABLE IF EXISTS notes;