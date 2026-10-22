import { tilesetRegion } from '@/lib/dungeon-tile-visuals'
import { mobSheetUrl } from '@/lib/map-mob-sprites'

/** Mob sheets keyed by `MobSprite.sheet`; only sheets the map uses are loaded. */
type MobAssets = Record<string, HTMLImageElement>

export type MapEntityAssets = {
  mobs: MobAssets
  terrainFeatures: HTMLImageElement
  items: HTMLImageElement
}

export type MapAssets = MapEntityAssets & {
  tiles: HTMLImageElement
  water: HTMLImageElement
  raisedTerrain: HTMLImageElement
  occlusionShadows: HTMLImageElement
  specks: HTMLImageElement
  /** `interfaces/icons.png`, for mob emotes. */
  icons: HTMLImageElement
  /** `effects/effects.png`, for laser beams. */
  effects: HTMLImageElement
  customTiles: {
    prisonQuest: HTMLImageElement
    cavesQuest: HTMLImageElement
    cityQuest: HTMLImageElement
    cityBoss: HTMLImageElement
    weakFloor: HTMLImageElement
    hallsSpecial: HTMLImageElement
    carpet: HTMLImageElement
    ratKingRoom: HTMLImageElement
  }
}

export const SPECKS_URL = '/assets/effects/specks.png'

const imageCache = new Map<string, Promise<HTMLImageElement>>()

export function loadImage(url: string): Promise<HTMLImageElement> {
  let promise = imageCache.get(url)
  if (!promise) {
    promise = new Promise((resolve, reject) => {
      const image = new Image()
      image.onload = () => resolve(image)
      image.onerror = () =>
        reject(new Error(`Failed to load map asset: ${url}`))
      image.src = url
    })
    imageCache.set(url, promise)
  }
  return promise
}

async function loadMobAssets(sheets: Iterable<string>): Promise<MobAssets> {
  const entries = await Promise.all(
    [...new Set(sheets)].map(
      async (sheet) => [sheet, await loadImage(mobSheetUrl(sheet))] as const
    )
  )
  return Object.fromEntries(entries)
}

export async function loadMapAssets(
  tileset: string,
  mobSheets: Iterable<string> = []
): Promise<MapAssets> {
  const region = tilesetRegion(tileset)
  const [
    tiles,
    terrainFeatures,
    water,
    items,
    mobs,
    raisedTerrain,
    occlusionShadows,
    specks,
    icons,
    effects,
    prisonQuest,
    cavesQuest,
    cityQuest,
    cityBoss,
    weakFloor,
    hallsSpecial,
    carpet,
    ratKingRoom,
  ] = await Promise.all([
    loadImage(`/assets/environment/tiles_${tileset}.png`),
    loadImage('/assets/environment/terrain_features.png'),
    loadImage(`/assets/environment/water${region}.png`),
    loadImage('/assets/sprites/items.png'),
    loadMobAssets(mobSheets),
    loadImage('/assets/environment/raised_terrain.png'),
    loadImage('/assets/environment/occlusion_shadows.png'),
    loadImage(SPECKS_URL),
    loadImage('/assets/interfaces/icons.png'),
    loadImage('/assets/effects/effects.png'),
    loadImage('/assets/environment/custom_tiles/prison_quest.png'),
    loadImage('/assets/environment/custom_tiles/caves_quest.png'),
    loadImage('/assets/environment/custom_tiles/city_quest.png'),
    loadImage('/assets/environment/custom_tiles/city_boss.png'),
    loadImage('/assets/environment/custom_tiles/weak_floor.png'),
    loadImage('/assets/environment/custom_tiles/halls_special.png'),
    loadImage('/assets/environment/custom_tiles/carpet.png'),
    loadImage('/assets/environment/custom_tiles/rat_king_room.png'),
  ])
  return {
    tiles,
    terrainFeatures,
    water,
    items,
    raisedTerrain,
    occlusionShadows,
    specks,
    icons,
    effects,
    mobs,
    customTiles: {
      prisonQuest,
      cavesQuest,
      cityQuest,
      cityBoss,
      weakFloor,
      hallsSpecial,
      carpet,
      ratKingRoom,
    },
  }
}
