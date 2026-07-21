import { Loader2, Search } from 'lucide-react'
import { type FormEvent, useEffect, useState } from 'react'

import { DepthIcon } from '@/components/DepthIcon'
import { FloorMapCanvas } from '@/components/FloorMapCanvas'
import { ItemIcon } from '@/components/ItemIcon'
import { Alert, AlertDescription, AlertTitle } from '@/components/ui/alert'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { Switch } from '@/components/ui/switch'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import {
  formatItemSource,
  isHighlightSource,
  type ParsedQuest,
  parseQuest,
} from '@/lib/labels'
import {
  analyzeSeed,
  type FloorReport,
  getSpdMeta,
  type IdentityEntry,
  type IdentityMaps,
  type SeedReport,
} from '@/lib/spd-wasm'
import { cn } from '@/lib/utils'

const MAP_SPOILERS_KEY = 'spd-analyzer-map-spoilers'
const IDENTITY_SPOILERS_KEY = 'spd-analyzer-identity-spoilers'
/** Migrated from pre-rename “Advanced mode”. */
const LEGACY_ADVANCED_KEY = 'spd-analyzer-advanced-mode'

function readLocalFlag(key: string, fallback = false): boolean {
  try {
    const v = localStorage.getItem(key)
    if (v === null) return fallback
    return v === '1'
  } catch {
    return fallback
  }
}

function writeLocalFlag(key: string, value: boolean) {
  try {
    localStorage.setItem(key, value ? '1' : '0')
  } catch {
    /* ignore */
  }
}

/** SPD region bands (same as tileset_for_depth). */
const REGIONS = [
  { id: 'sewers', label: 'Sewers', min: 1, max: 5 },
  { id: 'prison', label: 'Prison', min: 6, max: 10 },
  { id: 'caves', label: 'Caves', min: 11, max: 15 },
  { id: 'city', label: 'City', min: 16, max: 20 },
  { id: 'halls', label: 'Halls', min: 21, max: 26 },
] as const

/** Non-regular depths (bosses + LastLevel) — omitted from the Floors UI. */
const BOSS_DEPTHS = new Set([5, 10, 15, 20, 25, 26])

function groupFloorsByRegion(floors: FloorReport[]) {
  return REGIONS.map((region) => ({
    region,
    floors: floors
      .filter(
        (f) =>
          f.depth >= region.min &&
          f.depth <= region.max &&
          !BOSS_DEPTHS.has(f.depth)
      )
      .sort((a, b) => a.depth - b.depth),
  })).filter((g) => g.floors.length > 0)
}

function appearanceDescription(
  category: 'potion' | 'scroll' | 'ring',
  appearance: string
): string {
  const label = appearance.toLowerCase()
  switch (category) {
    case 'potion':
      return `${label} potion`
    case 'scroll':
      return `${label} rune`
    case 'ring':
      return `${label} gem`
  }
}

function itemAppearance(
  item: { category: string; class_name?: string | null },
  identities: IdentityMaps
): string | undefined {
  if (item.category === 'potion') {
    return identities.potions.find((p) => p.item === item.class_name)
      ?.appearance
  }
  if (item.category === 'scroll') {
    return identities.scrolls.find((s) => s.item === item.class_name)
      ?.appearance
  }
  if (item.category === 'ring') {
    return identities.rings.find((r) => r.item === item.class_name)?.appearance
  }
  return undefined
}

function IdentityGrid({
  entries,
  category,
}: {
  entries: IdentityEntry[]
  category: 'potion' | 'scroll' | 'ring'
}) {
  return (
    <div className="grid grid-cols-2 gap-x-3 gap-y-2 sm:grid-cols-3 md:grid-cols-4">
      {entries.map((e) => (
        <div key={e.item} className="flex min-w-0 items-center gap-2">
          <ItemIcon
            classNameItem={e.item}
            category={category}
            appearance={e.appearance}
            size={24}
            title={e.name}
            className="shrink-0"
          />
          <div className="min-w-0 leading-tight">
            <div className="truncate text-sm font-medium">{e.name}</div>
            <div className="text-muted-foreground truncate text-xs capitalize">
              {appearanceDescription(category, e.appearance)}
            </div>
          </div>
        </div>
      ))}
    </div>
  )
}

