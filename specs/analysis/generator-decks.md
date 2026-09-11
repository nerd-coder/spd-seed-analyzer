# Generator decks — verified facts

Target: **Shattered Pixel Dungeon v4.0.0 @ `2bb34a4e9`**. Java line numbers
are from that commit. Method: reading the pinned clone. Tables below are
`defaultProbs` (and related static arrays) from source, not sampled
distributions.

`Generator` is a static class. `Dungeon.init()` calls `Generator.fullReset()`
from a generator seeded with `seed+1` (`Dungeon.java:243-252`). Deck seeds,
`dropped` counters, and category weights then persist for the whole run.

## 1. Category sub-decks make item identity index-keyed

`Generator.random(Category)` (`Generator.java:698-741`) draws the class from a
per-category seeded sub-RNG when `defaultProbs` and `cat.seed` are set:

```
Random.pushGenerator(cat.seed);
for (int i = 0; i < cat.dropped; i++) Random.Long();
int i = Random.chances(cat.probs);
// reset(cat) and chances again if i == -1
cat.probs[i]--; cat.dropped++; Random.popGenerator();
return newInstance(classes[i]).random();
```

`cat.seed` is a `Random.Long()` from `fullReset()` (`Generator.java:625-635`).
The class of the *k*-th draw from a category is a function of
`(dungeon seed, k)`, independent of where on a floor stream the draw lands.
`.random()` on the instance runs **after** `popGenerator`, so level/curse
rolls use the ambient floor stream.

`WEAPON` / `MISSILE` / `ARMOR` do not use this path: they dispatch to
`randomWeapon` / `randomMissile` / `randomArmor`. `ARTIFACT` dispatches to
`randomArtifact()` (see §5).

`randomUsingDefaults` (`Generator.java:744-770`) bypasses `probs` and
`dropped`. Two-deck categories (`POTION`, `SCROLL`) use `defaultProbsTotal`.
`WEAPON` / `MISSILE` still pick a tier from `floorSetTierProbs` then call
defaults on that sub-tier. **Exception:** `Category.ARTIFACT` always re-enters
`random(cat)`.

## 2. Per-depth stream isolation

`Level.create()` wraps generation in
`Random.pushGenerator(Dungeon.seedCurDepth())` (`Level.java:219-221`).
`seedCurDepth()` is `seedForDepth(depth, branch)`: from the run seed, skip
`depth + 30*branch` longs, then take one (`Dungeon.java:414-428`). A stream
difference on floor *N* cannot reach floor *N+1* or another branch. For a
fixed challenge/trinket/meta profile, the cross-floor state that can change a
deck-drawn **class** is the `dropped` counters (and remaining `probs`).

## 3. RING draw-site list (complete)

Direct RING-deck draws are `Generator.random(Category.RING)` (or no-arg
`Generator.random()` landing on `RING`). Artifact-exhaustion through
`random(ARTIFACT)` is **not** a RING-deck site (§5). Imp’s own artifact-null
branch **is**.

| Site | Java |
|------|------|
| `createItems` general-deck picks | `RegularLevel.java:388` |
| Grassy grave tomb | `GrassyGraveRoom.java:67` |
| Mass grave (60%) | `MassGraveRoom.java:108` |
| Secret summoning skeleton | `SecretSummoningRoom.java:53` |
| Ordinary / golden / crystal-vault mimic prize (`useDecks=true`) | `Mimic.java:349` |
| Shop rare, 1/10 (including Imp shop stock) | `ShopRoom.java:339`, `ImpShopRoom.java:66-67` |
| Pit room main loot | `PitRoom.java:72` |
| Crystal choice hidden prize | `CrystalChoiceRoom.java:122-126` |
| Crystal vault prize cycle | `CrystalVaultRoom.java:97-105` |
| Imp first slot if `randomArtifact()` returns null | `Imp.java:319-327` |
| Imp dedicated ring, plus class-collision retries | `Imp.java:330-336` |

No-arg `Generator.random()` is itself a 35-item **general** category deck
(`Generator.java:675-682`). `RING` has `firstProb=1`, `secondProb=0`
(`Generator.java:241`), so only the first general deck (and later recycles
onto it) can land on RING. Direct `random(RING)` sites do not decrement
`categoryProbs`.

