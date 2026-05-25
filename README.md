# Lume

> Watch-and-capture knowledge companion. Press a hotkey, watch a video, get a structured document and a queryable knowledge base. Local-first, single-user.

**Status:** Week 1 — foundation. macOS only for v1.0.

## What this is

A Tauri 2 desktop app that records the active browser window when you press a global hotkey, transcribes the audio with Deepgram, analyses sampled frames with Claude vision, synthesises both streams into a structured markdown document, and indexes it in a local SQLite + sqlite-vec store you can query with chat.

No cloud sync. No accounts. Your recordings never leave the machine.

## Quick start

```bash
git clone <repo> && cd lume
pnpm install
cp .env.example .env.local           # then paste your API keys
pnpm tauri dev
```

Full setup steps and prerequisites: see [`RUNBOOK.md`](./RUNBOOK.md).

## Repo map

| File | What it is |
|---|---|
| [`AGENT.md`](./AGENT.md) | Contract for Claude Code working in this repo |
| [`SKILLS.md`](./SKILLS.md) | Tech stack reference and API patterns |
| [`LOOP.md`](./LOOP.md) | Current milestone, current task, done criteria |
| [`ARCHITECTURE.md`](./ARCHITECTURE.md) | System architecture and data flow |
| [`PROMPTS.md`](./PROMPTS.md) | Versioned Claude prompts (vision, synthesis, RAG) |
| [`DATA.md`](./DATA.md) | Schema, fixtures, sample document |
| [`DECISIONS.md`](./DECISIONS.md) | Decision log (ADR-style) |
| [`RUNBOOK.md`](./RUNBOOK.md) | Setup, build, sign, distribute |

The full v1.0 PRD lives outside the repo (sent separately). This repo's job is to encode what's been decided and let the build move.

## Stack

Tauri 2 · Rust · Next.js 14 (static export) · TypeScript · Tailwind · shadcn/ui · SQLite + sqlite-vec · ffmpeg sidecar · Deepgram Nova-3 · Claude Sonnet 4.6 / Opus 4.7 / Haiku 4.5 · Voyage 3 Large

## Ship target

v1.0 macOS, 6 weeks from start of Week 1.
