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

## ADR 012 — Tray → frontend navigation via a `navigate` event

**Date:** 2026-05-25
**Status:** Accepted

**Context:** The tray menu (Library / Settings) must change what the single static-export window shows. Routing is client-side (App Router), so Rust can't push a route directly. Two options: have Rust load a different URL into the webview, or have Rust emit an event the frontend turns into a `router.push`.

**Decision:** Rust emits a `navigate` event carrying the route string; a `<TrayNavigation />` client component (mounted in the root layout) listens and calls `router.push`. Tray handlers first `show()` + `set_focus()` the `main` window, then emit. Quit calls `app.exit(0)`.

**Rationale:**
- Keeps routing entirely client-side — no full reload, consistent with the static-export constraint.
- The same `navigate` event is reusable by the upcoming global-hotkey task (e.g. `⌃⌥L` → library) without new plumbing.

**Also recorded here:** settings live at the flat route `/settings`, not the `app/(settings)/` route group sketched in RUNBOOK §4 — two route groups (`(library)` and `(settings)`) would both resolve to `/` and collide. Library stays at `/` (`app/page.tsx`).

**Revisit if:** we move to multiple native windows (one per view), which would replace event-routing with per-window URLs.

---

## ADR 013 — Dev server on port 1420, not 3000

**Date:** 2026-05-25
**Status:** Accepted

**Context:** RUNBOOK §2 set Tauri's `devUrl` to `http://localhost:3000` and the `dev` script to plain `next dev`. On this machine another long-running service occupies port 3000, so `next dev` silently fell back to 3001 while Tauri kept loading `:3000` — the webview rendered the other app's 404 instead of Lume.

