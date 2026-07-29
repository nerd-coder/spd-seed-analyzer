# Troll Blacksmith reward — verified audit

Target: **Shattered Pixel Dungeon v3.3.8 @ `7b8b845a7`**. This note separates
the pre-generated Smith pool from the player's quest route and service choices.
It does not make a full seed-finder accuracy claim.

### Verdict

A fresh run under a fixed level-generation profile determines a four-item
Smith pool, but the dungeon seed alone does **not** determine a final reward.
The player can leave the mining quest early, spend or cash out the favor, or
buy Smith and choose exactly one of four mutually exclusive options.

The strongest fixed-profile generation contract is:

- the quest is attempted on depths 12–14, after ordinary room selection but
  before room shuffle; depth 14 is the final guaranteed attempt
  (`CavesLevel.java:89-92`, `RegularLevel.java:105-120`,
  `Blacksmith.java:354-368`);
- a successful attempt constructs `BlacksmithRoom` before choosing the quest
  type. Because it extends `StandardRoom`, its initializer spends the normal
  size-category roll even though the room later enforces minimum dimensions
  (`Blacksmith.java:355-364`, `StandardRoom.java:34-54,73-89`,
  `BlacksmithRoom.java:41-51`);
- Crystal and Gnoll are the only reachable types, selected uniformly. Fungi is
  explicitly incomplete and excluded (`Blacksmith.java:202-205,360-364`);
- the pool contains two distinct melee-weapon classes, one missile-weapon
  stack, and one armor (`Blacksmith.java:370-384`);
- every equipment tier roll uses floor set 3 regardless of the spawn depth:
  tier 3 / 4 / 5 has 20% / 40% / 40% weight
  (`Generator.java:613-619,779-785,808-818,841-851`);
- all four options receive one shared level: +0 / +1 / +2 / +3 with
  30% / 45% / 20% / 5% weight, and all are explicitly uncursed with their
  initial enchantment or glyph removed (`Blacksmith.java:386-407`);
- one weapon enchantment and one armor glyph are always generated, then one
  shared Parchment Scrap test either retains both or discards both
  (`Blacksmith.java:409-418`);
- a missile weapon is a `Weapon`, so the stored weapon enchantment applies to
  the missile option as well as both melee options
  (`MissileWeapon.java:63`, `WndBlacksmith.java:517-521`).

The possible identities come directly from the pinned tier tables
(`Generator.java:441-473,513-535`):

| Tier | Melee options | Missile options | Armor option |
|---|---|---|---|
| 3 | Sword, Mace, Scimitar, Round Shield, Sai, Whip | Throwing Spear, Kunai, Bolas | Mail Armor |
| 4 | Longsword, Battle Axe, Flail, Runic Blade, Assassin's Blade, Crossbow, Katana | Javelin, Tomahawk, Heavy Boomerang | Scale Armor |
| 5 | Greatsword, War Hammer, Glaive, Greataxe, Greatshield, Gauntlet, War Scythe | Trident, Throwing Hammer, Force Cube | Plate Armor |

### `useDecks` is a misleading parameter name

`generateRewards(true)` passes `true` unchanged to the Java parameters named
`useDefaults` in `randomWeapon` and `randomMissile`. The spawn-time Smith pool
therefore selects classes from the fixed default weights and does not consume
or depend on the persistent WEP/MIS sub-decks
(`Blacksmith.java:364,370-384`, `Generator.java:808-818,841-851`,
`Generator.java:743-768`).

The duplicate-melee loop redraws the complete second weapon until its concrete
class differs. It records rejected items for `undoDrop`, but default draws have
not decremented a deck in the first place. In addition, the pinned concrete
class call reaches the inverted `cls.isAssignableFrom(cat.superClass)` test,
which does not restore a weight (`Blacksmith.java:374-382`,
`Generator.java:656-673`). Duplicate rejection still consumes the discarded
weapon's tier, class, random level, and isolated effect-seed calls, so it is an
important part of the floor RNG sequence.

`WndSmith` has a defensive `generateRewards(false)` fallback when the stored
pool is null or empty (`WndBlacksmith.java:477-479`). That fallback is
deck-backed. It is not reached in an ordinary fresh quest: the pool was already
created at spawn, a confirmed choice clears it, maximum favor is 3000, and one
Smith purchase costs 2000, leaving too little favor for a second purchase
(`Blacksmith.java:450-476`, `WndBlacksmith.java:143-165,512-535`). It matters
only to an abnormal/migrated state in which the paid selection window has no
stored pool and must not be conflated with the normal seed contract.

