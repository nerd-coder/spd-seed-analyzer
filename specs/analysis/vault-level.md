# Dwarven vault — verified audit

Target: **Shattered Pixel Dungeon v4.0.0 @ `2bb34a4e9`**.

One `VaultLevel` exists per run, on **branch 1 at the Imp's city depth**.
Room-by-room catalogue: `vault-rooms.md`. Imp spawn and `rewardOptions`:
`quest-ambitious-imp.md`.

### Verdict

Under a fixed profile (spawn depth, hero class, challenges), vault
generation is a pure function of `Dungeon.seedForDepth(depth, 1)`. The
unprofiled seed-only contract is weaker:

- the vault is reached only after the Imp quest is given and the player
  accepts the `BRANCH_EXIT` (`CityLevel.java:141-178`);
- spawn depth 17 / 18 / 19 selects which vault seed is used
  (`Dungeon.java:356-368,414-430`);
- equipment and consumable **rules** (tiers, default weights, bans,
  enchant policy) are seed-fixed given that depth;
- concrete room identities, heap contents, and wandering mobs require
  that vault seed plus hero class (mirror only) and `DARKNESS` (torches);
- public maps should show painter-complete layout, not heaps or wandering
  mobs.

Vault generation uses `Generator.randomUsingDefaults` and direct
constructors. It does **not** mutate main-dungeon category decks.

### Seeding and entry

`Dungeon.newLevel` maps branch 1, depths 16–19, to `VaultLevel`
(`Dungeon.java:356-368`). Normal play only opens a `BRANCH_EXIT` from
`AmbitiousImpRoom` at the Imp's depth, destination `(depth, branch+1)`
(`AmbitiousImpRoom.java:110-116`). The matching vault entrance is
`BRANCH_ENTRANCE` back to `(depth, 0)` (`VaultEntranceRoom.java:142-147`).

`Level.create` pushes `Dungeon.seedCurDepth()` =
`seedForDepth(depth, branch)` with look-ahead `depth + 30 * branch`
(`Dungeon.java:414-430`, `Level.java:221`). Branch 1 skips limited drops
and feelings (`Level.java:224`). `InterlevelScene` does not hold allies
when the branch changes on depths 16–20 (`InterlevelScene.java:650-655`).
Walking onto vault transitions does nothing; `EscapeCrystal` is the exit
(`VaultLevel.java:206-209`, `EscapeCrystal.java:85-91,285-293`). After
`complete`, stepping in the vault force-returns the hero
(`VaultLevel.java:576-588`).

The level is generated on first entry, after the city floor (and
`rewardOptions`) already exist.

### Builder, rooms, size

`VaultLevel.build` first queues floor loot, then `RegularLevel.build`
(`VaultLevel.java:131-146`, `RegularLevel.java:105-121`):

1. Four `createEquipment(0)`, one `Dart`, five `createConsumabe(0)`, three
   `Generator.randomUsingDefaults(FOOD)` (`VaultLevel.java:134-143`). The
   first consumable call fills **all four** consumable tiers
   (`VaultLevel.java:431-488`).
2. `initRooms` (`VaultLevel.java:149-171`): entrance; `VaultRoom.setupChances`
   then rooms until `sizeFactor` sums to 9; always `VaultTokensRoom` and
   `VaultSimpleEnemyTreasureRoom`; `VaultTreasureRoom.generateRoomList`
   then **seven** of nine treasure classes; `VaultFinalRoom`.
3. Shuffle, `GridBuilder` until placement succeeds, `CityPainter`.

`GridBuilder` uses rigid 11-tile cells, extra-connection chance 0.55, and
interleaves multi-connection rooms with `maxConnections == 1` singles
(`GridBuilder.java:38-41,67-99,299-307`). Entrance is forced onto the
perimeter (`GridBuilder.java:104-125`). `VaultFinalRoom` is 21×21 (2×2
cells) and is moved to the end of the placement queue
(`VaultFinalRoom.java:65-83`, `GridBuilder.java:127-131`). Long rooms
(`VaultHallwayRoom`, `VaultLongRingsRoom`, `VaultTokensRoom`) occupy two
cells (`VaultLongRoom.java:50-72`).

Typical pack: 18–20 rooms on a ~5×5 cell grid (one large floor, not three
sequential depths). `VaultFinalRoom` refuses a path of fewer than three
rooms to the entrance; `VaultTokensRoom` refuses fewer than two
(`VaultFinalRoom.java:96-118`, `VaultTokensRoom.java:173-190`). Placement
can retry (`RegularLevel.java:112-118`, `GridBuilder.java:166-174`).

