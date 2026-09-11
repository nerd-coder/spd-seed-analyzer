# Weapon enchantments, curses, and armor glyphs

Target: **Shattered Pixel Dungeon v4.0.0 @ `2bb34a4e9`**. Java line numbers
are from that commit. Method: reading the pinned clone.

These tables are the ones `Weapon.Enchantment.random*` / `Armor.Glyph.random*`
use at generation. They are not decks: each call is `Random.chances` on the
rarity mix, then `Random.element` on that rarity’s list.

## Weapon enchantments

`Weapon.Enchantment` (`Weapon.java:518-540`):

| Rarity | Weight | Classes | Per-enchant share |
|--------|--------|---------|-------------------|
| Common | 50 | Blazing, Chilling, Kinetic, Shocking, Venomous | 10% each |
| Uncommon | 40 | Blocking, Blooming, Eldritch, Elastic, Lucky, Projecting, Unstable, Vorpal | 5% each |
| Rare | 10 | Corrupting, Crystal, Grim, Vampiric | 2.5% each |

`typeChances = {50, 40, 10}` (`Weapon.java:531-535`).

## Weapon curses

Equal `Random.element` among ten (`Weapon.java:537-540`):

Annoying, Displacing, Dazzling, Explosive, Friendly, Polarized, Pressurized,
Sacrificial, Wayward, Wondrous.

`random()` never selects a curse. `randomCurse(...)` is a separate entry
(`Weapon.java:656-664`). If the ignore list empties a rarity bucket, that
helper calls `random()` (good-enchant mix), not another curse roll
(`Weapon.java:659-660`).

## Armor glyphs

`Armor.Glyph` (`Armor.java:794-813`), same 50/40/10 mix:

| Rarity | Weight | Classes | Per-glyph share |
|--------|--------|---------|-----------------|
| Common | 50 | Obfuscation, Swiftness, Viscosity, Potential | 12.5% each |
| Uncommon | 40 | Brimstone, Stone, Entanglement, Repulsion, Camouflage, Flow | 6.67% each |
| Rare | 10 | Affection, AntiMagic, Thorns | 3.33% each |

Curses, equal among eight (`Armor.java:810-813`): AntiEntropy, Corrosion,
Displacement, Metabolism, Multiplicity, Stench, Overgrowth, Bulk.

`Armor.inscribe()` / `Glyph.random*` are the glyph path used at generation
(Ghost, Blacksmith, Imp plate, statues, crypt, vault, `Armor.random()`).

## How `enchant()` / `inscribe()` consume RNG

`Enchantment.random(toIgnore...)` (`Weapon.java:611-620`):

1. `Random.chances(typeChances)` → 0 common / 1 uncommon / 2 rare.
2. Copy that array, `removeAll(toIgnore)`, then `Random.element`.
3. Empty bucket → recursive `random()` with no ignore.

`Weapon.enchant()` with no args (`Weapon.java:466-471`) ignores the current
enchantment’s class, then `enchant(ench)` (no further RNG).
`Armor.inscribe()` is the same shape (`Armor.java:742-747`, `:863-872`).
`randomCurse` / `Glyph.randomCurse` skip the rarity roll and only `element`.

`Weapon.random()` / `MissileWeapon.random()` (`Weapon.java:422-451`,
`MissileWeapon.java:365-394`):

- Level: `Int(4)==0` then maybe `Int(5)==0` → +0 75% / +1 20% / +2 5%.
- `Random.pushGenerator(Random.Long())` isolates the effect from the floor
  stream (Parchment Scrap cannot desync later levelgen).
- One `Float`: curse if `< 0.3 * curseChanceMultiplier()`, else enchant if
  `>= 1 - 0.1 * enchantChanceMultiplier()`.
- Curse path: `Enchantment.randomCurse()` + `cursed = true`. Enchant path:
  `enchant()` → `Enchantment.random(old)`.

`Armor.random()` (`Armor.java:654-684`) is the same level roll and pushed
long; glyph chance uses **0.15** rather than 0.1.

Parchment Scrap multipliers (`ParchmentScrap.java:52-84`): enchant 1 / 2 / 4 /
7 / 10 for none / +0 / +1 / +2 / +3; curse 1 / 1.5 / 2 / 1 / 0. At scrap +3
the curse branch is unreachable (`0.3 * 0 == 0` is not `< 0`).

