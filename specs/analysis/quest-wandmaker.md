# Old Wandmaker reward — verified audit

Target: **Shattered Pixel Dungeon v3.3.8 @ `7b8b845a7`**. Java citations below
refer to that pinned clone. This note separates the reward contract from the
player's quest choices and from generation work that can happen before the NPC
callback. It is an internal audit; the public accuracy status remains the
user-facing coverage contract.

## Verdict

The room is eligible only on prison floors 7–9. `PrisonLevel.initRooms` calls
`Wandmaker.Quest.spawnRoom` before the room list is shuffled
(`PrisonLevel.java:83-86`), and `PrisonLevel.createMobs` calls
`spawnWandmaker` before ambient mobs (`PrisonLevel.java:88-92`). The spawn test
is `!spawned && (type != 0 || (depth > 6 && Random.Int(10-depth) == 0))`
(`Wandmaker.java:353-374`): depth 7, 8, or 9 may be the first successful floor;
depth 9 is the final guaranteed attempt. Once the room is added, the quest
type is fixed for the run: Mass Grave / Corpse Dust, Ritual Site / Embers, or
Rot Garden / Rotberry (`Wandmaker.java:357-369`).

At the NPC callback, the pinned call order is:

1. Try random cells in the entrance room, rejecting the entrance, solid or
   trapped cells, non-passable/empty-special terrain, and cells adjacent to a
   `Terrain.DOOR` (`Wandmaker.java:297-328`). The loop consumes one room random
   point per attempt; a valid map does not hit the Java loop's unbounded retry.
2. Set `spawned = true`, clear `given`, draw `wand1` from the seeded WAND
   category, clear its curse, and call `Wand.upgrade()` (`Wandmaker.java:329-337`).
3. Draw `wand2`; redraw while its concrete class equals `wand1`, then call
   `Generator.undoDrop` for every rejected item (`Wandmaker.java:338-348`). In
   this pinned version `undoDrop` does not restore a concrete class's deck
   probability (`Generator.java:656-673`), so every rejected duplicate advances
   the WAND deck and burns its full item-randomization stream.

`Wand.random()` rolls +0/+1/+2 with 2/3, 4/15, and 1/15 weights, then rolls a
30% curse (`Wand.java:546-566`). The quest clears the curse before `upgrade`,
which adds one level and consumes the `Random.Int(3)` curse-clear roll
(`Wandmaker.java:334-348`, `Wand.java:358-364`). Therefore each option is
always uncursed and +1, +2, or +3. The two concrete classes are always
distinct. The Rust port retains the exact generated pair internally and now
labels each option with the full 13-class WAND category as the conservative
cross-profile candidate set (`crates/spd-core/src/quests/wandmaker.rs`).

There is no seed-only concrete class or level across all playable generation
profiles. A fixed, fully replayed profile can still produce one exact pair;
the default analyzer must not turn that baseline sample into a universal class
or level claim. No runtime path directly advances the WAND deck: Shaman loot,
Scroll of Transmutation, and Cursed Wand all use `randomUsingDefaults`
(`Mob.java:992-1003`, `ScrollOfTransmutation.java:203-217`,
`ScrollOfTransmutation.java:336-350`, `CursedWand.java:363-371`). The WAND
deck can nevertheless be at a different levelgen index when state changes which
levelgen draw sites fire before the callback.

## Quest item and claim lifecycle

The room's quest route is generated before the NPC callback. Mass Grave always
places Corpse Dust and may add extra random loot
(`MassGraveRoom.java:53-87`); Ritual Site queues four Ceremonial Candles and
records the ritual center (`RitualSiteRoom.java:47-70`); Rot Garden places a
Rot Heart and its guaranteed seed is dropped only when the heart dies
(`RotGardenRoom.java:48-111`, `RotHeart.java:108-114`). Four candles can be in
the hero's inventory or ritual-room heaps; lighting all four creates the newborn
fire elemental (`CeremonialCandle.java:123-189`), whose enemy death drops one
Embers (`Elemental.java:259-267`, `Elemental.java:411-417`). These combat and
interaction steps determine whether the quest item is ever held, not the
already generated wand pair.

