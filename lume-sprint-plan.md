# Lume — sprint plan

**Scope:** everything left to ship v1.0 after the Week 1 foundation (window, tray, settings, hotkey wiring) is complete.

**Shape:** five sprints of 2 weeks each = 10 weeks of build, plus 1 buffer week = **11 weeks to v1.0 ship**.

**Cadence per sprint:**
- Day 1 — sprint planning, ticket grooming
- Day 2 to 9 — build
- Day 10 — demo to yourself (record a real session end-to-end as a smoke test), retro, tag release

---

## Status going in

**Sprint 0 (Week 1) — done**
- Tauri 2 + Next.js scaffold
- Tray icon + menu
- Window hide-on-close
- Global hotkeys registered (`⌃⌥R`, `⌃⌥L`)
- Settings UI with API key entry
- Keychain-backed secret storage
- SQLite + sqlite-vec init
- Screen recording permission flow

If anything in that list isn't actually done, finish it before Sprint 1.

---

## Sprint 1 — Capture (Weeks 2–3)

**Goal:** pressing the hotkey produces a clean MP4 of the active browser window with synced system audio, written to the session directory, with a session row recorded in SQLite.

**Definition of done:**
- A 5-minute YouTube video can be captured and the resulting MP4 plays correctly in QuickTime with synced audio
- Session row reflects status transitions: `recording` → `processing` → (stub) `ready`
- Tray icon visually reflects state (idle / recording / processing)
- Recording can be started and stopped reliably 10 times in a row without leaks or crashes

**Tickets:**

| ID | Title | Notes |
|---|---|---|
| S1-01 | Wire `scap` for window-targeted capture | macOS only this sprint |
| S1-02 | Capture active browser window by process name | Detect frontmost app, find its window, target capture |
| S1-03 | System audio capture via ScreenCaptureKit | Confirm sync in test recordings |
| S1-04 | MP4 writer: H.264 + AAC, 30fps, target 1280×720 | Via ffmpeg sidecar piping from scap |
| S1-05 | Session lifecycle: create row on start, update on stop | ulid as id; status enum |
| S1-06 | Tray icon state machine (idle / recording / processing) | Three SVG icons + animation on recording |
| S1-07 | Stop-recording confirmation if recording exceeds 90 min | Soft warning, not a hard cap |
| S1-08 | Persist recording to `{data_dir}/sessions/{id}/recording.mp4` | Use Tauri's app_data_dir, not hardcoded paths |
| S1-09 | Crash recovery: orphaned recording on next launch | Detect `recording` status on startup → move to `error` |
| S1-10 | Memory profile a 60-min capture | Confirm no leaks; cap memory at ~500MB |

**Risks to watch:**
- ScreenCaptureKit permission UX is fragile — schedule a half-day to nail the first-run flow
- scap's window-targeting on macOS may need workarounds for Chrome / Arc / Safari differences

---

## Sprint 2 — Audio pipeline + transcription (Weeks 4–5)

**Goal:** every captured session automatically produces a diarised transcript with word-level timestamps, stored as `transcript.json`.

**Definition of done:**
- After a capture stops, the audio is extracted, sent to Deepgram, and the JSON response is saved
- Library view shows the session with its real duration and a "transcribed" indicator
- Transcript is viewable in the session detail screen (basic text rendering, no formatting polish yet)
- Cost per session is recorded in the `cost_usd` column

**Tickets:**

| ID | Title | Notes |
|---|---|---|
| S2-01 | ffmpeg audio extraction: MP4 → 16kHz mono WAV | `-vn -ac 1 -ar 16000 -c:a pcm_s16le` |
| S2-02 | Deepgram client wrapper with retry + backoff | TS, in `lib/deepgram.ts` |
| S2-03 | Deepgram Nova-3 call: diarisation, smart format, paragraphs | Save raw JSON response |
| S2-04 | Pipeline stage runner with stage tracking | One row per stage in `pipeline_runs` (debug aid) |
| S2-05 | Error handling: Deepgram timeout / quota / network | Surface to UI with retry button |
| S2-06 | Library view skeleton | List of sessions, sorted by `recorded_at desc` |
| S2-07 | Session detail view skeleton | Two columns: doc placeholder + transcript |
| S2-08 | Transcript renderer with speaker labels and timestamps | Click timestamp → log only this sprint |
| S2-09 | Cost calculation: minutes × $0.0043 per session | Write to `sessions.cost_usd` |
| S2-10 | Audio.wav cleanup post-transcription | Configurable toggle in settings |