## 4. Runtime (and side-level) RING requests bypass the RING deck

These use `randomUsingDefaults(RING)` or `randomUsingDefaults()`:

| Site | Java |
|------|------|
| Mob loot, including Thief `oneOf(RING, ARTIFACT)` | `Mob.java:1106`, `Thief.java:52` |
| Ring of Wealth equipment | `RingOfWealth.java:288` |
| Scroll of Transmutation (ring, and artifact→ring) | `ScrollOfTransmutation.java:181`, `:276` |
| Cursed Wand mimic reward / transmogrify | `CursedWand.java:1081-1082`, `:1174-1175` |
| Runtime mimics (`useDecks=false`) | `CursedWand.java:1066`, `DistortionTrap.java:120` |
| Vault equipment rings | `VaultLevel.java:371` |
| Artifact exhaustion inside `random(ARTIFACT)` | `Generator.java:710` |

`MiningLevel` only calls `randomUsingDefaults(FOOD)` (`MiningLevel.java:199`,
`:207`).

## 5. Artifact exhaustion fallback is RING via defaults

```
case ARTIFACT:
    Item item = randomArtifact();
    // if we're out of artifacts, return a ring instead.
    // do not use decks for that ring, as the # of artifacts genned can vary by gameplay
    return item != null ? item : randomUsingDefaults(Category.RING);
```

(`Generator.java:706-710`.) `randomUsingDefaults(ARTIFACT)` also takes this
path (`Generator.java:751-752`). The fallback RING uses `defaultProbs` on the
**ambient** RNG and does not increment `RING.dropped`.

`randomArtifact()` (`Generator.java:856-878`) uses the ARTIFACT seeded deck
and never resets; `chances` of an all-zero table returns `-1` with no `Float`
(`Random.java:184-185`). `dropped` still increments on that miss. After
`popGenerator` it calls `Artifact.random()` (`Artifact.java:218-225`): always
+0, 30% curse.

Call sites that can hit the fallback (any `random` / `randomUsingDefaults` of
`ARTIFACT`, including no-arg landing on it):

- Pit, crystal vault cycle, crystal-choice hidden, shop rare
  (`PitRoom.java:75`, `CrystalVaultRoom.java:102-105`,
  `CrystalChoiceRoom.java:122-126`, `ShopRoom.java:343`)
- Ring of Wealth (`RingOfWealth.java:291`)
- Thief loot when the stolen category is ARTIFACT (`Thief.java:52` →
  `Mob.java:1106`)
- Cursed Wand transmogrify (`CursedWand.java:1174-1175`)
- no-arg `Generator.random()` on the second general deck (`RegularLevel.java:388`
  and the grave/summoning sites in §3)
- no-arg `randomUsingDefaults()` extras: spyglass (`RegularLevel.java:688`),
  Mimic Tooth extra (`Mimic.java:357`), Ebony Mimic extra
  (`EbonyMimic.java:101`), Gnoll Exile (`GnollExile.java:106-108`),
  secret chest chasm (`SecretChestChasmRoom.java:67-82`)

Imp does **not** use this fallback. It calls `randomArtifact()` directly, then
on null `Generator.random(RING)` — a RING **deck** draw (`Imp.java:319-327`).

`UnstableSpellbook`’s constructor still runs `setupScrolls`
(`UnstableSpellbook.java:90-103`) after `popGenerator`, so a shifted artifact
index can change floor-stream consumption.

## 6. Challenge and trinket rule-outs

`Challenges.isItemBlocked` (`Challenges.java:71-79`) is true only for a
`Dewdrop` under `NO_HERBALISM`. No RING/WAND/ARTIFACT/WEAPON/ARMOR generator
produces a Dewdrop, so the reroll loops in `Mimic.generatePrize`, `PitRoom`,
and `CrystalVaultRoom` never iterate from challenges.

`NO_SCROLLS` omits every second queued Scroll of Upgrade (`Level.java:234-240`)
with no RNG at that site. An empty `itemsToSpawn` can still change a later
painter fallback that consumes the floor stream.

