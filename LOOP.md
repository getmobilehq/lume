# Loop

The agent reads this at the start of every session to know where the build is and what to do next. Update on completion of every task.

---

## Current milestone

**Sprint 1 — Capture (Weeks 2–3).** Full plan: `lume-sprint-plan.md`.
_(Sprint 0 / Week 1 — Foundation: ✅ complete, tagged `v0.1.0` on 2026-05-26.)_

Goal: pressing the hotkey produces a clean MP4 of the active browser window with synced system audio, written to the session directory, with a session row in SQLite.

## Done criteria for Sprint 1

- [ ] A 5-min YouTube video captures; the MP4 plays in QuickTime with synced audio
- [ ] Session row transitions `recording` → `processing` → (stub) `ready`
- [ ] Tray icon reflects state (idle / recording / processing)
- [ ] Start/stop reliable 10× in a row — no leaks or crashes

---

## Current task (in progress)

**`S1-01a`** — scap capturer lifecycle: init, start, pull frames, stop (capture the full display first to prove the frame stream end-to-end). Window targeting is `S1-01b`/`S1-02`.

Done criteria: a Rust command starts/stops a scap capture of the main display, frames flow on a channel without leaking, and stop tears down cleanly.

---

## Up next (priority order)

Groomed from the `lume-sprint-plan.md` S1 backlog; tickets that felt >4h are split (suffixed a/b).

1. `S1-01b` — target a specific window with scap (foreground window)
2. `S1-02` — detect the active **browser** window by process name; feed it to `S1-01b`
3. `S1-03` — system audio capture via ScreenCaptureKit; confirm sync
4. `S1-04a` — MP4 video: pipe scap frames → ffmpeg, H.264 30fps 1280×720 (no audio yet)
5. `S1-04b` — mux AAC system audio into the MP4; verify A/V sync
6. `S1-05` — session lifecycle: insert row (ulid) on start, update status on stop
7. `S1-08` — persist to `{app_data_dir}/sessions/{id}/recording.mp4` (no hardcoded paths)
8. `S1-06a` — tray icon state swap (idle / recording / processing template icons)
9. `S1-06b` — recording animation on the tray icon
10. `S1-07` — soft warning if a recording exceeds 90 min
11. `S1-09` — crash recovery: orphaned `recording` rows on launch → `error`
12. `S1-10` — memory-profile a 60-min capture (cap ~500MB, no leaks)

---

## Blocked

Nothing currently blocked.

---

## Recently completed

- `wk1-foundation-tag` (2026-05-26) — Week 1 closed: all code linted/formatted/on `main`, Week 1 retro added to `DECISIONS.md`, annotated `v0.1.0` tag created and pushed.
- `wk1-foundation-perm-flow` (2026-05-26) — Library page gated on Screen Recording permission (`scap::has_permission`); when missing, shows setup instructions with Open Settings (`scap::request_permission` + opens the pane) and Restart Lume (`app.restart()`). Verified both paths: gate when absent, pass-through when granted. Dev caveat (unsigned binary not listed in TCC) in ADR 016.
- `wk1-foundation-db` (2026-05-26) — SQLite at `db.sqlite` (app data dir, WAL) with the full DATA.md schema applied as migration 0001; sqlite-vec loaded via rusqlite auto-extension; startup smoke test inserts + reads back one vector. Connection held in managed state. Replaced `tauri-plugin-sql` with rusqlite (links conflict + extension loading) — ADR 015.
- `wk1-foundation-stronghold` (2026-05-26) — API keys persist in a Stronghold vault (argon2) unlocked by a random password kept in the macOS Keychain via the `keyring` crate (`vault_password` command, `LumeError`). Settings form loads keys on mount, saves on submit. Keys survive relaunch; verified. Vault access opened once per session to avoid a StrictMode double-load race. See ADR 014.
- `wk1-foundation-settings-ui` (2026-05-25) — `/settings` form with masked Anthropic/Deepgram/Voyage key fields (shadcn input/label), zod validation, Save. Keys held in component state; persistence is `wk1-foundation-stronghold`. Verified in app. Also: dev server moved to port 1420 (ADR 013) after a 3000 collision, and the tray now uses a black template ring icon for menu-bar visibility.
- `wk1-foundation-hotkey` (2026-05-25) — registered `⌃⌥R` (emits `record-toggle`, logged via `log::info!` until capture exists) and `⌃⌥L` (shows+focuses library) via `tauri-plugin-global-shortcut`, hard-coded in lib.rs. Verified both in `pnpm tauri dev`.
- `wk1-foundation-window-hide` (2026-05-25) — close-requested on the main window hides it and prevents close (`on_window_event` in lib.rs); app stays alive in the tray, tray items re-show it, Quit still exits. Verified in `pnpm tauri dev`.
- `wk1-foundation-tray` (2026-05-25) — tray icon with Library / Settings / Quit. Library/Settings emit a `navigate` event that the frontend routes; Quit exits. Verified all four behaviours in `pnpm tauri dev`. Navigation pattern logged in ADR 012.
- `wk1-foundation-bootstrap` (2026-05-25) — Tauri 2 + Next.js scaffold per RUNBOOK §2. `pnpm tauri dev` launches the window; frontend, Rust plugins, scap, and shadcn all in. Deviations logged in ADR 011.

---

## Notes for the agent

- Paths: use Tauri's `app_data_dir()` (= `~/Library/Application Support/com.getmobilehq.lume/`, where the vault + db already live), never hardcoded paths. DATA.md/ARCHITECTURE.md still say `…/Lume/` — stale; reconcile when convenient.
- DB access is Rust-side via commands (ADR 015) — `lib/db.ts` is a typed wrapper over commands, not direct SQL.
- Vision/RAG model is `claude-sonnet-4-6` (the repo docs were corrected). The sprint plan still says "Sonnet 4.7" in S3/S5 — treat as 4.6 unless told otherwise.
- macOS capture risk: scap window-targeting differs across Chrome / Arc / Safari — expect per-browser workarounds.
- If a task takes longer than expected, write the surprise in `DECISIONS.md` before moving on.
