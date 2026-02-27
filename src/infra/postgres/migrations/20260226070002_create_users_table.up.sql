-- Enable extension if not already enabled (required for UUID generation)
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- create table if not exists
CREATE TABLE IF NOT EXISTS users (
    id          UUID PRIMARY KEY      DEFAULT uuid_generate_v4(),
    name        VARCHAR(255) NOT NULL,
    email       VARCHAR(255) UNIQUE NOT NULL,
    password    VARCHAR(255) NOT NULL,
    is_admin     BOOLEAN      NOT NULL DEFAULT FALSE,
    created_at  TIMESTAMPTZ  NOT NULL DEFAULT now(),
    updated_at  TIMESTAMPTZ  NOT NULL DEFAULT now()
);

-- Hash index for equality lookups on id
CREATE INDEX IF NOT EXISTS idx_users_id_hash
    ON users USING HASH (id);

CREATE INDEX IF NOT EXISTS idx_users_name_hash
    ON users USING HASH (name);

CREATE INDEX IF NOT EXISTS idx_users_email_hash
    ON users USING HASH (email);


-- Trigger
DROP TRIGGER IF EXISTS trg_set_updated_at ON users;

CREATE TRIGGER trg_set_updated_at
    BEFORE UPDATE
    ON users
    FOR EACH ROW
EXECUTE FUNCTION set_updated_at();