# Ambitious Imp quest — verified audit

Target: **Shattered Pixel Dungeon v4.0.0 @ `2bb34a4e9`**.

This note is the source of trust for the city Imp, the pre-rolled reward
pool, vault entry, score, and the depth-20 Imp shop. Vault layout and
in-vault loot live in `vault-level.md`.

### Verdict

The strongest unprofiled seed-only contract is:

- the Imp first attempts to spawn on depth 17, retries on 18, and is
  guaranteed on depth 19 (`Imp.java:306`);
- a successful spawn inserts `AmbitiousImpRoom` at the end of
  `CityLevel.initRooms`, then immediately fills `rewardOptions` with six
  items (`Imp.java:308-353`, `CityLevel.java:191-194`);
- that pool is created on the city floor even if the player never enters
  the vault;
- every option is forcibly uncursed (`Imp.java:351-353`);
- the player keeps **at most one** item, chosen when leaving the vault
  with `EscapeCrystal`, and only if score is at least 500
  (`EscapeCrystal.java:165-236`);
- the depth-20 Imp shop is an access condition (`earnedShop()`), not a
  guarantee that shop stock exists in the world
  (`Imp.java:397-399`, `CityBossLevel.java:377-379`).

Concrete classes, exact `+N` values, which city depth hosts the Imp, and
the WEP/MIS coin flip are **not** universal seed-only results. They
require a fixed generation profile (deck indices, artifact exhaustion,
challenges, trinkets, and current-floor RNG before `initRooms`).

### Spawn site and RNG test

`Level.create()` pushes `Dungeon.seedCurDepth()`, evaluates limited
drops and feeling (branch 0 only), then `build()`
(`Level.java:219-316`). `RegularLevel.build()` selects the builder and
calls `initRooms()` **before** the room shuffle
(`RegularLevel.java:105-110`). `CityLevel.initRooms()` is

```
return Imp.Quest.spawn(super.initRooms());
```

(`CityLevel.java:191-194`). The Imp room is therefore an extra special
room added after ordinary standard/special/secret selection, and before
shuffle, layout, painting, mobs, and ordinary items.

The spawn test is `!spawned && depth > 16 && Random.Int(20 - depth) == 0`
(`Imp.java:306`). Depth 17 uses `Int(3)`, depth 18 uses `Int(2)`, depth
19 uses `Int(1)` and always succeeds if still unspawned. Failed attempts
create neither room nor `rewardOptions`. There is no Monk/Golem roll on
a fresh run (`spawn` sets `oldQuest = false`, `Imp.java:311`).

`AmbitiousImpRoom` is a fixed 9×9 `SpecialRoom`. Its painter places the
Imp NPC, a `BRANCH_EXIT` at the room center (same depth, `branch + 1`),
and city-quest décor (`AmbitiousImpRoom.java:45-117`). NPC offset uses
`Random.IntRange(-1, 1)` during paint, after the reward pool already
exists (`AmbitiousImpRoom.java:73-79`).

### Pre-rolled `rewardOptions`

On a successful spawn, `rewardOptions` is cleared and six items are
appended in this order (`Imp.java:318-353`).

