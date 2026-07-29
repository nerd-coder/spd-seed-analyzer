# Generator decks — verified facts

Verified against SPD **v3.3.8 @ `7b8b845a7`** on 2026-07-28. Java line numbers
are from that commit. **Re-verify only when the pinned version changes** — the
deck mechanism, the call sites, and the challenge behaviour below are all
version-sensitive.

Method: reading the pinned clone, plus temporary `eprintln!` instrumentation in
`generator/state.rs` (`random_deck_item`, `random_artifact`) and
`quests/imp.rs` run over `analyze_seed(.., 19)`. Instrumentation was reverted;
the measurements are reproducible by re-adding it. The Imp's exact pre-reward
ring draw index is additionally pinned for five reference seeds by the
`imp-ring-deck` Java-oracle contract, which records `Category.RING.dropped`
immediately around `Imp.Quest.spawn` and is compared to Rust in
`quests/imp.rs` tests.

Sections 9–10 were measured the same way: temporary probe fields on
`WandmakerQuestState` / `GhostQuestState` recording the deck index and the
branch outcomes at the reward call, driven by `create_level_partial` over
floors 1–9 (Wandmaker) and 1–4 (Ghost) for 200 seeds
(`init_run(seed * 7919 + 13)`), default `MapTrinketProfile`. Reverted after.

## 1. Category sub-decks make item identity index-keyed, not stream-keyed

`Generator.random(Category)` (`items/Generator.java:709+`) draws the class from
a **per-category seeded sub-RNG**:

```java
if (cat.defaultProbs != null && cat.seed != null){
    Random.pushGenerator(cat.seed);
    for (int i = 0; i < cat.dropped; i++) Random.Long();
}
int i = Random.chances(cat.probs);   // then popGenerator(); cat.dropped++
```

`cat.seed` is rolled during `Generator.fullReset()` from the run seed. So the
class of the *k*-th item a run draws from a category is a **pure function of
(dungeon seed, k)**, independent of where in a floor's RNG stream the draw
lands. Ported at `generator/state.rs::random_deck_item`.

Confirmed empirically: drawing 10 rings from a fresh generator under two
different ambient RNG streams yields identical sequences, and that sequence
matches — in order — the rings a full 19-floor analysis of the same seed
generates.

`randomUsingDefaults` bypasses both `probs` and `dropped` entirely.

## 2. Cross-floor RNG coupling is deck counters only for a fixed profile

Level generation is wrapped in `Random.pushGenerator(Dungeon.seedCurDepth())`,
so a stream difference on floor *N* cannot directly reach floor *N+1*. For a
fixed challenge/trinket/meta profile, the cross-floor RNG state relevant to
item identities is the deck counters. Parity for any deck-drawn identity
therefore reduces to: **the same draw sites fire the same number of times in
the same order**. Player choices can change which draw sites fire on later
floors; the Sad Ghost examples are catalogued in
`specs/analysis/quest-sad-ghost.md`.

## 3. Ring-deck draw sites (complete)

Direct ring deck draws (`Generator.random`) are levelgen except for the
artifact-exhaustion fallback noted below:

| Site | Java |
|------|------|
| `createItems` general-deck picks | `RegularLevel.java:388` |
| Ordinary + golden mimic prizes | `Mimic.java:349` (`useDecks=true`) |
| Shop rare, 1/10 | `ShopRoom.java:339` |
| Pit room main loot | `PitRoom.java:72` |
| Crystal choice hidden prize | `CrystalChoiceRoom.java:124` |
| Crystal vault prize cycle | `CrystalVaultRoom.java:104` |
| Grassy grave / mass grave / secret summoning | `Generator.random()` |
| Artifact exhaustion fallback, including runtime artifact requests | `Generator.java:709` |
| Ambitious Imp reward | `Imp.java:234` |

All are present in `spd-core`. Note `Generator.random()` (no-arg) is itself a
category deck, so its ring count over a deck cycle is stable even if the
in-cycle order shifts.

## 4. Direct runtime ring requests bypass the ring deck

> Scope note: calls that directly request a runtime ring use defaults. Runtime
> artifact requests are different: ARTIFACT is deck-backed and falls through
> to the seeded RING deck after exhaustion. See §11.

All use `randomUsingDefaults`: mob loot including Thief's
`oneOf(RING, ARTIFACT)` (`Mob.java:997`), Ring of Wealth
(`RingOfWealth.java:288`), Scroll of Transmutation
(`ScrollOfTransmutation.java:276`), Cursed Wand (`CursedWand.java:1082`,
`:1170`), and runtime-spawned mimics (`useDecks=false` at
`CursedWand.java:1066`, `DistortionTrap.java:120`).

