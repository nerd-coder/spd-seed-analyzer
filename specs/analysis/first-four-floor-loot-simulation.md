# First-four-floor item and room-reward simulation — verified design boundary

Target: **Shattered Pixel Dungeon v3.3.8 @ `7b8b845a7`**.

This note defines the truthful scope for predicting item spawns and room
rewards on the first four main-path floors. It supplements the canonical deck
facts in `specs/generator-decks.md`; it does not change the public accuracy
status until the implementation and Java-oracle coverage described below land.

## Verdict

A seed can be replayed to predict first-generation floor spawns exactly **for
a fully declared generation profile**. This is deterministic replay, not a
probabilistic simulation:

- Floor 1 is exact for a fresh custom-seeded run with its declared challenge
  mask. It has no prior player-controlled generation history.
- Floors 2–4 are exact once the profile describes any held trinket instances,
  relevant challenge settings, and prior external artifact events. The Sad
  Ghost's pre-rolled reward pair is then exact too, although the player may
  later claim only one member.
- Without those inputs, the analyzer may report only facts common to all
  reachable profiles plus explicitly labelled conditional variants. It must
  not label one baseline replay as the seed's universal result.
- Combat drops and other runtime RNG events are not seed-generation spawns and
  remain out of scope. They must not be guessed or folded into a floor's
  guaranteed loot.

The practical product boundary is **first generation of main-path floors 1–4**.
Regenerated sealed floors and arbitrary play history are separate scopes.

## Why replay is possible

`Level.create()` pushes `Dungeon.seedCurDepth()` before it queues guaranteed
items, selects a feeling, builds and paints the floor, then calls
`createMobs()` and `createItems()` (`Level.java:215-317`). Each depth therefore
has an isolated ambient RNG stream. Combat calls on an already generated floor
cannot directly advance the ambient stream of the next floor.

Persistent state still crosses floor boundaries:

| State | Consequence for floors 1–4 |
|---|---|
| `Generator` category and item sub-deck counters | Determines later deck-drawn identities. A class is the seed's *k*-th draw from that category, but *k* changes when a conditional generation draw site fires. |
| Limited guaranteed-drop counters | Determines Food, Potions of Strength, Scrolls of Upgrade, Arcane Styli, Stones, and Catalyst queue entries. Room painters may consume an eligible queued item. |
| Held trinket instance state | Mossy Clump, Trap Mechanism, Mimic Tooth, and Rat Skull can alter current-floor generation; the former two retain instance-local feeling decks. |
| Artifact deck history | Artifact requests always use the artifact deck, including some runtime requests. A different artifact can consume a different RNG tail, notably Unstable Spellbook setup, which can move subsequent generation. |
| Challenge mask | Forbidden Runes changes the queued Upgrade Scroll sequence; Barren Land changes public plant presence. |

`Generator.random(Category)` uses a seeded per-category sub-generator and
advances `cat.dropped` after each draw (`Generator.java:698-742`). Thus a
fixed category draw index is reproducible independently of the ambient stream.
`randomUsingDefaults` bypasses most category decks, but explicitly routes
ARTIFACT back through `random(Category.ARTIFACT)` (`Generator.java:745-768`).
The artifact exception is why player history cannot be discarded.

These facts are also established, with the complete draw-site audit, in
`specs/generator-decks.md` §§1–5 and §11.

## What the first-floor replay can name

Under a fresh declared profile, a replay may state all of the following as
exact first-generation facts when the corresponding port path is verified:

- every queued guaranteed item and its quantity;
- each ordinary `RegularLevel.createItems` result: its class, quantity,
  upgrades, curse state, and container presentation (heap, chest, locked
  chest, skeleton, or a spawned Mimic);
- every painter-created room reward and required puzzle-support item;
- spawned Mimics and their carried generated rewards as **conditional
  obtainable** rewards, never as ground-heap spawns;
- the Sad Ghost's generated weapon/armor pair if it spawns on depth 2, 3, or
  4, along with the later one-of-two claim rule.

`RegularLevel.createItems` generates 3, 4, or 5 ordinary items with
60%/30%/10% weight, adding two on a Large floor, then chooses a heap/chest/
skeleton/Mimic presentation and may create a locked chest plus a Golden Key
(`RegularLevel.java:377-470`). A Mimic's carried reward is generated during
floor creation (`Mimic.java:332-360`), but it becomes obtainable only through
later player interaction. This distinction is required by SPAWN-PRESENCE.

Many room painters ask `Level.findPrizeItem()`. It first consumes a queued
Trinket Catalyst when present, otherwise chooses/removes a queued item, and
returns null when the queue is empty (`Level.java:799-826`). Exact replay must
therefore preserve painter order and the queue rather than independently roll
each room reward.

`RingRoom` names a sewer room shape, not a ring reward. It invokes
`placeCenterDetail` only when its smaller dimension is at least 10; smaller
instances contain no item at all (`RingRoom.java:48-103`). When the callback
does run, it passes `Level.findPrizeItem()` directly to `Level.drop`, so the
possible reward is an eligible queued guaranteed floor item, never a ring by
virtue of the room class (`RingRoom.java:102-103`, `Level.java:799-826`).

### Suspicious Chest and Pool reward bounds

`SuspiciousChestRoom` first takes an eligible guaranteed floor item and falls
back to randomized gold only when none is available
(`SuspiciousChestRoom.java:55-60`). The chest has a one-third baseline Mimic
chance, modified by Mimic Tooth; a Mimic keeps that prize and generates one
additional gold, missile, armor, weapon, or ring reward
(`SuspiciousChestRoom.java:65-70`, `Mimic.java:315-359`). Equipment and rings
from that extra reward begin at +0…+2. Public spawn reports keep a relocated
guaranteed item under its guaranteed-floor entry and expose only the possible
gold fallback and conditional Mimic bonus as additional room rewards.

