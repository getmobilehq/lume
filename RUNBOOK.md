# Runbook

## 1. Prerequisites

### Dev environment
```bash
# Node 20+
brew install node@20

# Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default stable

# Xcode Command Line Tools
xcode-select --install

# Package manager
brew install pnpm

# ffmpeg for local pipeline testing (bundled as sidecar later)
brew install ffmpeg
```

### API keys

| Service | Where | Used for |
|---|---|---|
| Anthropic | console.anthropic.com → API Keys | Vision (Sonnet 4.6), synthesis (Opus 4.7), titles (Haiku 4.5) |
| Deepgram | console.deepgram.com → API Keys | Audio transcription (Nova-3) |
| Voyage AI | dash.voyageai.com → API Keys | Embeddings (voyage-3-large) |

Paste into `.env.local`. Never commit.

---

## 2. First-time repo bootstrap

These are the commands that produce the working skeleton. Run them once after cloning into an empty directory.

```bash
# Scaffold Next.js (App Router, TS, Tailwind, no src dir)
pnpm create next-app@latest . \
  --typescript --tailwind --app --no-src-dir --import-alias "@/*"

# Configure static export — edit next.config.mjs:
#   const nextConfig = { output: 'export', images: { unoptimized: true } };

# Install Tauri 2
pnpm add -D @tauri-apps/cli@latest
pnpm add @tauri-apps/api@latest
pnpm tauri init
# Answers:
#   App name: Lume
#   Window title: Lume
#   Frontend dev URL: http://localhost:3000
#   Frontend build command: pnpm build
#   Frontend dist directory: ../out

# Tauri plugins (Rust side)
cd src-tauri
cargo add tauri-plugin-global-shortcut
cargo add tauri-plugin-sql --features sqlite
cargo add tauri-plugin-fs
cargo add tauri-plugin-store
cargo add tauri-plugin-notification
cargo add tauri-plugin-stronghold
cd ..

# Tauri plugins (JS side)
pnpm add \
  @tauri-apps/plugin-global-shortcut \
  @tauri-apps/plugin-sql \
  @tauri-apps/plugin-fs \
  @tauri-apps/plugin-store \
  @tauri-apps/plugin-notification \
  @tauri-apps/plugin-stronghold

# shadcn/ui
pnpm dlx shadcn@latest init
# Choose: TypeScript yes, default style, slate base, CSS vars yes, app/globals.css, @/components, @/lib/utils, RSC no

# Screen capture (Rust)
cd src-tauri
cargo add scap
cd ..

# Domain libraries (JS)
pnpm add @anthropic-ai/sdk @deepgram/sdk
pnpm add ulid zod
pnpm add -D @types/node
```

Then commit the result as the bootstrap baseline.

---

## 3. Daily development

```bash
pnpm tauri dev      # opens the desktop window with HMR
pnpm tauri build    # produces a .app bundle in src-tauri/target/release/bundle/macos
```

The dev window is a real Tauri app, not a browser. The Next.js dev server runs in the background and Tauri loads from it.

---

## 4. Project structure

```
lume/
├── app/                          # Next.js App Router
│   ├── (library)/page.tsx        # Library + chat (home view)
│   ├── (session)/[id]/page.tsx   # Single session view
│   ├── (settings)/page.tsx       # Settings
│   └── layout.tsx
├── components/                   # UI components (shadcn + custom)
├── lib/
│   ├── tauri.ts                  # Typed wrappers around invoke()
│   ├── anthropic.ts              # Claude API client
│   ├── deepgram.ts               # STT client
│   ├── voyage.ts                 # Embeddings client
│   ├── prompts.ts                # The prompts from PROMPTS.md as constants
│   └── db.ts                     # SQLite query helpers
├── src-tauri/
│   ├── src/
│   │   ├── lib.rs                # main entry
│   │   ├── capture.rs            # screen + audio capture
│   │   ├── pipeline.rs           # post-capture orchestration
│   │   ├── ffmpeg.rs             # sidecar invocation
│   │   ├── db.rs                 # SQLite + sqlite-vec setup
│   │   └── commands.rs           # IPC commands exposed to JS
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   └── icons/
├── public/
├── .env.example
├── .env.local                    # gitignored
└── …docs (AGENT, SKILLS, LOOP, etc.)
```

---

## 5. Known gotchas

**ScreenCaptureKit permission.** First capture triggers a system permission dialog. App must be re-launched after the user grants permission — Tauri doesn't pick it up live. Handle this in the first-run flow: detect, prompt, restart.

**Retina coordinates.** macOS Retina displays capture at 2× device pixel ratio. When sampling frames for Claude vision, resize before sending: ≤2576px on the long edge for current Claude 4.x models (the vision stage runs on Sonnet 4.6; Opus 4.7 uses the same limit), ≤1568px for older models.

**sqlite-vec extension loading.** `tauri-plugin-sql` does not directly support loading SQLite extensions. We load `sqlite-vec` via a Rust-side init hook before any queries run. See `src-tauri/src/db.rs` once scaffolded.

**ffmpeg as sidecar.** Bundle static ffmpeg binaries for x86_64 and aarch64 in `src-tauri/binaries/ffmpeg-{target-triple}`. Tauri's `shell` plugin (or the newer sidecar mechanism in Tauri 2) handles invocation. Download from osxexperts.net or build locally.

**Stronghold key.** First-run generates a stronghold password derived from a device-unique value. Without this, API keys can't be unlocked. Document the recovery path.

---

## 6. Build, sign, notarise (Week 6)

Deferred. See `DECISIONS.md` for the deferred Apple Developer account setup. Until then, run unsigned with `pnpm tauri build` and accept the Gatekeeper warning on first launch (right-click → Open).

---

## 7. Troubleshooting quick refs

| Symptom | Likely cause | Fix |
|---|---|---|
| `tauri dev` hangs on "waiting for frontend" | Next.js dev server crashed | Check terminal output, restart |
| "Operation not permitted" on capture | Screen Recording permission missing | System Settings → Privacy → Screen Recording → enable Lume → relaunch |
| Hotkey does nothing | Accessibility permission missing or hotkey collision | Check System Settings → Privacy → Accessibility |
| `sqlite_vec` not found | Extension not loaded before query | Confirm `db.rs` init runs at startup |
| Claude vision returns "image too large" | Frame above 2576px long edge (the vision model's limit) | Pre-resize before sending; see `lib/anthropic.ts` |