The ungenerated side-levels add no direct ring-deck site: `MiningLevel` only
calls `randomUsingDefaults(FOOD)`, `VaultLevel` only
`randomUsingDefaults(...)`.

## 5. Challenge item-block rerolls cannot shift a deck index

`Challenges.isItemBlocked` (`Challenges.java:71`) returns true **only** for a
`Dewdrop` under `NO_HERBALISM`. No ring/wand/artifact/weapon/armor generator
can produce a Dewdrop, so the reroll loops in `Mimic.generatePrize`, `PitRoom`,
and `CrystalVaultRoom` never iterate from challenges.

Of the nine challenges, only `NO_SCROLLS` touches generation at all
(`Level.java:234`, skips a Scroll of Upgrade `addItemToSpawn` — no RNG, no
deck at that call site). `DARKNESS` and `STRONGER_BOSSES` affect view distance
and boss setup. `NO_SCROLLS` is nevertheless not reward-neutral: omitting the
queued Scroll can empty `itemsToSpawn`, after which a room painter can take a
generated fallback that consumes the main floor stream before an NPC callback.
Sad Ghost has a reachable `RitualRoom` counterexample; see
`specs/analysis/quest-sad-ghost.md`.

## 6. Trinket offers are seed-determined; the held trinket is not

`TRINKET (0, 0, Trinket.class)` (`Generator.java:222`) has zero weight in both
general decks, so no trinket comes from floor loot. The only levelgen source is
the Trinket Catalyst: `Dungeon.trinketCataNeeded()` (`Dungeon.java:584`) grants
exactly one per run on floors 1–4, guaranteed by depth 4.

The catalyst window rolls `NUM_TRINKETS = 4` options through
`Generator.random(Category.TRINKET)` (`TrinketCatalyst.java:178`). Being the
deck's only consumer, **its four offers are always TRINKET draws 0–3** — fully
seed-determined and computable before any play. Scroll of Transmutation on the
taken trinket (`ScrollOfTransmutation.java:325`) is the only other route, and
lands on draw 4.

Measured over 400 seeds: Mimic Tooth is among the four offers in **101 (25.3%,
= 4/17)**, its first-appearance index uniform across the 17-class deck. So
~75% of seeds rule out taking Mimic Tooth **directly from the Catalyst**. They
do not rule it out for the whole run: a Scroll of Transmutation advances to
draw 4 (and later transmutations advance farther), so that route must be checked
separately.

The player chooses one offer, may transmute it, and chooses when/how far to
upgrade it. Those choices are not encoded by the seed. In particular Mossy
Clump, Mimic Tooth, and Rat Skull can change generation work before an NPC
reward callback; see the Sad Ghost audit in
`specs/analysis/quest-sad-ghost.md`.

Not having the trinket costs no RNG: `RegularLevel.java:406` evaluates
`Random.Float()` before applying `MimicTooth.mimicChanceMultiplier()`, so the
tooth-free stream is the baseline stream.

## 7. Measurements at the Ambitious Imp spawn (44 seeds, floors 1–19)

| Quantity | Distribution |
|----------|--------------|
| Ring draws preceding the Imp's | 0–6, mode 4 |
| Imp reward draws (curse reroll) | 1× in 30, 2× in 14 (30% curse chance) |
| Artifact draws before the spawn | 1–6, mode 3 |
| Artifact deck weight left (of 11) | 5–10, median 8 |

That 5–10 artifact headroom describes only the measured fresh baseline.
Repeatable runtime artifact requests can close it before floor 17, so it is not
a seed-only bound on the Imp reward.

## 8. Consequence for the Imp ring

For a fixed profile, the class is fixed by (seed, ring draw index *k*), and the
accepted ring class never feeds back into floor RNG because no subclass
overrides `Ring.random()` (`Ring.java:259`). This proves that changing only *k*
changes class without changing level or the later floor tail.

Across playable routes, neither *k* nor the concrete +2…+4 level is seed-only.
Mimic Tooth changes levelgen mimic prizes; Rat Skull can add a deck-using
Crystal Mimic prize; earlier map/challenge paths can change levelgen draw sites;
and repeatable runtime artifact requests fall through to RING after ARTIFACT
exhaustion. Mossy Clump can also short-circuit a current-floor feeling roll
before `Imp.Quest.spawn`, shifting the spawn attempt, target, curse retry, and
level rolls. Once the 12-class equal-weight ring deck resets, every class is
reachable. The cross-route public contract is therefore category-only: one
conditional cursed +2…+4 ring. Full evidence and claim/shop conditions are in
`specs/analysis/quest-ambitious-imp.md`.

