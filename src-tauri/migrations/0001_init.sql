-- Migration 0001 — initial schema (see DATA.md). Additive only; idempotent.

CREATE TABLE IF NOT EXISTS sessions (
  id              TEXT PRIMARY KEY,
  title           TEXT NOT NULL,
  source_url      TEXT,
  recorded_at     TEXT NOT NULL,
  duration_s      INTEGER NOT NULL,
  recording_path  TEXT NOT NULL,
  transcript_path TEXT NOT NULL,
  document_path   TEXT NOT NULL,
  status          TEXT NOT NULL,
  error_message   TEXT,
  error_stage     TEXT,
  cost_usd        REAL,
  created_at      TEXT NOT NULL,
  updated_at      TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_sessions_recorded_at ON sessions(recorded_at DESC);
CREATE INDEX IF NOT EXISTS idx_sessions_status ON sessions(status);

CREATE TABLE IF NOT EXISTS chunks (
  id            TEXT PRIMARY KEY,
  session_id    TEXT NOT NULL REFERENCES sessions(id) ON DELETE CASCADE,
  section_title TEXT NOT NULL,
  body          TEXT NOT NULL,
  start_ts_s    REAL NOT NULL,
  end_ts_s      REAL NOT NULL,
  position      INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_chunks_session ON chunks(session_id);

-- Vector index for chunks (sqlite-vec virtual table).
CREATE VIRTUAL TABLE IF NOT EXISTS chunk_vec USING vec0(
  chunk_id  TEXT PRIMARY KEY,
  embedding FLOAT[1024]
);

CREATE TABLE IF NOT EXISTS tags (
  session_id TEXT NOT NULL REFERENCES sessions(id) ON DELETE CASCADE,
  tag        TEXT NOT NULL,
  PRIMARY KEY (session_id, tag)
);

CREATE INDEX IF NOT EXISTS idx_tags_tag ON tags(tag);

CREATE TABLE IF NOT EXISTS settings (
  key   TEXT PRIMARY KEY,
  value TEXT NOT NULL
);

INSERT OR IGNORE INTO settings(key, value) VALUES ('schema_version', '1');