`VaultLevel` paints with hidden-door chance 0, water 0.15 / smoothness 12,
grass 0.30 / 3, and `nTraps() == 0` (`VaultLevel.java:189-203`). Water,
grass, and decorate run under a pushed generator after room paint
(`RegularPainter.java:135-153`). `RegularPainter` shuffles rooms **again**
before `paint()` (`RegularPainter.java:122-131`), so heap/mob assignment
order is that second shuffle.

Each `VaultRoom` is a `StandardRoom` and rolls `sizeCatProbs {0,1,0}`
(always LARGE, one `Random.chances`) at construction
(`StandardRoom.java:54-55`, `VaultRoom.java:38-40`). Long rooms also roll
`wide = Random.Int(2) == 0` (`VaultLongRoom.java:39`).

### Painter-complete layout vs player state

**Seed-fixed given (run seed, Imp depth, hero class, challenges):** room
set, grid graph, terrain, doors, branch transitions, flame-trap tiles and
`VaultFlameTraps` blob, pedestal / locked-door fixtures, laser/sentry
**positions** (the NPCs themselves are not public-map entities), water and
grass patches.

**Player/run-state inputs that change generation:**

- Imp spawn depth selects the vault seed.
- `Dungeon.hero.heroClass` is read while painting `VaultTokensRoom` to
  build the mirror reward (`VaultTokensRoom.java:109-111`,
  `VaultMirror.java:69-98`). The reward RNG is `pushGenerator(Random.Long())`,
  so class does not shift later ambient vault rolls.
- `DARKNESS` adds a torch among the two `VaultBeacon`s (the `Int(3)` is
  always consumed) and two extra torches under a pushed generator in
  `createItems` (`VaultEntranceRoom.java:118-135`,
  `VaultLevel.java:613-625`).

Hero inventory at entry does not feed vault RNG. `createMobs` on the
level is empty; rooms spawn their own mobs (`VaultLevel.java:571-573`).
No respawner (`VaultLevel.java:597-599`).

### Static hazards

| Hazard | Role | Rooms |
|---|---|---|
| `VaultLaser` | Immovable death-gaze beam; cooldown/dir set at paint | `VaultLasersRoom`, `VaultLaserTreasureRoom`, `VaultHardLaserTreasureRoom` |
| `VaultSentry` | Immovable cone scan | `VaultCrossRoom`, `VaultCircleRoom`, `VaultCircleScanTreasureRoom`, `VaultManyScansRoom` |
| `VaultFlameTrap` + `VaultFlameTraps` blob | Inactive trap tile; blob handles pulses | `VaultAlternatingFireRoom`, `VaultFlamePathRoom`, `VaultFlamesTreasureRoom` |
| `VaultTokenDoor` | Object on `LOCKED_DOOR`; 10 tokens to open | `VaultTokensRoom` (`VaultTokenDoor.java:83-103`) |

Laser / sentry / flame hits use two `hazardFreebies`, then −100 ranking
score (`VaultLaser.java:105-109`, `VaultSentry.java:143-147`,
`VaultFlameTraps.java:93-97`). That is runtime, not layout.

### Loot generation (defaults, not decks)

`createEquipment` builds per-tier lists of six items with
`Generator.randomUsingDefaults` for weapons, missiles, wands, and rings,
and `new Leather/Mail/Scale/PlateArmor` for armor (`VaultLevel.java:241-378`).
Default weights only; `cat.probs` / `dropped` are untouched
(`Generator.java:744-769`). Duplicate classes are rejected except T2
weapons at low tiers (commented theoretical cap ~12 per tier,
`VaultLevel.java:236-238`). Banned: `WandOfRegrowth`, `WandOfTransfusion`,
`WandOfCorruption`, `RingOfWealth`, `RingOfMight`, `RingOfForce`.

Per-tier row: two melee, one missile, one armor, one wand, one ring.
Levels: first melee is `+0` at T0 and `+(tier+1)` otherwise; every other
slot is `+tier` (`VaultLevel.java:266-375`). Enchant/glyph:
`Random.Int(3) >= lootTier` clears, else rolls (`VaultLevel.java:271-356`).
So T0 is never enchanted, T3 is always enchanted/inscribed, T1/T2 mix.
Then `cursed = false`; rings get `levelKnown`/`cursedKnown` only (type ID
is combat); other equipment `identify(false)` (`VaultLevel.java:417-423`).

