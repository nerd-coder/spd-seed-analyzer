# Floor layout certainty

Target: **Shattered Pixel Dungeon v4.0.2 @ `57a4e06a4`**. Public map layout
follows `MAP-LAYOUT-GOAL`: painter-complete rooms, connections, doors,
transitions, terrain, traps, plants, blobs, and custom tiles/carpets, captured
before NPC, mob, heap, forced-item, or Guide Page population. Those populated
entities are drawn on top and flagged in `uncertain_cells` when run history can
change them. A later floor can still depend on that population when it mutates
persistent generator state.

This note is Java generation fact. A declared run profile is a player-facing
description of challenges, held trinkets, and artifact events. It is not an
analyzer floor list.

## Verdict

- Depth 1 of a fresh custom-seeded main-path descent has no feeling roll and no
  prior player-controlled generation history. Its painter-complete map is
  seed-only once the challenge mask is known.
- From depth 2, feeling, room counts, and painted terrain still come from the
  per-depth stream (`Dungeon.seedCurDepth()`), but several run-profile inputs
  can change that stream's interpretation or consume extra generator state.
- Same seed plus the same Mossy Clump instance and level produce the same
  builder/room graph on the default feeling branch. GRASS/WATER still change
  water and grass fill. Trap Mechanism CHASM still changes padding and default
  fill. See `level-feelings.md`.
- Arbitrary play, sealed-floor Ankh regeneration, and undeclared trinket or
  artifact history remain labelled conditional.

## Seed-only versus declared profile

Each `Level.create` pushes `Dungeon.seedForDepth(depth, branch)` before queued
drops, feeling, `build`, `createMobs`, and `createItems`
(`Level.java:218-325`, `Dungeon.java:414-431`). Combat on an already generated
floor cannot advance the next floor's ambient stream.

| Painter-complete fact | Seed-only? | Extra input |
|---|---|---|
| Forced-drop queue before build (food, SoS, SoU, stylus, stones, catalyst) | Yes, given challenges and limited-drop counters | Forbidden Runes omits every second SoU (`Level.java:232-240`) |
| Feeling on depths 2+ of branch 0 non-boss floors | Yes if no feeling trinket is held | Mossy Clump / Trap Mechanism instance and `buffedLvl()` (`Level.java:259-298`) |
| Builder kind (loop vs figure-eight) and loop shape | Yes, given feeling | Shop-floor lazy stock can still move this stream (depths 6, 11, 16) |
| Standard / special / secret room multiset | Yes, given feeling | LARGE and SECRETS change counts; secret/special run decks persist across floors |
| Room bounds and connections | Yes, given the room list and no shop-stock RNG | Shop `spacesNeeded` generates stock while the builder sizes the room (`ShopRoom.java:72-97`) |
| Doors, stairs, painted terrain | Yes only if no paint-time item/mob RNG depends on player state | Forbidden Runes queue, Mimic Tooth, Rat Skull, prize `findPrizeItem`, Barren Land plants |
| Water, grass, traps, region deco | Yes, given feeling and Trap Mechanism reveal level | Isolated by one ambient `Random.Long()` (`RegularPainter.java:135-153`) |

`RegularLevel.build` chooses the builder, then `initRooms`, shuffles, places,
and paints (`RegularLevel.java:105-121`). `initRooms` uses LARGE to force max
standard/special counts and scale them, and SECRETS to add one secret
(`RegularLevel.java:124-166`). WATER, GRASS, DARK, TRAPS, and CHASM do not
change those counts.

Region standard/special rolls (non-LARGE):

| Region | Standard | Special |
|---|---|---|
| Sewers | 4–6 | 1–2 |
| Prison | 5–6 | 1–3 |
| Caves | 6–7 | 2–3 |
| City | 6–8 | 2–3 |
| Halls | 8–9 | 2–3 |

(`SewerLevel.java:88-99`, `PrisonLevel.java:95-106`, `CavesLevel.java:95-106`,
`CityLevel.java:93-104`, `HallsLevel.java:97-108`). Secret identities come from
the run-shuffled secret deck (`SecretRoom.java:61-62,90-100`). Per-floor counts
are `secretsForFloor` (`SecretRoom.java:66-88`). Specials come from the
run queue plus a laboratory on depth 3 or 4 of each chapter
(`SpecialRoom.java:131-138`, `Dungeon.java:589-598`).

`Builder.findFreeSpace` picks the closest overlapping room by Euclidean
`Point.length()` (`Builder.java:72-107`, `Point.java:80-82`). `GridBuilder` is
only `VaultLevel` (`VaultLevel.java:185-187`) and is not on the main path.

City hallway entrance/exit rooms place a fixed center tile and do not consume
the `Random.Int(2)` statue-versus-pedestal roll that ordinary hallway rooms
consume (`HallwayRoom.java:107-122`). Statues rooms still use `Random.Int(2)`
for even-dimension centers (`StatuesRoom.java:85-94`). Those are paint-stream
facts, not room-selection facts.

