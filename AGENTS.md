# AGENTS

Shattered Pixel Dungeon seed analyzer: **Bun + Vite + React + shadcn** UI, **Rust → WASM** engine.

Progress / resume: `specs/implementation.md`  
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
bun install
bun run dev          # wasm-pack + Vite
bun run build
bun run deploy       # build + Cloudflare Worker SPA (wrangler)
bun run test:rust    # cargo test -p spd-core
bun run build:wasm
bun run check        # biome check (TS/JS/CSS/JSON)
bun run check:fix    # biome check --fix
bun run format       # biome format + cargo fmt
bun run lint         # biome lint + cargo clippy -D warnings
bun run check:all    # biome + rust fmt/clippy checks
```

Use **rustup** cargo for wasm (`PATH` scripts prepend `$HOME/.cargo/bin`).

## Rules

- **RNG-PARITY** — Match SPD call order and algorithms (`java.util.Random`, watabou stack, decks). Prefer porting from the pinned clone over inventing shortcuts.
- **NO-CLAIM** — Do not claim full seed-finder accuracy while status is `partial`. Call out incompleteness.
- **CORE-FIRST** — Generation logic in `spd-core` only; `spd-wasm` stays a thin façade; UI does not reimplement RNG.
- **BUN-WEB** — Package manager is Bun. UI: Vite + React + shadcn. Do not introduce npm/yarn as primary.
- **WASM-REBUILD** — After Rust changes, rebuild wasm (`bun run build:wasm` / `dev`) before treating UI as verified.
- **TEST-RUST** — Add/extend `spd-core` tests for RNG and analyze paths; keep smoke coverage on `analyze_seed`.
- **ADV-MAP** — Floor maps and identity tables are opt-in spoilers (`Map spoilers` / `Show identities`). Keep default UX non-spoiler; map/identity data may still be in the report.
- **ASSETS-FLAT** — Assets live at `web/public/assets/{environment,sprites,…}`. No nested `assets/assets/`.
- **PIN-SPD** — Target the pinned SPD version/commit; note version impact when porting from a newer tree.
- **GPL-AWARE** — SPD is GPL-3.0; ports of generation logic inherit that constraint for distribution.
- **MIN-DIFF** — Prefer small, task-scoped diffs; no drive-by refactors.
- **HAND-OFF** — After multi-step work, update `specs/implementation.md` when behavior or next steps change.
- **SMALL-FILES** — Keep source files focused and reviewable. Soft target **≤ ~300 lines**; treat **~500 lines** as a hard ceiling for *new* growth (not an excuse to bloate existing files further). When a change would push a file past ~500, **extract a module first** (same package/`mod`, sibling component, or `lib/` helper) rather than appending. Split by **cohesive responsibility** (room family, UI panel, prize helpers), not arbitrary line cuts. Prefer many small modules + a thin orchestrator over god-files. Does **not** apply to generated output (`web/src/wasm/`), vendored assets, lockfiles, or third-party UI primitives under `web/src/components/ui/` unless we own substantial custom logic there. When expanding an already-oversized file (see known offenders below), budget extraction into the same task when practical.

## Do not

- Commit generated `web/src/wasm/` (gitignored).
- Panic-paths that become browser `"unreachable"` without bounds checks (see drop-cell occupancy).
- Full asset re-imports unless requested.
- Grow known god-files without extracting: especially `web/src/App.tsx`. (`level/special_loot/` was split; keep new room files focused.)
