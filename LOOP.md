# Loop

The agent reads this at the start of every session to know where the build is and what to do next. Update on completion of every task.

---

## Current milestone

**Week 1 — Foundation**

Goal: a Tauri 2 + Next.js app that launches, sits in the menu bar, responds to a global hotkey, and stores configuration securely.

## Done criteria for Week 1

- [ ] Repo bootstrapped per `RUNBOOK.md §2`
- [ ] `pnpm tauri dev` launches a window
- [ ] Tray icon visible, with menu: Library / Settings / Quit
- [ ] Window hides on close (does not quit), tray menu re-opens it
- [ ] Global hotkey `⌃⌥R` registered; emits a `record-toggle` event
- [ ] Global hotkey `⌃⌥L` registered; opens the library window
- [ ] Settings window accepts three API keys; stored in Keychain via stronghold
- [ ] SQLite database created at `~/Library/Application Support/Lume/db.sqlite`
- [ ] sqlite-vec loaded; smoke test inserts and queries one vector
- [ ] First-run flow: detects missing screen recording permission, instructs user, restarts on grant
- [ ] All Week 1 code linted, formatted, and merged to `main`

---

## Current task (in progress)

**`wk1-foundation-bootstrap`** — bootstrap the repo per RUNBOOK §2.

Steps:
1. Create empty repo, init git
2. Run all commands in `RUNBOOK.md §2`
3. Confirm `pnpm tauri dev` opens the default Tauri window
4. Commit as `chore: bootstrap tauri + next.js scaffold`

Definition of done for THIS task: dev window opens with no errors, baseline committed.

---

## Up next (priority order)

1. `wk1-foundation-tray` — implement tray icon and basic menu
2. `wk1-foundation-window-hide` — close hides instead of quits
3. `wk1-foundation-hotkey` — register both global hotkeys and wire events
4. `wk1-foundation-settings-ui` — settings screen with three API key fields
5. `wk1-foundation-stronghold` — Keychain-backed storage of keys
6. `wk1-foundation-db` — SQLite + sqlite-vec init on first run
7. `wk1-foundation-perm-flow` — screen recording permission detection and prompt
8. `wk1-foundation-tag` — tag `v0.1.0` and write a Week 1 retro to `DECISIONS.md`

---

## Blocked

Nothing currently blocked.

---

## Recently completed

(empty — Week 1 has not started yet)

---

## Notes for the agent

- Don't skip ahead. The hotkey task assumes settings exist for hotkey customisation later, but Week 1 uses hard-coded defaults
- Don't add UI polish during Week 1. Default shadcn looks. Polish is Week 6
- If a task takes longer than expected, write the surprise in `DECISIONS.md` before moving on
