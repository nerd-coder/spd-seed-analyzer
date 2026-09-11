# 01 — Generator decks, enchants, new draw sites

Status: **open**

Port `Generator` and weapon enchant *tables* to v4, then re-verify `specs/generator-decks.md`. Do not implement the vault level or replace the Imp quest report here — only the deck mechanics and oracles those later PRs will need.

## Why it matters

**DECK-FACTS / SEED-ANALYSIS / RNG-PARITY.**

v3.3.8 `Generator.random(ARTIFACT)` fell back to `random(Category.RING)`, which advances `RING.dropped`. Extra artifacts (Wealth farming, Spyglass hidden items landing on ARTIFACT, transmutation, …) therefore shifted later ring identities, including the Imp ring. v4.0.0 falls back to `randomUsingDefaults(Category.RING)` so that leak is closed (`Generator.java` around the `case ARTIFACT` branch; changelog: “generating extra artifacts could affect ring generation in later depths”).

The analyzer still does the v3 fallthrough:

```158:169:crates/spd-core/src/generator/state.rs
    pub fn random_category(&mut self, cat: Category, depth: i32) -> GeneratedItem {
        // ...
            Category::Artifact => {
                if let Some(item) = self.random_artifact(depth) {
                    item
                } else {
                    self.random_category(Category::Ring, depth)
                }
            }
```

`specs/generator-decks.md` §3–§4 and §11 currently document that leak as settled v3.3.8 truth. After this workstream they must document the v4 fallback and the new Imp/vault draw sites.

WEP_T3 static init in Java cloned `WEP_T1.defaultProbs` (length 1) instead of `WEP_T3.defaultProbs`. `fullReset()` copies `defaultProbs` so ordinary runs were already fine; v4 clones the T3 array. Analyzer `generator/categories.rs` already uses `WEP_T3_PROBS = [2; 6]` — keep it, and pin it in the lifecycle oracle so a restore/reset path cannot regress.

Enchant *type* weights stay `{50, 40, 10}`. Bucket membership changes, so a generated “random enchant” identity changes:

- common: +`Venomous`
- uncommon: +`Eldritch`, +`Vorpal`
- rare: +`Crystal`
- curses: +`Pressurized`, +`Wondrous`

`Unstable.java`’s on-hit pool also gained those names — that is combat (07), not this file.

## Java sources (what to port)

- `items/Generator.java`
  - `WEP_T3.probs = WEP_T3.defaultProbs.clone()` (was `WEP_T1.defaultProbs.clone()`).
  - `case ARTIFACT`: `return item != null ? item : randomUsingDefaults(Category.RING);` plus the comment “do not use decks for that ring, as the # of artifacts genned can vary by gameplay”.
  - pre-v3.3.0 artifact bundle conversion: tome special-case removed (save-compat; analyzer is not a loader, ignore unless `restoreFromBundle` is ported).
- `items/weapon/Weapon.java` — `Enchantment.common/uncommon/rare/curses` arrays; `typeChances` values unchanged.
- `items/weapon/enchantments/{Venomous,Vorpal,Eldritch,Crystal}.java` — **names only** for identity; do not port `proc`.
- `items/weapon/curses/{Pressurized,Wondrous}.java` — names only.
- New **levelgen** draw sites (document + oracle; Imp spawn implementation is 02):
  - `actors/mobs/npcs/Imp.java` `Quest.spawn` (~lines 308–358 in v4): `randomArtifact()`; on miss `random(RING)`; second `random(RING)` with class uniqueness; `Random.Int(2)` then `random(WEP_T5)+random(MIS_T4)` or `random(MIS_T5)+random(WEP_T4)`; `new PlateArmor().inscribe()` (no armor deck); `random(WAND)`. All `cursed = false`. Artifact `transferUpgrade(5)`; others `level(IntRange(…))` + `.enchant()` / `.inscribe()`.
  - `levels/VaultLevel.java` `createEquipment` / `createConsumabe`: **`randomUsingDefaults` only** for WEP/MIS/WAND/RING/FOOD. Does **not** advance RING/WEP/WAND decks. ARTIFACT is unused there. Banned rings: Wealth, Might, Force. Banned wands: Regrowth, Transfusion, Corruption.
- `items/trinkets/ExoticCrystals.java` — `consumableExoticChance` `0.125+0.125*level` → `0.2+0.2*level`. Call sites already roll `Random.Float()` in Generator (`Generator.java` ~731/759). Analyzer `consume_exotic_conversion_roll` in `generator/state.rs` always burns the float and never converts. Conversion identity is 05; this workstream only needs the roll to stay in the stream.

## Analyzer files that will need to change

