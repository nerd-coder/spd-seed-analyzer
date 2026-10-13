# Item sprite rects

Target: **Shattered Pixel Dungeon v4.0.2 @ `57a4e06a4`**.

`ItemSpriteSheet` stores every item as a top-left crop of its atlas
cell (`assignItemRect`, `ItemSpriteSheet.java:43-46`). The cell is
16×16 on `sprites/items.png` (256×512, 16 columns) and 8×8 on
`sprites/item_icons.png` (128×64, 16 columns; `ItemSpriteSheet.java:822-837`).
The crop is the sprite the game centers. Drawing the whole cell leaves
the art in the top-left.

`web/src/lib/item-sprite-rects.ts` is this table. `ItemBox` and heap
sprites read it. An unlisted `items.png` index is 16×16. An unlisted
`item_icons.png` index is 8×8.

## Version bump

When `PIN-SPD` moves, do this in the same change as the other
[analysis](README.md) re-verify:

1. Re-read `assignItemRect` and `Icons.assignIconRect` in the new `ItemSpriteSheet.java`.
2. Re-measure opaque pixels (alpha > 0) in `web/public/assets/sprites/items.png` and `web/public/assets/sprites/item_icons.png`.
3. Replace the tables below and `web/src/lib/item-sprite-rects.ts`.
4. Keep the film crop as the drawn size. Transparent pixels inside the crop stay in the frame. Pixels outside the crop are clipped.

## Measured pixels

Opaque bounds were measured on the pinned sheets (alpha > 0 inside each cell).
They match the film rect at the cell origin, except:

| Sheet | Index | Sprite | Film | Opaque (x, y, w, h) |
|---|---:|---|---|---|
| items.png | 47 | *(no film entry)* | 16×16 | `3, 15, 2, 1` |
| items.png | 101 | `MAGES_STAFF` | 15×16 | `0, 2, 15, 14` |
| items.png | 248 | `ARTIFACT_CHAINS` | 16×16 | `0, 0, 15, 16` |
| items.png | 260 | `ARTIFACT_ROSE1` | 14×14 | `1, 1, 13, 13` |

Reserved slots in a category run keep that run's film size and contain no opaque pixels. Index 47 has no `assignItemRect`; its 2×1 speck stays inside the 16×16 frame.

## `items.png`

Indices whose film is not 16×16. A range shares one size; the name is the first and last constant in that range.

