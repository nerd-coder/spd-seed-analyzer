# 06 — Assets, map renderer, Playwright snapshots

Status: **open**

Copy v4 pixel art into the flat asset tree and teach the map renderer carpets, wall-floor shadows, new terrain, and consumable icons. Refresh visual snapshots last, after layout workstreams have landed.

## Why it matters

**MAP-LAYOUT-GOAL / ASSETS-FLAT.**

v4’s first art wave is universal wall-floor shadowing, city carpets / special floor, overhauled quest rooms, and new consumable sprites. Public maps that still blit v3.3.8 `tiles_city.png` without `carpet.png` or `occlusion_shadows.png` are factually the wrong game even when terrain IDs match.

Playwright snapshots (`tools/visual/snapshots/`) are the regression gate CI will run (`test:visual:only`). They still name v3.3.8 until 00; PNGs stay stale until this workstream.

## Java / asset sources (what to copy or port visually)

From the v4 clone `core/src/main/assets/`:

- `environment/tiles_{sewers,prison,caves,caves_crystal,caves_gnoll,city,halls}.png` — all changed.
- `environment/terrain_features.png`
- New: `environment/custom_tiles/carpet.png`, `environment/occlusion_shadows.png`, `environment/raised_terrain.png`, `environment/custom_tiles/rat_king_room.png`
- Quest/boss sheets: `custom_tiles/{caves_quest,caves_boss,city_quest,city_boss,prison_quest}.png` (city_quest 1028→11705 bytes).
- `sprites/items.png` + `ItemSpriteSheet.java` (consumables: seeds, darts, potions, elixirs, stones, scrolls, spells, dewdrop, liquid metal, blood vial, energy/exotic crystals, dwarf tokens).
- New sprites the renderer may not need for public maps: `SentrySprite.java` (uses `red_sentry.png` still), `VaultBossElementalSprite.java`, `VaultMirrorSprite.java`, `VaultTokenDoorSprite.java`. Copy if the UI ever shows vault entities; public maps should not.

Renderer ports (logic, not Java runtime):

- `tiles/DungeonTileSheet.java` — `getVisualWithAlts` now walks `tileAltVisuals` with per-visual chance arrays (floor 52.5%/5% alts; mining crystal colour alts via `updateAltVariants()`). Analyzer `web/src/lib/dungeon-tile-visuals.ts` still uses v3 `commonAlts` (≥50) / `rareAlts` (≥95) plus `HIGH_GRASS_UNDERHANG` slots v4 removed.
- `tiles/WallOcclusionTilemap.java` — new; wall-meets-floor shadows from `occlusion_shadows.png`.
- `tiles/RaisedTerrainTilemap.java` — uses `raised_terrain.png`.
- `tiles/custom/Carpet.java` — stitch + city overrides 80–88.
- `tiles/DungeonTerrainTilemap.java` / `TerrainFeaturesTilemap.java` — smaller diffs (water allow, features).
- `sprites/ItemSprite.java` — hidden heap alpha 0.15→0.1 (Spyglass). Only if hidden heaps are drawn; public layout maps omit heaps.

## Analyzer files that will need to change

Assets (flat, no `assets/assets/`):

- `web/public/assets/environment/` — replace tilesheets; add `occlusion_shadows.png`, `raised_terrain.png`; `custom_tiles/carpet.png`, `custom_tiles/rat_king_room.png`; replace quest/boss PNGs.
- `web/public/assets/sprites/items.png` (and `item_icons.png` if v4 changed it).
- Do not copy `interfaces/change_icons.png` / music unless a UI screen needs them (07).

Renderer / UI:

- `web/src/lib/dungeon-tile-visuals.ts` — `Terrain.CUSTOM_DECO_WTR = 39`; alt table matching `DungeonTileSheet.getVisualWithAlts`; drop removed underhangs; mining crystal colour alts when `tileset` is `caves_crystal` / `caves_gnoll`.
- `web/src/lib/tiles.ts` — draw carpets from `custom_tiles` with `texture === "carpet"`; draw `custom_terrain`; blit occlusion after walls. `raised_terrain` if the current raised pass still samples `tiles_*.png` only.
- `web/src/lib/map-assets.ts` — load `carpet.png`, `occlusion_shadows.png`, `raised_terrain.png`, `rat_king_room.png`.
- `web/src/lib/spd-wasm.ts` — `custom_terrain`; `BranchFloorReport.kind: 'imp_vault'`; Imp quest types (02 may already).
- `web/src/lib/item-icons.ts` — re-map `CLASS_ICON` indices from v4 `ItemSpriteSheet.java` (potions/seeds/stones/scrolls moved).
- `web/src/components/FloorMapCanvas.tsx`, `FloorMapPreview.tsx`, `MapSettingsPanel.tsx` — only if new layers need toggles (they should not; carpets are layout).
- `web/src/components/seed/QuestCard.tsx`, `FloorDetail.tsx` — if 02 did not finish UI.
- `crates/spd-core/src/level/terrain.rs` / `report.rs` — `CUSTOM_DECO_WTR`, `custom_terrain` if 03 did not.

Visual harness:

- `tools/visual/tests/map-render-fixtures.ts` — add a city floor with carpets (depth 16–19), an Imp/vault branch identity, keep Crystal/Gnoll mining.
- `tools/visual/tests/map-render.spec.ts` — version strings already v4 from 00; update PNG baselines with `bun run test:visual:update` **after** assets+renderer+layout goldens are green.
- `tools/visual/tests/quest-rewards.spec.ts` — Imp card copy (six options, no tokens).
- `tools/visual/snapshots/*.png`

Oracle: `FloorVisualFacts` `custom_terrain` (00). Rust `FloorMap` must round-trip it so the canvas sees carpets.

## Acceptance

- `web/public/assets/environment/custom_tiles/carpet.png` and `occlusion_shadows.png` exist (flat). No nested `assets/assets/`.
- City floor canvas for a seed whose Java `customTiles` contains `Carpet` shows the carpet atlas, not `EMPTY_SP` brown.
- Wall/floor junctions show occlusion shadows consistent with `WallOcclusionTilemap` (pixel-compare a 1-tile fixture if a full floor is noisy).
- Mining Crystal/Gnoll snapshots use the new tilesheet alt chances (`updateAltVariants`).
- Consumable icons in the report (seeds, potions, stones, scrolls) match v4 `items.png` frames for the classes we already display.
- `MAP_RENDER_FIXTURES` snapshots regenerated and `bun run test:visual:only` green after `bun run build`.
- Imp vault nested map, if 02 UI landed, has a committed PNG.

## Suggested PRs

1. `chore(assets): sync v4 tilesheets, carpets, occlusion, and items.png` — copy only. `ASSETS-FLAT`.
2. `feat(web): render carpets, custom terrain, and wall occlusion` — `map-assets.ts`, `tiles.ts`, `dungeon-tile-visuals.ts`. Needs `FloorMap.custom_terrain` from 03/00.
3. `feat(web): retarget item-icons to v4 ItemSpriteSheet` — `item-icons.ts` + a screenshot of the identities panel if one exists.
4. `feat(web): generalize branch maps for Imp vault` — skip if 02 PR 5 done.
5. `test(visual): refresh Playwright map and quest-reward snapshots for v4.0.0` — last. Includes city carpets, mining, vault, CXG/HKT sewer.

## Dependencies

- **00** pin strings.
- **02** vault maps before vault snapshots.
- **03** city carpet `custom_tiles` + feeling/builder so city pixels are the right layout.
- **04** mining gold/entrance before Crystal/Gnoll PNG refresh.
- Do not update snapshots in the same PR as a layout RNG change.

## Explicit non-goals

- `interfaces/change_icons.png`, changelog scene, health-bar incoming-damage darkening, targeted-cell VFX (07).
- Music / fonts except `pixel_font.ttf` if a UI string already uses it (optional; not maps).
- Vault mob sprites on the public map.
- iOS / appstore metadata images under `metadata/en-US/images/`.
- Hidden-heap Spyglass alpha unless we draw heaps on public maps (we should not).
