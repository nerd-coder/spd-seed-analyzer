# SPD Seed Analyzer

Browser-based **Shattered Pixel Dungeon** seed analyzer.

- **Web:** Vite + React + **shadcn/ui** + Tailwind v4 (package manager: **Bun**)
- **Engine:** Rust → WebAssembly (`spd-core` / `spd-wasm`)

Generation logic is ported from the official game so results match the same RNG. Target version is pinned below.

| Field | Value |
|-------|--------|
| SPD version | v3.3.8 |
| SPD commit | `7b8b845a7` |
| Local source | `/Users/toan/code/repos/00-Evan/shattered-pixel-dungeon` |

## Prerequisites

- [Rustup](https://rustup.rs/) with `wasm32-unknown-unknown`  
  (`rustup target add wasm32-unknown-unknown`)  
  Prefer rustup’s `cargo`/`rustc` on `PATH` (scripts prepend `$HOME/.cargo/bin`).
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- [Bun](https://bun.sh/)

## Develop

```bash
# from repo root
bun install
bun run dev
```

This builds the WASM package into `web/src/wasm`, then starts the Vite dev server.

### shadcn/ui

UI lives under `web/` with `components.json` (New York / zinc). Add components with:

```bash
cd web && bunx shadcn@latest add <component>
```

## Scripts

| Command | Description |
|---------|-------------|
| `bun run dev` | Build WASM + start Vite |
| `bun run build` | Production build |
| `bun run build:wasm` | WASM only |
| `bun run test:rust` | `cargo test -p spd-core` |
| `bun run preview` | Preview production build |

## Status

- [x] Monorepo scaffold (Cargo + Bun/Vite/shadcn)
- [x] `java.util.Random` + watabou `Random` stack
- [x] `DungeonSeed` parse/format
- [x] WASM `parse_seed` / `analyze_seed`
- [x] Run init (potion/scroll/ring identity maps + room/generator deck RNG)
- [x] Item generator (decks, tiers, enchants/glyphs, item.random)
- [x] Partial levelgen (forced drops + feelings per floor)
- [x] Room selection (`initRooms` + builder kind) for regular floors
- [ ] Geometry build / painters / createItems (heaps, mobs, quests)

## License

GPL-3.0-or-later (derivative of Shattered Pixel Dungeon generation logic).
