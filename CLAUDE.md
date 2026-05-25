# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Current state of the repo

This repo is **documentation-only right now** — the application has not been bootstrapped yet. The `.md` files are the authoritative spec; the actual Tauri/Next.js scaffold gets created by running the commands in `RUNBOOK.md §2`. The first task (`LOOP.md` → "Current task") is to run that bootstrap.

Read `AGENT.md` first — it is the operating contract for this repo and its invariants override anything inferred from code.

The docs form a layered spec. Consult them in this order:

| File | Use it for |
|---|---|
| `AGENT.md` | Invariants, code style, what you may not decide alone |
| `LOOP.md` | What to build right now; update it on task completion |
| `ARCHITECTURE.md` | System layers, pipeline stages, data flow |
| `DATA.md` | SQLite schema, on-disk layout, fixtures |
| `SKILLS.md` | Per-library API patterns (Tauri, scap, ffmpeg, sqlite-vec, Anthropic, Deepgram, Voyage) |
| `PROMPTS.md` | Versioned Claude prompts (vision / synthesis / RAG) → become `lib/prompts.ts` |
| `DECISIONS.md` | ADR log; append a new ADR for any non-trivial architectural choice |
| `RUNBOOK.md` | Setup, bootstrap commands, structure, gotchas, build |

## What Lume is

A single-user, local-first macOS desktop app. Press a global hotkey, record the active browser window + system audio, then a pipeline transcribes (Deepgram), analyses frames (Claude vision), synthesises a markdown document (Claude), embeds it (Voyage), and indexes it in SQLite + sqlite-vec for chat-based RAG query. Nothing leaves the machine except explicit API calls.

## Commands

```bash
pnpm tauri dev      # run the desktop app with HMR (Next.js dev server + Tauri window)
pnpm tauri build    # .app bundle → src-tauri/target/release/bundle/macos

# Finish-of-task checks (run all four before marking a LOOP.md task done):
pnpm lint
pnpm tsc --noEmit
cargo fmt            # in src-tauri/
cargo clippy --all-targets -- -D warnings   # in src-tauri/; warnings are errors

# Tests
pnpm test           # TS unit (vitest)
cargo test          # Rust (in src-tauri/)
```

First-time bootstrap of the scaffold: follow `RUNBOOK.md §2` verbatim.

## Architecture essentials

**Two-language split, one boundary.** Anything touching the filesystem, screen, or audio lives in **Rust** (`src-tauri/`). All external AI API calls (Anthropic, Deepgram, Voyage) live in **TypeScript** (`lib/`) for prompt-iteration DX. UI is **React** (`app/`, `components/`). The *only* Rust↔TS boundary is Tauri `invoke` commands + `emit`/`listen` events — never HTTP between layers.

**Capture → pipeline.** Hotkey toggles `start_capture`/`stop_capture` (Rust/scap). On stop, the Rust orchestrator (`pipeline.rs`) spawns a task running stages in order, updating the session row at each: `audio_extract` → `frames_sample` (both Rust/ffmpeg sidecar) → `transcribe` (Deepgram) → `vision_per_window` (Sonnet, batched) → `synthesise` (Opus, extended thinking) → `title_and_tags` (Haiku) → `embed_and_index` (Voyage → SQLite). A failed stage sets `status='error'` + `error_stage` and retains partial artefacts so the user can retry from a stage without re-recording.

**Storage.** One SQLite file at `~/Library/Application Support/Lume/db.sqlite` (metadata, chunks, `chunk_vec` virtual table via sqlite-vec). Per-session artefacts (recording.mp4, audio.wav, frames/, transcript.json, windows.json, document.md) under `sessions/{id}/`. Schema and deletion order are in `DATA.md` — note `chunk_vec` is a virtual table and does **not** cascade on delete; delete its rows explicitly.

**RAG query.** Embed question (Voyage) → top-5 from `chunk_vec` → Claude Sonnet with prompt caching on the system prompt → answer cites `[Title, mm:ss](lume://session/{id}#t={s})`; the UI intercepts `lume://` links and routes to the session at that timestamp.

## Constraints that bite

- **Static export.** Next.js runs as `output: 'export'` inside Tauri. No `app/api/*` routes, no middleware, no `fetch` at render time, no `next/image` optimisation (`images: { unoptimized: true }`). All dynamic data flows through Tauri commands loaded client-side (`useEffect`).
- **Secrets in Keychain only.** API keys go through `tauri-plugin-stronghold`. Never write a key to `.env`, logs, or settings JSON at runtime. The only `.env` that exists is `.env.example`.
- **sqlite-vec extension** is not loadable via `tauri-plugin-sql` directly — load it Rust-side in `db.rs` init before any query (see `SKILLS.md`).
- **ffmpeg** ships as a sidecar binary at `src-tauri/binaries/ffmpeg-{target-triple}`, invoked via `tauri-plugin-shell`.
- **Frame resize for vision:** ≤2576px long edge for current Claude 4.x models (Sonnet 4.6 vision, Opus 4.7), ≤1568px for older models, before base64 send.
- **Screen Recording permission** isn't picked up live by Tauri — the first-run flow must detect, prompt, and restart on grant.

## Conventions

- TypeScript strict, no `any`; `zod` at every boundary (API responses, IPC payloads); prefer `type` over `interface`.
- Rust edition 2021; errors via `thiserror`; every command returns `Result<T, LumeError>` — no silent failures.
- Naming: snake_case (Rust), camelCase (TS), kebab-case filenames except PascalCase React components.
- No `console.log` in committed code — use the logger. Never log keys, paths beyond session id, or transcript content.
- Branches `wk{n}-{milestone}-{slug}`; Conventional Commits; tag each milestone `v0.{milestone}.0`.
- Every Anthropic/Deepgram/Voyage call estimates and records actual `cost_usd` to the session row.

## Things you must not decide alone (propose, don't do)

Adding any new external service; changing the data schema after Week 1; changing model selection for a synthesis stage. Anything touching the local-first invariant: refuse and escalate. See `AGENT.md`.

## Models in use

`claude-haiku-4-5-20251001` (title/tags), `claude-sonnet-4-6` (vision + RAG), `claude-opus-4-7` (final synthesis); Deepgram `nova-3`; Voyage `voyage-3-large` (1024-dim).
