# SPD Seed Analyzer — Implementation Progress

**Last updated:** 2026-07-21  
**Branch:** `main`  
**Pinned SPD:** v3.3.8 @ `7b8b845a7`  
**Local game source:** `/Users/toan/code/repos/00-Evan/shattered-pixel-dungeon`

---

## Goal

Browser seed analyzer for Shattered Pixel Dungeon:

- **UI:** Bun + Vite + React + shadcn/ui  
- **Engine:** Rust (`spd-core`) → WASM (`spd-wasm`)  
- **Output:** Per-floor items + (Advanced mode) floor maps using original tilesheets  

Reference behavior: headless Java seed finders (`Dungeon.init` → `newLevel` per depth).

---

## Repo layout

```text
spd-seed-analyzer/
├── Cargo.toml                 # workspace
├── package.json               # Bun scripts (build:wasm, dev, build)
├── crates/
│   ├── spd-core/              # pure Rust generation logic
│   └── spd-wasm/              # wasm-bindgen façade
├── web/                       # Vite app
│   ├── public/
│   │   ├── app_icon.jpg
│   │   └── assets/            # SPD asset tree (flattened)
│   │       └── environment/tiles_*.png
│   └── src/
│       ├── App.tsx            # seed form, advanced mode, results
│       ├── components/FloorMapCanvas.tsx
│       └── lib/{spd-wasm.ts,tiles.ts}
└── specs/implementation.md    # this file
```

### Commands

```bash
bun install
bun run dev          # wasm-pack + Vite (prefers $HOME/.cargo/bin for rustup)
bun run build
bun run test:rust    # cargo test -p spd-core
bun run build:wasm
bun run check        # biome (TS/JS/CSS/JSON)
bun run format       # biome format + cargo fmt
bun run lint         # biome lint + cargo clippy -D warnings
bun run check:all    # biome + rust fmt/clippy
```

**Note:** mise/Homebrew `rustc` may lack `wasm32-unknown-unknown`; scripts prepend rustup’s cargo.

---

## What works (done)

### Foundations
| Area | Location | Notes |
|------|----------|--------|
| `java.util.Random` LCG | `java_random.rs` | OpenJDK-compatible |
| watabou `Random` stack | `random.rs` | push/pop, scrambleSeed (MX3), chances, shuffle, NormalIntRange |
| `DungeonSeed` | `dungeon_seed.rs` | codes, numeric, fun-text seeds |
| Run init (`seed+1`) | `run.rs` | identities + room decks + generator fullReset |
| Potion/scroll/ring IDs | `items/identities.rs` | UI tables |
| Generator decks/tiers | `generator/` | random weapons/armor/missiles/artifacts + item.random |
| Depth seeds / limited drops | `dungeon/` | pos/sou/stylus/stones/cata/lab |

### Levelgen (partial)
| Area | Location | Notes |
|------|----------|--------|
| `initRooms` | `rooms/init_rooms.rs` | entrance/exit/standard/special/secret + shuffle |
| Room geometry | `rooms/room.rs` | connections, setSize (NormalIntRange) |
| Loop builder | `builders/` | placeRoom, findFreeSpace, tunnels, branches |
| Figure-eight | — | **Falls back to loop** (not parity) |
| Minimal paint | `level/terrain.rs` | SPD Terrain IDs; walls/empty/doors/entrance/exit |
| Special-room prizes | `level/special_loot.rs` | Crypt/Armory/Library/Treasury/Pool/Storage/Runestone/Lab/Statue + several secrets; room-shuffle + placeDoors RNG; may `findPrizeItem` from itemsToSpawn |
| Main createItems | `level/create_items.rs` | nItems loop, heap types; drop cells use map origin |
| Floor map export | `report.rs` `FloorMap` | width/height/tileset/tiles; items include `class_name` |

### Frontend
| Area | Notes |
|------|--------|
| Seed analyze UI | identities + floors + items |
| **Item icons** | `ItemIcon` + `lib/item-icons.ts` crops `/assets/sprites/items.png` (ItemSpriteSheet indices); potions/scrolls/rings use identity appearance |
| **Advanced mode** | localStorage; spoilers warning; shows canvas maps |
| Map canvas | `tiles.ts` + region tilesheets under `/assets/environment/` |
| Assets | Flattened to `web/public/assets/{environment,sprites,…}` (no nested `assets/assets`) |
| App icon | `web/public/app_icon.jpg` |

### Bugs fixed recently
- **WASM `"unreachable"`:** `create_items` used synthetic cell ids `x + y*1000` into `occupied[]` → OOB panic. Fixed via `TerrainMap::point_to_cell`.
- Awkward `public/assets/assets/` nesting flattened.

