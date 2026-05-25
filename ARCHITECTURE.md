# Architecture

## System overview

```
                ┌─────────────────┐
                │  Global hotkey  │
                └────────┬────────┘
                         ↓
                ┌─────────────────┐
                │  Tauri capture  │ ← active browser window + system audio
                │  (Rust / scap)  │
                └────────┬────────┘
                         ↓
                ┌─────────────────┐
                │     ffmpeg      │ extract audio + sample frames
                └────────┬────────┘
              ┌──────────┴──────────┐
              ↓                     ↓
       ┌──────────────┐      ┌──────────────┐
       │   Deepgram   │      │ Claude vision│
       │     STT      │      │  (per-window)│
       └──────┬───────┘      └──────┬───────┘
              └──────────┬──────────┘
                         ↓
                ┌─────────────────┐
                │ Claude synthesis│ → markdown document
                │   (Opus 4.7)    │
                └────────┬────────┘
                         ↓
                ┌─────────────────┐
                │  Voyage embed   │
                └────────┬────────┘
                         ↓
                ┌─────────────────┐    ┌─────────────────┐
                │ SQLite +        │ ←→ │   Query chat    │
                │  sqlite-vec     │    │      UI         │
                └─────────────────┘    └─────────────────┘
```

## Layers

### 1. Shell (Tauri 2)

- Owns the OS-level surfaces: window, tray, hotkey, notifications, Keychain
- Single Rust process, multiple Tauri windows
- All filesystem and screen access lives here

### 2. Capture (Rust)

- `src-tauri/src/capture.rs`
- Uses `scap` to record the active browser window with system audio
- Writes H.264 MP4 to `~/Library/Application Support/Lume/sessions/{id}/recording.mp4`
- Emits `capture-status` events to JS

### 3. Pipeline (Rust orchestrator + TS service callers)

- `src-tauri/src/pipeline.rs` — orchestrates the stages, updates session status, persists artefacts
- For each stage, calls into a JS handler via Tauri command (because Anthropic / Deepgram / Voyage SDKs are TS) — except ffmpeg which stays in Rust
- Stage ordering:
  1. `audio_extract` (Rust → ffmpeg sidecar)
  2. `frames_sample` (Rust → ffmpeg sidecar)
  3. `transcribe` (TS → Deepgram)
  4. `vision_per_window` (TS → Claude Sonnet 4.6, batch)
  5. `synthesise` (TS → Claude Opus 4.7, extended thinking)
  6. `title_and_tags` (TS → Claude Haiku 4.5)
  7. `embed_and_index` (TS → Voyage, then SQLite write)

### 4. Storage (SQLite + sqlite-vec)

- `~/Library/Application Support/Lume/db.sqlite` — metadata, chunks, vectors
- `~/Library/Application Support/Lume/sessions/{id}/` — raw recording, audio, frames, transcript JSON, document MD, windows JSON
- See `DATA.md` for schema

### 5. UI (Next.js 14, static export)

- `app/(library)/` — library list + chat input at top
- `app/(session)/[id]/` — single session view (doc + transcript sidebar)
- `app/(settings)/` — keys, hotkey, output dir, model toggle

### 6. RAG query loop

- User question in chat input
- TS embeds the question with Voyage
- SQL query against `chunk_vec` returns top-5 chunks
- Pass chunks + question to Claude Sonnet 4.6 with prompt caching on the system prompt
- Response includes citations in the form `[Title, mm:ss](lume://session/{id}#t={s})`
- Click handler in UI intercepts `lume://` and routes to session view at timestamp

## Data flow on capture

```
1. User presses ⌃⌥R
2. Rust hotkey handler emits `record-toggle`
3. JS handler calls `invoke('start_capture')`
4. Rust: create session id (ulid), insert session row with status='recording', start scap
5. (user watches)
6. User presses ⌃⌥R again
7. JS calls `invoke('stop_capture')`
8. Rust: stop scap, update status='processing', spawn pipeline task
9. Pipeline runs stages 1-7 (above), updating session row at each stage
10. On completion: status='ready', emit notification
```

## Failure handling

- Each pipeline stage wraps in try/catch (TS) or `Result<_, LumeError>` (Rust)
- On failure: session row gets `status='error'`, `error_message` filled
- Library UI shows error state with a "Retry from stage X" button
- Partial artefacts retained — no need to re-record if vision fails partway through

## Inter-process boundaries

- **JS ↔ Rust:** Tauri `invoke` for commands, `emit`/`listen` for events
- **Rust ↔ ffmpeg:** sidecar via `tauri-plugin-shell`, stdin/stdout streams
- **JS ↔ external APIs:** direct HTTPS with SDKs, no Rust proxy
- **No HTTP between Tauri windows** — use Tauri events

## Static export constraints

Because Next.js runs as a static export inside Tauri:
- No `fetch` at server-render time — all data loads via `useEffect` on the client
- No API routes (`app/api/*`) — would not be served
- No middleware
- No `next/image` optimisation (set `images: { unoptimized: true }`)
- All routes pre-rendered at build; dynamic ones use client-side routing

## Performance notes

- Recording at 30fps for a 60-min session: ~3 GB MP4 at H.264 quality 23
- Frame sampling produces 100–250 JPEGs total (≤500 KB each)
- Vision batch: 10 frames + transcript fragment fits in one Sonnet 4.6 call comfortably
- Embedding pass: ~20 chunks × 1024 dim = trivial (<1ms in sqlite-vec)
- RAG query: top-5 retrieval + Claude call typically <2s

## Future architectural notes

Listed in `DECISIONS.md` as deferred:
- Local Whisper fallback (v1.2)
- Multi-modal embeddings (v2)
- Cross-session entity merging (v2)
- Team sync (v3)
