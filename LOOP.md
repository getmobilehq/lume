# Loop

The agent reads this at the start of every session to know where the build is and what to do next. Update on completion of every task.

---

## Current milestone

**Week 1 — Foundation**

Goal: a Tauri 2 + Next.js app that launches, sits in the menu bar, responds to a global hotkey, and stores configuration securely.

## Done criteria for Week 1

- [x] Repo bootstrapped per `RUNBOOK.md §2`
- [x] `pnpm tauri dev` launches a window
- [x] Tray icon visible, with menu: Library / Settings / Quit
- [x] Window hides on close (does not quit), tray menu re-opens it
- [x] Global hotkey `⌃⌥R` registered; emits a `record-toggle` event
- [x] Global hotkey `⌃⌥L` registered; opens the library window
- [ ] Settings window accepts three API keys; stored in Keychain via stronghold
- [ ] SQLite database created at `~/Library/Application Support/Lume/db.sqlite`
- [ ] sqlite-vec loaded; smoke test inserts and queries one vector
- [ ] First-run flow: detects missing screen recording permission, instructs user, restarts on grant
- [ ] All Week 1 code linted, formatted, and merged to `main`

---

## Current task (in progress)

**`wk1-foundation-stronghold`** — Keychain-backed storage of the three API keys.

Done criteria: keys entered in `/settings` persist via `tauri-plugin-stronghold` (Keychain), survive relaunch, and load back into the form on mount; nothing written to disk in plaintext.

---

## Up next (priority order)

1. `wk1-foundation-db` — SQLite + sqlite-vec init on first run
2. `wk1-foundation-perm-flow` — screen recording permission detection and prompt
3. `wk1-foundation-tag` — tag `v0.1.0` and write a Week 1 retro to `DECISIONS.md`

---

## Blocked

Nothing currently blocked.

---

## Recently completed

- `wk1-foundation-settings-ui` (2026-05-25) — `/settings` form with masked Anthropic/Deepgram/Voyage key fields (shadcn input/label), zod validation, Save. Keys held in component state; persistence is `wk1-foundation-stronghold`. Verified in app. Also: dev server moved to port 1420 (ADR 013) after a 3000 collision, and the tray now uses a black template ring icon for menu-bar visibility.
- `wk1-foundation-hotkey` (2026-05-25) — registered `⌃⌥R` (emits `record-toggle`, logged via `log::info!` until capture exists) and `⌃⌥L` (shows+focuses library) via `tauri-plugin-global-shortcut`, hard-coded in lib.rs. Verified both in `pnpm tauri dev`.
- `wk1-foundation-window-hide` (2026-05-25) — close-requested on the main window hides it and prevents close (`on_window_event` in lib.rs); app stays alive in the tray, tray items re-show it, Quit still exits. Verified in `pnpm tauri dev`.
- `wk1-foundation-tray` (2026-05-25) — tray icon with Library / Settings / Quit. Library/Settings emit a `navigate` event that the frontend routes; Quit exits. Verified all four behaviours in `pnpm tauri dev`. Navigation pattern logged in ADR 012.
- `wk1-foundation-bootstrap` (2026-05-25) — Tauri 2 + Next.js scaffold per RUNBOOK §2. `pnpm tauri dev` launches the window; frontend, Rust plugins, scap, and shadcn all in. Deviations logged in ADR 011.

---

## Notes for the agent

- Don't skip ahead. The hotkey task assumes settings exist for hotkey customisation later, but Week 1 uses hard-coded defaults
- Don't add UI polish during Week 1. Default shadcn looks. Polish is Week 6
- If a task takes longer than expected, write the surprise in `DECISIONS.md` before moving on
