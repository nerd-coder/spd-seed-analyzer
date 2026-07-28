# Ambitious Imp reward — verified audit

Target: **Shattered Pixel Dungeon v3.3.8 @ `7b8b845a7`**. This note separates
the fixed quest contract from player/run state that is not encoded by the
dungeon seed. The public accuracy manifest should promise only the intersection
of all reachable routes unless the user explicitly selects a complete profile.

### Verdict

The Ambitious Imp always pre-rolls one ring when the quest room is selected,
but the concrete baseline ring is **not** a universal seed-only result. The
strongest unprofiled contract is:

- the Imp first attempts to spawn on depth 17, retries on 18, and is guaranteed
  to spawn by depth 19 (`Imp.java:212-241`);
- the target is Monks on depth 17, Golems on depth 19, and a 50/50 roll on
  depth 18 (`Imp.java:218-229`);
- exactly one ring is stored as the reward; generation retries while the ring
  is cursed, upgrades the accepted ring twice, and then forcibly curses it
  (`Imp.java:231-238`);
- therefore every possible reward is a cursed ring at **+2 through +4**. Its
  concrete class and concrete level require a fixed run-history/profile;
- the Monk route requires five Dwarf Tokens and the Golem route requires four
  (`Imp.java:112-126`);
- the reward is not guaranteed to become obtainable merely because its object
  was pre-rolled. The player must accept the quest, kill enough matching
  targets, return, and confirm the reward (`Imp.java:104-137`,
  `WndImp.java:57-86`);
- completing the quest is the condition for the depth-20 Imp shop to appear,
  but the stock is generated during depth-20 construction regardless of
  whether the quest is already complete (`CityBossLevel.java:190-201`,
  `ImpShopRoom.java:63-69`).

The analyzer may show a concrete baseline continuation for parity evidence. It
must not expose that class, level, spawn depth, or target as universal
seed-only fact without fixing the state routes below.

### Spawn and reward call order

`Level.create()` pushes the per-depth generator, evaluates forced-drop and
feeling state, then calls `build()` (`Level.java:215-316`).
`RegularLevel.build()` calls `builder()` and then `initRooms()` before room
shuffle, construction, and painting (`RegularLevel.java:104-121`). Finally,
`CityLevel.initRooms()` calls `Imp.Quest.spawn(super.initRooms())`
(`CityLevel.java:201-204`). The Imp reward is consequently created:

1. after the current floor's forced-drop/feeling checks, builder selection,
   standard/special/secret room selection, and room constructors;
2. before room shuffle, layout construction, painting, mob generation, and
   ordinary item generation.

The spawn condition is short-circuited as
`!spawned && depth > 16 && Random.Int(20-depth) == 0`
(`Imp.java:212-214`). Thus depth 17 uses `Int(3)`, depth 18 uses `Int(2)`, and
depth 19 uses `Int(1) == 0`. A successful depth-18 attempt consumes one further
`Int(2)` for the target; depths 17 and 19 do not (`Imp.java:218-229`). Failed
attempts create neither room nor reward.

For each reward attempt, `Generator.random(RING)` chooses the class from the
seeded category deck and calls the new ring's `random()` method
(`Generator.java:698-739`). `Ring.random()` rolls an initial +0/+1/+2 level and
then a 30% curse chance (`Ring.java:259-277`). Cursed attempts are discarded but
still permanently advance `RING.dropped` and consume their floor-stream rolls.

The first uncursed ring is upgraded twice. `Item.upgrade(2)` dispatches two
individual upgrades (`Item.java:401-415`); each `Ring.upgrade()` increments the
level and consumes `Random.Int(3)` for curse clearing (`Ring.java:227-235`). The
quest then sets `cursed = true` unconditionally (`Imp.java:233-238`). No ring
subclass in the pinned tree overrides `random()` or `upgrade()`, so changing
only the deck index changes the class but not the reward call shape.

### Acceptance, tokens, and claim are player-controlled

The first hero interaction has no yes/no reward choice: it sets `given = true`
and `completed = false` after showing the relevant target text
(`Imp.java:104-137`). The player can still choose whether and when to interact.
Kills made before that interaction do not grant quest tokens.

`Imp.Quest.process` grants one Dwarf Token only while the quest is spawned,
given, incomplete, and the current depth is not 20; the killed mob must match
the selected Monk/Golem target (`Imp.java:247-255`). Monk and Golem loot hooks
invoke that method before their ordinary loot (`Monk.java:74-79`,
`Golem.java:83-87`). Matching ambient mobs are present in the city spawn tables:
Monks from depth 17 and Golems from depth 18, with both on later city floors
(`MobSpawner.java:157-185`). Their ordinary combat loot is runtime RNG and is
not part of the quest reward.