**Risks:**
- Deepgram diarisation accuracy on quiet voiceovers — have a "redo with different settings" path ready
- Large audio files (>100MB for very long recordings) — confirm Deepgram handles streaming upload

---

## Sprint 3 — Visual analysis (Weeks 6–7)

**Goal:** sampled frames analysed with Claude Sonnet 4.7 vision, producing per-window JSON summaries stored in `windows.json`.

**Definition of done:**
- ffmpeg samples frames with scene-change + interval fallback
- Frames are batched into 5-minute windows aligned to transcript boundaries
- Each window goes through Claude vision and returns valid JSON matching the P1 schema in `PROMPTS.md`
- Failed windows can be retried without re-running the whole pipeline
- A 60-minute test session produces 12 window outputs in under 6 minutes total

**Tickets:**

| ID | Title | Notes |
|---|---|---|
| S3-01 | ffmpeg frame sampling: scene-change + 30s interval | Output JPEGs ≤1280px wide |
| S3-02 | Frame-to-window assignment based on timestamp | Up to 10 frames per window |
| S3-03 | Window builder: split transcript into 5-min windows | Boundary snaps to nearest sentence break |
| S3-04 | Anthropic SDK client wrapper | Retry + cost tracking |
| S3-05 | Per-window vision call with Sonnet 4.7 | Implements P1 prompt verbatim from `PROMPTS.md` |
| S3-06 | Zod schema for P1 output validation | Reject + retry on invalid JSON |
| S3-07 | Message Batches API for parallel windows | ~50% cost saving for non-urgent processing |
| S3-08 | Pre-resize frames to ≤2576px long edge | Required for Claude vision; avoids server-side downscale |
| S3-09 | Save `windows.json` to session dir | Array of validated P1 outputs |
| S3-10 | Per-stage cost accumulation into `sessions.cost_usd` | Visible in library row |

**Risks:**
- Frame sampling tuning — scene detection thresholds vary by content type
- Batches API has its own quirks vs sync calls — read docs carefully
- Token budget surprise on slide-heavy videos — many similar frames

---

## Sprint 4 — Synthesis + library polish (Weeks 8–9)

**Goal:** windows.json → final markdown document via Claude Opus 4.7, auto-titled and tagged, viewable in a polished session detail screen.

**Definition of done:**
- Each ready session has a `document.md` matching the structure in `PROMPTS.md` P2
- Title and tags auto-generated via Haiku 4.5
- Session detail view renders the markdown cleanly with timestamp links that scrub the embedded video
- Library view shows real titles and tags
- A real 60-min session goes from hotkey-stop to a readable document in under 8 minutes

**Tickets:**

| ID | Title | Notes |
|---|---|---|
| S4-01 | Opus 4.7 synthesis call with extended thinking | Implements P2 prompt |
| S4-02 | Markdown validator: frontmatter present, required sections | Reject + retry once if structure broken |
| S4-03 | Haiku 4.5 title + tags call | Implements P3 prompt |
| S4-04 | Persist `document.md` + update `sessions.title`, `tags` | One write transaction |
| S4-05 | Markdown renderer in session view | Use `react-markdown` with rehype-raw, custom timestamp link handler |
| S4-06 | Embedded video player with timestamp seek | `<video>` element + ref; intercept `lume://` clicks |
| S4-07 | Transcript sidebar: collapsible, scroll-sync with video | Optional polish; nice to have |
| S4-08 | Library row design: title, date, duration, tags, cost | Match minimalist principles in PRD §11 |
| S4-09 | Session status indicators in library | Processing spinner; error badge with retry |
| S4-10 | Bulk delete: select multiple sessions, delete | Confirm dialog, cascade through DB + files |

**Risks:**
- Opus 4.7 cost — one call per session, but extended thinking inflates output tokens. Monitor.
- Markdown rendering edge cases — strange characters in quotes, etc.

---

## Sprint 5 — RAG + ship prep (Weeks 10–11)

**Goal:** chat input on the library page returns synthesised answers with citation links across all sessions. App is signed, notarised, and shippable as a `.dmg`.

**Definition of done:**
- Voyage embeddings stored in `chunk_vec` for every ready session
- Chat input at top of library accepts a question
- Top-5 retrieval + Sonnet 4.7 streaming answer with `lume://` citations
- Clicking a citation jumps to the session and scrubs the video to the timestamp
- App is signed with Apple Developer ID, notarised, and installs cleanly on a fresh macOS machine
- First-run experience is smooth: open .dmg, drag to Applications, permissions walkthrough

**Tickets:**

