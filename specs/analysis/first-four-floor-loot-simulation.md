# First-four-floor item and room-reward simulation

Target: **Shattered Pixel Dungeon v4.0.0 @ `2bb34a4e9`**.

This note is the loot-side companion to `floor-layout-run-settings.md` and
`generator-decks.md`. Public reports follow `SEED-ANALYSIS` and
`SPAWN-PRESENCE`: guaranteed spawn presence under a declared profile, not
combat drops or internal heap lifecycle.

## Verdict

Floors 1–4 of the main path can be replayed exactly **for a fully declared
generation profile**. That is deterministic replay, not a probability model.

- Floor 1 is exact for a fresh custom-seeded run with a known challenge mask.
  There is no feeling roll and no prior player-controlled generation history
  (`Level.java:259-260`).
- Floors 2–4 are exact once the profile names held trinket instances, the
  challenge mask, and any external artifact events before each first
  generation. The Sad Ghost's pre-rolled reward pair is then exact too; the
  player later claims only one member.
- Without those inputs, report only facts common to every reachable profile,
  plus labelled variants. Do not treat one baseline replay as the seed's
  universal loot list.
- Combat drops and other runtime RNG are out of scope.

The product boundary is **first generation of main-path floors 1–4**.
Regenerated sealed floors, vault/mining branches, and the Imp quest are
separate scopes. No Imp draw site runs on these floors
(`CityLevel.java:192-194`).

## Why replay is possible

`Level.create()` pushes `Dungeon.seedCurDepth()` before queued guaranteed
items, feeling, build/paint, `createMobs()`, and `createItems()`
(`Level.java:218-325`). Each depth has an isolated ambient stream. Combat on
an already generated floor cannot advance the next floor's stream.

Persistent state that does cross floors 1–4:

| State | Effect on 1–4 |
|---|---|
| `Generator` category `dropped` counters | The *k*-th draw from a category is seed-keyed; *k* changes when a conditional draw site fires (`Generator.java:698-741`). |
| Limited-drop counters | Food, SoS, SoU, stylus, intuition stone (depths 1–3), and the single Trinket Catalyst (depths 1–3) (`Dungeon.java:529-587`). |
| Special / secret run queues | Laboratory can appear on depth 3 or 4 (`SpecialRoom.java:131-138`). Secret counts use the run's remaining region secrets (`SecretRoom.java:66-87`). |
| Held trinket instance | Mossy Clump / Trap Mechanism feeling decks; Mimic Tooth mimic prizes; Rat Skull statue/vault branches. |
| Artifact deck | `randomUsingDefaults` still routes ARTIFACT through the artifact deck (`Generator.java:744-752`). Exhaustion falls back to `randomUsingDefaults(RING)` and does not advance the RING deck (`Generator.java:706-710`). |
| Challenge mask | Forbidden Runes changes the SoU queue; Barren Land removes planted objects after the RNG work. |

`WEP_T3.probs` is cloned from `WEP_T3.defaultProbs` (`Generator.java:441-450`).
A Ghost tier-3 weapon is therefore the *k*-th WEP_T3 draw, not a T1 table.

## What a first-floor replay can name

Under a fresh declared profile, when the port path is verified:

- every queued guaranteed item and quantity;
- each ordinary `createItems` result: class, quantity, upgrades, curse, and
  container (heap, chest, locked chest, skeleton, or spawned Mimic);
- every painter-created room reward and required puzzle-support item;
- spawned Mimics and their generated extra rewards as **conditional
  obtainable** rewards, never as ground-heap spawns;
- the Sad Ghost weapon/armor pair if it spawns on depth 2, 3, or 4, plus the
  later one-of-two claim rule.

`RegularLevel.createItems` generates 3/4/5 ordinary items at 60%/30%/10%,
adds two on LARGE, then chooses presentation (`RegularLevel.java:377-447`). A
Mimic's extra reward is generated during floor creation (`Mimic.java:332-358`)
and is obtainable only through later interaction.

`findPrizeItem` prefers a queued Trinket Catalyst, otherwise a random queued
item, and returns null when empty (`Level.java:827-855`). Replay must preserve
painter order and the queue.