| Index | Sprite | W | H |
|---|---|---:|---:|
| 0 | `SOMETHING` | 8 | 13 |
| 1 | `WEAPON_HOLDER` | 14 | 14 |
| 2 | `ARMOR_HOLDER` | 14 | 12 |
| 3 | `MISSILE_HOLDER` | 15 | 15 |
| 4 | `WAND_HOLDER` | 14 | 14 |
| 5 | `RING_HOLDER` | 8 | 10 |
| 6 | `ARTIFACT_HOLDER` | 15 | 15 |
| 7 | `TRINKET_HOLDER` | 16 | 11 |
| 8 | `FOOD_HOLDER` | 15 | 11 |
| 9 | `BOMB_HOLDER` | 10 | 13 |
| 10 | `POTION_HOLDER` | 12 | 14 |
| 11 | `SEED_HOLDER` | 10 | 10 |
| 12 | `SCROLL_HOLDER` | 15 | 14 |
| 13 | `STONE_HOLDER` | 14 | 12 |
| 14 | `ELIXIR_HOLDER` | 12 | 14 |
| 15 | `SPELL_HOLDER` | 8 | 16 |
| 16 | `MOB_HOLDER` | 15 | 14 |
| 17 | `DOCUMENT_HOLDER` | 10 | 11 |
| 18 | `GOLD` | 15 | 13 |
| 21 | `DEWDROP` | 10 | 9 |
| 22 | `PETAL` | 8 | 8 |
| 23 | `SANDBAG` | 10 | 10 |
| 24 | `SPIRIT_ARROW` | 11 | 11 |
| 26–27 | `TENGU_BOMB … TENGU_SHOCKER` | 10 | 10 |
| 28 | `GEO_BOULDER` | 16 | 15 |
| 32–33 | `BONES … REMAINS` | 14 | 11 |
| 34–35 | `TOMB … GRAVE` | 14 | 15 |
| 36–39 | `CHEST … EBONY_CHEST` | 16 | 14 |
| 48 | `ANKH` | 10 | 16 |
| 49 | `STYLUS` | 12 | 13 |
| 50 | `SEAL` | 13 | 13 |
| 51 | `TORCH` | 12 | 15 |
| 52 | `BEACON` | 16 | 15 |
| 53–54 | `HONEYPOT … SHATTPOT` | 14 | 12 |
| 55–58 | `IRON_KEY … WORN_KEY` | 8 | 14 |
| 59 | `MASK` | 11 | 9 |
| 60 | `CROWN` | 13 | 7 |
| 62 | `MASTERY` | 13 | 16 |
| 63 | `KIT` | 16 | 15 |
| 64 | `SEAL_SHARD` | 12 | 12 |
| 65 | `BROKEN_STAFF` | 14 | 10 |
| 66 | `CLOAK_SCRAP` | 9 | 9 |
| 67 | `BOW_FRAGMENT` | 12 | 9 |
| 68 | `BROKEN_HILT` | 9 | 9 |
| 69 | `TORN_PAGE` | 11 | 13 |
| 70 | `TRINKET_CATA` | 12 | 11 |
| 80 | `BOMB` | 10 | 13 |
| 81 | `DBL_BOMB` | 14 | 13 |
| 82–85 | `FIRE_BOMB … SMOKE_BOMB` | 13 | 12 |
| 86–91 | `FLASHBANG … SHRAPNEL_BOMB` | 10 | 13 |
| 96 | `WORN_SHORTSWORD` | 13 | 13 |
| 97 | `CUDGEL` | 15 | 15 |
| 98 | `GLOVES` | 12 | 16 |
| 99 | `RAPIER` | 13 | 14 |
| 100 | `DAGGER` | 12 | 13 |
| 101 | `MAGES_STAFF` | 15 | 16 |
| 104 | `SHORTSWORD` | 13 | 13 |
| 105 | `HAND_AXE` | 12 | 14 |
| 108 | `DIRK` | 13 | 14 |
| 109 | `SICKLE` | 15 | 15 |
| 112 | `SWORD` | 14 | 14 |
| 113 | `MACE` | 15 | 15 |
| 114 | `SCIMITAR` | 13 | 16 |
| 117 | `WHIP` | 14 | 14 |
| 120 | `LONGSWORD` | 15 | 15 |
| 122–123 | `FLAIL … RUNIC_BLADE` | 14 | 14 |
| 124 | `ASSASSINS_BLADE` | 14 | 15 |
| 125 | `CROSSBOW` | 15 | 15 |
| 126 | `KATANA` | 15 | 16 |
| 131–132 | `GREATAXE … GREATSHIELD` | 12 | 16 |
| 133 | `GAUNTLETS` | 13 | 15 |
| 134 | `WAR_SCYTHE` | 14 | 15 |
| 145 | `THROWING_SPIKE` | 11 | 10 |
| 146 | `THROWING_KNIFE` | 12 | 13 |
| 147 | `THROWING_STONE` | 12 | 10 |
| 148 | `FISHING_SPEAR` | 11 | 11 |
| 149–150 | `SHURIKEN … THROWING_CLUB` | 12 | 12 |
| 151 | `THROWING_SPEAR` | 13 | 13 |
| 152 | `BOLAS` | 15 | 14 |
| 153 | `KUNAI` | 15 | 15 |
| 155 | `TOMAHAWK` | 13 | 13 |
| 156 | `BOOMERANG` | 14 | 14 |
| 158 | `THROWING_HAMMER` | 12 | 12 |
| 159 | `FORCE_CUBE` | 11 | 12 |
| 160–175 | `DART … DARTS+15` | 15 | 15 |
| 176 | `ARMOR_CLOTH` | 15 | 12 |
| 177 | `ARMOR_LEATHER` | 14 | 13 |
| 178 | `ARMOR_MAIL` | 14 | 12 |
| 179 | `ARMOR_SCALE` | 14 | 11 |
| 180–181 | `ARMOR_PLATE … ARMOR_WARRIOR` | 12 | 12 |
| 182 | `ARMOR_MAGE` | 15 | 15 |
| 183 | `ARMOR_ROGUE` | 14 | 12 |
| 184 | `ARMOR_HUNTRESS` | 13 | 15 |
| 185 | `ARMOR_DUELIST` | 12 | 13 |
| 186 | `ARMOR_CLERIC` | 13 | 14 |
| 208–223 | `WAND_MAGIC_MISSILE … WANDS+15` | 14 | 14 |
| 224–239 | `RING_GARNET … RINGS+15` | 8 | 10 |
| 240 | `ARTIFACT_CLOAK` | 9 | 15 |
| 241 | `ARTIFACT_ARMBAND` | 16 | 13 |
| 242 | `ARTIFACT_CAPE` | 16 | 14 |
| 243 | `ARTIFACT_TALISMAN` | 15 | 13 |
| 244 | `ARTIFACT_HOURGLASS` | 13 | 16 |
| 245 | `ARTIFACT_TOOLKIT` | 15 | 13 |
| 246 | `ARTIFACT_SPELLBOOK` | 13 | 16 |
| 249–252 | `ARTIFACT_HORN1 … ARTIFACT_HORN4` | 15 | 15 |
| 253–255 | `ARTIFACT_CHALICE1 … ARTIFACT_CHALICE3` | 12 | 15 |
| 256–257 | `ARTIFACT_SANDALS … ARTIFACT_SHOES` | 16 | 6 |
| 258 | `ARTIFACT_BOOTS` | 16 | 9 |
| 259 | `ARTIFACT_GREAVES` | 16 | 14 |
| 260–262 | `ARTIFACT_ROSE1 … ARTIFACT_ROSE3` | 14 | 14 |
| 263 | `ARTIFACT_TOME` | 14 | 16 |
| 264 | `ARTIFACT_KEY` | 8 | 16 |
| 272 | `RAT_SKULL` | 16 | 11 |
| 273 | `PARCHMENT_SCRAP` | 10 | 14 |
| 274 | `PETRIFIED_SEED` | 9 | 9 |
| 275 | `EXOTIC_CRYSTALS` | 15 | 13 |
| 276 | `MOSSY_CLUMP` | 12 | 11 |
| 277 | `SUNDIAL` | 16 | 12 |
| 278 | `CLOVER` | 11 | 15 |
| 279 | `TRAP_MECHANISM` | 13 | 15 |
| 280 | `MIMIC_TOOTH` | 8 | 15 |
| 281 | `WONDROUS_RESIN` | 12 | 11 |
| 282 | `EYE_OF_NEWT` | 12 | 12 |
| 283 | `SALT_CUBE` | 12 | 13 |
| 284 | `BLOOD_VIAL` | 6 | 15 |
| 285 | `OBLIVION_SHARD` | 7 | 14 |
| 286 | `CHAOTIC_CENSER` | 13 | 15 |
| 287 | `FERRET_TUFT` | 16 | 15 |
| 288 | `SPYGLASS` | 15 | 15 |
| 304–316 | `SCROLL_KAUNAN … SCROLLS+12` | 15 | 14 |
| 317 | `ARCANE_RESIN` | 12 | 11 |
| 318–335 | `SCROLLS+14 … EXOTIC_SCROLLS+15` | 15 | 14 |
| 336–351 | `STONE_AGGRESSION … STONES+15` | 14 | 12 |
| 352–364 | `POTION_CRIMSON … POTIONS+12` | 12 | 14 |
| 365 | `LIQUID_METAL` | 8 | 15 |
| 366–367 | `POTIONS+14 … POTIONS+15` | 12 | 14 |
| 368–383 | `EXOTIC_CRIMSON … EXOTIC_POTIONS+15` | 12 | 13 |
| 384–399 | `SEED_ROTBERRY … SEEDS+15` | 10 | 10 |
| 400–401 | `BREW_INFERNAL … BREW_BLIZZARD` | 11 | 13 |
| 402–403 | `BREW_SHOCKING … BREW_CAUSTIC` | 12 | 14 |
| 404 | `BREW_AQUA` | 9 | 11 |
| 405 | `BREW_UNSTABLE` | 11 | 13 |
| 406–415 | `BREWS+6 … ELIXIR_FEATHER` | 12 | 14 |
| 416 | `WILD_ENERGY` | 6 | 15 |
| 417 | `PHASE_SHIFT` | 12 | 10 |
| 418 | `TELE_GRAB` | 10 | 10 |
| 419 | `UNSTABLE_SPELL` | 12 | 13 |
| 420 | `CURSE_INFUSE` | 10 | 16 |
| 421 | `MAGIC_INFUSE` | 10 | 14 |
| 422 | `ALCHEMIZE` | 12 | 12 |
| 423 | `RECYCLE` | 12 | 13 |
| 424 | `RECLAIM_TRAP` | 14 | 11 |
| 425–430 | `RETURN_BEACON … SUMMON_ELE_CHAOS` | 8 | 16 |
| 432–434 | `MEAT … STEWED` | 15 | 11 |
| 435 | `OVERPRICED` | 14 | 11 |
| 436 | `CARPACCIO` | 15 | 11 |
| 437 | `RATION` | 16 | 12 |
| 438 | `PASTY` | 16 | 11 |
| 439 | `MEAT_PIE` | 16 | 12 |
| 440 | `BLANDFRUIT` | 9 | 12 |
| 441 | `BLAND_CHUNKS` | 14 | 6 |
| 442 | `BERRY` | 9 | 11 |
| 443 | `PHANTOM_MEAT` | 15 | 11 |
| 444 | `SUPPLY_RATION` | 16 | 12 |
| 448–449 | `STEAMED_FISH … FISH_LEFTOVER` | 16 | 12 |
| 451–452 | `EASTER_EGG … RAINBOW_POTION` | 12 | 14 |
| 453 | `SHATTERED_CAKE` | 14 | 13 |
| 454 | `PUMPKIN_PIE` | 16 | 12 |
| 455 | `VANILLA_CAKE` | 14 | 13 |
| 456 | `CANDY_CANE` | 13 | 16 |
| 457 | `SPARKLING_POTION` | 7 | 16 |
| 465 | `DUST` | 12 | 11 |
| 466 | `CANDLE` | 12 | 12 |
| 467 | `EMBER` | 12 | 11 |
| 468 | `PICKAXE` | 14 | 14 |
| 469 | `ORE` | 15 | 15 |
| 470 | `TOKEN` | 12 | 12 |
| 471 | `BLOB` | 10 | 9 |
| 472 | `SHARD` | 8 | 10 |
| 473 | `ESCAPE` | 8 | 16 |
| 474 | `STATUE` | 10 | 16 |
| 480 | `WATERSKIN` | 16 | 14 |
| 482 | `POUCH` | 14 | 15 |
| 484–485 | `BANDOLIER … HOLSTER` | 15 | 16 |
| 486 | `VIAL` | 12 | 12 |
| 496–502 | `GUIDE_PAGE … HALLS_PAGE` | 10 | 11 |