`TRINKET` has weight `0, 0` in the general decks (`Generator.java:222`) and
never appears in floor loot. The Trinket Catalyst is the levelgen source:
`Dungeon.trinketCataNeeded()` (`Dungeon.java:584-586`) is
`depth < 5 && !dropped && Random.Int(4-depth) == 0`, so depth 1 is `Int(3)`,
depth 2 is `Int(2)`, and depth 3 is `Int(1) == 0` (guaranteed if still
needed). Opening the catalyst rolls `NUM_TRINKETS = 4` options through
`Generator.random(TRINKET)` (`TrinketCatalyst.java:161-178`). That deck is not
consumed during levelgen; with no earlier TRINKET draw the four offers are
draws 0–3. Seventeen equal-weight classes (`Generator.java:580-600`) make any
given trinket appear among the four with probability 4/17. Scroll of
Transmutation on a taken trinket (`ScrollOfTransmutation.java:322-325`) is the
other consumer. Held trinkets (Mossy Clump, Mimic Tooth, Rat Skull, Trap
Mechanism, Parchment Scrap, Cracked Spyglass) are player state, not seed-only
rule-outs of later deck indices.

`RegularLevel.java:406` evaluates `Random.Float()` before applying
`MimicTooth.mimicChanceMultiplier()`, so the tooth-free stream is the
baseline. Spyglass extras use `randomUsingDefaults()` under a pushed long
(`RegularLevel.java:679-689`).

## 7. `undoDrop` is a no-op for concrete classes

```
if (cls.isAssignableFrom(cat.superClass)) { … cat.probs[i]++; }
```

(`Generator.java:662-672`.) `WandOfFireblast.class.isAssignableFrom(Wand.class)`
is false, so `undoDrop(item)` on a generated instance never puts a card back.
Callers: Wandmaker duplicate wands (`Wandmaker.java:339-346`), Blacksmith
duplicate weapons (`Blacksmith.java:368-375`), Crystal Path duplicate
potion/scroll (`CrystalPathRoom.java:185-194`). Rejected draws permanently
advance `dropped`.

Blacksmith spawn-time `generateRewards(true)` (`Blacksmith.java:358`,
`:363-376`) passes that flag into `randomWeapon(3, useDecks)` as
`useDefaults=true` (`Generator.java:809-818`). The stored pool therefore
avoids the WEP/MIS decks. `WndBlacksmith`’s `generateRewards(false)` fallback
(`WndBlacksmith.java:477-478`) is deck-backed.

## 8. Imp reward pool and vault treasure

`CityLevel.initRooms()` calls `Imp.Quest.spawn` (`CityLevel.java:192-193`)
during **branch 0** city generation, after feeling/builder/room selection and
before shuffle/paint/mobs/items. On success it adds `AmbitiousImpRoom` (vault
entrance) and fills `rewardOptions` (`Imp.java:305-354`):

1. `randomArtifact()`; if non-null, `identify(false)` + `transferUpgrade(5)`;
   if null, `Generator.random(RING)` then `level(IntRange(2,4))`.
2. `Generator.random(RING)` until the class differs from slot 1 (collision
   only when both are rings). Then `level(IntRange(2,4))`. No `undoDrop`.
3. `Random.Int(2)`: either `WEP_T5` + `MIS_T4`, or `MIS_T5` + `WEP_T4`, each
   `Generator.random(sub-tier)` then `.enchant()`.
4. `new PlateArmor().inscribe()` — no ARMOR deck.
5. `Generator.random(WAND)`, then `level(IntRange(2,4))` and full charges.

Every option is then `cursed = false`. Weapon/wand `.random()` still ran on
the floor stream before those overwrites.

`VaultLevel` is `branch == 1` at depths 16–19 (`Dungeon.java:356-368`). It
overrides `initRooms` (no Imp spawn), `createMobs`, and `createItems`.
`build()` pre-rolls four T0 equipment drops plus three
`randomUsingDefaults(FOOD)` (`VaultLevel.java:131-145`), then `super.build()`.
`setupEquipmentAtTier` (`VaultLevel.java:241-376`) fills four tiers, each
2× melee + missile + constructed armor + wand + ring, all
`randomUsingDefaults` on `WEP_T*` / `MIS_T*` / `WAND` / `RING`. Duplicate
class retries (and banned `WandOfRegrowth` / `Transfusion` / `Corruption`,
`RingOfWealth` / `Might` / `Force`) stay on defaults. Treasure rooms only
pop those lists.

