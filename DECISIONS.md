# Decisions

Lightweight ADRs. Append-only. Each decision gets a short context, the choice, and what would make us revisit it.

---

## ADR 001 — Tauri 2 over Electron

**Date:** 2026-05-25
**Status:** Accepted

**Context:** Need a desktop shell with native screen capture, global hotkeys, tray menu, and a Keychain-backed secret store. Two realistic options: Tauri 2 or Electron.

**Decision:** Tauri 2.

**Rationale:**
- Smaller bundle (~10-15 MB vs Electron's ~100+ MB)
- Rust backend gives perf for media handling without leaving the shell
- Native menus and tray feel better on macOS
- Sidecar mechanism for ffmpeg is cleaner than Electron's spawn patterns

**Revisit if:** screen capture or system audio capture proves fragile on macOS and the Tauri 2 ecosystem doesn't catch up within Week 2.

---

## ADR 002 — Next.js static export over Vite + React

**Date:** 2026-05-25
**Status:** Accepted

**Context:** Tauri's default templates use Vite. We're choosing Next.js 14 with static export instead.

**Decision:** Next.js 14 App Router, `output: 'export'`, no server-side anything.

**Rationale:**
- Matches the rest of the personal-project stack (Paxr, Raffu, Univelcity)
- App Router patterns are well-known
- Static export works inside Tauri without friction; we don't need SSR or API routes

**Trade-off accepted:** all dynamic logic flows through Tauri commands. No `next/image` optimisation. No middleware.

**Revisit if:** static export bloat or build time becomes painful (>30s incremental).

---

## ADR 003 — Deepgram Nova-3 over Whisper

**Date:** 2026-05-25
**Status:** Accepted (v1.0)

**Context:** Need speech-to-text with diarisation and word-level timestamps. Options: Deepgram Nova-3, OpenAI Whisper API, local whisper.cpp, AssemblyAI.

**Decision:** Deepgram Nova-3 for v1.0.

**Rationale:**
- Diarisation built-in and reliable
- $0.0043/min — cheapest option with this quality bar
- Smart formatting reduces post-processing
- Fast turnaround vs OpenAI Whisper API

**Revisit if:** Deepgram outage frequency exceeds 1%/month, or we need offline operation (then move to local whisper.cpp in v1.2).

---

## ADR 004 — Three-model Claude tier (Haiku / Sonnet / Opus)

**Date:** 2026-05-25
**Status:** Accepted

**Context:** Could use one model for everything. Choosing to tier.

**Decision:**
- Haiku 4.5 for title and tags (cheap, one short call)
- Sonnet 4.6 for per-window vision and RAG queries (workhorse, ~12 calls per session, batched)
- Opus 4.7 for final synthesis (one call per session, quality matters most)

**Rationale:** vision/RAG are batched and many — Sonnet's price-perf wins. Final synthesis is one-shot and the document is what the user lives with for months — Opus quality justifies the cost.

**Revisit if:** Sonnet 4.6 final synthesis quality is indistinguishable from Opus in blind testing on 10 real captures.

---

## ADR 005 — Voyage 3 Large over OpenAI embeddings

**Date:** 2026-05-25
**Status:** Accepted

**Context:** Need embeddings for RAG. Options: OpenAI text-embedding-3-large, Voyage 3 Large, Cohere embed-v3.

**Decision:** Voyage 3 Large.

**Rationale:**
- Higher retrieval quality in independent benchmarks (BEIR, MTEB)
- 1024 dim — same as OpenAI 3-large, fits in sqlite-vec cleanly
- $0.18 per M tokens — cheaper than OpenAI

**Revisit if:** retrieval quality on real Lume corpora underperforms versus OpenAI in side-by-side tests.

---

## ADR 006 — sqlite-vec for local vector storage

**Date:** 2026-05-25
**Status:** Accepted

**Context:** Need a local vector store. Options: sqlite-vec, lancedb, qdrant embedded, duckdb-vss, faiss in a wrapper.

**Decision:** sqlite-vec.

**Rationale:**
- Embeds in the same SQLite file as metadata — single backup story
- Mature enough for v1 (asg017 maintains actively)
- No additional process or daemon
- Performs fine to ~100k vectors

**Revisit if:** we exceed ~100k chunks (roughly 5000 hours of video) and ANN latency exceeds 100ms.

---

## ADR 007 — macOS first, Windows in v1.1

**Date:** 2026-05-25
**Status:** Accepted

**Context:** Tauri supports all three desktop OSes. Single-user developer = limited testing surface.

**Decision:** macOS only for v1.0. Windows in v1.1. Linux indefinitely deferred.

**Rationale:** Joseph's primary machine is Mac. Capture APIs differ per OS. Shipping all three in 6 weeks is unrealistic without sacrificing quality.

**Revisit if:** a Windows user asks for it loudly enough.

---

## ADR 008 — Local-first, no cloud sync in v1.0

**Date:** 2026-05-25
**Status:** Accepted

**Context:** Could build with Supabase from the start. Choosing not to.

**Decision:** All data stays on the user's machine in v1.0.

**Rationale:**
- Privacy posture from day one
- Removes auth, billing, multi-tenancy from the v1 scope
- Aligns with how the user uses the product (single Mac, single user)

**Revisit when:** considering team / shared knowledge base in v3.

---

## ADR 009 — Browser-window-only capture in v1.0

**Date:** 2026-05-25
**Status:** Accepted

**Context:** Could allow capture of any screen surface. Constraining for v1.

**Decision:** v1.0 captures the foreground browser window only. Any-window in v1.1.

**Rationale:**
- Synthesis prompts can be tuned for browser-style content (talks, demos, courses)
- Reduces source-of-truth ambiguity in early testing
- Simpler permission story (Screen Recording on the browser process)

**Revisit:** start of v1.1.

---

## ADR 010 — Deferred: Apple Developer ID for signing

**Date:** 2026-05-25
**Status:** Deferred to Week 6

**Context:** Apple Developer Program costs $99/year and requires DUNS-equivalent. Without it, builds aren't signed and Gatekeeper shows a warning.

**Decision:** Run unsigned for Weeks 1-5. Sign and notarise in Week 6 as part of the ship task.

**Revisit:** Week 6 task `wk6-polish-distribution`.

---

## ADR 011 — Bootstrap landed on Next.js 16, not 14

**Date:** 2026-05-25
**Status:** Accepted

**Context:** RUNBOOK §2 bootstraps with `pnpm create next-app@latest`, and the stack docs (README, ARCHITECTURE) name "Next.js 14". As of bootstrap, `@latest` resolves to Next.js 16.2.6 (React 19.2, Tailwind 4). The scaffold also differs from the runbook in two minor ways: `next.config.ts` instead of `.mjs`, and shadcn's CLI no longer exposes a base-color flag (defaults to `neutral`; we kept it since theming is a Week 6 task).

**Decision:** Build on Next.js 16 / React 19 / Tailwind 4. Static export (`output: 'export'`, `images.unoptimized`) verified working — `pnpm build` emits `out/`. Treat the "Next.js 14" references as historical; the static-export constraints in ARCHITECTURE.md still hold.

**Rationale:**
- No reason to pin an older major when the static-export contract is unchanged.
- React 19 is supported by shadcn and the Tauri plugins.

**Also recorded here:** crate renamed `app` → `lume` (lib `lume_lib`); bundle identifier set to `com.getmobilehq.lume` (was the placeholder `com.tauri.dev`).

**Revisit if:** a dependency in the pipeline (Anthropic/Deepgram/Voyage SDKs) or a Tauri plugin proves incompatible with React 19 / Next 16.

---

## How to add an ADR

Copy the template below, append to the bottom of this file, give it the next number.

```
## ADR NNN — {Short title}

**Date:** YYYY-MM-DD
**Status:** Proposed | Accepted | Superseded by ADR XXX

**Context:** {1-3 sentences on the problem}

**Decision:** {what we chose}

**Rationale:**
- {bullet}
- {bullet}

**Revisit if:** {trigger}
```
