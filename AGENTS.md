# AGENTS

Shattered Pixel Dungeon seed analyzer: **Bun + Vite + React + shadcn** UI, **Rust → WASM** engine.

Progress / resume: `specs/implementation.md` (create one if missing and saving progress is requested)
Pinned game: SPD **v3.3.8** @ `7b8b845a7` — local clone often at `/Users/toan/code/repos/00-Evan/shattered-pixel-dungeon`

## Layout

| Path | Role |
|------|------|
| `crates/spd-core` | Pure Rust RNG + generation (no wasm) |
| `crates/spd-wasm` | `wasm-bindgen` exports |
| `web/` | Vite app; `web/src/wasm/` is **generated** |
| `web/public/assets/` | SPD assets (flat; tiles under `environment/`) |
| `specs/` | Design / implementation notes |

## Commands

```bash
mise install         # once: rust (rustup-backed), bun, temurin-17, wasm-pack
bun install
bun run dev          # wasm-pack + Vite
bun run build
bun run deploy       # build + Cloudflare Worker SPA (wrangler)
bun run test:rust    # cargo test -p spd-core
bun run test:map-render # map-render fixture registry
bun run test:visual  # build + Playwright Chromium pixel comparison
bun run install:visual-browser # once: install Playwright Chromium
bun run build:wasm
bun run check        # biome check (TS/JS/CSS/JSON)
bun run check:fix    # biome check --fix
bun run check:rust   # cargo fmt --check + clippy -D warnings (CI)
bun run format       # biome format + cargo fmt
bun run lint         # biome lint + cargo clippy -D warnings
bun run check:all    # biome + rust fmt/clippy checks
```

Local tooling is managed by **mise** (`mise.toml`: rust pinned, rustup-backed).
Keep mise activated in your shell, or prefix commands with `mise exec --`.
mise sets `RUSTUP_TOOLCHAIN`, and the rustup proxies in `~/.cargo/bin` honor
it, so the `PATH` prepends in package.json scripts are no-ops locally (they
exist for CI, which sets up its own toolchain). The
`wasm32-unknown-unknown` target persists in the shared `~/.rustup` store
(`rustup target add wasm32-unknown-unknown` once). Beware: running cargo
outside mise uses the rustup *default* toolchain, which may differ from the
pin (rustfmt output can drift between versions).

### CI parity (before done)

GitHub Actions `check` job (`.github/workflows/ci.yaml`) runs, in order:

1. `bun run check` — Biome  
2. `bun run check:rust` — `cargo fmt --all -- --check` + `cargo clippy --workspace --all-targets -- -D warnings`  
3. `bun run test:rust` — `cargo test -p spd-core`  
4. `bun run build` — wasm-pack + Vite production build  
5. `bun run test:visual:only` — Playwright deterministic map comparison

## Rules