## 9. Old Wandmaker reward (floors 7–9)

**Scope correction (2026-07-29):** the measurements and concrete-pair facts in
this section use the fresh no-map-affecting-trinket, no-challenge,
no-external-artifact profile. They verify the fixed call shape, not a universal
pair for every playable route. A successful Mossy Clump check short-circuits a
main-stream feeling roll and can shift room work before the quest room is
selected; Trap Mechanism can alter painting after selection. Mimic Tooth, Rat
Skull, artifact constructor history, and challenge-dependent prize queues can
also add or remove painter and levelgen item draws before the NPC callback. The
full state audit is in `specs/analysis/quest-wandmaker.md`.

`Wandmaker.Quest.spawnWandmaker` (`Wandmaker.java:303`) runs at the **start** of
`PrisonLevel.createMobs`, before `super.createMobs()`
(`PrisonLevel.java:89-92`), i.e. after paint and after an NPC placement loop
that burns two `Random.IntRange` calls per attempt against painted terrain.

Reward code (`Wandmaker.java:334-346`):

```java
wand1 = Generator.random(WAND); wand1.cursed = false; wand1.upgrade();
wand2 = Generator.random(WAND);
while (wand2.getClass() == wand1.getClass()) { toUndo.add(wand2); wand2 = Generator.random(WAND); }
for (Item i : toUndo) Generator.undoDrop(i);
wand2.cursed = false; wand2.upgrade();
```

Verified properties:

- **Level** is `1 + n` from `Wand.random()` (`Wand.java:546-566`): +1 66.7%,
  +2 26.7%, +3 6.7%. **Never cursed** — `cursed = false` precedes `upgrade()`,
  and `Wand.upgrade()`'s `Random.Int(3)` curse-clear (`Wand.java:358-364`) is
  consumed but cannot re-curse.
- **Floor-stream cost is class-independent.** No `Wand` subclass overrides
  `random()` or `upgrade()`, and no wand instance initializer or constructor
  touches `Random` (checked all 13).
- **`undoDrop` is a no-op for concrete classes** (`Generator.java:662-673`:
  `cls.isAssignableFrom(cat.superClass)` is false for e.g. `WandOfFireblast`),
  so rejected duplicate draws permanently advance the deck. `undo_drop` at
  `generator/state.rs:418` correctly does nothing.
- **`wand2`'s level is coupled to identity** — unlike the Imp ring. The
  duplicate-rejection loop burns an extra `Wand.random()` off the floor stream,
  so resolving `wand2`'s level requires knowing the deck index. `wand1`'s level
  is decoupled.
- **No runtime path directly moves the wand deck.** Every `Category.WAND`
  reference outside levelgen uses `randomUsingDefaults`: Shaman loot via
  `Mob.createLoot` (`Mob.java:996`), Scroll of Transmutation
  (`ScrollOfTransmutation.java:211`, `:339`), Cursed Wand
  (`CursedWand.java:363`, `:371`). Consistent with §4. Player state can still
  change which levelgen WAND draw sites run before the reward, so the concrete
  deck index and both floor-stream levels require a fixed profile.

Measured for that fixed profile (200 seeds, floors 1–9):

| Quantity | Distribution |
|----------|--------------|
| Spawn depth | 7: 32%, 8: 34%, 9: 34% |
| Quest type (1/2/3) | uniform ⅓ |
| WAND deck index *k* at the reward | 0–4, mode 1 (0: 27, 1: 87, 2: 57, 3: 28, 4: 1) |
| `wand2` duplicate rejections | 0 in 189, 1 in 11 (**5.5%**) |
| `wand1` level | +1 65.5%, +2 28%, +3 6.5% |

Collision rate matches theory: 13 classes × weight 3 (`Generator.java:240`), so
a fresh deck gives 2/38 ≈ 5.3%.

## 10. Sad Ghost reward (floors 2–4)

**Scope correction (2026-07-29):** the measurements and fixed-call-shape facts
below use the default no-map-affecting-trinket, no-challenge, no-external-
artifact profile. They do not establish that the concrete pair is a pure
function of the seed across every playable route. Mossy Clump can change the
paint stream before `Ghost.Quest.spawn`; Mimic Tooth and Rat Skull can change
pre-Ghost mimic/statue generation; and external artifact history can change a
later artifact constructor tail. The complete state audit is in
`specs/analysis/quest-sad-ghost.md`.