const QUEST_KIND_STYLES: Record<
  ParsedQuest['kind'],
  { badge: string; border: string }
> = {
  ghost: {
    badge: 'bg-sky-500/15 text-sky-800 dark:text-sky-200 border-sky-500/30',
    border: 'border-sky-500/25 bg-sky-500/5',
  },
  wandmaker: {
    badge:
      'bg-violet-500/15 text-violet-800 dark:text-violet-200 border-violet-500/30',
    border: 'border-violet-500/25 bg-violet-500/5',
  },
  blacksmith: {
    badge:
      'bg-amber-500/15 text-amber-900 dark:text-amber-200 border-amber-500/30',
    border: 'border-amber-500/25 bg-amber-500/5',
  },
  imp: {
    badge: 'bg-rose-500/15 text-rose-800 dark:text-rose-200 border-rose-500/30',
    border: 'border-rose-500/25 bg-rose-500/5',
  },
  other: {
    badge: '',
    border: 'border-border bg-muted/40',
  },
}

function QuestCard({ quest }: { quest: string }) {
  const parsed = parseQuest(quest)
  const styles = QUEST_KIND_STYLES[parsed.kind]
  return (
    <div
      className={cn('rounded-lg border px-3 py-2.5 space-y-1.5', styles.border)}
    >
      <div className="flex flex-wrap items-center gap-2">
        <Badge variant="outline" className={cn('font-medium', styles.badge)}>
          {parsed.title}
        </Badge>
        {parsed.detail && (
          <span className="text-muted-foreground text-xs">{parsed.detail}</span>
        )}
      </div>
      {parsed.rewards && (
        <p className="text-sm leading-snug">
          <span className="text-muted-foreground text-xs font-medium tracking-wide uppercase mr-1.5">
            Rewards
          </span>
          {parsed.rewards}
        </p>
      )}
      {!parsed.rewards && (
        <p className="text-muted-foreground text-xs">{parsed.raw}</p>
      )}
    </div>
  )
}

function FloorDetail({
  floor,
  identities,
  mapSpoilers,
}: {
  floor: FloorReport
  identities: IdentityMaps
  mapSpoilers: boolean
}) {
  return (
    <div className="space-y-3">
      <div className="flex flex-wrap items-center gap-2">
        {floor.feeling && floor.feeling !== 'none' && (
          <Badge variant="secondary" className="capitalize">
            {floor.feeling}
          </Badge>
        )}
        {floor.builder && (
          <Badge variant="outline" className="font-mono text-xs">
            {floor.builder}
          </Badge>
        )}
        {floor.quests && floor.quests.length > 0 && (
          <Badge variant="default" className="text-xs">
            Quest
          </Badge>
        )}
        {floor.map && mapSpoilers && (
          <Badge variant="outline" className="font-mono text-xs">
            {floor.map.width}×{floor.map.height} · {floor.map.tileset}
          </Badge>
        )}
        {!floor.feeling &&
          !floor.builder &&
          !(floor.quests && floor.quests.length > 0) &&
          !(floor.map && mapSpoilers) && (
            <span className="text-muted-foreground text-xs">
              Floor {floor.depth}
            </span>
          )}
      </div>

      {mapSpoilers && floor.map && (
        <div className="overflow-x-auto">
          <FloorMapCanvas map={floor.map} scale={2} />
        </div>
      )}

      {floor.quests && floor.quests.length > 0 && (
        <div className="space-y-2">
          <p className="text-muted-foreground text-xs font-medium tracking-wide uppercase">
            Quests
          </p>
          <div className="space-y-2">
            {floor.quests.map((q, i) => (
              <QuestCard key={`${floor.depth}-quest-${i}`} quest={q} />
            ))}
          </div>
        </div>
      )}

      {floor.rooms && floor.rooms.length > 0 && (
        <div className="space-y-1">
          <p className="text-muted-foreground text-xs font-medium tracking-wide uppercase">
            Rooms
          </p>
          <p className="text-sm leading-relaxed">
            {floor.rooms.map((r) => r.replace(/Room$/, '')).join(' · ')}
          </p>
        </div>
      )}

      {floor.items.length === 0 ? (
        <p className="text-muted-foreground text-sm">No items listed.</p>
      ) : (
        <ul className="space-y-1.5 text-sm">
          {floor.items.map((item, i) => {
            const sourceLabel = formatItemSource(item.source)
            const highlight = isHighlightSource(item.source)
            return (
              <li
                key={`${floor.depth}-${i}`}
                className="flex items-start gap-2"
              >
                <ItemIcon
                  classNameItem={item.class_name}
                  category={item.category}
                  appearance={itemAppearance(item, identities)}
                  size={16}
                  title={item.name}
                  className="mt-0.5"
                />
                <span className="min-w-0 flex flex-wrap items-baseline gap-x-1.5 gap-y-0.5">
                  <span>{item.name}</span>
                  {sourceLabel && (
                    <Badge
                      variant={highlight ? 'secondary' : 'outline'}
                      className="font-normal text-[10px] px-1.5 py-0 h-5"
                      title={item.source ?? undefined}
                    >
                      {sourceLabel}
                    </Badge>
                  )}
                </span>
              </li>
            )
          })}
        </ul>
      )}
    </div>
  )
}

