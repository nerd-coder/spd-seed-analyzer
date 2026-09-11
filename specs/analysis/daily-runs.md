# Daily Run seeds

Target: **Shattered Pixel Dungeon v4.0.0 @ `2bb34a4e9`**.

## Seed derivation

SPD stores the Daily date as UTC-midnight epoch milliseconds and sets the root
dungeon seed with:

```text
daily seed = UTC midnight epoch milliseconds + 26^9
```

`DungeonSeed.TOTAL_SEEDS` is `26^9 = 5,429,503,678,976`
(`DungeonSeed.java:31`). The Daily branch adds that offset to
`SPDSettings.lastDaily()` and formats `customSeedText` as `yyyy-MM-dd` in UTC
(`Dungeon.java:217-223`).

Today's Daily truncates `Game.realTime` to a multiple of `86,400,000` (UTC
midnight). It then clamps to Unix day **20,544**, `2026-04-01`
(`HeroSelectScene.java:739-746`):

```text
time = Game.realTime - (Game.realTime % DAY)
time = Math.max(time, 20_544 * DAY)
SPDSettings.lastDaily(time)
```

If `lastDaily + DAY` is still in the future, the button starts a **replay** of
that already chosen date and does not write a new `lastDaily`
(`HeroSelectScene.java:747-750`).

Examples:

```text
2026-04-01 -> 7,204,505,278,976
2026-04-02 -> 7,204,591,678,976
2026-09-11 -> 7,218,588,478,976
```

Daily seeds are at least `TOTAL_SEEDS`, so they sit outside the
user-enterable `[0, TOTAL_SEEDS)` code range (`Dungeon.java:218-220`,
`DungeonSeed.java:78-80`).

Starting a Daily requires the Victory badge unless debug
(`HeroSelectScene.java:703-709`).

## What Daily does not select

The date does not choose challenges or hero. `Dungeon.init()` copies
`SPDSettings.challenges()` and `GamesInProgress.selectedClass.initHero`
(`Dungeon.java:233-237,281-287`). Daily and replay flags are stored beside the
seed (`Dungeon.java:210-211,632-633,732-733`) and do not alter it.

The ordinary Start button clears both Daily flags before `initSeed`
(`HeroSelectScene.java:157-159`). Pre-victory play also forces challenges and
custom seed empty (`HeroSelectScene.java:236-239`).

Daily runs neither leave nor receive remains (`Bones.java:64-68,154-158`).
Seeded/challenged runs also refuse prior-run Bones items
(`Bones.java:89-93,198-201`). That does not change painter-complete public
maps.
