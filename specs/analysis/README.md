# Analysis — SPD v4.0.2 source of trust

Target: **Shattered Pixel Dungeon v4.0.2 @ `57a4e06a4`**
(`57a4e06a4caf162446d1c28caa7983f0493fecf0`). Java clone:
`/Users/toan/code/repos/00-Evan/shattered-pixel-dungeon`.

These notes and `specs/generator-decks.md` are the generation facts for this
pin. The analyzer remains a partial port; see `specs/implementation.md`.

Public reports follow **SEED-ANALYSIS**, **SPAWN-PRESENCE**, and
**MAP-LAYOUT-GOAL**. Close the gap with `plan/`.

## Index

| File | Covers |
|------|--------|
| [generator-decks.md](generator-decks.md) | Category sub-decks, RING sites, artifact→ring defaults fallback, Imp/vault draw sites, `WEP_T3` probs |
| [weapon-enchants.md](weapon-enchants.md) | Weapon enchant/curse tables, glyph tables, `enchant()`/`inscribe()` streams, Unstable’s combat list |
| [quest-ambitious-imp.md](quest-ambitious-imp.md) | Imp spawn, six-item `rewardOptions`, vault entry, score / `earnedShop`, depth-20 shop |
| [vault-level.md](vault-level.md) | `VaultLevel` seed, GridBuilder, defaults-only loot, hazards, boss lock-in, public-map scope |
| [vault-rooms.md](vault-rooms.md) | Vault room catalogue, treasure round-robin, token heaps |
| [quest-sad-ghost.md](quest-sad-ghost.md) | Ghost spawn depths, pre-rolled pair, Parchment keep test |
| [quest-wandmaker.md](quest-wandmaker.md) | Wandmaker spawn, WAND deck + `undoDrop`, Mass Grave / Ritual paint order |
| [quest-blacksmith.md](quest-blacksmith.md) | Smith pool, Crystal/Gnoll, mining gold-in-room |
| [floor-layout-run-settings.md](floor-layout-run-settings.md) | Painter-complete facts vs declared profile |
| [level-feelings.md](level-feelings.md) | `Int(14)` feelings, Mossy Clump / Trap Mechanism decks |
| [daily-runs.md](daily-runs.md) | Daily seed formula; first Daily is `2026-04-01` |
| [first-four-floor-loot-simulation.md](first-four-floor-loot-simulation.md) | Floors 1–4 replay boundary |
| [map-tiles-and-assets.md](map-tiles-and-assets.md) | Terrain IDs, carpets, occlusion, asset inventory |

Citations are `Class.java:start-end` in the pinned clone.