function FloorsByRegion({
  floors,
  identities,
  mapSpoilers,
}: {
  floors: FloorReport[]
  identities: IdentityMaps
  mapSpoilers: boolean
}) {
  const groups = groupFloorsByRegion(floors)
  if (groups.length === 0) return null

  const defaultRegion = groups[0].region.id

  return (
    <Tabs defaultValue={defaultRegion} className="gap-0">
      <TabsList className="w-full flex-wrap h-auto sm:w-auto">
        {groups.map(({ region, floors: regionFloors }) => (
          <TabsTrigger key={region.id} value={region.id}>
            {region.label}
            <span className="text-muted-foreground ml-1 tabular-nums text-xs">
              {regionFloors[0].depth}
              {regionFloors.length > 1
                ? `–${regionFloors[regionFloors.length - 1].depth}`
                : ''}
            </span>
          </TabsTrigger>
        ))}
      </TabsList>

      {groups.map(({ region, floors: regionFloors }) => {
        const defaultFloor = String(regionFloors[0].depth)
        return (
          <TabsContent key={region.id} value={region.id}>
            <Tabs defaultValue={defaultFloor} className="gap-4">
              <TabsList
                variant="line"
                className="h-auto w-full flex-wrap justify-start"
              >
                {regionFloors.map((floor) => {
                  const hasQuest = (floor.quests?.length ?? 0) > 0
                  const feelingLabel =
                    floor.feeling && floor.feeling !== 'none'
                      ? floor.feeling
                      : null
                  const titleParts = [
                    `Floor ${floor.depth}`,
                    feelingLabel,
                    hasQuest ? 'quest' : null,
                  ].filter(Boolean)
                  return (
                    <TabsTrigger
                      key={floor.depth}
                      value={String(floor.depth)}
                      className="h-auto flex-col gap-0.5 px-2 py-1.5 relative"
                      title={titleParts.join(' · ')}
                    >
                      {/* MenuPane-style: feeling depth icon above the number */}
                      <DepthIcon feeling={floor.feeling} size={24} />
                      <span className="font-mono text-xs tabular-nums leading-none">
                        {floor.depth}
                      </span>
                      {hasQuest && (
                        <span
                          className="bg-primary absolute top-0.5 right-0.5 size-1.5 rounded-full"
                          aria-hidden
                        />
                      )}
                    </TabsTrigger>
                  )
                })}
              </TabsList>

              {regionFloors.map((floor) => (
                <TabsContent key={floor.depth} value={String(floor.depth)}>
                  <FloorDetail
                    floor={floor}
                    identities={identities}
                    mapSpoilers={mapSpoilers}
                  />
                </TabsContent>
              ))}
            </Tabs>
          </TabsContent>
        )
      })}
    </Tabs>
  )
}

/** Full main-path depth range (SPD clamps to 26). */
const ANALYZE_FLOORS = 26