The reward window opens at five tokens for Monks or four for Golems
(`Imp.java:112-126`). Confirming it removes **all** Dwarf Tokens, identifies the
single pre-rolled ring, and either picks it up or drops it at the Imp when the
backpack is full. Only then does the Imp flee and `Quest.complete()` clear the
stored reward and mark completion (`WndImp.java:69-86`, `Imp.java:257-267`).
Closing/never confirming the window, never killing enough targets, or never
returning leaves the reward unclaimed. If the quest is postponed until ascent,
the quest Imp destroys itself while Ascension Challenge is active
(`Imp.java:65-80`).

There is no alternate reward and no reward reroll at claim time.

### Current-floor state can change spawn, target, and level

The depth seed isolates ambient floor RNG between floors, but it does not erase
player state consulted before `Imp.Quest.spawn` on the current floor.

**Mossy Clump is a concrete pre-Imp divergence.** On a normal-feeling roll,
`Level.create()` evaluates the Mossy chance first and evaluates the Trap
Mechanism chance only if Mossy fails (`Level.java:278-290`). A successful Mossy
roll therefore removes one ambient `Random.Float()` relative to the no-Mossy
route. The chosen Grass/Water feeling comes from the Clump's persistent seeded
six-card state (`MossyClump.java:54-92`), but that pushed generator does not put
the missing ambient call back.

Because `builder()` and all of `super.initRooms()` occur after that short
circuit and before the Imp callback, the offset can change:

- whether the quest spawns on depth 17 or 18, and hence its eventual spawn
  depth;
- the depth-18 Monk/Golem roll;
- which ring attempts are rejected as cursed;
- the accepted ring's initial level and both upgrade rolls.

Consequently the unprofiled public level is **+2…+4**, not one exact `+N`, and
the public target is the depth-to-target rule rather than the baseline target.
The held Clump, its upgrade level, when it was first held, and its saved feeling
deck are player-state inputs (`MossyClump.java:66-119`).

This is also covered by a fixed Rust regression: numeric seed 0 with no map
trinket and with Mossy Clump +3 first held on depth 17 spawns the same depth-19
Golem quest, but pre-rolls +3 and +4 rewards respectively. The comparison uses
the full floor analyzer and reads the quest's retained internal reward level
(`crates/spd-core/src/quests/imp/tests.rs`). Companion regressions show depth 17
versus 18 for seed 5, and opposite depth-18 targets for seed 26 under the same
two profiles.

Trap Mechanism alone does not offset the current pre-Imp stream: both the
tooth-free baseline and a Trap Mechanism route evaluate the Mossy float and the
Trap float, and its feeling deck runs under a pushed generator
(`Level.java:283-290`, `TrapMechanism.java:78-104`). TRAPS/CHASM do not alter
`RegularLevel.builder()` or the pre-Imp room-count conditions
(`RegularLevel.java:104-165`). Its painter and trap-reveal effects occur after
the reward, although prior Trap-affected floors can still move the persistent
ring deck as described next.

The limited Strength Potion, Upgrade Scroll, Stylus, and Enchantment Stone
checks also occur before the callback (`Level.java:222-249`,
`Dungeon.java:529-576`). Their counters are generation counters in the pinned
main-path lifecycle, not inventory/pickup choices. `NO_SCROLLS` skips placement
without changing the current pre-Imp call count (`Level.java:228-236`), so its
Imp effect is through earlier floor history rather than a new current-floor
reward call.

### Every ring class remains possible without a fixed history

The fixed-path class is a pure function of `(seed, RING draw index)`, but the
draw index is persistent across floors. It is not a pure function of the seed
across every playable route.

1. **Mimic Tooth.** Level-generated mimics immediately create an extra prize;
   their ring branch uses `Generator.random(RING)` and advances the seeded deck
   (`Mimic.java:294-353`). Tooth changes mimic chances in ordinary chests,
   golden chests, Suspicious Chest, Treasury, and Crystal Vault, and adds a
   further defaults-based item (`RegularLevel.java:397-439`,
   `SuspiciousChestRoom.java:55-70`, `TreasuryRoom.java:46-62`,
   `CrystalVaultRoom.java:74-82`, `Mimic.java:355-358`). Which offered trinket
   the player chooses, whether it is transmuted, its level, and when it is held
   are direct choices. The four Catalyst offers are drawn only when its window
   is opened (`TrinketCatalyst.java:161-189`), and trinket transmutation makes
   later TRINKET draws (`ScrollOfTransmutation.java:322-333`); checking only the
   first four offers cannot rule Tooth out.

