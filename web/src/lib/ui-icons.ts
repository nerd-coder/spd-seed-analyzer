/**
 * Frames from SPD `interfaces/icons.png` (256×128).
 * Depth icons match Icons.DEPTH_* used in MenuPane (feeling-aware).
 * Seed analyzer always uses the custom-seed row (runTypeOfsY = 8).
 */

export const UI_ICONS = {
  url: "/assets/interfaces/icons.png",
  sheetW: 256,
  sheetH: 128,
} as const;

export type IconFrame = {
  x: number;
  y: number;
  w: number;
  h: number;
};

/** Seeded-run depth icons (y = 80 + 8). Regular run would use y = 80. */
const SEED_DEPTH_Y = 88;

/** Small depth icons next to the floor number in-game. */
export const DEPTH_ICONS: Record<string, IconFrame> = {
  none: { x: 32, y: SEED_DEPTH_Y, w: 6, h: 7 },
  chasm: { x: 40, y: SEED_DEPTH_Y, w: 7, h: 7 },
  water: { x: 48, y: SEED_DEPTH_Y, w: 7, h: 7 },
  grass: { x: 56, y: SEED_DEPTH_Y, w: 7, h: 7 },
  dark: { x: 64, y: SEED_DEPTH_Y, w: 7, h: 7 },
  large: { x: 72, y: SEED_DEPTH_Y, w: 7, h: 7 },
  traps: { x: 80, y: SEED_DEPTH_Y, w: 7, h: 7 },
  secrets: { x: 88, y: SEED_DEPTH_Y, w: 7, h: 7 },
};

/** Larger stairs variants (Icons.STAIRS_*). */
export const STAIRS_ICONS: Record<string, IconFrame> = {
  none: { x: 0, y: 64, w: 15, h: 16 },
  chasm: { x: 16, y: 64, w: 15, h: 16 },
  water: { x: 32, y: 64, w: 15, h: 16 },
  grass: { x: 48, y: 64, w: 15, h: 16 },
  dark: { x: 64, y: 64, w: 15, h: 16 },
  large: { x: 80, y: 64, w: 15, h: 16 },
  traps: { x: 96, y: 64, w: 15, h: 16 },
  secrets: { x: 112, y: 64, w: 15, h: 16 },
};

export function depthIconFrame(feeling?: string | null): IconFrame {
  const key = (feeling ?? "none").toLowerCase();
  return DEPTH_ICONS[key] ?? DEPTH_ICONS.none;
}

export function stairsIconFrame(feeling?: string | null): IconFrame {
  const key = (feeling ?? "none").toLowerCase();
  return STAIRS_ICONS[key] ?? STAIRS_ICONS.none;
}

/** CSS background crop for a frame, scaled by `scale` (1 = native px). */
export function uiIconStyle(frame: IconFrame, scale = 2) {
  const dw = frame.w * scale;
  const dh = frame.h * scale;
  return {
    width: dw,
    height: dh,
    backgroundImage: `url(${UI_ICONS.url})`,
    backgroundRepeat: "no-repeat" as const,
    backgroundSize: `${UI_ICONS.sheetW * scale}px ${UI_ICONS.sheetH * scale}px`,
    backgroundPosition: `-${frame.x * scale}px -${frame.y * scale}px`,
    imageRendering: "pixelated" as const,
    flexShrink: 0,
  };
}
