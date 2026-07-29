# Sad Ghost reward — verified audit

Target: **Shattered Pixel Dungeon v3.3.8 @ `7b8b845a7`**. This note records
the generation contract and the inputs that are not encoded by the dungeon
seed. It is intentionally separate from the public accuracy manifest; the
manifest states what the UI may promise.

### Verdict

The seed does **not** strictly determine the final reward the player receives.
It determines a pre-rolled pair under a fixed level-generation/player-state
profile, while the run can change both the pair and which member of the pair is
claimed.

The strongest seed-only contract is:

- the quest can spawn only on depth 2, 3, or 4; depth 4 is the final guaranteed
  attempt (`Ghost.java:303-318`);
- the quest type is tied to the spawn depth: Fetid Rat / Gnoll Trickster /
  Great Crab for depths 2 / 3 / 4 (`Ghost.java:316-318`);
- one weapon and one armor are generated, and the player later chooses exactly
  one of them (`WndSadGhost.java:81-130`);
- armor tier and class are the same fact: tier 2/3/4/5 means Leather/Mail/
  Scale/Plate (`Ghost.java:322-328`);
- the weapon is drawn from the corresponding seeded WEP_T2…WEP_T5 sub-deck
  (`Ghost.java:329-331`, `Generator.java:709-731`);
- both rewards receive the same level, +0 through +3, with 50%/30%/15%/5%
  weights (`Ghost.java:339-351`);
- both are explicitly made uncursed before the reward is offered
  (`Ghost.java:333-337`);
- one weapon enchantment and one armor glyph are generated before the keep/drop
  decision.  They are either both retained or both discarded
  (`Ghost.java:353-362`), never independently mixed.

The tier roll alone rules out most melee classes (`Generator.java:429-474`):

| Tier | Reachable Ghost weapon classes |
|---|---|
| 2 | Shortsword, Hand Axe, Spear, Quarterstaff, Dirk, Sickle |
| 3 | Sword, Mace, Scimitar, Round Shield, Sai, Whip |
| 4 | Longsword, Battle Axe, Flail, Runic Blade, Assassin's Blade, Crossbow, Katana |
| 5 | Greatsword, War Hammer, Glaive, Greataxe, Greatshield, Gauntlet, War Scythe |

Pickaxe is listed in WEP_T2 but has zero deck weight, so it is impossible as a
Ghost reward in this pinned version (`Generator.java:429-439`).

The selected item is applied to the hero (or dropped if the backpack is full),
then `Ghost.Quest.complete()` clears both stored options
(`WndSadGhost.java:106-130`, `Ghost.java:400-414`).  The unselected option is
therefore not a second obtainable reward.

### Parchment Scrap is a direct, non-RNG choice

`ParchmentScrap.enchantChanceMultiplier()` is 1 without the trinket, then 2,
4, 7, and 10 at +0…+3 (`ParchmentScrap.java:48-65`).  The Ghost keeps both
the pre-generated enchantment and glyph when:

```text
enchantRoll <= 0.2 × multiplier
```

Consequently:

| Effective Parchment Scrap | Keep probability | Seed-rolled requirement |
|---|---:|---|
| none | 20% | always, or no scrap |
| +0 | 40% | no scrap / +0 |
| +1 | 80% | no scrap / +0 / +1 |
| +2 | 100% | every roll |
| +3 | 100% | never required (same result as +2) |

For a particular seed/profile, the enchantment identity, glyph identity, and
the minimum effective scrap level are deterministic.  Whether the player has
that scrap level when the reward is claimed is not a seed fact.  The retained
effect is applied only when the player confirms the chosen item
(`WndSadGhost.java:106-119`); opening/canceling the reward window does not roll
anything.

### Inputs that can change the pre-rolled pair

The Java level lifecycle calls `SewerLevel.createMobs()` after room painting
and before the ordinary mob/item population (`Level.java:300-303`,
`SewerLevel.java:140-143`).  Therefore any player/meta state that changes the
main floor stream or the level-generation deck history before that callback can
change the Ghost pair.

1. **Map-affecting trinkets.**  Mossy Clump can replace the normal feeling with
   grass or water (`Level.java:278-287`, `MossyClump.java:54-91`).  That changes
   painter work before `Ghost.Quest.spawn`; in a throwaway Rust probe comparing
   `analyze_seed_with_profile` with no trinket versus a +3 Mossy Clump held from
   depth 2, the first five numeric seeds already produced different Ghost
   depth/tier/level or reward identities.  The probe was removed after the
   comparison.  Trap Mechanism changes the
   feeling/trap presentation through the same hook (`TrapMechanism.java:54-103`);
   its reward-stream impact is not yet closed by a dedicated quest oracle and
   must not be assumed away.