### Parchment Scrap is read when the floor is generated

The effect decision is made inside spawn-time `generateRewards`, not when the
player later confirms an option. Acquiring or upgrading Parchment Scrap after
the Blacksmith floor was generated cannot change the stored pool. Confirmation
only applies the already stored enchantment or glyph
(`Blacksmith.java:364,409-418`, `WndBlacksmith.java:517-523`).

`ParchmentScrap.enchantChanceMultiplier()` is 1 without the trinket, then 2,
4, 7, and 10 at +0…+3 (`ParchmentScrap.java:48-65`). The pool retains both
effects when:

```text
enchantRoll <= 0.3 × multiplier
```

Consequently:

| Effective Parchment Scrap when the floor is generated | Keep probability | Minimum state for a fixed roll |
|---|---:|---|
| none | 30% | none |
| +0 | 60% | none or +0 |
| +1 | 100% | none, +0, or +1 |
| +2 / +3 | 100% | never required; +1 already suffices |

The enchantment and glyph identities and the minimum effective scrap level are
deterministic for a fixed generation profile. Whether the player actually had
that state before entering the spawn floor is a player-route fact. The test
always consumes one main-stream float, so Parchment changes the stored effects
without desynchronizing later floor generation.

### Favor, side level, and the choose-one lifecycle

Accepting the quest gives or drops the unique Pickaxe. The branch cannot be
entered until the quest was accepted, the Pickaxe is present, and the player
confirms entry (`Blacksmith.java:87-136`, `CavesLevel.java:117-160`). Crystal
and Gnoll choose different mining terrain and enemies, but both use the same
favor formula (`MiningLevel.java:72-83,144-169`).

The mining painter targets 45–47 total Dark Gold, subtracting gold already
created by rooms and placing the remainder as mineable wall deposits
(`MiningLevel.java:144-149`, `MiningLevelPainter.java:46-65,67-115`). The Gnoll
route can put 4–5 gold in each secret-room chest, and the boss can expose wall
gold as dropped items (`MineSecretRoom.java:47-70`,
`GnollGeomancer.java:401-412`). These placement/lifecycle details change how
the player obtains the gold, not the generated total.

The player may leave the branch without defeating its boss and may confirm the
warning at any collected-gold amount (`MiningLevel.java:238-297`). Completion
then sets:

```text
favor = min(2000, collectedDarkGold × 50) + (bossBeaten ? 1000 : 0)
```

and consumes the held Dark Gold and Pickaxe (`Blacksmith.java:450-476`). Smith
costs 2000 favor, so it is available after either:

- collecting at least 40 Dark Gold; or
- defeating the quest boss and collecting at least 20 Dark Gold.

The level contains enough generated gold for the first route, but collecting
it, beating the boss, returning the Pickaxe, and choosing how to spend favor
are player actions, not seed-guaranteed item spawns.

Paying for Smith subtracts 2000 and increments `smiths` before opening a window
that cannot be dismissed with Back (`WndBlacksmith.java:143-165`,
`WndBlacksmith.java:454-505`). The player can inspect/cancel an individual
confirmation and choose another option. On confirmation, exactly one item is
identified and picked up or dropped at the hero, the window closes, and the
entire stored pool is cleared (`WndBlacksmith.java:507-553`). Closing the game
after payment does not create another choice: interacting again reopens the
same stored pool while `smiths > 0` (`Blacksmith.java:148-159`).

Therefore none of the four concrete options is individually guaranteed to
spawn, and the other three must not be reported as obtainable additional
rewards. They are mutually exclusive choices conditional on buying Smith.

### Other favor services are player-state transformations

These services do not generate a seed-guaranteed item:

- **Reforge** costs 500, then 1500, then 2500. It requires two different item
  instances of the same concrete class; both must be identified, uncursed, and
  upgradable. The higher `trueLevel` item survives (the first UI slot wins a
  tie), the other stack is destroyed, and the survivor gains one upgrade.
  Existing good weapon enchantments and armor glyphs are explicitly preserved
  (`WndBlacksmith.java:113-121,255-309,319-361`).
- **Harden** has the same 500/1500/2500 cost progression. It marks an eligible
  identified, uncursed weapon enchantment or armor glyph as hardened; the same
  item cannot be hardened twice (`WndBlacksmith.java:123-131,365-407`).
