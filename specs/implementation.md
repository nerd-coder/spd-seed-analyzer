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
- **Output:** Per-floor items + (Map spoilers) floor maps using original tilesheets  


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
│       ├── App.tsx            # seed form, spoiler toggles, results
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
| Room geometry | `rooms/room.rs` | connections, setSize (NormalIntRange); `Room.random` |
| Loop builder | `builders/` | placeRoom, findFreeSpace, tunnels, branches |
| Figure-eight | — | **Falls back to loop** (not parity) |
| Minimal paint | `level/terrain.rs` | SPD Terrain IDs; walls/empty/doors/entrance/exit; solid/openSpace helpers |
| Special-room prizes | `level/special_loot.rs` | Crypt/Armory/Library/Treasury/Pool/Storage/Runestone/Lab/Statue + several secrets; room-shuffle + placeDoors RNG; may `findPrizeItem` from itemsToSpawn |
| Shop stock | `level/shop.rs` | `ShopRoom.generateItems` (FOR_SALE); bag pick hero-less (scroll holder first); generated post-build (not mid-setSize) |
| Ghost quest | `quests/ghost.rs` | `Ghost.Quest.spawn` on sewers: chance, placement (approx openSpace), weapon/armor rewards |
| Wandmaker quest | `quests/wandmaker.rs` | `spawnRoom` on prison 7–9 (before shuffle); two +1 wands; MassGrave/Ritual/RotGarden side-effects (approx) |
| Blacksmith quest | `quests/blacksmith.rs` | `Blacksmith.Quest.spawn` on caves 12–14 (before shuffle); `generateRewards(true)` smith pool (2 weapons + missile + armor + optional enchant/glyph); room paint drops 2 equip |
| Imp quest | `quests/imp.rs` | `Imp.Quest.spawn` on city 17–19 (before shuffle); cursed +2 ring reward generated at initRooms |
| Crystal rooms | `level/special_loot.rs` | Vault (wand/ring/artifact crystal chests + mimic chance), Choice (pots/scrolls + chest), Path (3+3 consumables with dedup/sort) |
| Main createItems | `level/create_items.rs` | nItems loop, heap types; drop cells use map origin |
| Floor map export | `report.rs` `FloorMap` | width/height/tileset/tiles; items include `class_name` |

### Frontend
| Area | Notes |
|------|--------|
| Seed analyze UI | identities + floors + items |
| **Item icons** | `ItemIcon` + `lib/item-icons.ts` crops `/assets/sprites/items.png` (ItemSpriteSheet indices); potions/scrolls/rings use identity appearance |
| **Spoiler toggles** | localStorage; identity table + map spoilers off by default |
| Map canvas | `tiles.ts` + region tilesheets under `/assets/environment/` |
| Assets | Flattened to `web/public/assets/{environment,sprites,…}` (no nested `assets/assets`) |
| App icon | `web/public/app_icon.jpg` |

### Bugs fixed recently
- **WASM `"unreachable"` (depth 26):** UI always analyzes 26 floors. Depth 26 is SPD `LastLevel` (not RegularLevel); `secrets_for_floor` used `region = depth/5 == 5` into a 5-slot array → OOB panic → browser `"unreachable"`. Fixed by skipping non-regular depths (`regular_level()`: bosses 5/10/15/20/25 + last 26) and bounds-guarding `secrets_for_floor`.
- **WASM `"unreachable"` (drop cells):** `create_items` used synthetic cell ids `x + y*1000` into `occupied[]` → OOB panic. Fixed via `TerrainMap::point_to_cell`.
- Awkward `public/assets/assets/` nesting flattened.

---

## Accuracy disclaimer

Results are **partial**. Not game-parity yet because:

1. Full `RegularPainter` (water/grass/traps) incomplete — special-room prizes approximate  
2. Some special/secret rooms still stubbed (sentry/traps/fire/sacrifice, honeypot fidelity, …)  
3. Shop stock timing is post-build (SPD generates during room `setSize`); bag choice is hero-less  
4. Ghost quest rewards ported; placement uses minimal openSpace; full `createMobs` not ported  
5. Wandmaker + Blacksmith + Imp quests ported; RotGarden full paint/mobs incomplete; CrystalPath/Choice placement geometry approximate  
6. Figure-eight builder incomplete  
7. `randomDropCell` simplified vs full map flags  
8. Sewer room-count tables used for all regions  

Status string: `"partial"`.

---

## Not done / next phases

### P1 — Special-room loot + quests (high value for seed-finder UX)
- ~~Port `paint()` prize logic: Crypt, Armory, Library, Treasury, Statue, Pool, secrets~~ (partial; see `special_loot.rs`)  
- ~~Shop (FOR_SALE) stock~~ (approx; see `level/shop.rs`)  
- ~~Ghost.Quest rewards~~ (see `quests/ghost.rs`)  
- ~~Wandmaker.Quest~~ (see `quests/wandmaker.rs`; MassGrave loot + ritual candles + rot-garden RNG approx; RotGarden heart/lasher not ported)  
- ~~Imp.Quest~~ (see `quests/imp.rs`; cursed +2 ring at initRooms; AmbitiousImpRoom paint is placement RNG only)  
- ~~Crystal rooms~~ (Vault / Choice / Path prize gen in `special_loot.rs`; Path geometry not painted, placement RNG approximate)  
- ~~Blacksmith.Quest~~ (see `quests/blacksmith.rs`; smithRewards at initRooms; room paint equip drops; mining branch not ported)  
- Remaining: sentry/traps/fire/sacrifice prize rooms, honeypot secret fidelity  
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

### Spoiler toggles (UI)
- **Show identities (spoilers):** `localStorage["spd-analyzer-identity-spoilers"]` = `"1"` | `"0"` (default off). Hides the Identities card when off.
- **Map spoilers:** `localStorage["spd-analyzer-map-spoilers"]` = `"1"` | `"0"` (default off). Maps only rendered when on (map data still returned from WASM).
- Legacy: `spd-analyzer-advanced-mode` is still read as a fallback for map spoilers.

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
2. Open `crates/spd-core/src/lib.rs` → `analyze_seed` / `level/mod.rs` / `level/special_loot.rs` / `quests/{ghost,wandmaker,blacksmith,imp}.rs` / `level/shop.rs`  
3. Next recommended work: **SentryRoom / TrapsRoom / MagicalFireRoom / SacrificeRoom** prizes (or **SecretHoneypot** fidelity); then **Java golden checks** when ready; painter water/grass/traps (P2) for drop-cell parity  
4. Icons: `web/src/lib/item-icons.ts` + `components/ItemIcon.tsx` (items.png sheet)  
5. Do not re-copy full asset tree; use `web/public/assets/` as flattened SPD assets  
6. After Rust changes: `bun run build:wasm` (or `bun run dev`)  
7. Dev dump: `cargo run -p spd-core --example dump_seed -- SEED FLOORS`  
