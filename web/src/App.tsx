import { Info, Loader2, Search, X } from 'lucide-react'
import { type FormEvent, useCallback, useEffect, useRef, useState } from 'react'

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
import { Separator } from '@/components/ui/separator'
import { Switch } from '@/components/ui/switch'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from '@/components/ui/tooltip'
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
/** Persisted open seed inputs (re-analyzed on load). */
const SESSIONS_KEY = 'spd-analyzer-open-seeds'
const ACTIVE_SEED_KEY = 'spd-analyzer-active-seed'

/** Full main-path depth range (SPD clamps to 26). */
const ANALYZE_FLOORS = 26
/** Delay between sequential re-analyzes after refresh (ms). */
const REANALYZE_GAP_MS = 350

type SeedSession = {
  /** Stable tab id (input key). */
  id: string
  /** Original user input used for analyze. */
  input: string
  status: 'pending' | 'loading' | 'ready' | 'error'
  report: SeedReport | null
  error: string | null
}

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

function normalizeSeedInput(raw: string): string {
  return raw.trim()
}

function sessionIdFor(input: string): string {
  return normalizeSeedInput(input).toUpperCase()
}

function loadPersistedSeeds(): string[] {
  try {
    const raw = localStorage.getItem(SESSIONS_KEY)
    if (!raw) return []
    const parsed = JSON.parse(raw) as unknown
    if (!Array.isArray(parsed)) return []
    return parsed
      .filter((s): s is string => typeof s === 'string')
      .map(normalizeSeedInput)
      .filter(Boolean)
  } catch {
    return []
  }
}

function savePersistedSeeds(inputs: string[]) {
  try {
    localStorage.setItem(SESSIONS_KEY, JSON.stringify(inputs))
  } catch {
    /* ignore */
  }
}

function loadActiveSeedId(): string | null {
  try {
    return localStorage.getItem(ACTIVE_SEED_KEY)
  } catch {
    return null
  }
}

function saveActiveSeedId(id: string | null) {
  try {
    if (id) localStorage.setItem(ACTIVE_SEED_KEY, id)
    else localStorage.removeItem(ACTIVE_SEED_KEY)
  } catch {
    /* ignore */
  }
}

function sleep(ms: number) {
  return new Promise<void>((resolve) => {
    setTimeout(resolve, ms)
  })
}

