# Skills

What the agent needs to know how to do in this repo, and where to find canonical references.

## Tauri 2

- Docs: https://v2.tauri.app
- Initialise: `pnpm tauri init`
- Dev: `pnpm tauri dev`
- Build: `pnpm tauri build`
- Config: `src-tauri/tauri.conf.json`
- Capabilities (permissions): `src-tauri/capabilities/`

**Common patterns:**
- IPC command in Rust → exposed via `#[tauri::command]` → invoked from JS via `invoke('command_name', { args })`
- Background tasks: `tauri::async_runtime::spawn`
- Window management via `tauri::WebviewWindow`
- Event emission Rust → JS: `app.emit("session-status", payload)` ; on JS side `listen('session-status', cb)`

## Global hotkey

- Plugin: `tauri-plugin-global-shortcut`
- Register from Rust on app setup; emit a window event when triggered; JS handler decides what to do
- Default: `CmdOrCtrl+Alt+R` for record-toggle, `CmdOrCtrl+Alt+L` for library

## Screen + audio capture

- Crate: `scap` (https://github.com/CapSoftware/scap)
- macOS: wraps ScreenCaptureKit; captures display, window, or area with system audio in one stream
- API outline:
  ```rust
  let opts = Options { fps: 30, target: Target::Window(window), output_type: FrameType::BGRAFrame, show_cursor: true, ... };
  let capturer = Capturer::new(opts);
  capturer.start_capture();
  // ... pull frames from rx ...
  capturer.stop_capture();
  ```
- Save raw to a `.mp4` via ffmpeg pipe; do NOT keep frames in RAM for long recordings

## ffmpeg sidecar

- Tauri 2 sidecar mechanism: declare in `tauri.conf.json` → `bundle.externalBin`
- Binaries: `src-tauri/binaries/ffmpeg-{target-triple}` (e.g. `ffmpeg-aarch64-apple-darwin`)
- Invocation via `tauri-plugin-shell`:
  ```rust
  let (mut rx, child) = app.shell().sidecar("ffmpeg")?.args(["-i", input, ...]).spawn()?;
  ```
- Audio extract: `ffmpeg -i recording.mp4 -vn -ac 1 -ar 16000 -c:a pcm_s16le audio.wav`
- Frame sample (scene + interval): `ffmpeg -i recording.mp4 -vf "select='gt(scene,0.4)+not(mod(n,150))',scale=1280:-2" -vsync vfr frames/%06d.jpg`

## SQLite + sqlite-vec

- Plugin: `tauri-plugin-sql` for basic SQLite
- Extension loading is NOT supported by the plugin out of the box. Load `sqlite-vec` via a Rust-side `rusqlite::Connection::load_extension` call before any vector queries, OR use `rusqlite` directly and skip the plugin
- Crate: `sqlite-vec` (https://github.com/asg017/sqlite-vec)
- Pattern:
  ```rust
  unsafe { sqlite_vec::sqlite3_vec_init(conn.handle(), ...); }
  // then queries work normally
  ```

## Anthropic API

- SDK: `@anthropic-ai/sdk` (TypeScript)
- Models in use:
  - `claude-haiku-4-5-20251001` — titles and tags
  - `claude-sonnet-4-6` — per-window vision and synthesis, RAG answers
  - `claude-opus-4-7` — final document synthesis (one call per session)
- Vision: pass image content blocks with base64-encoded JPEG (`media_type: "image/jpeg"`). Pre-resize to ≤2576px long edge for current Claude 4.x models (Sonnet 4.6 vision, Opus 4.7), ≤1568px for older models.
- Use **prompt caching** on the RAG path — system prompt + tools cached, only the retrieved chunks and user question vary
- Use **Message Batches API** for per-window vision calls — not latency-critical, ~50% cheaper
- Use **extended thinking** only on the final Opus synthesis call

Docs:
- Overview: https://docs.claude.com/en/api/overview
- Vision: https://platform.claude.com/docs/en/build-with-claude/vision
- Prompt caching: https://platform.claude.com/docs/en/build-with-claude/prompt-caching
- Batches: https://platform.claude.com/docs/en/build-with-claude/batch-processing

## Deepgram

- SDK: `@deepgram/sdk` (TypeScript)
- Model: `nova-3`
- Features: diarisation, smart formatting, paragraph segmentation
- Output: JSON with word-level timestamps and speaker labels
- Docs: https://developers.deepgram.com

## Voyage AI

- No official SDK in active maintenance — use `fetch` directly against `https://api.voyageai.com/v1/embeddings`
- Model: `voyage-3-large`
- Returns 1024-dim float vectors
- Batch up to 128 chunks per call
- Docs: https://docs.voyageai.com

## shadcn/ui

- Installed components live in `components/ui/`
- Add new ones with `pnpm dlx shadcn@latest add {component}`
- Theme: Tailwind config + CSS variables (light/dark, single accent in deep teal `#0F6E56`)

## Logging

- Rust: `tracing` + `tracing-subscriber` writing to `~/Library/Application Support/Lume/logs/lume.log`
- TS: a thin wrapper around `console` in dev, no-op in release, plus optional Rust IPC for persistent logs
- Log levels: `error`, `warn`, `info`, `debug`, `trace`
- Never log API keys, recording paths beyond the session id, or transcript content

## Testing

- Rust: `cargo test`
- TS unit: vitest (`pnpm test`)
- E2E: deferred to Week 5+; will use `tauri-driver` once ready

## Build artifacts

- macOS: `src-tauri/target/release/bundle/macos/Lume.app` and `.dmg` in `bundle/dmg/`
- Signed + notarised version requires Apple Developer ID — deferred to Week 6