2. **Mimic Tooth.**  Its multiplier changes the levelgen mimic decisions in
   Suspicious Chest, Treasury, and Crystal Vault
   (`SuspiciousChestRoom.java:65-70`, `TreasuryRoom.java:46-62`,
   `CrystalVaultRoom.java:75-82`).  A spawned mimic immediately generates an
   extra reward (`Mimic.java:319-358`); with `useDecks=true`, its weapon/ring
   branch advances the seeded WEP/RING deck.  A held Tooth also adds another
   default reward to each mimic. Thus Tooth can change both the weapon class
   and the main-stream position at which the Ghost reward is generated. The
   analyzer now replays this path when the user selects **Mimic Tooth +0…+3**
   and the first held depth: the level-specific multiplier is
   1.5×/2×/2.5×/3×, and each spawned mimic's extra default reward is consumed.
   With that profile, the displayed Ghost pair is precomputed for that exact
   Tooth state. “None” remains the tooth-free continuation. This does not
   infer whether the player can obtain or upgrade the Tooth by that depth.

3. **Rat Skull.**  It changes the Crystal Vault alternate-mimic chance
   (`CrystalVaultRoom.java:75-82`) and the Statue-room alternate branch
   (`Statue.java:198-212`).  An Armored Statue performs an additional random
   armor generation and glyph roll (`ArmoredStatue.java:49-56`), so this can
   move the main stream before the Ghost callback even though the statue's
   weapon draw itself is still one WEP draw.

4. **Artifact history.**  The ARTIFACT deck is the known runtime-movable deck;
   an artifact obtained outside level generation can change a later artifact
   class and, for `UnstableSpellbook`, consume a variable constructor tail.
   That changes the later floor stream before a Ghost callback.  The general
   deck audit is in `specs/generator-decks.md`, §11; its conclusions are valid
   for Ghost only when the no-external-artifact condition is also fixed.

5. **Challenges and queued limited drops — confirmed divergence.**
   `NO_SCROLLS` increments the same limited-drop counter but omits every second
   queued Scroll of Upgrade (`Level.java:228-235`). This is not an inert
   placement-only difference: `Level.findPrizeItem()` returns and removes an
   arbitrary queued item when one remains, but returns `null` when the queue is
   empty (`Level.java:799-826`). Sewer floors 2–4 can contain `RitualRoom`
   (`StandardRoom.java:171-192`); on its 50% queued-prize branch, an empty queue
   runs `Random.oneOf(POTION, SCROLL)` before `Generator.random(...)`
   (`RitualRoom.java:105-110`). That is an additional main-floor RNG call before
   `SewerLevel.createMobs()` invokes `Ghost.Quest.spawn`
   (`Level.java:300-303`, `SewerLevel.java:140-143`).

   Therefore the challenge mask is a real pre-Ghost profile input. It can
   change the reward rolls whenever removing the last eligible queued item
   makes such a fallback reachable. The pinned source explicitly documents
   that this design “doesn't quite remove” `NO_SCROLLS` level-generation impact
   (`Level.java:229-233`). We cannot save a no-effect claim for queued Upgrade
   Scrolls. Other challenge checks do not directly alter the Ghost method.

6. **The run route.**  The player may decline the initial Ghost conversation,
   fail to create the quest miniboss because no respawn cell is available, not
   kill the miniboss, or never confirm a reward.  These change whether a
   reward is claimed, not the already generated pair.  The miniboss's ordinary
   loot (Gnoll missile, Great Crab meat, etc.) is combat/runtime loot and is not
   part of the Ghost quest reward contract.

Parchment Scrap changes retention only; it deliberately does not alter the
number of Ghost RNG calls.  The stateful trinkets and artifact/challenge paths
above can change the stream before the callback and therefore can change the
tiers, shared level, enchant/glyph identities, spawn depth, and weapon deck
index as well.

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
- a requirement for Parchment Scrap +3 (effective +2 is already sufficient);
- any trinket-dependent branch whose prerequisite trinket was not available
  before the target floor (for example, no earlier Catalyst/Transmutation route
  and no matching Catalyst offer).

Without that profile, the seed can still provide a **set of possible routes**,
but a single baseline WEP deck index is not a sound universal candidate set:
Mimic Tooth, Rat Skull, artifact history, and challenge-dependent queue paths
can add or remove pre-Ghost draws. The current analyzer's
`category_class_history()` projection is therefore exact only for the fixed
baseline path it actually simulated; an explicit Mimic Tooth profile instead
replays its selected Tooth path and reports its single resulting class
(`crates/spd-core/src/quests/ghost.rs:145-220`,
`crates/spd-core/src/generator/state.rs:434-451`).

### Analyzer status after this phase

The Rust port correctly models the fixed-path Ghost reward call order,
uncursed pair, shared level, pre-generated effect identities, and conditional
Parchment threshold (`crates/spd-core/src/quests/ghost.rs:142-216`). Its public
projection exposes the tier and minimum scrap condition
(`crates/spd-core/src/level/state.rs:322-435`). It also enumerates the no-Tooth
and held Mimic Tooth +0…+3 paths when explicitly profiled. Other stateful
pre-Ghost branches above remain unenumerated, so the default report is still a
baseline continuation rather than proof that every concrete Ghost class, tier,
level, or spawn depth is seed-only.

Next quest phase: Old Wandmaker.  Reuse this same separation between (a) the
pre-rolled contract, (b) player choices, and (c) stateful draws before the NPC
callback.
