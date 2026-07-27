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