---

## Accuracy disclaimer

Results are **partial**. Not game-parity yet because:

1. Full `RegularPainter` (water/grass/traps) incomplete — special-room prizes approximate  
2. Some special/secret rooms still stubbed (crystal rooms, sentry/traps/fire, shop stock, …)  
3. Quests (Ghost / Wandmaker / Imp / …) not ported  
4. Figure-eight builder incomplete  
5. `createMobs` (statues listed as loot only; mimics simplified) incomplete  
6. `randomDropCell` simplified vs full map flags  
7. Sewer room-count tables used for all regions  

Status string: `"partial"`.

---

## Not done / next phases

### P1 — Special-room loot + quests (high value for seed-finder UX)
- ~~Port `paint()` prize logic: Crypt, Armory, Library, Treasury, Statue, Pool, secrets~~ (partial; see `special_loot.rs`)  
- Remaining: Shop (FOR_SALE), crystal rooms, sentry/traps/fire/sacrifice, honeypot secret fidelity  
- Ghost.Quest / Wandmaker.Quest / Imp.Quest reward generation at correct RNG points (`createMobs` order)  
- Golden tests vs Java oracle for a handful of seeds  

### P2 — Painter parity
- Water/grass/trap placement RNG (affects special-room drop-cell parity)  
- Region-specific painters (SewerPainter, …)  
- Improve door placement / connection corridors  

### P3 — Builder parity
- Full `FigureEightBuilder`  
- Connection room variants fidelity  
- Robust build retries matching `Level.create` outer loop when paint fails  

### P4 — Map rendering polish
- Autotiling / raised walls (DungeonTileSheet)  
- Optional item/mob markers on canvas  
- Water animation sheets  

### P5 — Seed finder mode (post-v1)
- Constraint search over seeds (any/all items by floor)  

### P6 — Correctness infrastructure
- `tools/java-oracle`: headless dump JSON from local SPD clone  
- Golden fixtures in repo  

---

## Key algorithms / sequences to preserve

### `Dungeon.init` (run)
```text
Random.pushGenerator(seed + 1)
  Scroll.initLabels / Potion.initColors / Ring.initGems
  SpecialRoom.initForRun / SecretRoom.initForRun
  Generator.fullReset
Random.resetGenerators
```

### Per floor (`Level.create` subset we implement)
```text
Random.pushGenerator(seedForDepth(seed, depth, 0))
  forced drops (food, SoU, PoS, …) + feeling
  builder()  // loop vs figure-eight + curve params
  initRooms() + shuffle
  retry builder.build until success
  paint_minimal → FloorMap
  createItems main loop
Random.popGenerator
depth++
```

### `seedForDepth`
```text
pushGenerator(seed); Long() × depth; result = Long(); pop
```

---

## Frontend contracts

### WASM
- `parse_seed(input) → SeedInfo`
- `analyze_seed(input, floors) → SeedReport`
- `spd_version()` / `spd_commit()`

### `FloorMap` JSON
```json
{
  "width": 41,
  "height": 43,
  "tileset": "sewers",
  "tiles": [4, 4, 1, 5, 7, 8, ...]
}
```
Tiles are SPD `Terrain` values. Client maps to flat tilesheet indices (`web/src/lib/tiles.ts`).

### Advanced mode
- Key: `localStorage["spd-analyzer-advanced-mode"]` = `"1"` | `"0"`
- Maps only rendered when advanced is on (data still returned from WASM)

---

## Testing

```bash
cargo test -p spd-core
# includes analyze_smoke::analyze_seed_smoke / analyze_several_seeds
```

When adding features: prefer oracle comparisons for identity maps first, then single-floor items.

---

## License

SPD is GPL-3.0. This project ports generation logic → treat as **GPL-3.0-or-later** when publishing. Assets are from SPD and under the same license constraints.

---

## How to resume (clean context)

1. Read this file + `README.md`  
2. Open `crates/spd-core/src/lib.rs` → `analyze_seed` / `level/mod.rs` / `level/special_loot.rs`  
3. Next recommended work: remaining special rooms (Shop, crystal) + **Ghost quest** + Java golden checks  
4. Icons: `web/src/lib/item-icons.ts` + `components/ItemIcon.tsx` (items.png sheet)  
5. Do not re-copy full asset tree; use `web/public/assets/` as flattened SPD assets  
6. After Rust changes: `bun run build:wasm` (or `bun run dev`)  
7. Dev dump: `cargo run -p spd-core --example dump_seed -- SEED FLOORS`  
