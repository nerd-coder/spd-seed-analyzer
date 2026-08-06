import { tilesetRegion } from '@/lib/dungeon-tile-visuals'

const MOB_ASSET_URLS = {
  rat: '/assets/sprites/rat.png',
  snake: '/assets/sprites/snake.png',
  skeleton: '/assets/sprites/skeleton.png',
  swarm: '/assets/sprites/swarm.png',
  thief: '/assets/sprites/thief.png',
  shopkeeper: '/assets/sprites/shopkeeper.png',
  dm100: '/assets/sprites/dm100.png',
  guard: '/assets/sprites/guard.png',
  necromancer: '/assets/sprites/necromancer.png',
} as const

export type MobAssetKey = keyof typeof MOB_ASSET_URLS

type MobAssets = Record<MobAssetKey, HTMLImageElement>

export type MapEntityAssets = MobAssets & {
  terrainFeatures: HTMLImageElement
  items: HTMLImageElement
}

export type MapAssets = MapEntityAssets & {
  tiles: HTMLImageElement
  water: HTMLImageElement
  customTiles: {
    prisonQuest: HTMLImageElement
    cavesQuest: HTMLImageElement
    cityQuest: HTMLImageElement
    cityBoss: HTMLImageElement
    weakFloor: HTMLImageElement
    hallsSpecial: HTMLImageElement
  }
}

const imageCache = new Map<string, Promise<HTMLImageElement>>()

function loadImage(url: string): Promise<HTMLImageElement> {
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

async function loadMobAssets(): Promise<MobAssets> {
  const entries = await Promise.all(
    Object.entries(MOB_ASSET_URLS).map(
      async ([key, url]) => [key, await loadImage(url)] as const
    )
  )
  return Object.fromEntries(entries) as MobAssets
}

export async function loadMapAssets(tileset: string): Promise<MapAssets> {
  const region = tilesetRegion(tileset)
  const [
    tiles,
    terrainFeatures,
    water,
    items,
    mobs,
    prisonQuest,
    cavesQuest,
    cityQuest,
    cityBoss,
    weakFloor,
    hallsSpecial,
  ] = await Promise.all([
    loadImage(`/assets/environment/tiles_${tileset}.png`),
    loadImage('/assets/environment/terrain_features.png'),
    loadImage(`/assets/environment/water${region}.png`),
    loadImage('/assets/sprites/items.png'),
    loadMobAssets(),
    loadImage('/assets/environment/custom_tiles/prison_quest.png'),
    loadImage('/assets/environment/custom_tiles/caves_quest.png'),
    loadImage('/assets/environment/custom_tiles/city_quest.png'),
    loadImage('/assets/environment/custom_tiles/city_boss.png'),
    loadImage('/assets/environment/custom_tiles/weak_floor.png'),
    loadImage('/assets/environment/custom_tiles/halls_special.png'),
  ])
  return {
    tiles,
    terrainFeatures,
    water,
    items,
    ...mobs,
    customTiles: {
      prisonQuest,
      cavesQuest,
      cityQuest,
      cityBoss,
      weakFloor,
      hallsSpecial,
    },
  }
}