export default function App() {
  const [seed, setSeed] = useState('GFX-PZH-DCH')
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [report, setReport] = useState<SeedReport | null>(null)
  const [meta, setMeta] = useState<{ version: string; commit: string } | null>(
    null
  )
  const [mapSpoilers, setMapSpoilers] = useState(() => {
    const legacy = readLocalFlag(LEGACY_ADVANCED_KEY)
    return readLocalFlag(MAP_SPOILERS_KEY, legacy)
  })
  const [identitySpoilers, setIdentitySpoilers] = useState(() =>
    readLocalFlag(IDENTITY_SPOILERS_KEY)
  )

  useEffect(() => {
    getSpdMeta()
      .then(setMeta)
      .catch((e: unknown) => {
        setError(e instanceof Error ? e.message : String(e))
      })
  }, [])

  function toggleMapSpoilers(next: boolean) {
    setMapSpoilers(next)
    writeLocalFlag(MAP_SPOILERS_KEY, next)
  }

  function toggleIdentitySpoilers(next: boolean) {
    setIdentitySpoilers(next)
    writeLocalFlag(IDENTITY_SPOILERS_KEY, next)
  }

  async function onAnalyze(e: FormEvent) {
    e.preventDefault()
    setLoading(true)
    setError(null)
    try {
      const result = await analyzeSeed(seed.trim(), ANALYZE_FLOORS)
      setReport(result)
    } catch (err) {
      setReport(null)
      setError(err instanceof Error ? err.message : String(err))
    } finally {
      setLoading(false)
    }
  }

  return (
    <div className="mx-auto flex min-h-svh w-full max-w-4xl flex-col gap-6 px-4 py-10">
      <header className="space-y-4">
        <div className="flex flex-col">
          <div className="flex flex-wrap items-end gap-3">
            <div
              className="relative h-16 md:h-20"
              style={{ aspectRatio: '616/200' }}
            >
              <img
                src="/assets/title.gif"
                alt="Shattered Pixel Dungeon"
                className="absolute inset-0 h-full w-full object-contain"
                style={{ imageRendering: 'pixelated' }}
              />
              <img
                src="/assets/title_overlay.png"
                alt="SEED Analyzer"
                className="absolute inset-0 h-full w-full object-contain"
                style={{ imageRendering: 'pixelated' }}
              />
            </div>
            {meta && (
              <Badge
                variant="secondary"
                className="font-mono text-xs mb-1 md:mb-2"
              >
                v{meta.version}@{meta.commit}
              </Badge>
            )}
          </div>
        </div>
        <p className="text-muted-foreground text-sm max-w-xl">
          Enter a Shattered Pixel Dungeon seed to inspect generation data
          (layout, special-room loot, crystal rooms, quest rewards).
          Calculations run in Rust via WebAssembly. Analysis is still partial —
          not full game parity.
        </p>
      </header>

      <Card>
        <CardHeader>
          <CardTitle>Seed</CardTitle>
          <CardDescription>
            Accepts codes like <code className="font-mono">ABC-DEF-GHI</code>,
            numeric seeds, or free-text fun seeds.
          </CardDescription>
        </CardHeader>
        <CardContent>
          <form onSubmit={onAnalyze} className="flex flex-col gap-4">
            <div className="grid gap-2">
              <Label htmlFor="seed">Seed</Label>
              <Input
                id="seed"
                value={seed}
                onChange={(e) => setSeed(e.target.value)}
                placeholder="XXX-XXX-XXX"
                autoComplete="off"
                spellCheck={false}
                className="font-mono uppercase"
              />
            </div>

            <div className="space-y-3">
              <div className="flex items-start justify-between gap-4 rounded-lg border p-3">
                <div className="space-y-1">
                  <Label
                    htmlFor="identity-spoilers"
                    className="text-sm font-medium"
                  >
                    Show identities (spoilers)
                  </Label>
                  <p className="text-muted-foreground text-xs leading-relaxed">
                    Reveals potion, scroll, and ring color/rune/gem → type
                    mappings for this seed.
                  </p>
                </div>
                <Switch
                  id="identity-spoilers"
                  checked={identitySpoilers}
                  onCheckedChange={toggleIdentitySpoilers}
                />
              </div>

              <div className="flex items-start justify-between gap-4 rounded-lg border p-3">
                <div className="space-y-1">
                  <Label htmlFor="map-spoilers" className="text-sm font-medium">
                    Map spoilers
                  </Label>
                  <p className="text-muted-foreground text-xs leading-relaxed">
                    Shows full floor maps. Can heavily affect how you experience
                    a seeded run — leave off for item lists only.
                  </p>
                </div>
                <Switch
                  id="map-spoilers"
                  checked={mapSpoilers}
                  onCheckedChange={toggleMapSpoilers}
                />
              </div>
            </div>

            {mapSpoilers && (
              <Alert variant="destructive">
                <AlertTitle>Spoiler warning</AlertTitle>
                <AlertDescription>
                  Floor maps reveal layout, entrances, exits, and room shapes
                  before you play. Use only if you want that information.
                </AlertDescription>
              </Alert>
            )}

            <div>
              <Button type="submit" disabled={loading || !seed.trim()}>
                {loading ? <Loader2 className="animate-spin" /> : <Search />}
                Analyze
              </Button>
            </div>
          </form>
        </CardContent>
      </Card>

      {error && (
        <Alert variant="destructive">
          <AlertTitle>Error</AlertTitle>
          <AlertDescription>{error}</AlertDescription>
        </Alert>
      )}

      {report && (
        <>
          <Card>
            <CardHeader>
              <CardTitle className="font-mono">
                {report.seed.code ?? report.seed.formatted}
              </CardTitle>
              <CardDescription className="space-y-1">
                <span className="block">
                  Numeric:{' '}
                  <span className="font-mono text-foreground">
                    {report.seed.numeric}
                  </span>
                </span>
                <span className="block">
                  Status: <Badge variant="outline">{report.status}</Badge>
                </span>
              </CardDescription>
            </CardHeader>
            <CardContent className="space-y-3">
              {report.message && (
                <Alert>
                  <AlertTitle>Partial analysis</AlertTitle>
                  <AlertDescription>{report.message}</AlertDescription>
                </Alert>
              )}
              <p className="text-muted-foreground text-xs leading-relaxed">
                Includes approximate special-room, shop, and crystal-room prizes
                plus Ghost / Wandmaker / Blacksmith / Imp quest rewards. Painter
                parity, figure-eight builder, and full createMobs are still
                incomplete — treat high-value finds as leads, not guarantees.
              </p>
            </CardContent>
          </Card>

          {identitySpoilers && (
            <Card>
              <CardHeader>
                <CardTitle>Identities</CardTitle>
                <CardDescription>
                  Unidentified appearances for this seed (from run init RNG).
                </CardDescription>
              </CardHeader>
              <CardContent>
                <Tabs defaultValue="potions">
                  <TabsList className="w-full sm:w-auto">
                    <TabsTrigger value="potions">Potions</TabsTrigger>
                    <TabsTrigger value="scrolls">Scrolls</TabsTrigger>
                    <TabsTrigger value="rings">Rings</TabsTrigger>
                  </TabsList>
                  <TabsContent value="potions" className="mt-4">
                    <IdentityGrid
                      entries={report.identities.potions}
                      category="potion"
                    />
                  </TabsContent>
                  <TabsContent value="scrolls" className="mt-4">
                    <IdentityGrid
                      entries={report.identities.scrolls}
                      category="scroll"
                    />
                  </TabsContent>
                  <TabsContent value="rings" className="mt-4">
                    <IdentityGrid
                      entries={report.identities.rings}
                      category="ring"
                    />
                  </TabsContent>
                </Tabs>
              </CardContent>
            </Card>
          )}

          {report.floors.length > 0 && (
            <Card>
              <CardHeader>
                <CardTitle>Floors</CardTitle>
                <CardDescription>
                  Partial levelgen: layout, special/secret rooms, shops, crystal
                  rooms, and quest rewards when reported. Boss floors (5 / 10 /
                  15 / 20 / 25) and Last Level (26) are hidden. Floors with a
                  quest show a small indicator on the depth tab.
                  {mapSpoilers
                    ? ' Maps use original region tilesheets when available.'
                    : ' Enable Map spoilers to view floor maps.'}
                </CardDescription>
              </CardHeader>
              <CardContent>
                <FloorsByRegion
                  floors={report.floors}
                  identities={report.identities}
                  mapSpoilers={mapSpoilers}
                />
              </CardContent>
            </Card>
          )}
        </>
      )}
    </div>
  )
}