`PoolRoom` likewise has a one-third attempt to take an eligible guaranteed
floor item. Otherwise it generates weapon, missile, or armor from one floor set
above the normal depth set, clears the cursed flag and any curse enchantment,
then has a one-third chance to upgrade once (`PoolRoom.java:103-143`). The
fallback therefore spans +0…+3 after the equipment's normal +0…+2 randomization;
its tier bounds follow `Generator.floorSetTierProbs[depth / 5 + 1]`
(`Generator.java:613-619`, `:775-853`). The public report pairs these invariant
bounds with the explicitly labelled fresh/no-history concrete replay.

## Floors 2–4 and the Sad Ghost

The Ghost spawn attempt happens at the start of `SewerLevel.createMobs`, after
painting but before ordinary population. It can spawn only on depths 2, 3, or
4; its quest target is respectively Fetid Rat, Gnoll Trickster, or Great Crab
(`Ghost.java:303-318`). The reward pair is generated immediately:

- one armor tier/class and one weapon tier are rolled;
- the weapon identity comes from the corresponding seeded WEP_T2…WEP_T5 deck;
- both items share one +0…+3 upgrade roll and are explicitly uncursed;
- an enchantment and glyph are both retained or both removed; the player later
  selects exactly one reward.

The detailed contract, Parchment Scrap condition, and pre-Ghost state inputs
are in `specs/analysis/quest-sad-ghost.md`. Parchment Scrap changes whether
the already-generated effects are retained, but not the number of reward RNG
calls. It is a claim-time condition rather than a simulation branch that
changes later floor generation.

## Minimal profile and branch model

The simulator should accept a player-meaningful event history, never exposed
deck counters. For the first-four-floor scope:

```text
scope: first_generation_main_path
challenges: complete mask (initially implement Forbidden Runes + Barren Land)
trinket_instances: chronological acquire / upgrade / transmute-away events
artifact_events: artifact obtained or transmuted outside level generation,
                 before first generating depth N
claim_state: Parchment Scrap level for Ghost-effect eligibility only
```

The first implementation can offer a **fresh baseline** whose explicit
assumptions are: no map-affecting trinket held before it is reachable, no
external artifact event, and the selected challenge mask. It should then
enumerate only finite, seed-reachable branches:

1. precompute the four Catalyst offers (TRINKET draws 0–3) and the first
   floor on which a selected trinket can take effect;
2. reject a trinket branch if that offer cannot be selected and alchemized
   before the claimed floor;
3. replay each remaining held-trinket and challenge branch in full;
4. retain facts common to every replay as guaranteed, and attach a precise
   condition to each differing result.

Do **not** enumerate every possible combat route. Runtime artifact acquisition,
transmutation, and equipment effects create an open-ended action space. The
analyzer has no settings for those events; it can only add a modelled branch
once the event sequence and parity evidence are bounded, and must communicate
the condition.

`TrinketSelectionReport` supplies the Catalyst offer sequence, first
alchemy-pot depth, and first effective depth. `MapProfile` now records all nine
challenge choices, chronological trinket acquire/upgrade/transmute events,
external artifact events, and Parchment Scrap claim state
(`crates/spd-core/src/trinkets.rs`). Acquire and transmute events derive a new
instance identity, so Mossy Clump and Trap Mechanism decks restart only for a
new instance. The lifecycle does not yet apply Rat Skull, Cracked Spyglass,
Barren Land, Badder Bosses, or artifact events; those inputs keep results
assumed rather than claiming an exact branch.

## Replay architecture

Use a single mutable dungeon lifecycle for each branch:

```text
init run from seed
for depth 1..=4:
  apply profile events effective before this first generation
  execute full level generation in SPD call order
  snapshot public painter-complete map and spawn/reward facts
  continue private NPC, mob, room, and item population
  retain all persistent decks, queues, and trinket instances for depth + 1
project exact / conditional / invariant public results
```

The snapshot must occur after painter-complete terrain and room rewards but
the same lifecycle must continue through population. A separate layout-only
pass loses deck mutations from automatic population and can make a later
floor's replay wrong. This is the unresolved architectural issue documented
in `specs/analysis/floor-layout-run-settings.md`.

The current Rust core already follows the relevant broad ordering in
`create_level_internal`: queue forced items, build, paint special rooms, spawn
NPCs, populate mobs, then run `create_items_main`
(`crates/spd-core/src/level/mod.rs:71-525`). Its public projection deliberately
withholds later ordinary loot and room details after a runtime-sensitive
boundary (`crates/spd-core/src/level/state.rs:175-558`). That conservatism is
correct until the profile and oracle work below verify the stronger claim.

## Required verification before a stronger public claim

Add Java-oracle fixtures that compare, per floor and per branch:

- queued guaranteed items after painter consumption;
- room-reward classes/properties and containers;
- ordinary heap/chest/skeleton/locked-chest/Mimic results;
- Ghost spawn/no-spawn and both generated reward options;
- persistent generator counters/state after the private population tail.

Cover at minimum: fresh baseline seeds with each Ghost depth, Forbidden Runes,
Barren Land, every supported held-trinket at +0 and +3 from its first legal
depth, Rat Skull's statue/crystal branches, Cracked Spyglass followed by an
artifact-sensitive later floor, and an external artifact event before a Ghost
floor. Only after those fixtures pass may floors 2–4 from baseline/conditional
be promoted to exact for the corresponding profile.

## Non-claims

- No claim about combat drops, farming, or runtime item effects.
- No claim that the same seed has one universal later-floor loot list.
- No item-cell guarantees in the public API; map entity placement remains
  internal parity evidence.
- No claim for regenerated floors, side levels, or profiles not replayed.
