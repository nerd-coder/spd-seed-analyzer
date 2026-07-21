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
- **ADV-MAP** — Floor maps are Advanced-mode spoilers. Keep default UX non-spoiler; map data may still be in the report.
- **ASSETS-FLAT** — Assets live at `web/public/assets/{environment,sprites,…}`. No nested `assets/assets/`.
- **PIN-SPD** — Target the pinned SPD version/commit; note version impact when porting from a newer tree.
- **GPL-AWARE** — SPD is GPL-3.0; ports of generation logic inherit that constraint for distribution.
- **MIN-DIFF** — Prefer small, task-scoped diffs; no drive-by refactors.
- **HAND-OFF** — After multi-step work, update `specs/implementation.md` when behavior or next steps change.

## Do not

- Commit generated `web/src/wasm/` (gitignored).
- Panic-paths that become browser `"unreachable"` without bounds checks (see drop-cell occupancy).
- Full asset re-imports unless requested.
