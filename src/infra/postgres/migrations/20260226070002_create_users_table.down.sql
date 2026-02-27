-- Add down migration script here
DROP TRIGGER IF EXISTS trg_set_updated_at ON users;
DROP TABLE IF EXISTS users;