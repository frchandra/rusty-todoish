-- Enable extension if not already enabled (required for UUID generation)
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create table
CREATE TABLE IF NOT EXISTS notes
(
    id           UUID PRIMARY KEY      DEFAULT uuid_generate_v4(),
    title        VARCHAR(255) NOT NULL UNIQUE,
    content      TEXT         NOT NULL,
    is_published BOOLEAN      NOT NULL DEFAULT FALSE,
    created_at   TIMESTAMPTZ  NOT NULL DEFAULT now(),
    updated_at   TIMESTAMPTZ  NOT NULL DEFAULT now()
);

-- Hash index for equality lookups on id
CREATE INDEX IF NOT EXISTS idx_notes_id_hash
    ON notes USING HASH (id);

-- Trigger function
CREATE OR REPLACE FUNCTION set_updated_at()
    RETURNS TRIGGER AS
$$
BEGIN
    NEW.updated_at = now();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger
DROP TRIGGER IF EXISTS trg_set_updated_at ON notes;

CREATE TRIGGER trg_set_updated_at
    BEFORE UPDATE
    ON notes
    FOR EACH ROW
EXECUTE FUNCTION set_updated_at();