`RingRoom` is a sewer shape. It calls `placeCenterDetail` only when the smaller
dimension is at least 10; the callback drops `findPrizeItem()`, never a ring
by virtue of the room class (`RingRoom.java:52-56,102-103`).

### Suspicious Chest, Pool, Crystal Choice, Honeypot, Grassy Grave

`SuspiciousChestRoom` takes an eligible guaranteed item and falls back to gold
(`SuspiciousChestRoom.java:55-59`). Mimic chance is `(1/3) * MimicTooth`
multiplier; a Mimic keeps that prize and rolls one extra
gold/missile/armor/weapon/ring (`SuspiciousChestRoom.java:65-70`,
`Mimic.java:332-353`). Keep the relocated guaranteed item under its guaranteed
entry; expose gold fallback and Mimic bonus as extra room rewards.

`PoolRoom` has a one-third attempt to take a guaranteed item. Otherwise it
generates weapon, missile, or armor from one floor set above the normal depth
set, clears curse, and has a one-third chance to upgrade once
(`PoolRoom.java:102-140`). Fallback equipment is +0…+3 after the usual +0…+2
roll.

`CrystalChoiceRoom` creates 3–4 potion/scroll rewards and one hidden
wand/ring/artifact (`CrystalChoiceRoom.java:105-126`). Artifact exhaustion
falls back to `randomUsingDefaults(RING)` (`Generator.java:706-710`). Universal
contract and a concrete fresh-profile result are two views of one spawn.

`SecretHoneypotRoom` always creates one Shattered Pot, one Honeypot, and one
`Bomb.random()` (Bomb or Double Bomb; Double on `Int(4) == 0`)
(`SecretHoneypotRoom.java:46-59`, `Bomb.java:229-235`).

`GrassyGraveRoom` uses `max(width-2, height-2)/2` tombs, one general
`Generator.random()` reward, and gold in the rest (`GrassyGraveRoom.java:56-67`).
Room size is fixed during build. Gold is bounded by `30 + depth*10` through
`60 + depth*20` (`Gold.java:90-92`).

## Floors 2–4 and the Sad Ghost

Ghost spawn is the first call in `SewerLevel.createMobs`, after paint. It can
spawn only on depths 2–4 (`Ghost.java:303-318`). Depth 2/3/4 are Fetid Rat /
Gnoll Trickster / Great Crab. The reward pair is generated immediately:

- armor tier 2–5 (Leather/Mail/Scale/Plate) and weapon tier from the same
  weights (`Ghost.java:321-331`);
- weapon identity from seeded `WEP_T2`…`WEP_T5`;
- shared +0…+3 upgrade, both explicitly uncursed;
- enchantment and glyph both kept or both discarded.

Parchment Scrap changes whether those already generated effects are retained,
not the number of RNG calls. Details: `specs/analysis/quest-sad-ghost.md`.

## Isolated tails that must not be folded into guaranteed loot

After ordinary drops, `createItems` isolates bones, Dried Rose petals, Cached
Rations, guide/lore pages, ebony mimics, and Cracked Spyglass extras each
behind `Random.pushGenerator(Random.Long())` (`RegularLevel.java:469-690`).
Those ambient `Long`s are seed-stable. Bones are skipped on Daily and on
custom seeds (`Bones.java:154-158,198-201`). Spyglass extras can still mutate
the artifact deck. Guide pages depend on journal meta. None of these are
combat drops; only seed-reachable, profile-declared ones belong in the
report.

## Profile

```text
scope: first_generation_main_path floors 1-4
challenges: full 9-bit mask
trinket_instances: chronological acquire / upgrade / transmute
artifact_events: obtained or transmuted outside levelgen before depth N
claim_state: Parchment Scrap level for Ghost effects only
```

Enumerate finite seed-reachable branches (Catalyst offers 0–3, first alchemy
depth, first effective depth). Do not enumerate combat routes.

Replay one lifecycle per branch: apply profile events, run full generation in
SPD order, snapshot public map and spawn facts after painter-complete room
rewards, then continue private NPC/mob/item population so decks survive into
the next depth.