| ID | Title | Notes |
|---|---|---|
| S5-01 | Chunking: H2 sections of `document.md` | Each becomes one chunk row |
| S5-02 | Voyage 3 Large embedding call (batched up to 128) | Implement in `lib/voyage.ts` |
| S5-03 | Insert into `chunks` + `chunk_vec` in one transaction | Roll back on failure |
| S5-04 | Backfill existing sessions on first launch of v1.0 | Migration |
| S5-05 | Chat input UI at top of library | Streaming token rendering |
| S5-06 | Retrieval: embed question, top-5 via sqlite-vec | Cosine similarity |
| S5-07 | Sonnet 4.7 RAG call with prompt caching | Implements P4 prompt |
| S5-08 | Citation renderer: parse `lume://session/{id}#t={s}` | Click handler scrolls + scrubs |
| S5-09 | Apple Developer Program enrolment | Allow 24h for approval |
| S5-10 | Signing + notarisation pipeline | `tauri build` + `notarytool` |
| S5-11 | `.dmg` packaging with custom layout | Drag-to-Applications |
| S5-12 | First-run onboarding flow | API key entry → permission grant → first capture prompt |
| S5-13 | App icon final | Refine the placeholder concept |
| S5-14 | Crash logging to local file | Never to network |
| S5-15 | End-to-end smoke test: 5 real captures by you | If any fail, blocker for ship |

**Risks:**
- Notarisation rejections often need 2-3 iterations — start early in the sprint
- Apple Developer enrolment can stall on identity verification — start Day 1 of Sprint 5

---

## Sprint 6 — Buffer + polish (Week 12)

**Not a real sprint** — a buffer week for the things that always run over. Use it for:
- Bug fixes from Sprint 5 smoke tests
- Performance tuning on long recordings
- Documentation pass (`RUNBOOK.md`, user-facing readme)
- Recording the first 10 demo sessions on real content you actually want indexed
- Posting the build internally (or just to your team) for early feedback

If Sprint 5 finished clean, this becomes the first week of Sprint 7 work below.

---

## Beyond v1.0 — v1.1 sprint (Weeks 13–14)

Locked in `DECISIONS.md` as deferred. Sketch only:

**Goal:** Windows support, any-window capture, source URL capture, export to Notion/Obsidian.

| Theme | Tickets |
|---|---|
| Windows port | scap on Windows path validation; WASAPI loopback for audio; Windows code-signing cert; first-run permission flow (none required); `.msi` installer |
| Any-window capture | Drop the browser-only constraint; capture target picker; multi-monitor handling |
| Source URL capture | macOS accessibility API to read browser URL bar; fallback for unsupported browsers |
| Export | Notion API integration; Obsidian: write directly to vault folder; Drive: upload as Google Doc |
| Auto-stop | Detect video-ended state on common platforms (YouTube, Vimeo, Loom) |

---

## How to feed this into a tracker

If you use Monday.com (which you do):
- Create a board called "Lume v1.0"
- Columns: ID, Title, Sprint, Status, Estimate (hrs), Notes
- Group by sprint
- Paste the tickets from each Sprint table above
- Set Sprint 1 tickets to "Working on it", everything else to "Pending"

If you use Linear / Jira / etc:
- One project, six cycles named "Sprint 1" through "Sprint 5" + "Buffer"
- Use the IDs above as the issue prefix

---

## Estimates and capacity

These sprints assume **~15 hours/week** on Lume (your 20% sliver after VMO2 + your other ventures). At that pace:
- A 2-week sprint = ~30 hours of focused work
- Each sprint has 10-15 tickets, averaging ~2-3 hours each
- Buffer is real — not aspirational

If you can give 25+ hours/week, compress to 7 weeks total. If less than 10, expect 14-16 weeks.

---

## Decision points along the way

End of each sprint, ask:
1. **Did the demo work?** Capture a real session end-to-end. If not, slip the next sprint by half a sprint.
2. **Is the cost-per-session still under $1.50?** If higher, tune frame sampling and model selection.
3. **Is the synthesised document still readable?** If quality drifts, revisit prompts in `PROMPTS.md`.
4. **Anything in `DECISIONS.md` to update?** Append-only.

---

## What you do now

1. Confirm Sprint 0 is actually done (checklist above)
2. Paste these tickets into your tracker
3. Start Sprint 1, Day 1: grooming pass on the S1 tickets — break any that feel >4 hours into smaller ones
4. Open `LOOP.md` and update it to "Sprint 1 in progress, current task S1-01"

When Sprint 1 closes, ping me and we'll do a focused planning pass on Sprint 2 — by then we'll know whether anything in S1 surprised us and the S2 plan may need adjustment.
