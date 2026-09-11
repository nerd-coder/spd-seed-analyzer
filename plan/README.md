# Close-out: analyzer v3.3.8 → SPD v4.0.0

Status of this index: **open**. Do not claim the analyzer already handles v4.

| | |
|--|--|
| Current pin | SPD **v3.3.8** @ `7b8b845a76fe76c6b7c031ae9e570852411f56db` |
| Target pin | SPD **v4.0.0** @ `2bb34a4e91d29c8785a9363cad6ddfe5122b1d4f` |
| Java clone | `/Users/toan/code/repos/00-Evan/shattered-pixel-dungeon` (HEAD is the target) |
| Analyzer | `/Users/toan/code/NerdCoder/spd-seed-analyzer` |
| Resume file | `specs/implementation.md` — do not edit from this plan |

The analyzer today is a v3.3.8 port. v4 Java is a different game for Imp, city layout, enchants, and several painter/builder RNGs. Closing the gap is a sequence of small PRs, not one rewrite.

`specs/analysis/` is the v4 source of trust (index: `specs/analysis/README.md`). Do not overwrite it from these workstreams. Re-verify `specs/generator-decks.md` in **01**, in the same change that ports Generator.

## Principles

- **PIN-SPD** — every PR targets `2bb34a4e9`. Do not mix v3.3.8 Java citations with v4 behaviour.
- **RNG-PARITY** — port call order from the clone. No invented shortcuts.
- **DECK-FACTS** — ring/artifact/weapon identities follow `Generator` decks. v4 changes the artifact→ring fallback and adds Imp/vault draw sites.
- **SEED-ANALYSIS** — public reports are guaranteed spawns and seed-determined options, with conditions labelled. Vault score and combat AI are not seed facts.
- **SPAWN-PRESENCE** — assert whether an item is guaranteed to spawn. Do not leak queue/heap lifecycle.
- **MAP-LAYOUT-GOAL** — public maps are painter-complete layout: rooms, terrain, doors, transitions, traps, plants, blobs, carpets/custom terrain. No NPC/mob/heap/forced-item on the public map.
- **CORE-FIRST** — generation in `spd-core`; `spd-wasm` stays a façade; UI does not reimplement RNG.
- **SMALL-FILES** — vault/GridBuilder/new rooms are new modules, not append-only growth of `quests/imp.rs` or `level/special_loot/quest_rooms.rs`.
- **CI-BEFORE-DONE** — `bun run check:rust` + `bun run test:rust` on every Rust PR; add `bun run check` + `bun run build` if TS/wasm changed; add `bun run test:visual:only` after renderer/snapshot PRs.
- **NO-CLAIM** — analyzer remains `partial` until a workstream’s acceptance is green.

## Changelog classification (v4.0 in-game list)

Source: `core/.../ui/changelist/v4_X_Changes.java` (`add_v4_0_Changes`). Context only.