`Ghost.Quest.spawn` (`Ghost.java:303-362`) runs at the **start** of
`SewerLevel.createMobs`, before `super.createMobs()` (`SewerLevel.java:140-143`)
— same placement-loop-then-reward shape as the Wandmaker.

Verified properties:

- **The armor uses no deck at all.** `Random.chances({0,0,10,6,3,1})` selects a
  tier and the class is constructed directly (`Ghost.java:322-328`): 2 =
  `LeatherArmor`, 3 = `MailArmor`, 4 = `ScaleArmor`, 5 = `PlateArmor`. Tier and
  class are the same fact once the tier roll is fixed.
- **Only the weapon is deck-drawn** — `Generator.random(wepTiers[tier-1])`
  (`Ghost.java:331`), i.e. the per-tier `WEP_T2…WEP_T5` decks, each with its own
  seed and `dropped` counter.
- **The weapon draw's floor-stream cost is class-independent**:
  `randomize_weapon` spends `Int(4)` [+ `Int(5)`] + one `Long`, with the
  curse/enchant roll inside a pushed generator. So the deck index cannot move
  anything that follows.
- **The shared upgrade level** is one `Random.Float()` → +0 50%, +1 30%,
  +2 15%, +3 5%, applied to **both** items (`Ghost.java:339-351`). Neither
  `upgrade()` spends RNG on this path: `Weapon.upgrade(false)`
  (`Weapon.java:375-389`) and `Armor.upgrade(false)` (`Armor.java:454-468`) only
  roll when an enchantment/glyph is already present, and the Ghost clears both
  first.
- **Enchant and glyph are always generated**, deliberately, so the roll count is
  constant (`Ghost.java:353-356`). `Enchantment.random()` (`Weapon.java:606-615`)
  and `Glyph.random()` (`Armor.java:863-872`) each spend exactly
  `Random.chances` + `Random.element` — no deck, no player state.
- **Within that fixed profile, Parchment Scrap is the direct reward-local
  player-dependent bit**:
  `enchantRoll > 0.2f * ParchmentScrap.enchantChanceMultiplier()` clears both
  (`Ghost.java:358-362`). Multipliers are 1 / 2 / 4 / 7 / 10 for no scrap / +0 /
  +1 / +2 / +3 (`ParchmentScrap.java:52-65`), so the thresholds are 0.2 / 0.4 /
  0.8 / 1.4 / 2.0. **At Parchment Scrap +2 or better the reward is always
  enchanted.** The roll count does not change, so this never desyncs the stream.

Net for the fixed profile: everything except the weapon's *class* is
independent of the WEP sub-deck. This is not a seed-only claim across different
held-trinket, challenge, or artifact-history profiles; those can move the main
floor stream before the Ghost rolls tiers, level, and effects.

Measured (200 seeds, floors 1–4):

| Quantity | Distribution |
|----------|--------------|
| Spawn depth | 2: 37.5%, 3: 29.5%, 4: 33% |
| Weapon tier | 2: 46.5%, 3: 32%, 4: 15%, 5: 6.5% |
| Armor tier | 2: 46%, 3: 35.5%, 4: 13%, 5: 5.5% |
| Tier-deck index *k* at the weapon draw | **0 in 64%, ≤1 in 85%**, max 6 |
| Shared level | +0 50.5%, +1 34%, +2 12%, +3 3.5% |
| Lowest Parchment Scrap that enchants | none 21.5%, +0 38.5%, +1 76.5%, +2 100% (cumulative) |

*k* is far tighter than the Imp's or the Wandmaker's because only floors 1–3
precede the draw and the five weapon tiers are separate decks.

## 11. Fixed-profile ordinary floor loot: ARTIFACT is the runtime deck leak

`RegularLevel.createItems` drops `3 + chances{6,3,1}` (`+2` if `LARGE`) items
from the no-arg `Generator.random()` (`RegularLevel.java:380-388`). Three
mechanisms would have to leak for run history to move them; two are closed.

- **Floor stream** — closed by the per-depth `pushGenerator` of §2.
- **General category deck** (`categoryProbs`, picks *which* category) — only
  no-arg `Generator.random()` decrements it (`Generator.java:676-683`), and its
  four call sites are all levelgen: `RegularLevel.java:388`,
  `GrassyGraveRoom.java:67`, `MassGraveRoom.java:86`,
  `SecretSummoningRoom.java:53`. `randomUsingDefaults()` reads `defaultCatProbs`
  and never decrements. **Closed.**