T0/T1 share `lowerTierIdx`; T2/T3 share `higherTierIdx`, cycling an even
mix (`VaultLevel.java:214-215,381-425`). T0 is filled during `build`
before rooms; T1–T3 fill lazily on first `createEquipment` during paint.

Consumable tiers are fixed class pools with `Random.oneOf` then
`Collections.shuffle`; T0–T2 prepend `PotionOfHealing`, T3 shuffles one
in (`VaultLevel.java:431-482`). `findT2SolveItem` / `findT3SolveItem`
both search `itemsToSpawn` for `StoneOfBlink` then `PotionOfInvisibility`
(`VaultLevel.java:493-526`). Those classes are **not** in the T0 queue;
they appear only if `VaultHardLaserTreasureRoom` / `VaultManyScansRoom`
already painted `addItemToSpawn`. Paint order is seed-fixed. On a miss
the room falls back to `createConsumabe`.

`findPrizeItem` steals from `itemsToSpawn` (`Level.java:829-855`). Hallway
and long-rings take an `EquipableItem`; bookcase takes any prize. Leftovers
become ordinary heaps in `createItems` (`VaultLevel.java:602-611`). Some
treasure rooms also queue `PotionOfLiquidFlame`, `PotionOfPurity`,
`PotionOfInvisibility`, or `StoneOfBlink` for that leftover pass.

**Guaranteed if the vault is generated:** two `VaultBeacon`s in the
entrance (`VaultEntranceRoom.java:118-135`); `ImpStatue` plus the six
pre-rolled Imp options on final pedestals (`VaultFinalRoom.java:228-236`);
per spawned treasure room, the equipment/consumable/token heaps listed in
`vault-rooms.md`. Heap **identities** follow the default-weight rules
above, not main-dungeon decks.

**Player-path-dependent presence:** anything the hero does not pick up
before `EscapeCrystal` is lost; only one item may leave
(`quest-ambitious-imp.md`). Combat `DwarfToken` drops are runtime
(`lootChance = 1` on vault mobs, except Multiple-Enemy ghouls with
`maxLvl = 0`). Generation-time token heaps are 5–7 (all three T1 rooms
plus 2 of 3 T2 and 2 of 3 T3; Single-Enemy and Multiple-Enemy drop none).
The token door costs 10, so heaps alone never open it.

`VaultMirror.createReward` is a class-specific unique (Broken Seal +1 with
glyph; Mage's Staff +3 enchanted; Cloak +8 charged; Spirit Bow enchanted;
unique `MirrorSword` +3 enchanted; Holy Tome +8 charged)
(`VaultMirror.java:69-98`). Taking it sets `mirrorUsed`; further tokens
are discarded on pickup (`DwarfToken.java:51-57`). The duelist sword is
`unique` and cannot leave the vault (`VaultMirror.java:198-205`).

### Boss

`VaultBossElemental` is **not** created at paint time. Stepping within
Chebyshev distance 3 of the locked door locks the entry, seals the level,
and `GameScene.add`s the boss at the room center
(`VaultFinalRoom.java:247-265`). Constructor `Random.Int(3)` for initial
form, plus combat cooldowns, use live game RNG, not the vault seed
(`VaultBossElemental.java:93-97,146-148`). Fixed generation stats: HT 600,
EXP 30, defenseSkill 20 (`VaultBossElemental.java:83-91`). Death calls
`unseal`, which opens both doors and `identify(false)` on remaining
unidentified rings (`VaultLevel.java:638-645`, `VaultFinalRoom.java:329-340`).

### Main-dungeon decks

Vault loot paths call `randomUsingDefaults` or `new` / `Reflection.newInstance`.
They do not decrement `cat.probs` or increment `cat.dropped` for RING /
WEP_Tx / MIS_Tx / WAND / FOOD. ARTIFACT is not drawn. The Imp's
city-floor `rewardOptions` draws (already done) are the deck effect; the
vault does not add another.

### Public-map recommendation

Show: room graph and types, terrain (including pedestals, bookshelves,
`EMPTY_SP`, locked doors), regular doors, branch transitions, flame-trap
tiles, `VaultFlameTraps` coverage, water, grass.

Keep internal: wandering vault mobs, heaps, `ImpStatue`, Imp
`rewardOptions`, `VaultBeacon`s, token door / mirror / laser / sentry
**actors** (their pedestal or locked-door tiles stay), boss, and any
player-path item claim. Label spawn depth and hero class / `DARKNESS` only
when those actually change layout or the mirror.