- **Upgrade** costs 1000, then 2000. It accepts an identified, uncursed,
  upgradable item only below +2 and invokes that concrete item's normal
  `upgrade()` behavior (`WndBlacksmith.java:133-141,409-451`). Any RNG or
  enchantment loss from that call depends on the selected inventory item and
  its current state, not just the dungeon seed.
- The returned Pickaxe costs 250 unless completion initially earned at least
  2500 favor, in which case it remains free; remaining favor can instead be
  cashed out one-for-one as ordinary Gold (`Blacksmith.java:474-482`,
  `WndBlacksmith.java:79-111,168-190`).

At maximum favor the player can afford at most two Reforge services, two
Harden services, two Upgrade services, or one Smith, with mixed combinations
depending on order. Spending, eligibility, selected inventory, and service
order are all direct player choices.

### Stateful routes before reward creation

Every ordinary main-path floor pushes a generator derived from that depth, so
earlier floor-stream consumption cannot directly leak into the target floor
(`Level.java:215-218,295-316`). Spawn-time default equipment selection also
closes WEP/MIS deck-history leaks. In a fresh run, the remaining relevant
pre-reward state is narrower:

1. **Mossy Clump can change the active floor stream.** `Level.create` evaluates
   feeling overrides before `build`; a successful Mossy check short-circuits
   the following Trap Mechanism float (`Level.java:255-292`). Mossy has a
   25%/50%/75%/100% override chance at +0…+3
   (`MossyClump.java:54-64`). `builder()` and all ordinary room selection then
   precede `Blacksmith.Quest.spawn` (`RegularLevel.java:105-165`,
   `CavesLevel.java:89-92`). The missing second float moves the spawn check,
   quest type, reward classes, shared level, and effects. A concrete pool is
   exact only for the selected held-trinket profile and first-held depth.

2. **Trap Mechanism alone does not move the fresh spawn-time pool.** Its
   override is evaluated in the second float slot already consumed by the
   no-trinket path, and its Traps/Chasm feelings do not change the room counts
   that only special-case Large/Secrets (`Level.java:255-292`,
   `RegularLevel.java:129-163`, `TrapMechanism.java:54-64,83-104`). Its trap
   reveal hook runs during painting, after the Smith pool exists. This is a
   fixed fresh-lifecycle conclusion, not a claim about floor regeneration.

3. **Parchment Scrap changes only effect retention.** Its direct generation-
   time condition is described above; the pool's classes, levels, and main RNG
   tail are unchanged.

4. **Mimic Tooth, Rat Skull, artifact history, and ordinary challenge hooks do
   not precede the fresh reward callback.** Their mimic/statue/room-paint and
   item-population effects occur after `initRooms`, while the pool uses default
   WEP/MIS weights. The only challenge check before build is Forbidden Runes'
   decision to enqueue every second Upgrade Scroll; it does not add an RNG call
   or change room selection before spawn (`Level.java:228-236`,
   `RegularLevel.java:105-165`). These states can change later Blacksmith-room
   ground equipment, but not the already stored Smith pool.

5. **Floor reset is a real player-route divergence.** Resetting a level clears
   a pending next-floor PitRoom and generates another level without rewinding
   persistent limited-drop counters, special-room rotation, or secret-room
   allocation (`InterlevelScene.java:805-813`, `SpecialRoom.java:131-189`,
   `SecretRoom.java:66-100`). Resetting a pre-Blacksmith floor can therefore
   change the run state reaching depths 12–14. Resetting an unsuccessful spawn
   depth can also create another spawn attempt at that same depth; resetting a
   floor after the static quest was marked spawned can remove access to its
   room because the quest does not spawn twice. The analyzer's ordinary replay
   is the fresh, once-generated-floor route and does not enumerate reset paths.

### Analyzer status after this phase

The Rust port matches the fixed-profile normal `generateRewards(true)` call
shape: default equipment weights, duplicate rejection, shared level, cleared
curse/effects, generated enchant/glyph identities, and minimum Parchment Scrap
condition. This audit corrected the missile option so it carries the same
potential weapon enchantment as both melee options
(`crates/spd-core/src/quests/blacksmith.rs`).

The public projection still needs to present all four items as mutually
exclusive Smith choices, not guaranteed spawns; apply the missile enchantment
to the projected item; include the missile in the Parchment condition; and say
that the Scrap must be held when the spawn floor is first generated. It should
also retain the explicit fresh/map-profile assumption and not claim reset-path
coverage.