1. **Artifact, or ring fallback.** `Generator.randomArtifact()` draws
   from the seeded ARTIFACT deck (`Generator.java:856-878`). Cloak of
   Shadows and Holy Tome have weight 0, leaving eleven draw-able classes
   (`Generator.java:560-576`). A hit is `identify(false)` then
   `transferUpgrade(5)` (`Imp.java:321-322`, `Artifact.java:139-141`).
   Displayed upgrade is `round(level * 10 / levelCap)`
   (`Artifact.java:123-125`): **+5** at `levelCap` 10, **+6** at 5 (Ethereal
   Chains, Timekeeper's Hourglass), **+7** at 3 (Sandals of Nature). If
   the deck is empty, `randomArtifact` returns null (and still increments
   `ARTIFACT.dropped`, `Generator.java:867-875`); the code then draws
   `Generator.random(RING)` — the **RING deck**, not defaults — and sets
   `level(Random.IntRange(2, 4))` without identifying
   (`Imp.java:323-327`).
2. **Ring.** `do { ring = Generator.random(RING) } while (ring.getClass()
   == artif.getClass())`, then `level(Random.IntRange(2, 4))`, still
   unidentified (`Imp.java:330-336`). The class check can fire only when
   slot 1 was already a ring. Each rejected draw still consumes the RING
   deck and `Ring.random()` (`Ring.java:259-277`).
3. **WEP/MIS coin flip.** `Random.Int(2) == 0` chooses
   `WEP_T5` at `IntRange(2, 4)` plus `MIS_T4` at `IntRange(3, 5)`;
   otherwise `MIS_T5` at `IntRange(2, 4)` plus `WEP_T4` at
   `IntRange(3, 5)` (`Imp.java:338-344`). Each call is
   `Generator.random(category)` (tier **decks**, `Generator.java:698-741`),
   then `.enchant().identify(false).level(...)`. `Weapon.random()` /
   `MissileWeapon.random()` still run inside `Generator.random` and
   consume ambient rolls plus a pushed effect stream
   (`Weapon.java:422-451`, `MissileWeapon.java:365-394`); the later
   `.enchant()` overwrites that effect on the ambient stream
   (`Weapon.java:466-471`, `Enchantment.random` at `Weapon.java:611-620`,
   type weights 50 / 40 / 10).
4. **Plate armor.** `new PlateArmor().inscribe().identify(false).level(
   IntRange(2, 4))` (`Imp.java:345`). No ARMOR deck. `inscribe()` always
   applies `Glyph.random` (50 / 40 / 10, `Armor.java:742-747,863-871`).
5. **Wand.** `Generator.random(WAND)` (WAND deck), `identify(false)`,
   `level(IntRange(2, 4))`, `curCharges = maxCharges` (`Imp.java:346-349`,
   `Wand.random` at `Wand.java:547-566`).

Every option then gets `cursed = false` (`Imp.java:351-353`). `item.random()`
curse rolls are discarded.

`Generator.random(cat)` for RING / WEP_T4 / WEP_T5 / MIS_T4 / MIS_T5 /
WAND uses the category's isolated `cat.seed` stream for the class, then
`item.random()` on the **city-floor ambient** stream
(`Generator.java:712-740`). Artifact class selection is the same pattern
(`Generator.java:860-878`). `UnstableSpellbook`'s constructor then burns
a full `SCROLL.defaultProbsTotal` chances loop on ambient
(`UnstableSpellbook.java:84-104`), so drawing that artifact shifts every
later slot on this floor.

T4/T5 melee classes (equal weight 2): Longsword, Battle Axe, Flail, Runic
Blade, Assassin's Blade, Crossbow, Katana / Greatsword, War Hammer,
Glaive, Greataxe, Greatshield, Gauntlet, War Scythe
(`Generator.java:452-474`). T4/T5 missiles (equal weight 3): Javelin,
Tomahawk, Heavy Boomerang / Trident, Throwing Hammer, Force Cube
(`Generator.java:521-535`). WAND is thirteen classes at weight 3
(`Generator.java:397-412`). RING is twelve classes at weight 3
(`Generator.java:544-558`).

### Pool exists without entering the vault

`rewardOptions` is filled during city `initRooms`. The vault is a later
branch level. If the player never uses the `BRANCH_EXIT`, the six items
remain on the quest object and are not world heaps. `VaultFinalRoom.paint`
is what drops them onto pedestals and `rewardOptions.clear()`
(`VaultFinalRoom.java:228-236`). The city Imp has no reward window on a
fresh run (`Imp.java:153-181`).

### Claiming one item

Vault entry (below) strips gear. Inside the vault the six options sit on
`VaultFinalRoom` pedestals with a unique `ImpStatue` on the center
pedestal (`VaultFinalRoom.java:228-229`). The locked door opens only after
the boss fight (`VaultFinalRoom.java:329-340`). Leaving uses
`EscapeCrystal`:

- score **< 50**: flavour only; the hero does not leave
  (`EscapeCrystal.java:144-147`);
- score **50–499**: leave with no preserved item
  (`EscapeCrystal.java:234-236`);
- score **≥ 500**: pick exactly one non-crystal item, with level / unique
  filters that tighten below 4000 (`EscapeCrystal.java:165-198`).
  Quantity > 1 consumables keep a single copy (`EscapeCrystal.java:211-213`).
  The six pre-rolled options are equipment or a wand at +2…+5, so they fail
  the ≤1000 consumable-only filter and the <4000 `level() ≤ 0 or 1` filter.
  Taking one of those six out requires statue score (≥ 4000). Lower scores
  can still preserve vault T0/T1 loot or a consumable.

`leaveVault` restores stored belongings, then collects that one item (or
stashes it as `Imp.Quest.reward` if the backpack is full, and the city Imp
drops it on its turn) (`EscapeCrystal.java:258-346`, `Imp.java:77-82`).
Everything else held in the vault is discarded. `ImpStatue` is unique, so
it cannot be the preserved item (`ImpStatue.java:32-33`).

### Vault entry

`CityLevel.activateTransition` on `BRANCH_EXIT` refuses the door when the
quest is old, already complete, not yet given, or the hero has
`AscensionChallenge` or `LostInventory` (`CityLevel.java:141-147`).
Otherwise a yes/no prompt runs. Accepting: `hero.live()` (non-persistent
buffs cleared, hunger/regen reset), full HP, belongings / gold / energy /
quickslots stored on an `EscapeCrystal`, inventory cleared, `ClothArmor`
equipped (`CityLevel.java:160-178`, `EscapeCrystal.java:301-319`). Walking
onto the tile does not travel; the crystal is the only way out of the
vault (`VaultLevel.java:206-209`).

Giving the quest is the first Imp interaction; it sets `given = true`
with no reward choice (`Imp.java:154-167`). The player can refuse to talk
or refuse the door.

### Score, `earnedShop`, depth-20 shop

`EscapeCrystal` scores a vault leave (`EscapeCrystal.java:93-140`):

| Source | Points |
|---|---|
| `ImpStatue` in belongings | 4000, skips the rest |
| Explore | `1000 * levelExplorePercent` (80% discoverable tiles seen = 1.0, `VaultLevel.java:174-181`) |
| Token door destroyed | 1250 |
| Door still present | `min(1000, 100 * token quantity)` |
| Boss summoned and still on the level | `(int)(750 * HP/HT)` |
| Boss summoned and gone | 750 |

The total is then rounded down to a multiple of 50. Hazard hits spend two
freebies, then subtract 100 from `Statistics.questScores[3]` (not from
`Imp.Quest.score`) (`VaultLaser.java:105-109`, `VaultSentry.java:143-147`,
`VaultFlameTraps.java:93-97`). `complete(score)` sets `Imp.Quest.score`,
adds that unpenalized value to `Statistics.questScores[3]`, and removes
the landmark (`Imp.java:385-391`).

`earnedShop()` is `completed && (oldQuest || score > 2000)`
(`Imp.java:397-399`). A statue run or a high partial (> 2000) unlocks the
shop; leaving with ≤ 2000 completes the quest without a shop. After
completion the city Imp uses score-band text and flees once `score > 2000`
and the hero is out of FOV (`Imp.java:84-86,174-180`). Ascension destroys
the Imp (`Imp.java:72-75`).

`CityBossLevel.build` always constructs `ImpShopRoom` and calls `paint()`,
which generates and stores stock without placing it
(`CityBossLevel.java:193-203`, `ImpShopRoom.java:63-69`). Depth 20 stock
uses `ShopRoom.generateItems` (`ShopRoom.java:222-266`), including deck
draws `Generator.random(wepTiers[4])` and `misTiers[4]` (WEP_T5 / MIS_T5).
Placement happens when Dwarf King is defeated if `earnedShop()` is already
true, or on a later load (`CityBossLevel.java:128-130,377-379`,
`ImpShopRoom.java:154-159`). Completing the quest does not reroll stored
stock. Inventory-sensitive shop decisions use the hero state present when
depth 20 is first generated.

### `oldQuest` / `WndImpOld` — legacy saves only

A missing `OLD_QUEST` bundle key restores `oldQuest = true`
(`Imp.java:280-284`). Fresh `spawn` always sets `oldQuest = false`
(`Imp.java:311`). The old interaction is token turn-in via `WndImpOld`
against a single stored `reward` (`Imp.java:128-152`, `WndImpOld.java:42-86`).
`oldProcess` drops `DwarfToken` from Monks or Golems
(`Imp.java:367-375`); `oldComplete` grants 4000 ranking points and shop
access via the `oldQuest` flag (`Imp.java:377-383,397-399`). This is
save-compat, not a custom-seed generation path.

### Deck-counter effects on later floors

A successful Imp spawn, on the city floor, consumes:

| Draw | Deck? |
|---|---|
| `randomArtifact()` | ARTIFACT yes (or `dropped++` on exhaustion) |
| RING fallback and the always-present ring (plus duplicate rejects) | RING yes |
| one of WEP_T5 or WEP_T4 | that tier deck |
| one of MIS_T4 or MIS_T5 | that tier deck |
| WAND | WAND yes |
| Plate armor | no |

Those counters persist into later city generation on the same floor, depth
20 shop WEP_T5/MIS_T5 draws, and Halls. Vault generation itself does **not**
use these decks (`vault-level.md`).

### Inputs that change the pre-rolled pool

Seed-isolated per floor, but **not** independent of run state:

- **Current-floor RNG before `initRooms`.** Limited-drop checks and the
  feeling roll run before `Imp.Quest.spawn` (`Level.java:224-299`).
  `Int(14)` cases LARGE / SECRETS / extra FOOD change room counts or add
  a FOOD-deck draw before the Imp. In the default branch both Mossy and
  Trap-Mechanism floats are always consumed (`Level.java:289-297`);
  `getNextFeeling` is an isolated `Dungeon.seed+1` generator
  (`MossyClump.java:71-91`, `TrapMechanism.java:83-103`), and
  GRASS/WATER/TRAPS/CHASM do not change `initRooms` counts, so those
  overrides do not shift this floor's Imp stream.
- **Persistent decks.** Any earlier RING / ARTIFACT / WEP_T4 / WEP_T5 /
  MIS_T4 / MIS_T5 / WAND draw (mimics, shops, crystal vault, general
  category, …) changes slot identity. After eleven artifacts are gone,
  slot 1 is a deck ring and the duplicate loop can consume extra RING
  draws.
- **Challenges and trinkets.** `NO_SCROLLS` does not add a new pre-Imp
  ambient call on the current floor (`Level.java:234-240`) but can change
  earlier-floor deck counts. Parchment Scrap does not affect the visible
  Imp enchants (those use ambient `.enchant()` / `inscribe()` after a
  pushed `item.random()` effect stream). Thirteen-leaf Clover rewrites
  `Random` globally. Mimic Tooth, Rat Skull, and other trinkets that add
  or skip deck-using sites on earlier floors still move the counters.
- **Hero class** does not feed `rewardOptions`. It does feed the vault
  mirror (`vault-level.md`).