`paintTraps` is painter-complete trap terrain, inside the isolated
`Random.Long()` above. Candidates are `EMPTY` trap-placeable cells
(`RegularPainter.java:429-447`). Open space means both axes have a passable
neighbor. `PathFinder.CIRCLE4` is `{-width, +1, +width, -1}`
(`PathFinder.java:73`), so indices 0 and 2 are vertical and 1 and 3 are
horizontal. `Level.setSize` installs that array for the level width before
painting (`Level.java:352`). The test is passable on `(0 or 2)` and `(1 or 3)`
(`RegularPainter.java:456-465`). A trap with `avoidsClosedSpaces`
(`Trap.java:70-71`) draws `Random.element(validOpenSpace)` when that list is
non-empty. Every other trap, and an `avoidsClosedSpaces` trap when the open
list is empty, draws from `validCells` (`RegularPainter.java:475-484`). After
each placement, that cell leaves both lists, and its four orthogonal neighbors
leave `validOpenSpace` only (`RegularPainter.java:485-492`). Two
`avoidsClosedSpaces` traps therefore cannot be orthogonally adjacent while the
second still draws from `validOpenSpace`.

`StatuesRoom`, `StatuesEntranceRoom`, and `StatuesExitRoom` do not call
`Statue.random`. They paint `STATUE` tiles and carpets. When `width() >= 11`
or `height() >= 11`, the entrance and exit variants also set the center
transition, paint non-`STATUE` cells of the center carpet rect to `EMPTY_DECO`,
and insert that carpet (`StatuesEntranceRoom.java:46-91`,
`StatuesExitRoom.java:46-91`). The special `StatueRoom` calls `Statue.random()`
during `paint` and adds the mob (`StatueRoom.java:70-72`).
`Statue.random(boolean)` spends one parent-stream `Random.Float()` on armored
versus normal (`1/10f * RatSkull.exoticChanceMultiplier()`, half as effective
above 0.1), then runs `createWeapon` inside `Random.pushGenerator(Random.Long())`
(`Statue.java:206-221`). Rat Skull can change the type. It cannot change how
many parent-stream calls the weapon roll makes: always that `Float()` plus
that `Long()`, so the weapon roll cannot desync later levelgen. The statue and
its weapon are mob state, outside the public map. The `Long()` is consumed
when the statue is created, which current SPD does during special statue-room
paint, before later rooms in the shuffled paint loop and before `paintDoors`
and the isolated water/grass/trap generator. A public-map cutoff that skips
mob creation still has to consume it.

## Feeling order (summary)

On branch 0, non-boss, `depth > 1`: `Random.Int(14)` then, on cases 7–13, two
pre-rolled floats and optional Mossy/Trap override (`Level.java:259-298`).
LARGE also queues a second food from `Generator` before build. Full chances,
decks, and same-seed rules: `level-feelings.md`.

CHASM uses padding 2 and fills unset cells with chasm
(`RegularPainter.java:79-81`, `Level.java:334`). WATER/GRASS change region
fill percents (for example sewers 0.85/0.80 versus 0.30/0.20,
`SewerLevel.java:102-107`). TRAPS paints `5 * nTraps` traps with extras
visible (`RegularPainter.java:474-475`). SECRETS pulls hidden-door chance
toward 50% (`RegularPainter.java:187-197`). DARK only shortens `viewDistance`.

## Challenges that touch painter-complete maps

Nine bits exist (`Challenges.java:30-38`). Only these alter initial layout:

- **Forbidden Runes** (`NO_SCROLLS`): every second queued SoU is omitted
  (`Level.java:232-240`). `findPrizeItem` may then take a different queued
  item or return null (`Level.java:827-855`).
- **Barren Land** (`NO_HERBALISM`): `Level.plant` still does the grass/RNG
  work, then returns null (`Level.java:1051-1069`). Plants are part of the
  public layout. Its only other hook, `Challenges.isItemBlocked`, blocks
  Dewdrops, which floor generation never creates (`Challenges.java:71-78`).
  Oracle fixtures `aaa-aaa-{aan,aag,aac}-final-heaps-floor-{2,8,11}-barren-land.json`
  (Plants/Garden, Rot Garden, Caves Secret Garden) match terrain, mobs, and
  heaps.
- **Badder Bosses** (`STRONGER_BOSSES`): depth 15 inactive-trap chance is 1/4
  instead of 1/8 (`CavesBossLevel.java:126-133`).
- **Darkness**: `viewDistance` 2 (`Level.java:158`). Extra torches are isolated
  item drops, not terrain (`RegularLevel.java:472-490`).

On Hunger, Faith Is My Armor, Pharmacophobia, Swarm Intelligence, and Champion
Enemies have no initial painter-complete terrain hook.

