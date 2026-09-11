# Old Wandmaker reward — verified audit

Target: **Shattered Pixel Dungeon v4.0.0 @ `2bb34a4e9`**. This note separates
the reward contract from the player's quest choices and from generation work
that can happen before the NPC callback.

## Verdict

The room is eligible only on prison floors 7–9. `PrisonLevel.initRooms` calls
`Wandmaker.Quest.spawnRoom` before the room list is shuffled
(`PrisonLevel.java:83-86`, `RegularLevel.java:105-120`), and
`PrisonLevel.createMobs` calls `spawnWandmaker` before ambient mobs
(`PrisonLevel.java:88-91`). The spawn test is
`!spawned && (type != 0 || (depth > 6 && Random.Int(10-depth) == 0))`
(`Wandmaker.java:353-376`): depth 7, 8, or 9 may be the first successful floor;
depth 9 is the final guaranteed attempt. Once the room is added, the quest type
is fixed for the run: Mass Grave / Corpse Dust, Ritual Site / Embers, or
Rot Garden / Rotberry (`Wandmaker.java:357-370`).

At the NPC callback, the call order is:

1. Try random cells in the entrance room, rejecting the entrance, solid or
   trapped cells, non-passable/empty-special terrain, and cells adjacent to a
   `Terrain.DOOR` (`Wandmaker.java:297-328`). The loop consumes one room random
   point per attempt; a valid map does not hit the Java loop's unbounded retry.
2. Set `spawned = true`, clear `given`, draw `wand1` from the seeded WAND
   category, clear its curse, and call `Wand.upgrade()` (`Wandmaker.java:331-336`).
3. Draw `wand2`; redraw while its concrete class equals `wand1`, then call
   `Generator.undoDrop` for every rejected item (`Wandmaker.java:338-348`).
   `undoDrop` tests `cls.isAssignableFrom(cat.superClass)`, which is false for a
   concrete wand class (`Generator.java:658-673`), so every rejected duplicate
   advances the WAND deck and burns its full item-randomization stream.

`Wand.random()` rolls +0/+1/+2 with 2/3, 4/15, and 1/15 weights, then rolls a
30% curse (`Wand.java:547-567`). The quest clears the curse before `upgrade`,
which adds one level and consumes `Random.Int(3)` (`Wand.java:359-365`). Each
option is always uncursed and +1, +2, or +3. The two concrete classes are
always distinct. The 13-class WAND category is equal-weight 3
(`Generator.java:397-412`).

There is no seed-only concrete class or level across all playable generation
profiles. A fixed, fully replayed profile can still produce one exact pair.
No runtime path directly advances the WAND deck: Shaman loot, Scroll of
Transmutation, and Cursed Wand all use `randomUsingDefaults`
(`Mob.java:1102-1106`, `ScrollOfTransmutation.java:203-217,336-350`,
`CursedWand.java:362-371`). The WAND deck can nevertheless sit at a different
levelgen index when state changes which levelgen draw sites fire before the
callback.

## Quest rooms paint before the wand pair

`RegularPainter.paint` shuffles rooms, then calls `r.paint` on the main floor
stream before the isolated water/grass/trap/decorate generator
(`RegularPainter.java:122-153`). Quest-room paint therefore moves
`spawnWandmaker` and the wand rolls.

**Mass Grave** is a locked 11×10 special room that only accepts a bottom door
within two cells of the room center (`MassGraveRoom.java:49-53,126-134`). Paint
places statues and custom deco, then 1–2 skeletons constrained to the top rows,
then the loot table (`MassGraveRoom.java:55-123`):

- 100% Corpse Dust, two guaranteed 1-coin stacks;
- 2×30% extra Gold, 60% `Generator.random()`, 30% `Generator.randomArmor()`.

The general-category draw can consume WAND (and other decks) on this floor
before the NPC callback. Skeleton and heap placement retry against the new
row/column constraints, so the number of main-stream calls before those loot
rolls is layout-dependent.

**Ritual Site** is a `StandardRoom` whose instance initializer still spends
`setSizeCat()` (`StandardRoom.java:53-54`; default weights force NORMAL). Paint
draws top-row wall cages, rolls `Random.Int(2)` for a cage/table row, places
each cage with `Random.IntRange(1, 2)` when both cells are free, then retries
`Random.IntRange(1, 9)` for a spare cage (`RitualSiteRoom.java:52-115`). The
ritual marker is centered one row below `center()`; four Ceremonial Candles are
queued and `ritualPos` is recorded (`RitualSiteRoom.java:117-131`). Those paint
rolls consume the main stream even though the room draws no wand.

**Rot Garden** paints terrain, a Rot Heart, and up to six Rot Lashers; it does
not draw Generator items (`RotGardenRoom.java:48-137`). The guaranteed
Rotberry seed drops only when the heart dies (`RotHeart.java:109-113`).

## Quest item and claim lifecycle

