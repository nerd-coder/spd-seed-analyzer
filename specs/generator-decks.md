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

## 2. Cross-floor coupling is deck counters only

Level generation is wrapped in `Random.pushGenerator(Dungeon.seedCurDepth())`,
so a stream difference on floor *N* cannot reach floor *N+1*. The only
cross-floor state is the deck counters. Parity for any deck-drawn identity
therefore reduces to: **the same draw sites fire the same number of times in
the same order**.

## 3. Ring-deck draw sites (complete)

Deck draws (`Generator.random`), all levelgen:

| Site | Java |
|------|------|
| `createItems` general-deck picks | `RegularLevel.java:388` |
| Ordinary + golden mimic prizes | `Mimic.java:349` (`useDecks=true`) |
| Shop rare, 1/10 | `ShopRoom.java:339` |
| Pit room main loot | `PitRoom.java:72` |
| Crystal choice hidden prize | `CrystalChoiceRoom.java:124` |
| Crystal vault prize cycle | `CrystalVaultRoom.java:104` |
| Grassy grave / mass grave / secret summoning | `Generator.random()` |
| Artifact exhaustion fallback | `Generator.java:709` |
| Ambitious Imp reward | `Imp.java:234` |

All are present in `spd-core`. Note `Generator.random()` (no-arg) is itself a
category deck, so its ring count over a deck cycle is stable even if the
in-cycle order shifts.

## 4. Runtime paths do **not** touch the ring deck

All use `randomUsingDefaults`: mob loot including Thief's
`oneOf(RING, ARTIFACT)` (`Mob.java:997`), Ring of Wealth
(`RingOfWealth.java:288`), Scroll of Transmutation
(`ScrollOfTransmutation.java:276`), Cursed Wand (`CursedWand.java:1082`,
`:1170`), and runtime-spawned mimics (`useDecks=false` at
`CursedWand.java:1066`, `DistortionTrap.java:120`).

The ungenerated side-levels are likewise irrelevant: `MiningLevel` only calls
`randomUsingDefaults(FOOD)`, `VaultLevel` only `randomUsingDefaults(...)`.

## 5. Challenges cannot shift any deck index

`Challenges.isItemBlocked` (`Challenges.java:71`) returns true **only** for a
`Dewdrop` under `NO_HERBALISM`. No ring/wand/artifact/weapon/armor generator
can produce a Dewdrop, so the reroll loops in `Mimic.generatePrize`, `PitRoom`,
and `CrystalVaultRoom` never iterate from challenges.

Of the nine challenges, only `NO_SCROLLS` touches generation at all
(`Level.java:234`, skips a Scroll of Upgrade `addItemToSpawn` — no RNG, no
deck). `DARKNESS` and `STRONGER_BOSSES` affect view distance and boss setup.
Nothing changes deck draw counts.

## 6. Trinket availability is seed-determined

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
~75% of seeds can rule Mimic Tooth out of a run entirely.

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

The artifact headroom means the exhaustion-to-ring fallback needs 5–10 extra
runtime Ring of Wealth artifact draws before floor 17 to trigger; it is
computable per seed rather than assumed.

## 8. Consequence for the Imp ring

The class is fixed by (seed, ring draw index *k*); the `+2…+4` level is fixed
by the floor's own stream and is exact regardless, since no ring subclass
overrides `Ring.random()` (`Ring.java:259`) and the class never feeds back into
RNG consumption. With 12 equal-weight ring classes, *k* off by one changes the
class ~11/12 of the time.

Only Mimic Tooth and artifact exhaustion can move *k*, and both are checkable
from the seed — see the plan in [implementation.md](implementation.md).

## 9. Old Wandmaker reward (floors 7–9)

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
- **No runtime path can move the wand deck.** Every `Category.WAND` reference
  outside levelgen uses `randomUsingDefaults`: Shaman loot via
  `Mob.createLoot` (`Mob.java:996`), Scroll of Transmutation
  (`ScrollOfTransmutation.java:211`, `:339`), Cursed Wand
  (`CursedWand.java:363`, `:371`). Consistent with §4.

Measured (200 seeds, floors 1–9):

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

`Ghost.Quest.spawn` (`Ghost.java:303-362`) runs at the **start** of
`SewerLevel.createMobs`, before `super.createMobs()` (`SewerLevel.java:140-143`)
— same placement-loop-then-reward shape as the Wandmaker.

Verified properties:

- **The armor uses no deck at all.** `Random.chances({0,0,10,6,3,1})` selects a
  tier and the class is constructed directly (`Ghost.java:322-328`): 2 =
  `LeatherArmor`, 3 = `MailArmor`, 4 = `ScaleArmor`, 5 = `PlateArmor`. Tier and
  class are the same fact; no run history can shift it.
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
- **Whether the reward keeps them is the only player-dependent bit**:
  `enchantRoll > 0.2f * ParchmentScrap.enchantChanceMultiplier()` clears both
  (`Ghost.java:358-362`). Multipliers are 1 / 2 / 4 / 7 / 10 for no scrap / +0 /
  +1 / +2 / +3 (`ParchmentScrap.java:52-65`), so the thresholds are 0.2 / 0.4 /
  0.8 / 1.4 / 2.0. **At Parchment Scrap +2 or better the reward is always
  enchanted.** The roll count does not change, so this never desyncs the stream.

Net: everything except the weapon's *class* is independent of the deck.

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