**Decision:** Pin the dev server to port 1420 (Tauri's conventional dev port): `next dev -p 1420` in `package.json`, and `devUrl: http://localhost:1420` in `tauri.conf.json`.

**Rationale:**
- 1420 is far less likely to collide than 3000.
- The explicit `-p` makes `next dev` fail loudly on a conflict (EADDRINUSE) instead of silently picking another port and desyncing from `devUrl`.

**Revisit if:** we ever need multiple Lume dev servers at once (then make the port configurable).

---

## ADR 014 — API-key storage: Stronghold vault unlocked by a Keychain-held password

**Date:** 2026-05-26
**Status:** Accepted

**Context:** Invariant #2 requires API keys to live in the Keychain, never on disk in plaintext. `tauri-plugin-stronghold` stores an *encrypted vault file on disk* (IOTA Stronghold), not in the Keychain, and needs a password to unlock. So the real question was where that password comes from.

**Decision:** Generate a random password on first run and store it in the **macOS Keychain** via the `keyring` crate (`apple-native` backend), exposed through the `vault_password` Tauri command. The Stronghold vault (`vault.hold`) is encrypted with it using argon2 (salt at `salt.txt`). The three keys are stored in a Stronghold client `lume`; the settings form loads them on mount and writes them on save. Chosen over deriving the password from a device value (predictable, weaker).

**Consequences / notes:**
- First Keychain access shows a macOS prompt; "Always Allow" stops it recurring. Unsigned dev builds may re-prompt across rebuilds.
- The vault is opened once per session (a cached promise in `lib/secrets.ts`) — React's dev double-mount otherwise races two `Stronghold.load`s on the same snapshot and reports "no data present".
- First Rust command boundary, so this also introduced `LumeError` (thiserror, serialises to its message) per invariant #3.
- **Path note:** vault/salt live in Tauri's `app_data_dir()` = `~/Library/Application Support/com.getmobilehq.lume/` (bundle identifier), not the `~/Library/Application Support/Lume/` path written in DATA.md/ARCHITECTURE.md. The DB task (`wk1-foundation-db`) must use the same identifier-based dir for consistency, or we override the data dir everywhere.

**Revisit if:** we add Windows (v1.1) — `keyring` needs the `windows-native` backend and the prompt behaviour differs.

---

## ADR 015 — rusqlite (not tauri-plugin-sql) for the database layer

**Date:** 2026-05-26
**Status:** Accepted; supersedes the `tauri-plugin-sql` choice implied by RUNBOOK §2

**Context:** We need the sqlite-vec extension loaded. `tauri-plugin-sql` (sqlx) can't load SQLite extensions, and SKILLS.md already anticipated using rusqlite. Adding `rusqlite` (bundled) alongside `tauri-plugin-sql` fails to build: both `rusqlite`'s and sqlx's `libsqlite3-sys` declare `links = "sqlite3"`, and Cargo forbids two packages linking the same native library — independent of version.

**Decision:** Use `rusqlite` (bundled) as the sole database layer and remove `tauri-plugin-sql` (and the JS `@tauri-apps/plugin-sql`). sqlite-vec is registered once as a SQLite auto-extension (`sqlite3_auto_extension`) before any connection opens, so every connection has `vec0`. The connection is opened in `db.rs`, schema applied from `migrations/0001_init.sql`, and held in Tauri managed state (`Mutex<Connection>`) for future query commands.

**Consequences:**
- All DB access is Rust-side via commands, not JS-side plugin queries. ARCHITECTURE.md's "lib/db.ts" becomes typed wrappers around those commands rather than direct SQL.
- `db.sqlite` lives in `app_data_dir()` (`~/Library/Application Support/com.getmobilehq.lume/`), beside the Stronghold vault — consistent with ADR 014, and differs from the `…/Lume/` path written in DATA.md/ARCHITECTURE.md.
- Migrations run on startup; `settings.schema_version` tracks the version (currently 1).

**Revisit if:** we later want JS-side ad-hoc SQL badly enough to ship sqlite-vec as a loadable dylib for sqlx — unlikely.

---

## ADR 016 — Screen Recording permission: dev-build TCC caveat

**Date:** 2026-05-26
**Status:** Accepted

**Context:** The first-run flow detects Screen Recording permission via `scap::has_permission()` (`CGPreflightScreenCaptureAccess`), prompts via `scap::request_permission()` (`CGRequestScreenCaptureAccess`), and restarts on grant. In `pnpm tauri dev` the running executable is the bare, unsigned binary `target/debug/lume` — macOS attributes its screen-recording request to the launching process and does not reliably add a `lume` entry to the Screen & System Audio Recording list, so the binary can't be toggled there directly.

**Decision:** Ship the detect→instruct→restart flow as-is. In dev, verify the *granted* path by launching from a terminal that already holds Screen Recording permission (the binary inherits it); the live grant against a listed "Lume" entry is validated when we build the signed `.app` (Week 6, ADR 010). Both branches were verified this way: gate appears when permission is absent; Library passes through when granted.

**Consequences:**
- No code change needed for production — a signed/notarised `Lume.app` registers correctly as "Lume" in the TCC list.
- The flow re-checks only on launch (not live), matching macOS behaviour; the Restart Lume button (`app.restart()`) is the apply step.

**Revisit if:** the signed build still mis-registers — then add an explicit capture attempt to force TCC registration.

---

## Week 1 retro

**Date:** 2026-05-26
**Tag:** `v0.1.0`

**Shipped (all Week 1 done criteria met):** Tauri 2 + Next.js scaffold; tray with Library/Settings/Quit; close-to-hide; global hotkeys `⌃⌥R` (record-toggle) / `⌃⌥L` (library); settings UI with three API-key fields; Keychain-unlocked Stronghold vault persisting those keys; SQLite + sqlite-vec with the full DATA.md schema and a startup smoke test; first-run Screen Recording permission flow.

**What deviated from the plan (ADRs 011–016):**
- Next.js **16**, not 14 (`@latest`); dev server on **1420**, not 3000 (local 3000 was taken).
- **rusqlite replaced `tauri-plugin-sql`** — the plugin can't load extensions and conflicts on `libsqlite3-sys`. Consequence: **all DB access is Rust-side via commands**, so ARCHITECTURE.md's `lib/db.ts` becomes typed wrappers around commands, not direct SQL.
- Secrets: Stronghold vault on disk, unlocked by a **random password in the macOS Keychain** (the docs' "Keychain" was conceptual).
- Tray uses a **black template ring** icon for menu-bar visibility.

**Surprises / drag:** the macOS permission story ate the most time — unsigned dev binaries don't list themselves in the Screen Recording TCC pane, so the granted path can only be exercised via an already-permitted terminal until we ship a signed `.app` (ADR 016). React's dev double-mount also raced the Stronghold snapshot ("no data present") until the vault was opened once per session (ADR 014).

**Carried into Week 2:**
- **Path discrepancy:** everything lives in `~/Library/Application Support/com.getmobilehq.lume/` (Tauri's identifier-based `app_data_dir`), not the `…/Lume/` path written in DATA.md/ARCHITECTURE.md. Reconcile the docs or override the data dir.
- Signed-build validation of the screen-recording grant is deferred to Week 6.
- Minor: a stray `~/package-lock.json` makes Next.js guess the wrong workspace root (harmless warning; silence with `turbopack.root` if it annoys).

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
