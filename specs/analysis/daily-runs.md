# Daily Run seeds (SPD v3.3.8)

Verified against pinned SPD commit `7b8b845a7`.

## Seed derivation

SPD stores a Daily Run date as UTC-midnight epoch milliseconds and derives the
root dungeon seed with:

```text
daily seed = UTC midnight epoch milliseconds + 26^9
```

`DungeonSeed.TOTAL_SEEDS` is `26^9 = 5,429,503,678,976`
(`core/src/main/java/com/shatteredpixel/shatteredpixeldungeon/utils/DungeonSeed.java:31`).
The Daily branch adds that offset to `SPDSettings.lastDaily()` and formats the
identifier as `yyyy-MM-dd` in UTC
(`core/src/main/java/com/shatteredpixel/shatteredpixeldungeon/Dungeon.java:216-223`).

The game finds today's Daily by truncating the device's epoch-millisecond clock
to a multiple of `86,400,000`, so rollover is at UTC midnight. It clamps the
result to Unix day 20,148, the first Daily on `2025-03-01`
(`core/src/main/java/com/shatteredpixel/shatteredpixeldungeon/scenes/HeroSelectScene.java:694-697,739-746`).

Examples:

```text
2025-03-01 -> 7,170,290,878,976
2025-03-02 -> 7,170,377,278,976
2026-08-06 -> 7,215,478,078,976
```

## Separate run state

The date does not select challenges. Daily status, replay status, selected hero,
and the challenge mask are stored independently
(`Dungeon.java:233-243,624-636,719-743`). Daily and replay flags do not alter
the root seed. Daily status does suppress remains/bones interactions
(`core/src/main/java/com/shatteredpixel/shatteredpixeldungeon/Bones.java:56-68,154-158`),
which does not affect this analyzer's painter-complete public floor maps.
