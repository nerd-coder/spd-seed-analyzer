# 05 — Enchants, trinkets, challenges

Status: **closed**

Generation-facing item identity changes after the v4 tables and Exotic Crystals chance. Combat, energy-cost comments, and Swarm Intelligence stay out.

## Why it matters

**SEED-ANALYSIS / DECK-FACTS.**

Generated weapons/armor roll `Enchantment.random()` / `Glyph.random()` / curse tables. v4 added six weapon effects, so Ghost weapons, Imp/vault take-out weapons, statue/weapon-room prizes, and BlacksmithRoom gear can spawn with new names. Type weights are still `{50,40,10}`; per-enchant odds drop because buckets grew.

Exotic Crystals conversion chance is the only trinket **number** that changes a generated class (`0.2 + 0.2*level`, was `0.125 + 0.125*level`). Analyzer already burns `Random.Float()` on every potion/scroll (`consume_exotic_conversion_roll` in `generator/state.rs`) but never converts because it models “no ExoticCrystals held” as chance 0. Held-trinket profiles that include Exotic Crystals must start converting or later-floor identities (and the float is already consumed either way).

Swarm Intelligence is a challenge AI overhaul (`actors/buffs/SwarmIntelTracker.java`, `actors/mobs/Mob.java`). It does not touch `Generator`, `Level.create`, or quest spawn. **Out of seed-analysis scope.**

## Java sources (what to port)

Enchants (identities; `proc` is 07):

- `items/weapon/Weapon.java` — `common/uncommon/rare/curses` (also 01).
- New classes under `items/weapon/enchantments/` and `items/weapon/curses/` listed in 01.
- Glyph tables in `items/armor/Armor.java` — **no v4 membership change**. Viscosity/Flow/Swiftness/AntiMagic diffs are combat/VFX.

Trinkets:

- `items/trinkets/ExoticCrystals.java` — `consumableExoticChance`. Call sites: `Generator.java` ~731/759 (deck + defaults), `SecretLaboratoryRoom.java:101`, `SecretLibraryRoom.java:100`, `CrystalPathRoom.java:173/176`, `RingOfWealth.java:264/266` (runtime), `Mob.java:1111` (runtime loot — not public).
- `items/trinkets/CrackedSpyglass.java` — `extraLootChance` **unchanged** (`0.375f*(level+1)`). `upgradeEnergyCost` formula `6+2*level()` unchanged (comment only). Changelog “bonus item opacity 15%→10%” is `sprites/ItemSprite.java` `heap.hidden ? 0.1f` — renderer, not generation. Spyglass still adds 0–2 hidden `randomUsingDefaults` items in `RegularLevel.createItems` (v3 path; no Java diff in that method in the v3.3.8→v4 Generator/RegularLevel stats).
- `items/trinkets/ChaoticCenser.java` — `GAS_CAT_CHANCES` / Stench↔Regrowth swap are **runtime** spew. `upgradeEnergyCost` comment-only. Out.
- Other trinket files in the diff are comment-only energy costs (`6+2*level()`).

Challenges:

- `Challenges.java` — **no v3.3.8→v4 diff**.
- Swarm: `actors/buffs/SwarmIntelTracker.java` (new), `actors/mobs/Mob.java` large AI diff. Out.

Combat-only item numbers (07): Force Cube 10–30, Ring of Haste 15%, Stone of Aggression vs bosses, Dried Rose HP, etc.

## Analyzer files that will need to change

- `crates/spd-core/src/items/enchants.rs` — if not already done in 01.
- `crates/spd-core/src/items/randomize.rs` — no control-flow change; identities flow through `enchants::random_weapon_enchant`.
- `crates/spd-core/src/generator/state.rs` — `consume_exotic_conversion_roll` should call `ExoticCrystals.consumableExoticChance(held_level)` and swap to exotic class names when it hits. Held profile is `level/trinkets.rs` / `trinkets/model.rs`.
- `crates/spd-core/src/level/special_loot/secret_rooms.rs` — SecretLaboratory already burns a float; apply conversion when chance > 0.
- `crates/spd-core/src/level/special_loot/crystal_path.rs` — same.
- `crates/spd-core/src/trinkets/` — no new kinds.
- `web/src/lib/item-icons.ts` — enchant icons if the UI shows them; consumable sheet indices are 06.
- Finder: `web/src/components/finder/finder-items.ts` — optional new enchant filter values.
- Tests: Ghost enchant names in `quests/ghost.rs` tests; Imp/vault weapons in 02 tests; a held-ExoticCrystals potion conversion test on SecretLaboratory.

## Acceptance

- `random_weapon_enchant(None)` sample includes Venomous, Vorpal, Eldritch, Crystal; curses include Pressurized, Wondrous. Glyph tables unchanged vs v3.3.8 analyzer.
- With Exotic Crystals held at +0, potion/scroll generation converts with probability 0.2 (Java `consumableExoticChance(0)`); +1 → 0.4, etc. With no trinket, still one `Random.Float()` and no conversion (existing tests in `generator/tests.rs` / `secret_rooms.rs` stay valid).
- Spyglass hidden-item **count and Generator path** unchanged; do not “fix” opacity in core.
- No Swarm / Censer gas / Haste / Force Cube code in `spd-core`.
- `bun run test:rust` for enchants + exotic conversion. UI icon polish can wait for 06.

## Suggested PRs

1. `feat(core): add v4 weapon enchant and curse identities` — skip if 01 PR 3 already landed; otherwise this is the tables PR.
2. `feat(core): apply Exotic Crystals conversion chance on potion and scroll generation` — `state.rs`, SecretLaboratory, SecretLibrary, CrystalPath; unit tests with `MapTrinketProfile` holding `ExoticCrystals` at 0 and 3.
3. `test(core): pin Ghost and Imp weapon enchant names against v4 tables` — uses 02 Imp pool once it exists; otherwise Ghost-only.
4. `docs(core): record Spyglass opacity and Censer gases as non-generation` — a short note in `specs/generator-decks.md` trinket section (or this workstream only; do not invent analysis docs).

## Dependencies

- **01** for tables if not combined.
- **02** before Imp/vault enchant goldens.
- **06** for consumable sprite indices if conversion shows exotic potion images.

## Explicit non-goals

- Swarm Intelligence (range 2–12, visual buff) — **out of seed-analysis scope**. Challenges still only affect generation as today (`NO_SCROLLS` SoU skip, `NO_HERBALISM` dewdrop, `DARKNESS` torches).
- Censer gas weights, Spyglass heap alpha, trinket energy-cost comments.
- Kinetic conserved damage, Corrupting/Grim vs smite, Crystal durability/`repair`, Unstable on-hit pool (07).
- Force Cube, Ring of Haste, Stone of Aggression, Dried Rose, Barkskin, Hold Fast, Monk energy (07).
- Armor glyph set changes (none).