2. **Rat Skull.** Crystal Vault chooses its second prize before rolling an
   alternate Crystal Mimic. Rat Skull raises that mimic chance, and the spawned
   mimic generates a deck-using prize whose 1/5 ring branch advances RING
   (`CrystalVaultRoom.java:60-82`, `Mimic.java:322-353`). This is a direct ring
   route, not merely a main-stream effect. Rat Skull also changes Statue
   variants (`Statue.java:198-212`), which can shift later floor RNG and other
   decks.

3. **Map-trinket and challenge history.** Mossy Clump and Trap Mechanism change
   earlier painters before ordinary items and mimic generation. `NO_SCROLLS`
   can change whether an earlier room consumes a queued item or generates a
   fallback, as established in `specs/quest-rewards/sad-ghost.md`. Those changes
   can alter earlier general-category outcomes, mimic branches, and therefore
   the RING draw count even though each later floor receives a fresh ambient
   seed.

4. **The ARTIFACT deck is runtime-movable.** Runtime artifact sources include
   Ring of Wealth equipment drops (`RingOfWealth.java:270-292`), mob loot via
   `randomUsingDefaults` (`Mob.java:989-1008`), artifact transmutation
   (`ScrollOfTransmutation.java:295-319`), and Cursed Wand transmogrification
   (`CursedWand.java:1141-1181`). Artifact class history can also change whether
   an `UnstableSpellbook` constructor burns its variable setup tail on a prior
   floor (`UnstableSpellbook.java:84-103`), moving subsequent level-generation
   draws on that floor.

   After the eleven available artifacts are exhausted, every
   `Generator.random(ARTIFACT)` falls through to `Generator.random(RING)`
   (`Generator.java:698-709`, `:854-878`). Ring of Wealth procs are driven from
   ordinary mob deaths (`Mob.java:963-972`), equipment drops repeat over time
   (`RingOfWealth.java:111-164`), and regular levels respawn mobs
   (`Level.java:709-763`). There is therefore no sound two-extra-draw or other
   small finite bound before depth 17.

The ring deck has twelve classes with equal positive weights and resets when
its probabilities are empty (`Generator.java:544-558`, `:711-727`). With
repeatable post-exhaustion artifact requests, the pre-Imp index can traverse a
complete reset deck. **All twelve ring classes must remain candidates** for an
unprofiled seed. The sound public projection is category-only “cursed +2…+4
ring reward,” not the baseline class or a three-class list.

Runtime ring generation that directly calls `randomUsingDefaults(RING)` does
not itself advance the seeded ring deck (`Generator.java:743-760`). It still
matters when it produces Ring of Wealth or otherwise enables one of the
stateful routes above. Side-level generation uses defaults and does not add a
separate direct RING-deck route; the persistent state acquired or used there can
still matter later.

### Depth-20 Imp shop contract

The city boss level always constructs an `ImpShopRoom` and calls its `paint()`
during `build()` (`CityBossLevel.java:190-201`). That paint call only generates
and stores shop stock; it does not place the items or shopkeeper
(`ImpShopRoom.java:63-69`). Thus:

- completing the quest before entering depth 20 does not make stock generation
  happen earlier;
- completing it after depth 20 was generated does not reroll the stored stock;
- the stock's inventory-sensitive bag and Hourglass sand decisions use the
  player state present when depth 20 was first generated
  (`ShopRoom.java:268-275`, `:310-329`);
- the shop is placed when Dwarf King is defeated if the quest is already
  complete (`CityBossLevel.java:352-385`), or on a later load after completion
  (`CityBossLevel.java:123-129`, `ImpShopRoom.java:154-160`);
- if the quest is never completed, neither the shopkeeper nor for-sale heaps
  appear, despite the internally stored stock.

Quest completion is therefore an access condition, not a seed guarantee about
spawned shop items. The Imp reward was generated on depth 17–19, so depth-20
stock generation cannot feed back into that reward.

### Analyzer status after this phase

The Rust quest port matches the pinned fixed-path spawn condition, depth target,
curse-retry loop, two upgrade calls, forced final curse, and Java-oracle ring
deck indices. It records the target/token contract and retains all twelve ring
classes in quest-local metadata (`crates/spd-core/src/quests/imp.rs`). Fixed
Mossy regressions cover reward-level, spawn-depth, and depth-18 target
divergence.

The public projection now keeps only the route-independent contract: a
conditional cursed +2…+4 ring, the depth-to-target rule, and the 5-Monk /
4-Golem token requirements. It exposes neither the sampled class and level nor
a partial candidate list, so exact ring searches cannot treat a baseline route
as evidence. The depth-20 shop remains explicitly conditional on quest
completion. `specs/generator-decks.md` records Rat Skull and the unbounded
artifact-exhaustion route as settled deck facts.

Overall analysis remains partial for the broader reasons in
`specs/accuracy.json`; this Imp phase does not claim complete run-history or
floor-reset coverage.
