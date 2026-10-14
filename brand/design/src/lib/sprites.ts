/**
 * Sprite crops for the preview, from SPD v4.0.2 (`57a4e06a4`).
 * Item cells are `ItemSpriteSheet` indices on items.png (16px grid, 16 cols);
 * widths and heights are the film crops from web/src/lib/item-sprite-rects.ts.
 */

export type Sprite = {
  url: string
  sheetWidth: number
  sheetHeight: number
  x: number
  y: number
  w: number
  h: number
}

const ITEMS = { url: '/sprites/items.png', width: 256, height: 512 }

function item(index: number, w: number, h: number): Sprite {
  return {
    url: ITEMS.url,
    sheetWidth: ITEMS.width,
    sheetHeight: ITEMS.height,
    x: (index % 16) * 16,
    y: Math.floor(index / 16) * 16,
    w,
    h,
  }
}

/** First frame of a mob `TextureFilm`. */
function npc(name: string, sheetWidth: number, w: number, h: number): Sprite {
  return {
    url: `/sprites/${name}.png`,
    sheetWidth,
    sheetHeight: 16,
    x: 0,
    y: 0,
    w,
    h,
  }
}

export const SPRITES = {
  sword: item(112, 14, 14),
  longsword: item(120, 15, 15),
  greatsword: item(128, 16, 16),
  runicBlade: item(123, 14, 14),
  dagger: item(100, 12, 13),
  warHammer: item(129, 16, 16),
  leatherArmor: item(177, 14, 13),
  mailArmor: item(178, 14, 12),
  scaleArmor: item(179, 14, 11),
  plateArmor: item(180, 12, 12),
  gold: item(18, 15, 13),
  ankh: item(48, 10, 16),
  honeypot: item(53, 14, 12),
  ironKey: item(55, 8, 14),
  food: item(437, 16, 12),
  pasty: item(438, 16, 11),
  potionCrimson: item(352, 12, 14),
  potionAmber: item(353, 12, 14),
  potionGolden: item(354, 12, 14),
  potionJade: item(355, 12, 14),
  potionTurquoise: item(356, 12, 14),
  potionAzure: item(357, 12, 14),
  potionIndigo: item(358, 12, 14),
  potionMagenta: item(359, 12, 14),
  scrollKaunan: item(304, 15, 14),
  scrollSowilo: item(305, 15, 14),
  scrollLaguz: item(306, 15, 14),
  scrollYngvi: item(307, 15, 14),
  scrollGyfu: item(308, 15, 14),
  scrollRaido: item(309, 15, 14),
  ringGarnet: item(224, 8, 10),
  ringRuby: item(225, 8, 10),
  ringTopaz: item(226, 8, 10),
  ringEmerald: item(227, 8, 10),
  ringSapphire: item(231, 8, 10),
  ringAmethyst: item(232, 8, 10),
  wandMagicMissile: item(208, 14, 14),
  wandFireblast: item(209, 14, 14),
  wandFrost: item(210, 14, 14),
  wandLightning: item(211, 14, 14),
  wandCorruption: item(217, 14, 14),
  wandRegrowth: item(219, 14, 14),
  cloakOfShadows: item(240, 9, 15),
  hornOfPlenty: item(249, 15, 15),
  driedRose: item(260, 14, 14),
  chaliceOfBlood: item(253, 12, 15),
  seedSungrass: item(387, 10, 10),
  seedFadeleaf: item(394, 10, 10),
  stoneAugmentation: item(337, 14, 12),
  ghost: npc('ghost', 128, 14, 15),
  wandmaker: npc('wandmaker', 64, 12, 14),
  blacksmith: npc('blacksmith', 64, 13, 16),
  imp: npc('imp', 64, 12, 14),
} as const satisfies Record<string, Sprite>

export type SpriteName = keyof typeof SPRITES
