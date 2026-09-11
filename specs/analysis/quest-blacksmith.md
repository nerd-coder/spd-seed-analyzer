# Troll Blacksmith reward — verified audit

Target: **Shattered Pixel Dungeon v4.0.0 @ `2bb34a4e9`**. This note separates
the pre-generated Smith pool from the player's quest route and service choices.

### Verdict

A fresh run under a fixed level-generation profile determines a four-item
Smith pool, but the dungeon seed alone does **not** determine a final reward.
The player can leave the mining quest early, spend or cash out favor, or buy
Smith and choose exactly one of four mutually exclusive options.

The strongest fixed-profile generation contract is:

- the quest is attempted on depths 12–14, after ordinary room selection but
  before room shuffle; depth 14 is the final guaranteed attempt
  (`CavesLevel.java:90-91`, `RegularLevel.java:105-120`,
  `Blacksmith.java:347-360`);
- a successful attempt constructs `BlacksmithRoom` before choosing the quest
  type. Because it extends `StandardRoom`, its initializer spends the normal
  size-category roll even though the room later enforces minimum dimensions 8
  (`Blacksmith.java:347-354`, `StandardRoom.java:34-54,73-89`,
  `BlacksmithRoom.java:44-51`);
- Crystal and Gnoll are the only reachable types, selected uniformly with
  `Random.IntRange(1, 2)`. Fungi exists as type 3 in code and in mining-branch
  switches, but is explicitly incomplete and excluded from the roll
  (`Blacksmith.java:202-205,353-354`, `MiningLevel.java:72-81,144-169`);
- `generateRewards(true)` runs immediately in that `initRooms` spawn, before
  painting (`Blacksmith.java:356-357`);
- the pool contains two distinct melee-weapon classes, one missile-weapon
  stack, and one armor (`Blacksmith.java:363-377`);
- every equipment tier roll uses floor set 3 regardless of the spawn depth:
  tier 3 / 4 / 5 has 20% / 40% / 40% weight
  (`Generator.java:613-619,780-785,809-818,842-851`);
- all four options receive one shared level: +0 / +1 / +2 / +3 with
  30% / 45% / 20% / 5% weight, and all are explicitly uncursed with their
  initial enchantment or glyph removed (`Blacksmith.java:379-400`);
- one weapon enchantment and one armor glyph are always generated, then one
  shared Parchment Scrap test either retains both or discards both
  (`Blacksmith.java:402-411`);
- a missile weapon is a `Weapon`, so the stored weapon enchantment applies to
  the missile option as well as both melee options
  (`MissileWeapon.java:66`, `WndBlacksmith.java:517-521`).

Possible identities come from the tier tables (`Generator.java:441-473,513-535`):

| Tier | Melee options | Missile options | Armor option |
|---|---|---|---|
| 3 | Sword, Mace, Scimitar, Round Shield, Sai, Whip | Throwing Spear, Kunai, Bolas | Mail Armor |
| 4 | Longsword, Battle Axe, Flail, Runic Blade, Assassin's Blade, Crossbow, Katana | Javelin, Tomahawk, Heavy Boomerang | Scale Armor |
| 5 | Greatsword, War Hammer, Glaive, Greataxe, Greatshield, Gauntlet, War Scythe | Trident, Throwing Hammer, Force Cube | Plate Armor |

### `useDecks` is a misleading parameter name

`generateRewards(true)` passes `true` unchanged to the parameters named
`useDefaults` in `randomWeapon` and `randomMissile`. The spawn-time Smith pool
therefore selects classes from the fixed default weights and does not consume
or depend on the persistent WEP/MIS sub-decks
(`Blacksmith.java:356,363-377`, `Generator.java:809-818,842-851,743-768`).

The duplicate-melee loop redraws the complete second weapon until its concrete
class differs. It records rejected items for `undoDrop`, but default draws have
not decremented a deck. In addition, `undoDrop` reaches the inverted
`cls.isAssignableFrom(cat.superClass)` test, which does not restore a weight
(`Blacksmith.java:367-375`, `Generator.java:658-673`). Duplicate rejection still
consumes the discarded weapon's tier, class, random level, and isolated
effect-seed calls, so it is part of the floor RNG sequence.

`WndSmith` has a defensive `generateRewards(false)` fallback when the stored
pool is null or empty (`WndBlacksmith.java:477-479`). That fallback is
deck-backed. It is not reached in an ordinary fresh quest: the pool was already
created at spawn, a confirmed choice clears it, maximum favor is 3000, and one
Smith purchase costs 2000, leaving too little favor for a second purchase
(`Blacksmith.java:443-476`, `WndBlacksmith.java:143-165,512-535`).

### Parchment Scrap is read when the floor is generated

The effect decision is made inside spawn-time `generateRewards`, not when the
player later confirms an option. Acquiring or upgrading Parchment Scrap after
the Blacksmith floor was generated cannot change the stored pool. Confirmation
only applies the already stored enchantment or glyph
(`Blacksmith.java:356,402-411`, `WndBlacksmith.java:517-523`).

`ParchmentScrap.enchantChanceMultiplier()` is 1 without the trinket, then 2,
4, 7, and 10 at +0…+3 (`ParchmentScrap.java:48-65`). The pool retains both
effects when `enchantRoll <= 0.3 × multiplier`:

| Effective Parchment Scrap when the floor is generated | Keep probability | Minimum state for a fixed roll |
|---|---:|---|
| none | 30% | none |
| +0 | 60% | none or +0 |
| +1 | 100% | none, +0, or +1 |
| +2 / +3 | 100% | never required; +1 already suffices |

The enchantment and glyph identities and the minimum effective scrap level are
deterministic for a fixed generation profile. The test always consumes one
main-stream float.