- `crates/spd-core/src/generator/state.rs` — artifact fallback; keep `consume_exotic_conversion_roll`.
- `crates/spd-core/src/generator/categories.rs` — confirm WEP_T3 probs; no class-list change.
- `crates/spd-core/src/generator/tests.rs`, `generator/tests/lifecycle.rs`, `generator/tests/rollover.rs`
- `crates/spd-core/src/items/enchants.rs` — tables (can land in 01 or 05; Imp `.enchant()` in 02 needs them).
- `specs/generator-decks.md` — retarget pin, rewrite §3 ring-site table, §4 “VaultLevel only randomUsingDefaults”, §7 Imp measurements, §11 ARTIFACT leak (fallback no longer moves RING.dropped).
- Oracles:
  - `tools/java-oracle/src/.../ImpRingDeckOracle.java` — today records `RING.dropped` before/after `CityLevel.initRooms`. v4 Imp spawn also advances ARTIFACT, WEP_T4/T5, MIS_T4/T5, WAND. Extend the contract (or add `ImpRewardDeckOracle`) with those `dropped` counters and the reward-option class list.
  - `tools/java-oracle/src/.../GeneratorLifecycleOracle.java`, `GeneratorDeckOracle.java`
  - fixtures `tools/java-oracle/fixtures/generator/*-imp-ring-deck.json` and lifecycle/rollover JSON
- Tests that include those fixtures: `crates/spd-core/src/quests/imp/tests.rs` (will keep failing on quest semantics until 02; deck counters can be asserted as soon as spawn RNG exists — if 02 has not landed, assert Generator isolation tests only).
- `crates/spd-core/src/generator/tests.rs` artifact-exhaustion cases (search `random_artifact` / Ring fallback).

## Acceptance

- Exhausting the artifact deck and then calling `random(ARTIFACT)` returns a **defaults** ring and leaves `RING.dropped` unchanged. Covered by a `spd-core` unit test that would have failed on v3.3.8 behaviour.
- `Generator.fullReset` / lifecycle oracle: `WEP_T3.probs` length 6, values `{2,2,2,2,2,2}`.
- `random_weapon_enchant` / `random_weapon_curse` can emit the six new names; `typeChances` still `{50,40,10}`. A 10k-draw histogram is optional; a table-equality test against the Java arrays is enough.
- Regenerated `--generator-lifecycle`, `--generator-deck-rollover`, and expanded Imp deck fixtures match Rust.
- `specs/generator-decks.md` header is v4.0.0 @ `2bb34a4e9`, with `file:line` citations from that commit. §11 no longer claims artifact exhaustion moves the ring deck.
- `bun run check:rust` and `bun run test:rust` green for generator/enchant tests. Imp *quest report* tests may stay red until 02 if this PR does not touch `quests/imp.rs`.

## Suggested PRs

1. `fix(core): use ring defaults when the artifact deck is exhausted` — `state.rs` + unit test + `specs/generator-decks.md` §4/§11.
2. `fix(core): clone WEP_T3 default probs and pin them in the lifecycle oracle` — tiny; may fold into PR 1.
3. `feat(core): add v4 weapon enchant and curse tables` — `items/enchants.rs` + table test. Needed before 02’s `.enchant()` identities.
4. `test(oracle): capture Imp spawn and vault usingDefaults draw sites` — extend `ImpRingDeckOracle` / add a vault-equipment oracle that forces `VaultLevel` like `MiningLevelOracle.java` and records `RING/WAND/ARTIFACT.dropped` before vs after `create()` (must stay flat: vault must not move RING.dropped).
5. `docs(decks): re-verify generator-decks.md for v4.0.0` — Imp spawn and vault usingDefaults facts live in `specs/generator-decks.md` §3/§4/§7–§8, from the committed `*-imp-ring-deck.json` and `aaa-aaa-aaa-floor-17.json` oracles.

## Dependencies

- **00** — oracles must run on v4 HEAD.
- Blocks **02** (Imp spawn is a new deck consumer; city floor stream starts with those draws).
- Enchant tables also block correct Ghost/Imp/vault weapon *names* in 04/02; Ghost spawn logic otherwise unchanged.

## Explicit non-goals

- Replacing monks/golems with the vault quest report (02).
- `GridBuilder` / vault room paint (02).
- MossyClump feeling floats (03).
- Exotic Crystals actually converting potions/scrolls (05) — keep burning `Random.Float()`.
- `Unstable.proc` pool, Crystal durability, Kinetic/Corrupting/Grim smite order (07).
- Runtime `randomUsingDefaults(RING)` call sites (Wealth, Thief, Transmutation) — already defaults; only the ARTIFACT fallthrough changed.