## `item_icons.png`

Indices whose film is not 8×8.

| Index | Sprite | W | H |
|---|---|---:|---:|
| 0–2 | `RING_ACCURACY … RING_ELEMENTS` | 7 | 7 |
| 3 | `RING_ENERGY` | 7 | 5 |
| 4 | `RING_EVASION` | 7 | 7 |
| 5 | `RING_FORCE` | 5 | 6 |
| 6 | `RING_FUROR` | 7 | 6 |
| 7 | `RING_HASTE` | 6 | 6 |
| 8–9 | `RING_MIGHT … RING_SHARPSHOOT` | 7 | 7 |
| 10 | `RING_TENACITY` | 6 | 6 |
| 11 | `RING_WEALTH` | 7 | 6 |
| 32 | `SCROLL_UPGRADE` | 7 | 7 |
| 33 | `SCROLL_IDENTIFY` | 4 | 7 |
| 34 | `SCROLL_REMCURSE` | 7 | 7 |
| 35–36 | `SCROLL_MIRRORIMG … SCROLL_RECHARGE` | 7 | 5 |
| 37 | `SCROLL_TELEPORT` | 7 | 7 |
| 38 | `SCROLL_LULLABY` | 7 | 6 |
| 39 | `SCROLL_MAGICMAP` | 7 | 7 |
| 40 | `SCROLL_RAGE` | 6 | 6 |
| 41 | `SCROLL_RETRIB` | 5 | 6 |
| 42 | `SCROLL_TERROR` | 5 | 7 |
| 43 | `SCROLL_TRANSMUTE` | 7 | 7 |
| 48 | `SCROLL_ENCHANT` | 7 | 7 |
| 49 | `SCROLL_DIVINATE` | 7 | 6 |
| 50 | `SCROLL_ANTIMAGIC` | 7 | 7 |
| 51 | `SCROLL_PRISIMG` | 5 | 7 |
| 52 | `SCROLL_MYSTENRG` | 7 | 5 |
| 53 | `SCROLL_PASSAGE` | 5 | 7 |
| 54 | `SCROLL_SIREN` | 7 | 6 |
| 55 | `SCROLL_FORESIGHT` | 7 | 5 |
| 56 | `SCROLL_CHALLENGE` | 7 | 7 |
| 57 | `SCROLL_PSIBLAST` | 5 | 6 |
| 58 | `SCROLL_DREAD` | 5 | 7 |
| 59 | `SCROLL_METAMORPH` | 7 | 7 |
| 80 | `POTION_STRENGTH` | 7 | 7 |
| 81 | `POTION_HEALING` | 6 | 7 |
| 82 | `POTION_MINDVIS` | 7 | 5 |
| 83 | `POTION_FROST` | 7 | 7 |
| 84 | `POTION_LIQFLAME` | 5 | 7 |
| 85 | `POTION_TOXICGAS` | 7 | 7 |
| 86 | `POTION_HASTE` | 6 | 6 |
| 87 | `POTION_INVIS` | 5 | 7 |
| 88 | `POTION_LEVITATE` | 6 | 7 |
| 89 | `POTION_PARAGAS` | 7 | 7 |
| 90 | `POTION_PURITY` | 5 | 7 |
| 91 | `POTION_EXP` | 7 | 7 |
| 96 | `POTION_MASTERY` | 7 | 7 |
| 97 | `POTION_SHIELDING` | 6 | 6 |
| 98 | `POTION_MAGISIGHT` | 7 | 5 |
| 99–101 | `POTION_SNAPFREEZ … POTION_CORROGAS` | 7 | 7 |
| 102 | `POTION_STAMINA` | 6 | 6 |
| 103–104 | `POTION_SHROUDFOG … POTION_STRMCLOUD` | 7 | 7 |
| 105 | `POTION_EARTHARMR` | 6 | 6 |
| 106–107 | `POTION_CLEANSE … POTION_DIVINE` | 7 | 7 |