- **RNG-PARITY** — Match SPD call order and algorithms (`java.util.Random`, watabou stack, decks). Prefer porting from the pinned clone over inventing shortcuts.
- **DECK-FACTS** — `specs/generator-decks.md` records verified `Generator` deck behaviour for the pinned version: category sub-decks (`seed`/`dropped`), per-depth stream isolation, the full ring draw-site list, which runtime paths bypass the decks, challenge and trinket rule-outs, and measured distributions. Read it before re-deriving any of that from the Java clone or claiming that run history can change a deck-drawn identity. It is settled truth until `PIN-SPD` moves; on a version bump re-verify it and update it in the same change. Add new verified deck findings there — evidence with `file:line` citations and the measurement method, not plans or coverage prose (those belong in `specs/implementation.md` and `specs/accuracy.json`).
- **NO-CLAIM** — Do not claim full seed-finder accuracy while status is `partial`. Call out incompleteness.
- **SEED-ANALYSIS** — Focus reports and seed-finder matches on facts determined by the seed for the pinned game version, especially items, loot, and rewards that are guaranteed to spawn. When player state or choices can change the concrete result, analyze the generation path deeply enough to report the possible outcomes and clearly distinguish their conditions while preserving every deterministic property, even when the result is partial. For example, report a guaranteed cursed weapon with `+2` upgrades even when the weapon type varies with player state. Exclude information driven by runtime RNG or otherwise not determined during seed generation, such as combat drops. Keep the public projection consistent across reports, map markers/heaps, WASM, search evidence, and UI.
- **SPAWN-PRESENCE** — For item results, assert only whether the item is guaranteed to spawn. Do not expose internal queue, room-consumption, heap, or placement lifecycle details when they do not change item presence.
- **MAP-LAYOUT-GOAL** — Public floor maps visualize deterministic painter-complete floor layout only: rooms, terrain, doors, transitions, traps, plants, and blobs captured before NPC, mob, heap, forced-item, Guide Page, or other item population. Do not expose item/mob placement or let their lifecycle uncertainty suppress an otherwise deterministic layout. Model and label player/meta assumptions only when they can change layout itself; keep final entity-rich maps internal for parity evidence.
- **ACCURACY-MANIFEST** — `specs/accuracy.json` is the canonical source of truth for user-facing coverage, known gaps, and accuracy status. It is rendered verbatim in the UI accuracy dialog, so write it at player altitude: no seed IDs, fixture filenames, oracle/replay jargon, or internal-vs-public wording. Parity evidence lives in the tests and fixtures themselves, not here. Any change that adds, removes, verifies, or invalidates generation behavior must update the manifest in the same change *when it changes what a user can rely on*; fixture-level work that does not move a coverage claim only needs `lastReviewed`. Keep the pinned version/commit and overall status aligned with `spd-core`; do not duplicate coverage prose in Rust or UI code—the backend message stays generic and the UI renders the manifest.
- **CORE-FIRST** — Generation logic in `spd-core` only; `spd-wasm` stays a thin façade; UI does not reimplement RNG.
- **BUN-WEB** — Package manager is Bun. UI: Vite + React + shadcn. Do not introduce npm/yarn as primary.
- **DEV-SERVER** — Never start the development server yourself. Check whether it is currently running; if it is not running or needs a restart, ask the user to start or restart it.
- **WASM-REBUILD** — After Rust changes, rebuild wasm (`bun run build:wasm` / `dev`) before treating UI as verified.
- **TEST-RUST** — Add/extend `spd-core` tests for RNG and analyze paths; keep smoke coverage on `analyze_seed`.
- **CI-BEFORE-DONE** — Before marking a task complete, committing work as finished, or claiming “done”, run the same checks as CI’s `check` job (see **CI parity** above). Minimum for any Rust-touching change: `bun run check:rust` and `bun run test:rust`. If TS/web or wasm exports changed, also `bun run check` and `bun run build`. If browser rendering or the visual harness changed, also run `bun run test:visual:only` after the build. Fix fmt/clippy/test/build/visual failures before hand-off; do not skip with “clippy later”.
- **ASSETS-FLAT** — Assets live at `web/public/assets/{environment,sprites,…}`. No nested `assets/assets/`.
- **PIN-SPD** — Target the pinned SPD version/commit; note version impact when porting from a newer tree.
- **HAND-OFF** — Optional `specs/implementation.md` is the resume point: current state plus the plan for what comes next. Update it after multi-step work when behavior or next steps change. Keep it **lean and concise** — a short state summary and the ordered next steps, nothing more. It is not a changelog: do not accumulate completed-work history, session logs, or narrative of what was already tried. Rewrite stale sections in place and delete finished ones rather than appending; git history is the record of the past.
- **SMALL-FILES** — Keep source files focused and reviewable. Soft target **≤ ~300 lines**; treat **~500 lines** as a hard ceiling for *new* growth (not an excuse to bloate existing files further). When a change would push a file past ~500, **extract a module first** (same package/`mod`, sibling component, or `lib/` helper) rather than appending. Split by **cohesive responsibility** (room family, UI panel, prize helpers), not arbitrary line cuts. Prefer many small modules + a thin orchestrator over god-files. Does **not** apply to generated output (`web/src/wasm/`), vendored assets, lockfiles, or third-party UI primitives under `web/src/components/ui/` unless we own substantial custom logic there. When expanding an already-oversized file, budget extraction into the same task when practical.
- **CONVENTIONAL-COMMITS** — Commit messages must follow the Conventional Commits format (e.g. `feat: add feature`, `fix: bug fix`, `docs: update docs`). Use types, scopes, bodies, and footers as per the specification for clarity and machine-readability.
