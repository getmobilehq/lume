# Data

## Database

Single SQLite file at `~/Library/Application Support/Lume/db.sqlite`. sqlite-vec loaded as an extension on connection.

### Schema

```sql
-- Sessions: one row per capture
CREATE TABLE sessions (
  id              TEXT PRIMARY KEY,           -- ulid
  title           TEXT NOT NULL,
  source_url      TEXT,                       -- optional, v1.1
  recorded_at     TEXT NOT NULL,              -- iso8601
  duration_s      INTEGER NOT NULL,
  recording_path  TEXT NOT NULL,
  transcript_path TEXT NOT NULL,
  document_path   TEXT NOT NULL,
  status          TEXT NOT NULL,              -- recording | processing | ready | error
  error_message   TEXT,
  error_stage     TEXT,                       -- which pipeline stage failed
  cost_usd        REAL,
  created_at      TEXT NOT NULL,
  updated_at      TEXT NOT NULL
);

CREATE INDEX idx_sessions_recorded_at ON sessions(recorded_at DESC);
CREATE INDEX idx_sessions_status ON sessions(status);

-- Chunks: H2 sections of the synthesised document
CREATE TABLE chunks (
  id            TEXT PRIMARY KEY,
  session_id    TEXT NOT NULL REFERENCES sessions(id) ON DELETE CASCADE,
  section_title TEXT NOT NULL,
  body          TEXT NOT NULL,
  start_ts_s    REAL NOT NULL,
  end_ts_s      REAL NOT NULL,
  position      INTEGER NOT NULL
);

CREATE INDEX idx_chunks_session ON chunks(session_id);

-- Vector index for chunks (sqlite-vec virtual table)
CREATE VIRTUAL TABLE chunk_vec USING vec0(
  chunk_id   TEXT PRIMARY KEY,
  embedding  FLOAT[1024]
);

-- Tags: many-to-many between sessions and tag strings
CREATE TABLE tags (
  session_id TEXT NOT NULL REFERENCES sessions(id) ON DELETE CASCADE,
  tag        TEXT NOT NULL,
  PRIMARY KEY (session_id, tag)
);

CREATE INDEX idx_tags_tag ON tags(tag);

-- Settings: simple kv store for non-secret app state
-- (secrets live in Keychain via stronghold, never here)
CREATE TABLE settings (
  key   TEXT PRIMARY KEY,
  value TEXT NOT NULL
);
```

## On-disk artefacts

```
~/Library/Application Support/Lume/
├── db.sqlite
├── sessions/
│   └── {session_id}/
│       ├── recording.mp4         # H.264 + AAC, 30fps, 1280×720 typical
│       ├── audio.wav             # 16kHz mono PCM, deleted after transcribe (configurable)
│       ├── transcript.json       # Deepgram raw response
│       ├── frames/
│       │   ├── 000045.jpg        # filename = timestamp in seconds × 1000, zero-padded
│       │   └── 000093.jpg
│       ├── windows.json          # array of P1 (per-window) outputs
│       └── document.md           # final synthesised doc
└── logs/
    └── lume.log
```

## Sample fixtures

For testing the pipeline end-to-end without a real capture.

### `fixtures/sample-transcript.json` (Deepgram shape, abbreviated)

```json
{
  "metadata": { "duration": 305.4 },
  "results": {
    "channels": [{
      "alternatives": [{
        "transcript": "...",
        "words": [
          { "word": "we", "start": 12.34, "end": 12.5, "speaker": 0 },
          { "word": "discussed", "start": 12.5, "end": 13.1, "speaker": 0 }
        ],
        "paragraphs": {
          "transcript": "...",
          "paragraphs": [
            { "speaker": 0, "start": 12.34, "end": 47.8, "sentences": [...] }
          ]
        }
      }]
    }]
  }
}
```

### `fixtures/sample-window-output.json` (P1 output)

```json
{
  "section_title": "Agent-native processes and tooling",
  "summary": "The speaker contrasts traditional BPM tools with emerging agent-native platforms. They argue that Level 6 of process maturity requires living, executable processes that an agent can modify. Slide visible at 14:22 shows the maturity ladder with Level 6 highlighted.",
  "key_points": [
    "Level 6 processes can be modified by agents at runtime",
    "Celonis and ARIS remain stuck at Level 4",
    "Vendor lock-in is a real risk in agent-native platforms"
  ],
  "notable_quotes": [
    { "speaker": "Jamie", "text": "The killer feature isn't automation, it's mutation.", "timestamp_s": 873.2 }
  ],
  "visual_highlights": [
    { "description": "Process maturity ladder with Level 6 highlighted in green", "timestamp_s": 862.0 }
  ],
  "entities": ["Celonis", "ARIS", "Monday.com"],
  "open_questions": [
    "How do you version control a process that the agent rewrites?"
  ]
}
```

### `fixtures/sample-document.md` (P2 output)

```markdown
---
title: Agent-native processes and the level-6 gap
recorded_at: 2026-05-22T14:00:00Z
duration: 00:48:12
tags: [agent-native, bpm, process-maturity, monday-com]
speakers: [Jamie, Marv]
---

# Agent-native processes and the level-6 gap

## TL;DR
- Level 6 of process maturity — where agents can mutate processes at runtime — is the next strategic gap
- Existing BPM vendors (Celonis, ARIS) plateau at Level 4 and require redesign, not patching
- Monday.com's emerging agent capabilities are early but architecturally on the right side

## Topics covered

### Process maturity ladder
Jamie walks through six levels of process maturity, dwelling on the gap between Level 5 (executable) and Level 6 (mutable). The maturity ladder slide [14:22] makes the point cleanly: …

### Vendor positioning
…

## Notable quotes
- > "The killer feature isn't automation, it's mutation." — Jamie, [14:33]

## Visual highlights
- Process maturity ladder with Level 6 highlighted [14:22]

## Entities and terms
- **Celonis** — process mining platform; cited as Level 4 ceiling
- **ARIS** — process modelling tool; same plateau as Celonis
- **Monday.com** — emerging Level 6 capabilities via native AI features

## Open questions
- How do you version control a process that the agent rewrites?
```

## Migrations

- All schema changes go through a numbered migration file in `src-tauri/migrations/`
- Migrations run on startup; current version stored in the `settings` table
- No destructive migrations in v1 — additive only

## Data deletion

User-initiated session deletion:
1. `DELETE FROM sessions WHERE id = ?` (cascades to chunks and tags)
2. `DELETE FROM chunk_vec WHERE chunk_id IN (?, ?, ?)` (no cascade in virtual tables)
3. `rm -rf sessions/{id}/`

There is no soft-delete in v1. Deletes are immediate and final.
