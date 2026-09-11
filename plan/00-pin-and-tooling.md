# 00 — Pin and tooling

Status: **closed**

No generation logic. After this workstream the clone, constants, oracle runner, and fixture *pin strings* all name v4.0.0. Rust still implements v3.3.8; tests against regenerated v4 goldens are expected red until 01–06 land.

## Why it matters

**PIN-SPD.** Every later PR is wrong if oracles still export from `7b8b845a7` or if `SPD_VERSION` still says v3.3.8. The Java oracle refuses to run unless `HEAD` equals `EXPECTED_COMMIT`. Visual tests embed `spd_version` / `spd_commit` in the report they screenshot.

## Java sources (compile/run only)

- Clone `HEAD` must be `2bb34a4e91d29c8785a9363cad6ddfe5122b1d4f` (`v4.0.0`).
- Oracle Java lives under `tools/java-oracle/src/com/shatteredpixel/shatteredpixeldungeon/tools/`. It is compiled against an export of that clone (`tools/java-oracle/run` copies `HEAD` to a temp tree).
- v4 API breaks the oracle may hit while compiling (fix here, no analyzer RNG):
  - `Imp.Quest.reward` is `Item`, not `Ring` (`actors/mobs/npcs/Imp.java`).
  - `Level.customTerrain` is a new `ArrayList<CustomTilemap>` (`levels/Level.java`). `FloorVisualFacts.java` currently captures `customTiles` + `customWalls` only.
  - `MineEntrance` paints `ENTRANCE_SP` / `EMPTY_SP` (`levels/rooms/quest/MineEntrance.java`); `MiningLevelOracle.java` still compiles but fixtures will change.
  - `RegularPainter.hiddenDoorChance` is now a public method (`levels/painters/RegularPainter.java`); vault uses it.

## Analyzer files that will need to change

Pin strings and runner only:

- `AGENTS.md` — “Pinned game: SPD v3.3.8 @ `7b8b845a7`”
- `README.md` — version table
- `crates/spd-core/src/lib.rs` — `SPD_VERSION = "v3.3.8"`, `SPD_COMMIT = "7b8b845a7"`
- `crates/spd-wasm/src/lib.rs` — re-exports those constants
- `crates/spd-core/src/dungeon_seed.rs` — Daily clamp moved: `FIRST_DAILY_DATE` `2025-03-01` / epoch day `20_148` → **`2026-04-01` / `20_544`** (`HeroSelectScene.java:742-744`). Update the version string in `DailyBeforeStart` and the smoke date in `analyze_smoke.rs` (`"2025-03-01"` is no longer a v4 Daily).
- `tools/java-oracle/run` — `EXPECTED_COMMIT="7b8b845a76fe76c6b7c031ae9e570852411f56db"`
- `tools/java-oracle/README.md`
- `tools/java-oracle/src/.../JavaOracle.java` and every oracle that inlines `"v3.3.8"` / `"7b8b845a7"`:
  - `FloorOracle.java`, `GeneratorDeckOracle.java`, `GeneratorLifecycleOracle.java`, `ImpRingDeckOracle.java`, `SacrificeRewardOracle.java`, `MiningLevelOracle.java`, plus the other `*Oracle.java` files in that package
- `tools/visual/tests/map-render.spec.ts`, `tools/visual/tests/quest-rewards.spec.ts` — `spd_version` / `spd_commit`
- `web/src/lib/dungeon-tile-visuals.ts`, `web/src/lib/map-entities.ts` — header comments naming v3.3.8
- `specs/observed-outcomes.json` — `"version": "v3.3.8"`
- `specs/generator-decks.md` — **do not rewrite facts here**; 01 owns that. A one-line “pin moved, facts stale until 01” is enough if the file would otherwise lie.
- `specs/analysis/*` — **do not touch** (parallel rewrite).
- `specs/implementation.md` — **do not touch**.

Fixture bodies (mechanical regen from v4 Java, still no Rust ports):

- `tools/java-oracle/fixtures/**/*.json` including `generator/`, `mining/`, `shop/`, `secret/`, `traces/`, `player-state/`, and every `*-final-heaps-floor-*.json`
- Playwright PNG baselines stay in 06. This workstream only changes the version strings those tests assert.

## Acceptance

- `tools/java-oracle/run AAA-AAA-AAA` succeeds against clone `HEAD == 2bb34a4e91d29c8785a9363cad6ddfe5122b1d4f` and writes `"spd": { "version": "v4.0.0", "commit": "2bb34a4e9" }`.
- `crates/spd-core` `SPD_VERSION` / `SPD_COMMIT` match that.
- `AGENTS.md` and `README.md` name v4.0.0 @ `2bb34a4e9`.
- Daily first date is `2026-04-01` (epoch day 20,544). `analyze_seed("2026-04-01", 1)` is the smoke date; `"2025-03-01"` returns `DailyBeforeStart`.
- Regenerated fixture JSON is committed. Existing Rust goldens (`crates/spd-core/tests/java_oracle_goldens.rs` and children, `quests/imp/tests.rs` ring-deck includes) **fail** against them until 01–04. That failure is the gap, not a skip.
- Oracle Java compiles with JDK 17 against the v4 export. No `Ring` cast on `Imp.Quest.reward`.
- Visual tests still run; they may fail pixels (06) but must not assert `v3.3.8`.
- `bun run check:rust` still compiles. Do not “fix” generation to silence golden diffs in this workstream.

## Suggested PRs

1. `chore(pin): bump SPD pin to v4.0.0 @ 2bb34a4e9` — constants, AGENTS.md, README.md, oracle `EXPECTED_COMMIT`, wasm re-exports, visual spec version strings, Daily first-date `2026-04-01` / epoch day `20544`.
2. `fix(oracle): compile java-oracle against v4 Level.customTerrain and Imp.Quest` — oracle Java only; add `custom_terrain` capture next to `custom_tiles` / `custom_walls` in `FloorVisualFacts.java` and the JSON schema the Rust goldens already parse (`crates/spd-core/tests/java_oracle_goldens.rs` `OracleCustomTile`).
3. `test(oracle): regenerate v4 java-oracle fixtures` — rerun `./tools/java-oracle/run` for every contract in `tools/java-oracle/README.md` (identity, `--depth 1`, `--final-heaps-depth 1..26`, `--imp-ring-deck`, `--generator-lifecycle`, `--generator-deck-rollover`, `--mining-level`, `--shop-bag-selection`, `--secret-library-order`, traces). Commit JSON. Leave Rust tests red.
4. `docs(oracle): retarget java-oracle README to v4.0.0` — commit hash, usage blurb. No analysis docs.

PR 3 is mechanical but large; do not mix it with Generator ports.

## Dependencies

- None. First shippable workstream.
- Clone must already be at the target commit (it is).

## Explicit non-goals

- Any `spd-core` generation/layout/quest port.
- Rewriting `specs/generator-decks.md` facts (01) or `specs/analysis/*`.
- Refreshing `tools/visual/snapshots/*.png` (06).
- Copying `web/public/assets/` (06).
- Making `bun run test:rust` green against the new goldens.
- Dropped pre-v3.1.1 save support (07).
