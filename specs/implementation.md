# SPD Seed Analyzer — Next Phase

Pinned target: SPD v3.3.8 @ `7b8b845a7`; accuracy remains `partial`.

SecretGardenRoom now matches the pinned AAA floor-4 terrain, four plants, and
foliage cells.

## Broaden the LoopBuilder shop evidence

Only one pinned floor currently exercises `LoopBuilder`'s narrower shop
collision list. Capture a City or Prison shop trace for a second seed so the
path has more than a single regression case.

## Then narrow the Ambitious Imp ring identity to a candidate set

The reward class is currently dropped entirely (`ImpRing` is `Constrained`,
reported as `+N ring`). It can be narrowed to three candidates without
weakening any claim.

Each `Generator.Category` carries its own `seed` long plus a `dropped` counter
(`Generator.java:709+`), so the class of the *k*-th ring a run draws is a pure
function of the dungeon seed and *k* — independent of floor RNG position. Only
*k* is uncertain: measured over 44 seeds, 0–6 ring draws precede the Imp's
(mode 4), and the deck's 12 equal-weight classes mean an index off by one
changes the class ~11/12 of the time. The `+2…+4` level stays exact — it comes
from the floor's own stream and no ring subclass overrides `random()`.

Runtime ring sources bypass the deck (`Mob.createLoot`, Ring of Wealth,
Transmutation, Cursed Wand, runtime mimics all use `randomUsingDefaults`), so
*k* shifts only via Mimic Tooth extra mimic prizes, item-blocking challenges,
artifact-deck exhaustion falling back to `random(Category.RING)`, or a gap in
our own item-gen parity above the Imp floor.

1. In `quests/imp.rs`, alongside the accepted reward, capture the classes the
   ring deck would yield at draw index *k+1* and *k+2* (clone the generator;
   the sub-deck draw does not touch the floor stream).
2. Carry them to `ItemEntry` as an ordered candidate set, keeping level, curse,
   and spawn-presence exact. Seed search may match the set, never one class.
3. Update `specs/accuracy.json`: the Imp ring identity moves from omitted to a
   short candidate list, and say plainly that run history picks which one.