The first NPC interaction only presents the class-specific introduction and
sets `Quest.given = true` (`Wandmaker.java:104-212`). Later interaction checks
for the matching Corpse Dust, Embers, or Rotberry seed in the hero's belongings;
without it, the NPC only shows a reminder (`Wandmaker.java:111-153`). With the
item present, `WndWandmaker` displays both stored wands; each button opens an
info window and Cancel leaves state untouched (`WndWandmaker.java:77-105`,
`WndWandmaker.java:133-158`). Confirming exactly one option consumes one quest
item, identifies and picks up the selected wand (or drops it at the NPC when
the backpack cannot accept it), destroys the NPC, and clears both stored
options (`WndWandmaker.java:108-130`). Declining the conversation, never
obtaining/killing the quest target, canceling the reward window, or never
confirming a button therefore changes claim/presence only; it does not reroll a
wand. Under Ascension's challenge buff the NPC dies on its turn
(`Wandmaker.java:74-80`), so no reward can be claimed through that callback.

## Stateful generation before `spawnWandmaker`

The callback happens after the entire floor has selected its feeling, built and
shuffled rooms, painted every room, and run `buildFlagMaps` (`Level.java:255-313`).
The following stateful paths can change that pre-callback work or the persistent
decks it consumes.

### Held trinkets and their route to the floor

The default-feeling branch always performs the Mossy Clump check and, when it
does not short-circuit, the Trap Mechanism check (`Level.java:255-291`). Mossy
Clump can replace a normal feeling with grass or water
(`MossyClump.java:54-91`); a successful Mossy check also skips the second
main-stream float. That one-call offset changes room selection before
`PrisonLevel.spawnRoom`, so it can change this route's spawn floor, quest type,
and the later painter stream (`RegularLevel.java:124-165`).

Trap Mechanism can replace the normal feeling with traps or chasm
(`TrapMechanism.java:54-104`), but its successful route consumes the same two
pre-build floats as the no-trinket path. Those feelings do not change the room
counts, which only special-case Large and Secrets. Trap Mechanism therefore
does not move the fresh route's pre-`spawnRoom` stream by itself, but it does
change painting after room selection and before the NPC reward callback. Its
revealed-trap work runs under RegularPainter's isolated generator
(`RegularPainter.java:122-153`, `:465-490`); the surrounding changed painter
path, not that isolated presentation draw alone, is the reward lever.

The Catalyst offers four trinkets by seeded TRINKET draws
(`TrinketCatalyst.java:161-179`). The player may choose, upgrade, decline, or
later transmute an offer. Trinket transmutation itself advances TRINKET only
(`ScrollOfTransmutation.java:322-333`); a Wand or Staff transmutation uses
`randomUsingDefaults(WAND)` and cannot advance the WAND deck
(`ScrollOfTransmutation.java:203-217`, `:336-350`). What matters is the
stateful trinket's later map/painter callback, not the transmutation draw
itself. The first possible held depth and upgrade history are player choices;
the analyzer's baseline does not infer that a Catalyst offer was accepted.

### Mimic Tooth

Mimic Tooth changes the chance of a mimic in Suspicious Chest
(`SuspiciousChestRoom.java:55-70`), Treasury (`TreasuryRoom.java:46-62`),
Crystal Vault (`CrystalVaultRoom.java:75-89`), and the ordinary item population
(`RegularLevel.java:397-448`). `Mimic.spawnAt` immediately generates an extra
reward, and a held Tooth adds another default reward
(`Mimic.java:306-359`). These rewards can consume the main floor stream and,
for deck-backed categories, persistent deck draws before a later quest floor.
On a Wandmaker floor they run during room painting before `spawnWandmaker`;
on earlier floors they can move persistent artifact history and alter later
levelgen constructor tails. Tooth is therefore a real pre-callback profile
input, but “Tooth is among the first four Catalyst offers” is not a bound on
the WAND reward's next five classes. The five-candidate heuristic was removed
from the Rust port.