Champion Enemies still changes initial entities. `Level.createMob` calls
`ChampionEnemy.rollForChampion` (`Level.java:542`), whose `Random` calls are
constant (`ChampionEnemy.java:98`), but a Giant champion is `LARGE`
(`Char.java:1411-1413`), so `createMobs` placement retries differ
(`RegularLevel.java:275`) and every later level-creation draw on that floor
shifts. `Dungeon.mobsToChampion` also counts runtime spawns
(`ChampionEnemy.java:95,109-119`), so which mob champions after floor 1 is not
seed-determined. Measured with the oracle on FVX-YTN-SLC: of the nine bits,
only `champion-enemies` changes the floor-17 heaps.

What stays seeded (`level/create_mobs/champion.rs`):

- `Dungeon.init` sets `mobsToChampion = 1` before floor 1 (`Dungeon.java:237`),
  so floor 1's first unblocked mob is a champion with its rolled buff. A Giant
  there is placed as `LARGE`. Oracle fixtures
  `aaa-aaa-{acv,aes}-final-heaps-floor-1-champion-enemies.json` move such a
  Giant to open space; the replay matches terrain, mobs, and heaps.
- Later floors know only the buff roll. A mob rolls Giant (`Random.Int(6) == 3`)
  and is not blocked (Crab/HermitCrab ≤3, Thief/Bandit ≤4, Guard ≤7, Bat ≤9,
  `ChampionEnemy.java:111-115`): a `LARGE` check only rejects more probes, so a
  placement on an open-space cell is identical either way. Only a possible
  Giant placed outside open space marks the floor's entities and later
  generation as uncertain.

## Trinkets and artifacts

Layout-affecting trinkets, given class, `buffedLvl()`, and instance identity:

- **Mossy Clump** — GRASS/WATER override; does not change room counts.
- **Trap Mechanism** — TRAPS/CHASM override plus reveal chance during
  `paintTraps` (`TrapMechanism.java:66-76`, `RegularPainter.java:471-504`).
- **Mimic Tooth** — mimic chance and extra `generatePrize` during paint
  (`Mimic.java:306-358`). That can move the paint stream and item decks.
- **Rat Skull** — Crystal Vault chest versus Crystal Mimic
  (`CrystalVaultRoom.java:75-82`). The armored-statue roll is still one
  parent `Float()` (`Statue.java:208-214`); `createWeapon` then sits under
  one parent `Long()` (`Statue.java:216-219`), so the skull does not change
  how many parent calls follow.
- **Cracked Spyglass** — extra hidden loot after ordinary `createItems`, under
  an isolated `Random.Long()` (`RegularLevel.java:679-690`). The isolated
  stream protects ambient RNG; `Generator.randomUsingDefaults()` can still
  select ARTIFACT and mutate the artifact deck for later floors.

A transmute constructs a new trinket and copies level, not `levelFeels` /
`shuffles` (`ScrollOfTransmutation.java:322-334`). Upgrade of the same object
preserves the decks.

Runtime artifact requests always use the ARTIFACT deck; exhaustion falls back
to `randomUsingDefaults(RING)` (`Generator.java:706-710,744-752`). Artifact
state matters for special-room prizes during paint, lazy shop stock on 6/11/16,
and the depth-20 Imp shop before City decoration.

## Must stay labelled conditional

- Rat Skull, Cracked Spyglass, and Barren Land when those profile fields are
  unset.
- Every entity on a floor after floor 1 where a possible Giant champion sits
  outside open space (Champion Enemies).
- Any undeclared acquire / upgrade / transmute timing.
- Floor 1 tutorial hidden entrance doors (`SPDSettings.intro()`) and floor 2
  guidebook hiding (`RegularPainter.java:268-275`).
- Sealed-floor Ankh resurrection, which calls `Dungeon.newLevel()` again
  (`InterlevelScene.java:749-759`). `InterlevelScene.Mode.RESET` has no
  assignment site.
- Talent Cached Rations and Dried Rose petals (isolated item drops, not
  layout).

Custom seeded and challenged runs do not import prior-run Bones loot
(`Bones.java:198-201`). No Bones setting is required for that scope.

## Finite inputs at a floor boundary

For first generation of each main-path floor, the missing inputs are finite:

```text
challenges          = 9-bit mask
trinket_instances   = class, buffed level, instance id, first effective depth
artifact_events     = obtained or transmuted outside levelgen before depth N
limited_drops       = counters from prior generated floors under that profile
special/secret decks
mossy/trap          = remaining cards + shuffles on the current instance
```

Condition inputs that still exist in the game, and that a seed analyzer can
represent as a declared profile, are exactly those rows. Java does not make
later regular floors seed-only in the absence of that profile.

The truthful product claim is **first generation of each main-path floor under
a declared profile**. Room selection can be exact while painted doors or
terrain remain conditional; an unknown paint tail must not erase a known room
multiset.
