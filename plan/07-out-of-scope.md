# 07 — Out of scope for the v4 close-out

Status: **open** (deny-list; not a delivery workstream)

These v4 changes are real and appear in `v4_X_Changes.java` / the Java diff. They do not belong in seed analysis, public maps, or the finder unless a later project explicitly expands scope.

## Why it is out

**SEED-ANALYSIS** — skip runtime RNG, combat, and player-skill outcomes. **MAP-LAYOUT-GOAL** — skip NPC/mob/heap presentation. The analyzer is not a save loader, music player, or iOS client.

## Changelog / diff items (with reason)

### Combat AI and challenges

- **Swarm Intelligence overhaul** (`actors/buffs/SwarmIntelTracker.java`, `actors/mobs/Mob.java`) — triggers while an enemy sees you, range 2→12. No `Generator` / `Level.create` / quest spawn change. **Out of seed-analysis scope** (also called out in 05).
- **Vault-only enemy AI** (sleeping/wandering/investigating, `actors/mobs/quest/vault/*`, `VaultBossElemental.java`) — runtime stealth. Public vault maps stay painter-complete without mobs (02).
- Champion conversion, necromancer skeleton VFX, DM-300 electricity, crystal-guardian haste, chasm double-death, shocking-elemental kill credit — combat bugfixes.

### Runtime balance (does not change generation)

- **Force Cube** base damage 10–25 → 10–30 (`items/weapon/missiles/ForceCube.java`).
- **Ring of Haste** +17.5% → +15% per level (`items/rings/RingOfHaste.java`).
- **Stone of Aggression** no longer applies to bosses (`items/stones/StoneOfAggression.java`).
- **Dried Rose** ghost HP/damage (`items/artifacts/DriedRose.java`).
- **Footwear of Nature** / **Master Thieves' Armband** charge speed (`SandalsOfNature.java`, `MasterThievesArmband.java`).
- **Chaotic Censer** gas category weights (`ChaoticCenser.java`) — spew is runtime; energy-cost *formula* unchanged.
- **Cracked Spyglass** hidden-heap alpha 0.15→0.1 (`ItemSprite.java`). `extraLootChance` unchanged. Heap alpha is not a spawn fact.
- **Barkskin** talent (furrowed grass vs plants, 50%→33%) — talent combat.
- **Hold Fast** requires Warrior seal in the vault — talent combat.
- **Monk** energy not retained into vault / revives — hero state.
- **Combo Strike** / **Feint** / **Challenge** ability costs — hero combat.
- Kinetic conserved damage, Corrupting/Grim vs smite, Crystal enchant durability/`repair`, Unstable on-hit pool, Pressurized geyser, Wondrous cursed-wand procs, Venomous/Vorpal/Eldritch `proc` — enchant *combat*. Identities are 01/05.

### Saves, platforms, audio, chrome

- **Dropped saves prior to v3.1.1** (`Level.java` `version < v3_1_1`). Analyzer never loads `.bundle` saves. `Imp.Quest.oldQuest` / `WndImpOld` / MassGrave `Bones` pre-v4 layout / RitualMarker 3×3 restore are save-compat only.
- **iOS 12 end of support** — not this repo.
- Music playback internals, Android third-party appstores, Gradle wrapper, fastlane `metadata/en-US/*`.
- Changes-screen split, historical change icons (`ui/changelist/*`), Pixel Dungeon history tab.
- Health bars darkening for incoming DoT, persistent targeted-cell VFX, checked-cell performance, skeleton-key spectral walls, alchemy/well VFX in fog.

### Runtime item / hero bugs that do not move decks or layouts

- Helpful darts on allies, rapier lunge state, Wild Magic wand count, honeyed-healing value, lost-backpack shop stacks, potions counting as used on harmless splash, Fireblast vs ghouls, Feint waking sleepers, Combo parry timer, Empowered clobber threshold, Radiance magical damage, Battlemage lightning duration, telefrag on-kill, Champion second weapon “equipped”, Corruption partial damage on save/load.
- `Level.invalidHeroPos` / mind-vision skipping `Property.OBJECT` — FOV at play time.
- `Dungeon.interfloorTeleportAllowed` excluding vault — play time; analyzer already treats vault as a branch like mining.

## Analyzer files — do not change for these

Do not add Swarm, vault AI, or balance constants to:

- `crates/spd-core/src/level/create_mobs.rs` (beyond existing seed-determined encounter lists)
- `web/src/lib/map-entities.ts` vault-boss sprites on public maps
- `crates/spd-core/src/trinkets/` Censer gas tables
- Any new `actors/` combat port

## Acceptance

This workstream never “closes” by shipping code. It closes when 00–06 are done **without** having ported the list above. If a later seed-analysis need appears (e.g. modelling Swarm as a search constraint), start a new workstream; do not sneak it into 02–05.

## Suggested PRs

None.

## Dependencies

None. Read this before expanding 02 vault “fidelity” or 05 “trinket numbers”.

## Explicit non-goals

The entire file is non-goals. In particular do not:

- Simulate vault score to “predict” `earnedShop()`.
- Place `VaultBossElemental` on the public map.
- Port `EscapeCrystal` gear strip except as flavour text on vault access.
- Keep `oldQuest` dwarf-token Imp as a supported analyzer mode.
