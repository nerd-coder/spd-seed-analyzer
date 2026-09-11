# Level feelings, Mossy Clump, and Trap Mechanism

Target: **Shattered Pixel Dungeon v4.0.0 @ `2bb34a4e9`**.

Feelings are chosen during `Level.create`, after forced-drop queueing and
before `build` (`Level.java:218-316`). They apply only on branch 0, non-boss
floors with `Dungeon.depth > 1`. Depth 1 is always `Feeling.NONE` with no
feeling RNG.

## The 50% roll

```text
switch (Random.Int(14)):
  0 CHASM
  1 WATER
  2 GRASS
  3 DARK          // also viewDistance = round(5 * viewDistance / 8)
  4 LARGE         // also Generator.random(FOOD) into itemsToSpawn
  5 TRAPS
  6 SECRETS
  7–13 default    // two floats, then optional trinket override
```

(`Level.java:259-298`, enum at `Level.java:126-134`.) Seven of fourteen
outcomes are a named feeling (50%). Each named feeling is `1/14` (~7.14%).
The other seven go to the default branch.

LARGE and SECRETS are the only feelings that change `initRooms` counts
(`RegularLevel.java:129-163`). WATER and GRASS change region water/grass fill.
CHASM changes padding and default fill (`RegularPainter.java:79-81`,
`Level.java:334`). TRAPS paints `5 * nTraps` traps with extras visible
(`RegularPainter.java:473-474`). DARK only shortens view distance.

`nTraps()` is `NormalIntRange(2, 3 + depth/5)` on the depth stream before the
painter isolates decoration (`RegularLevel.java:193-195`,
`RegularPainter.java:135-153`).

## Default branch: two floats, then override

On cases 7–13 the depth stream always pays two `Random.Float()` calls, then:

```text
if (mossyChance < MossyClump.overrideNormalLevelChance())
    feeling = MossyClump.getNextFeeling();
else if (trapMechChance < TrapMechanism.overrideNormalLevelChance())
    feeling = TrapMechanism.getNextFeeling();
else
    feeling = NONE;
```

(`Level.java:286-297`.) Absent trinkets have override chance 0
(`MossyClump.java:54-64`, `TrapMechanism.java:54-64`), so both floats are
still consumed and the feeling stays NONE. `getNextFeeling` is not called.

Override chance by `trinketLevel` (`buffedLvl()`, or -1 if not held):

```text
level -1 -> 0
level  0 -> 0.25
level  1 -> 0.50
level  2 -> 0.75
level  3 -> 1.00     // every default-branch floor overrides
```

`Random.Float()` is in `[0, 1)` (`Random.java:77-79`), so a +3 clump always wins
the mossy test.

## Mossy Clump deck

`getNextFeeling` loads the held `MossyClump` from belongings. If none, it
returns NONE (`MossyClump.java:71-75`). Otherwise it owns a six-card list and
a `shuffles` counter on **that object**:

```text
cards: true, true, false, false, false, false
true  -> GRASS
false -> WATER
```

When the list is empty it pushes `Dungeon.seed+1`, fills those six booleans,
shuffles `shuffles+1` times, increments `shuffles`, and pops
(`MossyClump.java:76-88`). Drawing then `remove(0)`. The isolated generator
does not advance the depth stream.

The deck and `shuffles` are bundled on the item (`MossyClump.java:94-120`).
Transmutation constructs a new trinket and copies level/curse/knowledge, not
`levelFeels` or `shuffles` (`ScrollOfTransmutation.java:322-334`). Upgrading
the same object keeps the deck.

### Same seed, same clump, same layout

For a fixed seed, the per-depth `Int(14)` and the two default-branch floats
are functions of `seedForDepth` only. Whether a default-branch floor overrides
is `mossyChance < chance(level)` at generation time. `getNextFeeling` then
draws from an instance-local deck shuffled under `Dungeon.seed+1`.

Therefore two descents of the same seed that hold the **same Mossy Clump
instance at the same buffed level on the same floors** produce:

- the same override sequence (GRASS/WATER versus NONE);
- the same subsequent builder/room-selection RNG on those floors, because the
  depth stream always spent two floats and the deck shuffle was isolated;
- the same painter-complete room graph (GRASS/WATER do not change room
  counts);
- possibly different water/grass fill versus a run that never held the clump,
  because the feeling still selects region fill percents.

A new instance (transmute-away and reacquire) restarts `shuffles` at 0 with an
empty list. A level change mid-run changes which later default-branch floors
override and therefore how far the deck advances.

## Trap Mechanism deck

Same structure (`TrapMechanism.java:83-104`):

```text
cards: true, true, true, false, false, false
true  -> TRAPS
false -> CHASM
```

Also shuffled under `Dungeon.seed+1` on the item instance. Reveal chance while
painting traps is `0.1 + 0.1 * level` (`TrapMechanism.java:66-76`,
`RegularPainter.java:470-498`).

TRAPS does not change `initRooms`. CHASM does not change the room multiset
either, but padding 2 versus 1 and chasm default fill change the painted map.

Same-seed rules match Mossy: same instance, same level, same floors ⇒ same
TRAPS/CHASM/NONE sequence and the same builder RNG on the default branch.

## Both trinkets

A limited-drop run queues one Trinket Catalyst on depths 1–3
(`Dungeon.java:584-587`). The default branch still always rolls both floats.
Mossy is tested first. If mossy succeeds, `TrapMechanism.getNextFeeling` is
not called and the trap deck does not advance. If mossy fails (or is absent)
and trap succeeds, the trap deck advances.

The source comments that only one override chance is above 0 at a time
(`Level.java:287-288`). The if/else encodes mossy priority even if both items
are held.

## Analyzer condition inputs

The Java inputs that a declared profile must name for exact feelings are:
held Mossy Clump and/or Trap Mechanism, each object's instance identity,
`buffedLvl()` at each floor boundary, and whether the default `Int(14)` branch
was taken (that last one is seed-only). Those are the feeling condition
inputs that still exist.