The first NPC interaction only presents the class-specific introduction and
sets `Quest.given = true` (`Wandmaker.java:104-212`). Later interaction checks
for Corpse Dust, Embers, or a Rotberry seed; without it, the NPC only reminds
(`Wandmaker.java:111-153`). Four candles can sit in inventory or ritual-room
heaps; lighting all four creates the newborn fire elemental
(`CeremonialCandle.java:123-189`), whose enemy death drops one Embers
(`Elemental.java:414-417`).

With the quest item present, `WndWandmaker` shows both stored wands; Cancel
leaves state untouched (`WndWandmaker.java:77-105,133-158`). Confirming exactly
one option consumes the quest item, identifies and picks up the selected wand
(or drops it at the NPC), destroys the NPC, and clears both stored options
(`WndWandmaker.java:108-130`). Declining, never obtaining the quest item, or
never confirming a button changes claim/presence only. Under Ascension the NPC
dies on its turn (`Wandmaker.java:75-80`).

## Stateful generation before `spawnWandmaker`

The callback happens after feeling, room selection/shuffle, every `r.paint`,
doors, isolated decoration, and `buildFlagMaps` (`Level.java:259-321`).

### Held trinkets

The default-feeling branch always performs the Mossy Clump check and, when it
does not short-circuit, the Trap Mechanism check (`Level.java:259-297`). A
successful Mossy check skips the second main-stream float and can replace the
feeling with grass or water (`MossyClump.java:54-91`), which changes room
counts only for Large/Secrets but still moves the stream before `spawnRoom`.

Trap Mechanism can replace the feeling with traps or chasm
(`TrapMechanism.java:54-104`) on the same two pre-build floats as the
no-trinket path. It does not move the fresh route’s pre-`spawnRoom` stream by
itself; it does change painting after room selection. Revealed-trap work is
isolated (`RegularPainter.java:135-153`); the surrounding painter path is the
reward lever.

The Catalyst offers four trinkets by seeded TRINKET draws
(`TrinketCatalyst.java:161-179`). Trinket transmutation advances TRINKET only
(`ScrollOfTransmutation.java:322-333`); a wand or staff transmutation uses
`randomUsingDefaults(WAND)` (`ScrollOfTransmutation.java:203-217,336-350`).
Cracked Spyglass extra loot is isolated, uses defaults except ARTIFACT, and
runs after `createMobs` (`RegularLevel.java:679-689`), so it is not a
same-floor WAND lever.

### Mimic Tooth, Rat Skull, artifacts, challenges

Mimic Tooth changes mimic chances in Suspicious Chest, Treasury, Crystal Vault,
and ordinary item population (`SuspiciousChestRoom.java:65-70`,
`TreasuryRoom.java:46-62`, `CrystalVaultRoom.java:75-89`,
`RegularLevel.java:397-432`). `Mimic.spawnAt` immediately generates an extra
reward (`Mimic.java:306-358`). On a Wandmaker floor those paint-time mimics run
before `spawnWandmaker`; on earlier floors they can move persistent decks.

Rat Skull changes the alternate Statue chance (`Statue.java:206-216`) and
Crystal Vault’s alternate Crystal-Mimic chance (`CrystalVaultRoom.java:75-89`).
An Armored Statue adds an armor generation and glyph roll
(`ArmoredStatue.java:50-56`).

Artifacts are a runtime-movable seeded deck (`Generator.java:855-878`). Prison
paint can draw them in Pit Room, Crystal Vault, Crystal Choice, and (on a shop
floor in the prior region) Shop Room (`PitRoom.java:68-80`,
`CrystalVaultRoom.java:92-105`, `CrystalChoiceRoom.java:122-127`,
`ShopRoom.java:332-350`). `UnstableSpellbook` drains a variable scroll-
probability loop (`UnstableSpellbook.java:84-103`). Exhausted artifacts now
fall back to `randomUsingDefaults(RING)` rather than the RING deck
(`Generator.java:706-710`).

`NO_SCROLLS` omits every second queued Scroll of Upgrade (`Level.java:232-240`).
Prison standard rooms include queue consumers such as Study Room and Suspicious
Chest (`StandardRoom.java:156-166,169-176`); Study Room’s empty-queue fallback
rolls Potion/Scroll then `Generator.random` (`StudyRoom.java:81-89`). Parchment
Scrap is not a Wandmaker reward lever.

## What the seed can safely rule out

Without a fully fixed generation profile, the honest public contract is:

- spawn is impossible before depth 7 and cannot be later than depth 9;
- the quest type is one of the three room/type pairs above, but its successful
  floor can move when pre-floor state changes;
- exactly two distinct WAND-category classes are pre-rolled when the NPC
  callback succeeds;
- both options are uncursed and +1 through +3;
- the player can claim at most one option; the unselected option is cleared;
- no combat/runtime drop (including ordinary Shaman wand loot) is part of this
  reward contract.

The Rust port stores both exact options plus all 13 WAND classes internally for
parity (`crates/spd-core/src/quests/wandmaker.rs`). The public report keeps
generic +1..+3 wand options as the route-independent contract and may carry the
concrete pair as an explicitly labelled fresh/no-history baseline.
