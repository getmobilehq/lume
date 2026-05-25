# Agent contract

This file tells Claude Code how to operate inside the Lume repo. Read this first before any task.

## What Lume is

A single-user macOS desktop app that records browser videos on hotkey, transcribes and analyses them, and produces a queryable knowledge base — all locally. Tauri 2 shell, Next.js 14 frontend, Rust backend for capture and media processing, Anthropic / Deepgram / Voyage for AI services, SQLite + sqlite-vec for storage.

## Invariants — never violate

1. **Local-first.** No recording, transcript, document, or embedding ever leaves the user's machine except when explicitly sent to a configured API endpoint. No telemetry. No analytics. No background sync.
2. **Secrets in Keychain, not on disk.** All API keys go through `tauri-plugin-stronghold`. Never write a key to a `.env`, log, or settings JSON file at runtime. The only `.env` that exists is `.env.example`.
3. **No silent failures.** Every API call wraps in a typed error. Every Rust command returns `Result<T, LumeError>`. Errors surface to the UI with actionable text.
4. **Cost-aware.** Every Anthropic / Deepgram / Voyage call estimates token / minute usage and writes the actual cost to the session row. Surfaces in the UI.
5. **No external state in the critical path.** A session can be captured, transcribed, synthesised, and queried offline-after-capture *if* the user pre-downloads transcripts and accepts that vision/embed steps require network. Don't introduce a service that breaks the local-first promise.
6. **Frames are ephemeral by default.** Sampled JPEGs are kept for the v1.0 session-view thumbnail strip but can be deleted by the user without breaking the indexed document.
7. **Documents are human-readable markdown.** Always. Never a proprietary binary format. The user should be able to open `~/Library/Application Support/Lume/sessions/{id}/document.md` in any editor.

## How to talk to me (Claude Code voice in this repo)

- Be direct. Skip preamble.
- Show file diffs, not "here's the updated file" plus the whole thing.
- When unsure between two reasonable implementations, name them both and recommend one.
- Don't apologise. Don't say "I'd be happy to". Just do the work.
- Pushback is welcome. If a task in `LOOP.md` is wrong, say so before implementing it.

## Code style

- **TypeScript:** strict mode on. No `any`. Use `zod` for any boundary parsing (API responses, IPC payloads). Prefer `type` over `interface` unless extending.
- **Rust:** edition 2021. `cargo fmt` and `cargo clippy --all-targets -- -D warnings` must pass. Errors via `thiserror`. Async via `tokio`.
- **React:** function components, hooks. Server components allowed but only static — no `fetch` at render time (we're a static export). All dynamic data flows through Tauri commands.
- **Naming:** snake_case in Rust, camelCase in TS, kebab-case for filenames except React components (PascalCase).
- **Comments:** rare. Code should read itself. When a comment exists it explains *why*, never *what*.
- **No console.log in committed code.** Use the logger.

## Repo conventions

- Branch names: `wk{n}-{milestone}-{slug}` (e.g. `wk1-foundation-hotkey`)
- Commits: Conventional Commits (`feat:`, `fix:`, `refactor:`, `docs:`, `chore:`)
- PRs not required for solo dev, but every milestone closes with a tagged commit: `v0.{milestone}.0`

## What lives where

- Domain logic that touches the filesystem, screen, or audio → **Rust** (`src-tauri/`)
- API calls to Anthropic / Deepgram / Voyage → **TypeScript** (`lib/`). Reason: easier prompt iteration, better DX.
- UI → **React** (`app/`, `components/`)
- Tauri commands are the only allowed Rust ↔ TS boundary. No HTTP between layers.

## When you finish a task

1. Run `pnpm lint && pnpm tsc --noEmit` (TS side) and `cargo fmt && cargo clippy` (Rust side)
2. Update `LOOP.md`: move the task from "in progress" to "done", set the next task as "in progress"
3. If you made a non-trivial architectural decision, add an ADR to `DECISIONS.md`
4. Stop. Do not start the next task unless explicitly told to.

## What you don't decide alone

- Adding a new external service (LLM, STT, embedding provider) — propose, don't add
- Changing the data schema after Week 1 — propose, don't add
- Changing model selection (Sonnet ↔ Opus, etc.) for a synthesis stage — propose, don't add
- Anything that touches the local-first invariant — refuse, then escalate