| Highlight | Scope | Why |
|-----------|--------|-----|
| New City Quest / vault (~3 floors, 20+ rooms, hazards, boss, take one item) | **In** — 01, 02 | Seed-determined Imp spawn, reward pool, vault layout. Shop is conditional. Boss AI out. |
| Environment visuals (wall-floor shadowing, city special floor, carpets) | **In** — 03, 06 | MAP-LAYOUT-GOAL. Carpets are custom tiles; shadows are renderer. |
| Consumable sprites | **In** — 06 | UI icons / items.png. No generation. |
| New enchants/curses (Venomous, Vorpal, Eldritch, Crystal, Pressurized, Wondrous) | **In** — 01, 05 | Identity tables at generation. Combat procs out. |
| Existing enchant quirks (Kinetic / Corrupting / Grim vs smite) | **Out** — 07 | Combat proc order, not spawn identity. |
| Swarm Intelligence overhaul | **Out** — 07 | Challenge gameplay / AI. Does not change levelgen item or layout RNG. |
| New enemy AI (vault-only) | **Out** — 07 | Runtime. Vault maps stay painter-complete without mobs. |
| Artifact extra-gen affecting later rings (Generator fallback) | **In** — 01 | `random(ARTIFACT)` now falls back to `randomUsingDefaults(RING)`. |
| Mossy Clump same-seed layout | **In** — 03 | `Level.create` always draws two feeling floats. |
| Rare levelgen bugfixes | **In** — 03, 04 | `Builder.findFreeSpace`, Mining gold-in-room, MassGrave/Ritual/Blacksmith sizes, Hallway entrance RNG, RatKing 7×7, CavesPainter skips BlacksmithRoom. |
| Item buffs/nerfs (Force Cube, Haste, Spyglass opacity, Censer gases, Barkskin, …) | **Mostly out** — 05/07 | Only Exotic Crystals conversion chance changes generated identities. |
| Dropped pre-v3.1.1 saves | **Out** — 07 | Analyzer is not a save loader. |
| Daily first date clamp | **In** — 00 | `HeroSelectScene` clamps to Unix day 20,544 (`2026-04-01`). Not in the in-game “new content” list. |
| Music, iOS 12, health-bar VFX, changes-screen chrome | **Out** — 07 | |

## Ordered workstreams

| # | File | Ships |
|---|------|--------|
| 00 | [00-pin-and-tooling.md](00-pin-and-tooling.md) | Pin bump, oracle runner, fixture regeneration, Daily clamp `2026-04-01`, docs pointers. No generation logic. |
| 01 | [01-generator-decks.md](01-generator-decks.md) | Artifact→ring defaults, WEP_T3 clone, enchant tables, Imp/vault draw-site oracles. |
| 02 | [02-imp-vault-quest.md](02-imp-vault-quest.md) | Replace Imp ring quest; vault branch report; reward pool; shop score gate. |
| 03 | [03-layout-feelings-rooms.md](03-layout-feelings-rooms.md) | Mossy same-seed, Builder fix, city carpets, standard-room paint, RatKing 7×7. |
| 04 | [04-existing-quests.md](04-existing-quests.md) | Ghost/Wandmaker/Blacksmith/Mining deltas that change generation or layout visuals. |
| 05 | [05-items-trinkets-challenges.md](05-items-trinkets-challenges.md) | Enchant identities, Exotic Crystals chance; Swarm out. |
| 06 | [06-assets-and-ui.md](06-assets-and-ui.md) | Tilesheets, carpets, occlusion, consumable sprites, Playwright snapshots. |
| 07 | [07-out-of-scope.md](07-out-of-scope.md) | Combat AI, music, iOS, save-compat, runtime balance. |

Suggested merge order: **00 → 01 → 02 → 03 → 04 → 05 → 06**. 05 enchant tables can land with 01 if a PR needs Imp `.enchant()` identities immediately. 07 is a deny-list, not a PR.

City floor layouts depend on Imp.Quest.spawn consuming the floor stream inside `initRooms` **before** the builder. Do not claim city visual/oracle parity until 02’s spawn RNG is ported.

## How to close an item

1. Open the workstream file. Status starts `open`.
2. Take one suggested PR. Keep it independently shippable (oracles/tests in the same PR).
3. Port from the v4 clone at `2bb34a4e9`. Cite `file:line` in tests/comments the way existing `spd-core` ports do.
4. If it changes Generator decks, update `specs/generator-decks.md` in that PR (DECK-FACTS).
5. Do not edit `specs/analysis/*` here; if a fact moved, leave a one-line pointer in the workstream file.
6. Run CI parity for the files you touched.
7. When every acceptance line in the workstream is green, set its Status to `closed` and delete the finished section from `specs/implementation.md` rather than appending history.

Independent PRs must not require a later workstream to compile or to keep existing non-city goldens green, except where this index states a hard dependency.