### Rat Skull and other exotic replacements

Rat Skull changes the alternate Statue chance (`Statue.java:198-212`); an
Armored Statue performs an extra armor generation and glyph roll
(`ArmoredStatue.java:49-56`). It also changes Crystal Vault's alternate
Crystal-Mimic chance (`CrystalVaultRoom.java:75-89`), whose immediate mimic
prize has the same pre-callback stream effect. Piranha and Elemental exotic
selection consume their ordinary replacement roll, but their constructors do
not add a levelgen item draw in these prison room paths
(`Piranha.java:206-213`, `Elemental.java:600-614`); those subtype facts do not
by themselves change Wandmaker rewards. Rat Skull is not currently a supported
explicit map profile, so its concrete effect must remain a possible route,
not a public exact pair.

### Artifact history and constructor tails

Artifacts are a runtime-movable seeded deck. `Generator.randomArtifact` advances
the ARTIFACT dropped counter and returns null when exhausted
(`Generator.java:854-879`); `Generator.random(Category.ARTIFACT)` then falls
back to a ring (`Generator.java:698-710`). Prison room paint can draw artifacts
in Pit Room, Crystal Vault, Crystal Choice, and (on a shop floor in the prior
region) Shop Room (`PitRoom.java:65-80`, `CrystalVaultRoom.java:92-105`,
`CrystalChoiceRoom.java:117-127`, `ShopRoom.java:332-350`). Obtaining an
artifact through any earlier runtime path, or a Mimic Tooth extra reward, can
therefore change the later artifact identity. If that identity is
UnstableSpellbook, its constructor drains a variable scroll-probability loop
(`UnstableSpellbook.java:84-104`), changing the current floor stream before
the Wandmaker callback. A seeded/custom run excludes prior-run Bones loot
(`Bones.java:198-201`), but normal run history still needs an explicit no-external-
artifact assumption for a concrete pair.

### Challenges and queued limited drops

`NO_SCROLLS` increments the same limited-drop counter but omits every second
queued Scroll of Upgrade (`Level.java:228-235`). `findPrizeItem` removes a
queued item or returns null when the queue is empty (`Level.java:799-826`).
Prison's standard-room table includes queue consumers such as Ring Room,
Study Room, and Suspicious Chest (`StandardRoom.java:124-166`); Study Room's
empty-queue fallback rolls a Potion/Scroll category and then calls the
generator (`StudyRoom.java:78-89`). Thus a challenge-dependent queue state can
change which painter callback and RNG path runs before `spawnWandmaker`, even
though omitting the Scroll itself has no direct random call. The Rust analyzer
does not yet replay this challenge mask as a Wandmaker-specific profile; no
challenge-independent concrete class or level should be promised.

Other challenge checks in the pinned pre-callback path either do not alter room
painting or affect only post-callback ambient mobs. Parchment Scrap changes
weapon/armor effect retention inside an isolated item stream and does not alter
Wandmaker's wand randomization; it is not a Wandmaker reward lever.

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

The Rust implementation replays the fixed profile's call order and stores both
exact options plus all 13 WAND classes internally for parity. The public report
keeps generic +1..+3 wand options as the route-independent contract and may
also carry the concrete pair as an explicitly labelled fresh/no-history
baseline. Shared renderers must not expand the full category at a sampled
level or present the baseline pair as universal.

## Verification

`crates/spd-core/src/quests/wandmaker.rs` tests cover depth-9 guaranteed room
spawn, depth-6 rejection, deterministic pair generation, full-category
candidate coverage, uncursed/distinct options, and fixed-profile reward parity
for committed Java final-heaps fixtures (AAA floor 9, GFX floor 7, HKT floor 7).
