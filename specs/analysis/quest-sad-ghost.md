# Sad Ghost reward — verified audit

Target: **Shattered Pixel Dungeon v4.0.0 @ `2bb34a4e9`**. This note records
the generation contract and the inputs that are not encoded by the dungeon
seed. It is separate from the public accuracy status.

### Verdict

The seed does **not** strictly determine the final reward the player receives.
It determines a pre-rolled pair under a fixed level-generation/player-state
profile, while the run can change both the pair and which member is claimed.

The strongest seed-only contract is:

- the quest can spawn only on depth 2, 3, or 4; depth 4 is the final guaranteed
  attempt (`Ghost.java:303-318`);
- the quest type is tied to the spawn depth: Fetid Rat / Gnoll Trickster /
  Great Crab for depths 2 / 3 / 4 (`Ghost.java:316-318`);
- one weapon and one armor are generated in `Ghost.Quest.spawn`, and the player
  later chooses exactly one of them (`WndSadGhost.java:84-130`);
- armor tier and class are the same fact: tier 2/3/4/5 means Leather/Mail/
  Scale/Plate (`Ghost.java:321-328`);
- the weapon is drawn from the corresponding seeded WEP_T2…WEP_T5 sub-deck
  (`Ghost.java:329-331`, `Generator.java:698-741,789-794`);
- both rewards receive the same level, +0 through +3, with 50%/30%/15%/5%
  weights (`Ghost.java:338-351`);
- both are explicitly made uncursed before the reward is offered
  (`Ghost.java:333-337`);
- one weapon enchantment and one armor glyph are generated before the keep/drop
  decision. They are either both retained or both discarded
  (`Ghost.java:353-362`), never independently mixed.

The tier roll alone rules out most melee classes (`Generator.java:429-474`):

| Tier | Reachable Ghost weapon classes |
|---|---|
| 2 | Shortsword, Hand Axe, Spear, Quarterstaff, Dirk, Sickle |
| 3 | Sword, Mace, Scimitar, Round Shield, Sai, Whip |
| 4 | Longsword, Battle Axe, Flail, Runic Blade, Assassin's Blade, Crossbow, Katana |
| 5 | Greatsword, War Hammer, Glaive, Greataxe, Greatshield, Gauntlet, War Scythe |

Pickaxe is listed in WEP_T2 but has zero deck weight, so it is impossible as a
Ghost reward (`Generator.java:429-439`). `Generator.fullReset` copies
`WEP_T3.defaultProbs` onto live `probs`, so Mace has the same weight as the
other T3 weapons on a fresh run (`Generator.java:441-450,625-635`).

The selected item is applied to the hero (or dropped if the backpack is full),
then `Ghost.Quest.complete()` clears both stored options
(`WndSadGhost.java:107-130`, `Ghost.java:394-398`). The unselected option is
not a second obtainable reward.

### Spawn and reward call order

`Level.create` pushes the per-depth generator, evaluates forced drops and
feeling, then `build()` (`Level.java:218-325`). `SewerLevel.createMobs` calls
`Ghost.Quest.spawn(this, roomExit)` after painting and before ambient mobs
(`SewerLevel.java:140-142`, `RegularLevel.java:220-228`). Rewards are therefore
rolled at the NPC callback, not during `initRooms`.

`spawn` retries `room.random()` until the cell is non-solid open space and not
the exit (`Ghost.java:307-309`). Armor is constructed directly from the tier
roll; only the weapon is `Generator.random(wepTiers[wepTier-1])`. The shared
level is applied with `upgrade(itemLevel)`; neither `Weapon.upgrade(false)` nor
`Armor.upgrade(false)` spends RNG on this path because the Ghost clears
enchantment/glyph first (`Weapon.java:373-408`, `Armor.java:454-468`).

### Parchment Scrap is a direct, non-RNG choice

`ParchmentScrap.enchantChanceMultiplier()` is 1 without the trinket, then 2,
4, 7, and 10 at +0…+3 (`ParchmentScrap.java:48-65`). The Ghost keeps both
pre-generated effects when:

```text
enchantRoll <= 0.2 × multiplier
```

| Effective Parchment Scrap | Keep probability | Seed-rolled requirement |
|---|---:|---|
| none | 20% | always, or no scrap |
| +0 | 40% | no scrap / +0 |
| +1 | 80% | no scrap / +0 / +1 |
| +2 | 100% | every roll |
| +3 | 100% | never required (same result as +2) |

Enchantment identity, glyph identity, and the minimum effective scrap level are
deterministic for a fixed generation profile. Whether the player has that scrap
level when the reward is claimed is not a seed fact. Confirmation applies the
already stored effect (`WndSadGhost.java:113-117`); opening or canceling the
window does not roll. The test always consumes one main-stream float.