`createItems` drops `itemsToSpawn` and does not call `Generator.random()`
(`VaultLevel.java:602-627`).

## 9. `WEP_T3` initialization

Static init (`Generator.java:441-450`):

```
WEP_T3.classes = { Sword, Mace, Scimitar, RoundShield, Sai, Whip };
WEP_T3.defaultProbs = { 2, 2, 2, 2, 2, 2 };
WEP_T3.probs = WEP_T3.defaultProbs.clone();
```

`reset(WEP_T3)` clones `defaultProbs` again (`Generator.java:645-653`). The
live T3 deck is six equal weights. `fullReset` always `reset`s every category
before play.

## 10. Vault uses the same Generator static state

There is one `Category` enum and one set of `seed` / `dropped` / `probs`
fields. Vault generation runs under `seedForDepth(depth, 1)` but reads and
could write those fields. In this version every vault `Generator` call is
`randomUsingDefaults` on FOOD / WEP_T* / MIS_T* / WAND / RING, so vault
treasure does **not** increment those decks. A main-path floor generated after
a vault visit therefore sees the same RING/WAND/WEP/MIS/FOOD `dropped` values
the vault found. ARTIFACT is not requested in `VaultLevel`.

Imp’s pool is rolled on the branch-0 city floor that places the entrance,
before the player can enter the vault.

## Category tables (`defaultProbs`)

General 35-item pair (`firstProb`, `secondProb`): WEAPON 2/2, ARMOR 2/1,
MISSILE 1/2, WAND 1/1, RING 1/0, ARTIFACT 0/1, POTION 8/8, SEED 1/1, SCROLL
8/8, STONE 1/1, GOLD 10/10; TRINKET, FOOD, and all `WEP_T*` / `MIS_T*` are
0/0 (`Generator.java:221-252`). `fullReset` picks the starting general deck
with `Int(2)` (`Generator.java:626`).

| Category | Notes |
|----------|--------|
| POTION / SCROLL | Two 12-entry decks; SoS/SoU (index 0) weight 0. `defaultProbs` `{0,3,2,1,2,1,1,1,1,1,1,1}`, `defaultProbs2` `{0,3,2,2,1,2,1,1,1,1,1,0}` (`Generator.java:329-378`) |
| SEED | `{0,2,2,2,2,2,2,2,2,2,2,1}` — Rotberry 0, Starflower 1 (`Generator.java:346-360`). No-arg `random()` forces defaults for SEED (`Generator.java:684-688`) |
| STONE | `{0,2,2,2,2,2,2,2,2,2,2,0}` — Enchantment and Augmentation 0 (`Generator.java:380-395`) |
| WAND | 13 × weight 3 (`Generator.java:397-412`) |
| RING | 12 × weight 3 (`Generator.java:544-558`) |
| ARTIFACT | `{1,1,0,1,1,0,1,1,1,1,1,1,1}` — CloakOfShadows and HolyTome 0 (`Generator.java:560-576`) |
| TRINKET | 17 × weight 1 (`Generator.java:580-600`) |
| FOOD | `{4,1,0}` (`Generator.java:537-542`). Main-path `Level.create` uses the FOOD **deck** (`Level.java:226`, `:278`); vault uses defaults |
| WEP_T1 | `{2,0,2,2,2,2}` — MagesStaff 0 |
| WEP_T2 | `{2,2,2,2,2,2,0}` — Pickaxe 0 |
| WEP_T3–T5 | all 2s (6 / 7 / 7 classes) |
| MIS_T1 | `{3,3,3,0}` — Dart 0; MIS_T2–T5 all `{3,3,3}` |
| ARMOR / GOLD | no `defaultProbs`; ARMOR uses `floorSetTierProbs` (`Generator.java:613-618`, `:776-787`) |

`randomWeapon` / `randomMissile` / `randomArmor` pick tier from
`floorSetTierProbs[depth/5]` (T1 column is 0). Ghost and Imp pass a specific
`WEP_T*` / `MIS_T*` category instead.