function tabLabel(session: SeedSession): string {
  if (session.report) {
    return session.report.seed.code ?? session.report.seed.formatted
  }
  return session.input
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
      className={cn(
        'space-y-1.5 rounded-none border px-3 py-2.5',
        styles.border
      )}
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
          <span className="text-muted-foreground mr-1.5 text-xs font-medium tracking-wide uppercase">
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
                <span className="flex min-w-0 flex-wrap items-baseline gap-x-1.5 gap-y-0.5">
                  <span>{item.name}</span>
                  {sourceLabel && (
                    <Badge
                      variant={highlight ? 'secondary' : 'outline'}
                      className="h-5 px-1.5 py-0 text-[10px] font-normal"
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
      <TabsList className="h-auto w-full flex-wrap sm:w-auto">
        {groups.map(({ region, floors: regionFloors }) => (
          <TabsTrigger key={region.id} value={region.id}>
            {region.label}
            <span className="text-muted-foreground ml-1 text-xs tabular-nums">
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
                      className="relative h-auto flex-col gap-0.5 px-2 py-1.5"
                      title={titleParts.join(' · ')}
                    >
                      <DepthIcon feeling={floor.feeling} size={24} />
                      <span className="font-mono text-xs leading-none tabular-nums">
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

function SpoilerToggle({
  id,
  label,
  info,
  checked,
  onCheckedChange,
}: {
  id: string
  label: string
  info: string
  checked: boolean
  onCheckedChange: (next: boolean) => void
}) {
  return (
    <div className="flex items-center justify-between gap-3">
      <div className="flex min-w-0 items-center gap-1.5">
        <Label htmlFor={id} className="text-sm font-medium">
          {label}
        </Label>
        <Tooltip>
          <TooltipTrigger asChild>
            <button
              type="button"
              className="text-muted-foreground hover:text-foreground inline-flex size-5 shrink-0 items-center justify-center rounded-none outline-none focus-visible:ring-1 focus-visible:ring-ring"
              aria-label={`About ${label}`}
            >
              <Info className="size-3.5" />
            </button>
          </TooltipTrigger>
          <TooltipContent side="right" className="max-w-56 text-left">
            {info}
          </TooltipContent>
        </Tooltip>
      </div>
      <Switch id={id} checked={checked} onCheckedChange={onCheckedChange} />
    </div>
  )
}

function EmptyAnalysisPlaceholder() {
  return (
    <div className="flex min-h-[min(60svh,28rem)] flex-col items-center justify-center gap-3 px-6 text-center">
      <div
        className="relative h-14 w-full max-w-xs opacity-40"
        style={{ aspectRatio: '616/200' }}
      >
        <img
          src="/assets/title.gif"
          alt=""
          className="absolute inset-0 h-full w-full object-contain"
          style={{ imageRendering: 'pixelated' }}
          aria-hidden
        />
        <img
          src="/assets/title_overlay.png"
          alt=""
          className="absolute inset-0 h-full w-full object-contain"
          style={{ imageRendering: 'pixelated' }}
          aria-hidden
        />
      </div>
      <h2 className="font-heading text-base font-medium">
        No seeds analyzed yet
      </h2>
      <p className="text-muted-foreground max-w-sm text-sm leading-relaxed">
        Enter a seed in the left panel and click Analyze. Open seeds stay as
        tabs until you close them, and are restored after a refresh.
      </p>
    </div>
  )
}

function SeedReportView({
  report,
  identitySpoilers,
  mapSpoilers,
}: {
  report: SeedReport
  identitySpoilers: boolean
  mapSpoilers: boolean
}) {
  return (
    <div className="space-y-4">
      <Card>
        <CardHeader>
          <CardTitle className="font-mono">
            {report.seed.code ?? report.seed.formatted}
          </CardTitle>
          <CardDescription className="space-y-1">
            <span className="block">
              Numeric:{' '}
              <span className="text-foreground font-mono">
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
              rooms, and quest rewards when reported. Boss floors (5 / 10 / 15 /
              20 / 25) and Last Level (26) are hidden. Floors with a quest show
              a small indicator on the depth tab.
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
    </div>
  )
}

function SessionPane({
  session,
  identitySpoilers,
  mapSpoilers,
}: {
  session: SeedSession
  identitySpoilers: boolean
  mapSpoilers: boolean
}) {
  if (session.status === 'pending' || session.status === 'loading') {
    return (
      <div className="flex min-h-[12rem] flex-col items-center justify-center gap-2 py-12">
        <Loader2 className="text-muted-foreground size-6 animate-spin" />
        <p className="text-muted-foreground text-sm">
          Analyzing{' '}
          <span className="text-foreground font-mono">{session.input}</span>…
        </p>
      </div>
    )
  }

  if (session.status === 'error') {
    return (
      <Alert variant="destructive">
        <AlertTitle>Analysis failed</AlertTitle>
        <AlertDescription>
          {session.error ?? 'Unknown error analyzing this seed.'}
        </AlertDescription>
      </Alert>
    )
  }

  if (session.report) {
    return (
      <SeedReportView
        report={session.report}
        identitySpoilers={identitySpoilers}
        mapSpoilers={mapSpoilers}
      />
    )
  }

  return null
}

export default function App() {
  const [seedInput, setSeedInput] = useState('')
  const [sessions, setSessions] = useState<SeedSession[]>([])
  const [activeId, setActiveId] = useState<string | null>(null)
  const [analyzing, setAnalyzing] = useState(false)
  const [formError, setFormError] = useState<string | null>(null)
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

  const sessionsRef = useRef(sessions)
  sessionsRef.current = sessions
  const rehydrateDone = useRef(false)

  const persistSessions = useCallback((list: SeedSession[]) => {
    savePersistedSeeds(list.map((s) => s.input))
  }, [])

  const setActive = useCallback((id: string | null) => {
    setActiveId(id)
    saveActiveSeedId(id)
  }, [])

  const updateSession = useCallback(
    (id: string, patch: Partial<SeedSession>) => {
      setSessions((prev) => {
        const next = prev.map((s) => (s.id === id ? { ...s, ...patch } : s))
        return next
      })
    },
    []
  )

  const runAnalyze = useCallback(
    async (id: string, input: string) => {
      updateSession(id, { status: 'loading', error: null })
      try {
        const report = await analyzeSeed(input, ANALYZE_FLOORS)
        updateSession(id, { status: 'ready', report, error: null })
        return true
      } catch (err) {
        updateSession(id, {
          status: 'error',
          report: null,
          error: err instanceof Error ? err.message : String(err),
        })
        return false
      }
    },
    [updateSession]
  )

  // Meta + restore open seeds and re-analyze them slowly.
  useEffect(() => {
    getSpdMeta()
      .then(setMeta)
      .catch((e: unknown) => {
        setFormError(e instanceof Error ? e.message : String(e))
      })

    if (rehydrateDone.current) return
    rehydrateDone.current = true

    const saved = loadPersistedSeeds()
    if (saved.length === 0) return

    const restored: SeedSession[] = saved.map((input) => ({
      id: sessionIdFor(input),
      input,
      status: 'pending' as const,
      report: null,
      error: null,
    }))
    setSessions(restored)

    const preferred = loadActiveSeedId()
    const active =
      preferred && restored.some((s) => s.id === preferred)
        ? preferred
        : (restored[0]?.id ?? null)
    setActive(active)

    let cancelled = false
    ;(async () => {
      setAnalyzing(true)
      for (let i = 0; i < restored.length; i++) {
        if (cancelled) break
        const s = restored[i]
        // Skip if user already closed the tab during rehydrate.
        if (!sessionsRef.current.some((x) => x.id === s.id)) continue
        await runAnalyze(s.id, s.input)
        if (i < restored.length - 1 && !cancelled) {
          await sleep(REANALYZE_GAP_MS)
        }
      }
      if (!cancelled) setAnalyzing(false)
    })()

    return () => {
      cancelled = true
    }
  }, [runAnalyze, setActive])

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
    const input = normalizeSeedInput(seedInput)
    if (!input) return

    setFormError(null)
    const id = sessionIdFor(input)

    const existing = sessions.find((s) => s.id === id)
    if (existing) {
      setActive(id)
      if (existing.status === 'ready' || existing.status === 'loading') {
        return
      }
      setAnalyzing(true)
      await runAnalyze(id, existing.input)
      setAnalyzing(false)
      return
    }

    const session: SeedSession = {
      id,
      input,
      status: 'loading',
      report: null,
      error: null,
    }
    setSessions((prev) => {
      const next = [...prev, session]
      persistSessions(next)
      return next
    })
    setActive(id)
    setSeedInput('')
    setAnalyzing(true)
    await runAnalyze(id, input)
    setAnalyzing(false)
  }

  function closeSession(id: string) {
    setSessions((prev) => {
      const idx = prev.findIndex((s) => s.id === id)
      if (idx < 0) return prev
      const next = prev.filter((s) => s.id !== id)
      persistSessions(next)
      if (activeId === id) {
        const fallback = next[idx] ?? next[idx - 1] ?? next[0] ?? null
        setActive(fallback?.id ?? null)
      }
      return next
    })
  }

  return (
    <TooltipProvider delayDuration={200}>
      <div className="bg-background flex min-h-svh w-full flex-col lg:flex-row">
        {/* —— Main menu (sticky) —— */}
        <aside className="border-border bg-sidebar text-sidebar-foreground lg:sticky lg:top-0 lg:h-svh lg:w-80 lg:shrink-0 lg:overflow-y-auto lg:border-r">
          <div className="flex flex-col gap-4 p-4">
            <Card size="sm" className="overflow-hidden py-0">
              <div
                className="relative w-full bg-black"
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
              <CardContent className="space-y-1 py-3">
                <p className="text-muted-foreground text-xs leading-relaxed">
                  Partial seed analysis via Rust WASM — layout, loot, and quest
                  rewards (not full game parity).
                </p>
                {meta && (
                  <Badge variant="secondary" className="font-mono text-[10px]">
                    v{meta.version}@{meta.commit}
                  </Badge>
                )}
              </CardContent>
            </Card>

            <form onSubmit={onAnalyze} className="space-y-3">
              <div className="grid gap-1.5">
                <Label htmlFor="seed">Seed</Label>
                <Input
                  id="seed"
                  value={seedInput}
                  onChange={(e) => setSeedInput(e.target.value)}
                  placeholder="XXX-XXX-XXX"
                  autoComplete="off"
                  spellCheck={false}
                  className="font-mono uppercase"
                />
                <p className="text-muted-foreground text-[11px] leading-snug">
                  Codes, numeric seeds, or free-text fun seeds.
                </p>
              </div>
              <Button
                type="submit"
                className="w-full"
                disabled={analyzing || !normalizeSeedInput(seedInput)}
              >
                {analyzing ? (
                  <Loader2 className="animate-spin" />
                ) : (
                  <Search data-icon="inline-start" />
                )}
                Analyze
              </Button>
            </form>

            {formError && (
              <Alert variant="destructive">
                <AlertTitle>Error</AlertTitle>
                <AlertDescription>{formError}</AlertDescription>
              </Alert>
            )}

            <Separator />

            <div className="space-y-3">
              <div className="flex items-center gap-1.5">
                <p className="text-xs font-medium tracking-wide uppercase">
                  Spoilers
                </p>
                <Tooltip>
                  <TooltipTrigger asChild>
                    <button
                      type="button"
                      className="text-muted-foreground hover:text-foreground inline-flex size-5 items-center justify-center outline-none focus-visible:ring-1 focus-visible:ring-ring"
                      aria-label="About spoilers"
                    >
                      <Info className="size-3.5" />
                    </button>
                  </TooltipTrigger>
                  <TooltipContent side="right" className="max-w-56 text-left">
                    These options reveal seed secrets. Leave them off if you
                    want to keep exploration surprises.
                  </TooltipContent>
                </Tooltip>
              </div>

              <SpoilerToggle
                id="identity-spoilers"
                label="Identities"
                info="Reveals potion, scroll, and ring color/rune/gem → type mappings for the active seed."
                checked={identitySpoilers}
                onCheckedChange={toggleIdentitySpoilers}
              />
              <SpoilerToggle
                id="map-spoilers"
                label="Floor maps"
                info="Shows full floor minimaps with original region tilesheets. Heavily spoils layout before you play."
                checked={mapSpoilers}
                onCheckedChange={toggleMapSpoilers}
              />

              {mapSpoilers && (
                <Alert variant="destructive">
                  <AlertTitle>Map spoilers on</AlertTitle>
                  <AlertDescription>
                    Floor maps reveal layout, entrances, exits, and room shapes.
                  </AlertDescription>
                </Alert>
              )}
            </div>
          </div>
        </aside>

        {/* —— Content panel —— */}
        <main className="min-w-0 flex-1">
          {sessions.length === 0 ? (
            <EmptyAnalysisPlaceholder />
          ) : (
            <Tabs
              value={activeId ?? sessions[0].id}
              onValueChange={setActive}
              className="gap-0"
            >
              <div className="border-border bg-background/95 sticky top-0 z-10 border-b px-3 pt-3 pb-0 backdrop-blur supports-backdrop-filter:bg-background/80">
                <TabsList
                  variant="line"
                  className="h-auto w-full flex-wrap justify-start gap-1"
                >
                  {sessions.map((s) => (
                    <TabsTrigger
                      key={s.id}
                      value={s.id}
                      className="group/tab max-w-[12rem] gap-1 pr-1"
                    >
                      {(s.status === 'loading' || s.status === 'pending') && (
                        <Loader2 className="size-3 shrink-0 animate-spin" />
                      )}
                      <span className="truncate font-mono text-xs">
                        {tabLabel(s)}
                      </span>
                      <button
                        type="button"
                        className="text-muted-foreground hover:text-foreground hover:bg-muted inline-flex size-5 shrink-0 items-center justify-center rounded-none opacity-60 group-hover/tab:opacity-100 group-data-active/tabs-trigger:opacity-100"
                        aria-label={`Close ${tabLabel(s)}`}
                        onClick={(e) => {
                          e.preventDefault()
                          e.stopPropagation()
                          closeSession(s.id)
                        }}
                      >
                        <X className="size-3" />
                      </button>
                    </TabsTrigger>
                  ))}
                </TabsList>
              </div>

              <div className="space-y-4 p-4 md:p-6">
                {sessions.map((s) => (
                  <TabsContent key={s.id} value={s.id} className="mt-0">
                    <SessionPane
                      session={s}
                      identitySpoilers={identitySpoilers}
                      mapSpoilers={mapSpoilers}
                    />
                  </TabsContent>
                ))}
              </div>
            </Tabs>
          )}
        </main>
      </div>
    </TooltipProvider>
  )
}