Direct `Enchantment.random()` / `Glyph.random()` / no-arg `enchant()` /
`inscribe()` use the **ambient** generator (the floor stream during
`Level.create()`), not the parchment-isolated one.

## Generation sites

| Site | What is rolled | Stream |
|------|----------------|--------|
| `Weapon.random` / `MissileWeapon.random` / `Armor.random` | 30% curse or 10%/15% good effect | Isolated long |
| Sad Ghost (`Ghost.java:335-361`) | Clears the weapon’s random effect; always `Enchantment.random()` + `Glyph.random()`; keep both iff `enchantRoll <= 0.2 * parchment` | Floor |
| Troll Blacksmith (`Blacksmith.java:392-411`) | Clears all four rewards; always `Enchantment.random()` + `Glyph.random()`; keep both iff `enchantRoll <= 0.3 * parchment` | Floor |
| Imp pool (`Imp.java:338-345`) | `.enchant()` on the two weapons; `new PlateArmor().inscribe()` | Floor (overwrites `Weapon.random`) |
| Statue (`Statue.java:63-71`) | `cursed = false` then `enchant(Enchantment.random())` | Floor |
| Armored statue (`ArmoredStatue.java:51-56`) | Same weapon path plus `Generator.randomArmor()` then `inscribe(Glyph.random())` | Floor |
| Vault equipment (`VaultLevel.java:271-356`) | After defaults + `level(...)`: `Int(3) >= lootTier` → `enchant(null)` / `inscribe(null)`, else `enchant()` / `inscribe()`. T0 always clears; T3 always applies | Floor |
| Vault mirror (`VaultMirror.java:69-97`) | Warrior seal `Glyph.random()`; mage staff / spirit bow / mirror sword `enchant()` under `pushGenerator(Random.Long())` | Isolated long |
| Sacrifice room (`SacrificeRoom.java:87-104`) | Always `Enchantment.randomCurse()`; apply if the prize has no good enchant; then force cursed | Floor |
| Crypt (`CryptRoom.java:77-94`) | Always `Glyph.randomCurse()`; same keep/apply rule | Floor |
| Shop melee + missile (`ShopRoom.java:256-266`) | `enchant(null)`, uncursed, level 0 | — |
| Pool / sentry / traps / secret maze | `enchant(null)` / `inscribe(null)` only when the random effect is a curse | Floor |
| Golden / ebony mimic prize pass | Same curse-clear as pool | Floor |

Ghost and Blacksmith generate the stored enchant/glyph **before** the keep
roll so Parchment Scrap cannot change the number of RNG calls
(`Ghost.java:353-356`, `Blacksmith.java:402-405`). The effect is applied at
claim (`WndSadGhost.java:114-116`, `WndBlacksmith.java:517-520`).

Imp weapons chain `Generator.random(WEP_T*|MIS_T*).enchant()`: the sub-deck
picks the class, `Weapon.random()` may already roll an isolated effect, then
`.enchant()` overwrites it on the floor stream. Plate armor is constructed
directly (no `Armor.random()`), so its only glyph roll is `inscribe()`.

Statue levelgen uses decks (`Statue.random()` → `createWeapon(true)`);
`DistortionTrap` and `GuardianTrap` use defaults. Guardian then
`enchant(null)` (`GuardianTrap.java:85-87`).

Runtime-only (not seed generation): `StoneOfEnchantment`,
`ScrollOfEnchantment` (common + uncommon + `random` with those two ignored),
`Stylus.inscribe()`, `CurseInfusion`, `CursingTrap`, Ring of Wealth
re-enchant, Tormented Spirit cleanse (`TormentedSpirit.java:62-67`).

## Unstable and the newer enchants

`Unstable.proc` instantiates `Random.oneOf(randomEnchants)`
(`Unstable.java:34-61`). That list:

Blazing, Blocking, Blooming, Chilling, Kinetic, Corrupting, Elastic, Grim,
Lucky, Shocking, Vampiric, **Venomous**, **Vorpal**, **Eldritch**.

Projecting is omitted (`Unstable.java:44`, no on-hit effect). **Crystal is
not in the list.** Unstable itself is not in the list. This array is combat
RNG, not a generation table; generating Unstable still goes through the
uncommon weapon table.