### Inputs that can change the pre-rolled pair

Any player/meta state that changes the main floor stream or levelgen deck
history before `Ghost.Quest.spawn` can change the pair.

1. **Map-affecting trinkets.** Mossy Clump can replace a normal feeling with
   grass or water and, on success, skips the Trap Mechanism float
   (`Level.java:259-297`, `MossyClump.java:54-91`). That moves room selection
   and painter work before the callback. Trap Mechanism consumes the same two
   pre-build floats as the no-trinket path; Traps/Chasm do not change room
   counts, which only special-case Large/Secrets (`TrapMechanism.java:54-104`,
   `RegularLevel.java:124-165`). Its trap-reveal work is isolated inside
   `RegularPainter` (`RegularPainter.java:135-153`); surrounding painter
   differences can still move the reward stream.

2. **Mimic Tooth.** Its multiplier changes mimic decisions in Suspicious Chest,
   Treasury, Crystal Vault, and ordinary item population
   (`SuspiciousChestRoom.java:65-70`, `TreasuryRoom.java:46-62`,
   `CrystalVaultRoom.java:75-82`, `RegularLevel.java:397-432`). A spawned mimic
   immediately generates an extra reward; with `useDecks=true` its weapon/ring
   branch advances the seeded WEP/RING deck (`Mimic.java:306-358`). A held Tooth
   also adds a default extra reward. Multipliers are 1.5×/2×/2.5×/3× at +0…+3
   (`MimicTooth.java:52-61`). Ordinary `createItems` mimics run **after** Ghost
   on the spawn floor; paint-time mimics on that floor and any earlier-floor
   deck draws run **before** a later Ghost callback.

3. **Rat Skull.** It changes Crystal Vault’s alternate-mimic chance
   (`CrystalVaultRoom.java:75-82`) and the Statue-room alternate branch
   (`Statue.java:206-216`). An Armored Statue performs an extra armor generation
   and glyph roll (`ArmoredStatue.java:50-56`).

4. **Artifact history.** The ARTIFACT deck is runtime-movable. An artifact
   obtained outside levelgen can change a later class; `UnstableSpellbook`
   consumes a variable constructor tail (`UnstableSpellbook.java:84-103`).
   Cracked Spyglass extra loot is isolated and uses
   `Generator.randomUsingDefaults()` after `createMobs`
   (`RegularLevel.java:679-689`, `CrackedSpyglass.java:52-61`), so it is not a
   same-floor WEP lever. When that default category roll lands on ARTIFACT, it
   still advances the artifact deck (`Generator.java:746-752`) and can move a
   later Ghost floor’s constructor tail.

5. **Challenges and queued limited drops.** `NO_SCROLLS` increments the limited-
   drop counter but omits every second queued Scroll of Upgrade
   (`Level.java:232-240`). `findPrizeItem` returns null when the queue is empty
   (`Level.java:827-854`). Sewer floors 2–4 can contain `RitualRoom`
   (`StandardRoom.java:154,169-173`); on its 50% queued-prize branch, an empty
   queue runs `Random.oneOf(POTION, SCROLL)` before `Generator.random(...)`
   (`RitualRoom.java:105-110`). That is an extra main-floor call before
   `Ghost.Quest.spawn`. The source documents that this design “doesn’t quite
   remove” `NO_SCROLLS` levelgen impact (`Level.java:234-237`).

6. **The run route.** Declining the conversation, failing to place the miniboss
   (`Ghost.java:185-209`), not killing it, or never confirming a reward changes
   claim/presence, not the already generated pair. Miniboss combat loot is not
   part of this contract. Under Ascension the NPC dies on its turn
   (`Ghost.java:90-96`).

### What a seed can rule out

With a fixed generation profile (challenge mask, held trinket state, prior
artifact history, and the same levelgen room path), the seed can compute and
rule out:

- depth 1 and every depth after 4;
- every quest type not matching the actual spawn depth;
- cursed Ghost weapons/armor;
- armor classes from tiers other than the rolled armor tier;
- unequal weapon/armor upgrade levels;
- a weapon class not present at the reachable WEP-tier deck index;
- a one-sided enchantment/glyph result;
- a requirement for Parchment Scrap +3 (effective +2 already suffices);
- a trinket-dependent branch whose prerequisite trinket was not available
  before the target floor.

Without that profile, a single baseline WEP deck index is not a universal
candidate set: Mimic Tooth, Rat Skull, artifact history, and challenge-dependent
queue paths can add or remove pre-Ghost draws. The analyzer’s
`category_class_history()` projection is exact only for the fixed baseline path
it simulated; each labelled Mimic Tooth condition instead replays that Tooth
path (`crates/spd-core/src/quests/ghost.rs`).