- **Per-category sub-decks** (pick the class) — closed for RING (§4), WAND (§9),
  and the WEP/MIS tiers: every runtime weapon site passes `useDefaults=true` or
  `useDecks=false` (`Statue.random(false)` from `DistortionTrap.java:125`,
  `WndBlacksmith.java:478 generateRewards(false)`). Armor uses no deck at all
  (`randomArmor` picks a tier by `chances` and indexes `cat.classes` directly).
  SEED is deliberately deck-free at levelgen — `Generator.java:684-688`
  special-cases it to `randomUsingDefaults` precisely because grass, not
  levelgen, is its dominant source.

  Troll Blacksmith's normal spawn-time `generateRewards(true)` also avoids the
  decks: its misleading `useDecks` parameter is passed directly into
  `randomWeapon` / `randomMissile` as `useDefaults=true`
  (`Blacksmith.java:364-384`, `Generator.java:808-818`, `:841-851`). The
  defensive `WndBlacksmith.generateRewards(false)` fallback is deck-backed,
  but the stored pool is already present in an ordinary fresh quest and one
  Smith purchase cannot reach that fallback. Full lifecycle details are in
  `specs/analysis/quest-blacksmith.md`.

**Not closed: ARTIFACT.** `randomUsingDefaults(cat)` routes artifacts straight
back into the deck (`Generator.java:745-750`, comment: *"except for artifacts,
which must always use a deck"*), so these runtime sources all advance
`ARTIFACT.dropped`:

| Runtime source | Java |
|----------------|------|
| Ring of Wealth equipment drop | `RingOfWealth.java:291` |
| Thief steal-loot | `Thief.java:52` → `Mob.java:997` |
| Scroll of Transmutation | `ScrollOfTransmutation.java:298` |
| Cursed Wand | `CursedWand.java:1170` |
| Gnoll Exile / Ebony Mimic / tooth-mimic extras | `randomUsingDefaults()` no-arg landing on ARTIFACT |

Note `RingOfWealth.java:288` (the RING case §4 cites) is a different branch from
`:291`.

Cracked Spyglass is an additional **automatic level-generation** route, not a
runtime drop. `RegularLevel.createItems` adds its 0–2 hidden items through
no-arg `Generator.randomUsingDefaults()` (`RegularLevel.java:679-689`, with the
level-dependent chance at `CrackedSpyglass.java:52-61`). The pushed floor RNG
isolates ambient random calls, but it does not roll back persistent Generator
state. When the default category roll lands on ARTIFACT, holding/upgrading the
Spyglass advances the same artifact deck before later floors. Method: pinned-
source call-site audit from `createItems` through
`Generator.randomUsingDefaults()` (`Generator.java:694-768`); no probability
measurement was used for this reachability fact.

The drift is not cosmetic. `randomArtifact` calls
`Reflection.newInstance(cls).random()` **after** `popGenerator`
(`Generator.java:866-878`), and `UnstableSpellbook`'s constructor burns a
variable number of `Random.chances` rolls in `setupScrolls`
(`UnstableSpellbook.java:90-103`). A shifted artifact index therefore changes
floor-stream consumption and desyncs every later draw on that floor, which in
turn shifts the deck counters every later floor reads. Deck exhaustion also
falls through to a RING draw (`Generator.java:707`), the §8 Imp lever.

**Consequence for a fixed challenge/trinket/meta profile:** everything generated
before the run's *first* levelgen ARTIFACT deck draw is free of artifact-history
drift. Everything after it is seed-determined under one additional condition —
*no artifact acquired outside level generation earlier in the run*.
`random_artifact` (`generator/state.rs:327`) is the single Rust site that moves
the counter, so the artifact gate is one monotone flag. Other player-state
hooks can still change which generation path reaches that draw; the quest
audits track those separately.

Measured over 200 seeds (`init_run(seed * 7919 + 13)`, floors 1–24, default
`MapTrinketProfile`), floor of the first levelgen artifact draw, cumulative:

| Floor | 1 | 2 | 3 | 4 | 9 | 13 | 24 |
|-------|---|---|---|---|---|----|----|
| Seeds covered | 15% | 34% | 48% | 55.5% | 87% | 95% | 100% |

Every seed draws an artifact somewhere in floors 1–24; mean main-loop drops per
floor is 3.83. Method: a throwaway `examples/loot_probe.rs` walking
`create_level_partial` and counting `ItemCategory::Artifact` in `placed_items`.