### Favor, mining branch, and the choose-one lifecycle

Accepting the quest gives or drops the unique Pickaxe. The branch cannot be
entered until the quest was accepted, the Pickaxe is present, and the player
confirms entry (`Blacksmith.java:87-136`, `CavesLevel.java:117-160`). Crystal
and Gnoll choose different mining terrain and enemies, but both use the same
favor formula (`MiningLevel.java:72-83,144-169`). Fungi tiles, water/grass
weights, fungal spinners, and exit-warning text remain in those switches and
are unreachable from the type roll.

The mining painter targets 45–47 total Dark Gold, subtracting gold already
created by rooms and placing the remainder as mineable wall deposits
(`MiningLevel.java:144-149`, `MiningLevelPainter.java:53-116`). A gold-wall
candidate requires a non-wall neighbor that also lies inside the same room
(`MiningLevelPainter.java:80-87`). The Gnoll route can put 4–5 gold in each
secret-room chest, and the boss can expose wall gold as dropped items
(`MineSecretRoom.java:47-52`, `GnollGeomancer.java:404-407`). These
placement/lifecycle details change how the player obtains the gold, not the
generated total. `MineEntrance` paints `ENTRANCE_SP` plus surrounding
`EMPTY_SP` (`MineEntrance.java:85-88`).

The player may leave the branch without defeating its boss and may confirm the
warning at any collected-gold amount (`MiningLevel.java:238-297`). Completion
then sets `favor = min(2000, collectedDarkGold × 50) + (bossBeaten ? 1000 : 0)`
and consumes the held Dark Gold and Pickaxe (`Blacksmith.java:443-469`). Smith
costs 2000 favor, so it is available after collecting at least 40 Dark Gold, or
after defeating the quest boss and collecting at least 20.

Paying for Smith subtracts 2000 and increments `smiths` before opening a window
that cannot be dismissed with Back (`WndBlacksmith.java:143-165,454-505`). On
confirmation, exactly one item is identified and picked up or dropped, the
window closes, and the entire stored pool is cleared
(`WndBlacksmith.java:507-553`). None of the four concrete options is
individually guaranteed to spawn.

`BlacksmithRoom.paint` places the NPC at a fixed cell, two pedestal equipment
drops from `Generator.random(oneOf(ARMOR, WEAPON, MISSILE))`, and a branch exit
whose left/right side starts as `Random.Int(2)` unless a top door forces it
(`BlacksmithRoom.java:54-110`). Those ground drops run during paint, after the
Smith pool already exists. `CavesPainter.decorate` skips `BlacksmithRoom` when
filling standard-room corners (`CavesPainter.java:51-54`); that decorate pass
is isolated (`RegularPainter.java:135-153`) and does not move the stored pool.

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
  `upgrade()` (`WndBlacksmith.java:133-141,409-451`).
- The returned Pickaxe costs 250 unless completion initially earned at least
  2500 favor, in which case it remains free; remaining favor can instead be
  cashed out one-for-one as ordinary Gold (`Blacksmith.java:467-476`,
  `WndBlacksmith.java:79-111,168-190`).

At maximum favor the player can afford at most two Reforge services, two Harden
services, two Upgrade services, or one Smith, with mixed combinations depending
on order.

### Stateful routes before reward creation

Every ordinary main-path floor pushes a generator derived from that depth
(`Level.java:218-221`). Spawn-time default equipment selection also closes
WEP/MIS deck-history leaks. In a fresh run, the remaining pre-reward state is:

1. **Mossy Clump can change the active floor stream.** `Level.create` evaluates
   feeling overrides before `build`; a successful Mossy check short-circuits
   the following Trap Mechanism float (`Level.java:259-297`,
   `MossyClump.java:54-64`). `builder()` and ordinary room selection then
   precede `Blacksmith.Quest.spawn` (`RegularLevel.java:105-165`,
   `CavesLevel.java:90-91`). The missing second float moves the spawn check,
   quest type, reward classes, shared level, and effects.

2. **Trap Mechanism alone does not move the fresh spawn-time pool.** Its
   override occupies the second float slot already consumed by the no-trinket
   path, and Traps/Chasm do not change the room counts that only special-case
   Large/Secrets (`Level.java:259-297`, `RegularLevel.java:129-163`,
   `TrapMechanism.java:54-64,83-104`).

3. **Parchment Scrap changes only effect retention.** Classes, levels, and the
   main RNG tail are unchanged.

4. **Mimic Tooth, Rat Skull, Cracked Spyglass, artifact history, and ordinary
   challenge hooks do not precede the fresh reward callback.** Their
   mimic/statue/room-paint and item-population effects occur after `initRooms`,
   while the pool uses default WEP/MIS weights. Spyglass extra loot is isolated
   and default-weighted except ARTIFACT, and it runs in `createItems`
   (`RegularLevel.java:679-689`). Forbidden Runes' every-second Upgrade Scroll
   omission has no RNG call and does not change room selection before spawn
   (`Level.java:232-240`). These states can change later Blacksmith-room ground
   equipment, not the already stored Smith pool.

5. **Floor reset is a real player-route divergence.** Resetting a level clears
   a pending next-floor PitRoom and generates another level without rewinding
   persistent limited-drop counters, special-room rotation, or secret-room
   allocation (`InterlevelScene.java:805-813`, `SpecialRoom.java:131-187`). The
   analyzer's ordinary replay is the fresh, once-generated-floor route.

The Rust port matches the fixed-profile normal `generateRewards(true)` call
shape (`crates/spd-core/src/quests/blacksmith.rs`). The public projection must
present all four items as mutually exclusive Smith choices, not guaranteed
spawns, and must say that Parchment Scrap is evaluated when the spawn floor is
first generated.
