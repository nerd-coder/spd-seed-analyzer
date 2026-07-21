import { useStore } from '@nanostores/react'
import { Info, Loader2, Search, X } from 'lucide-react'
import { type FormEvent, useEffect, useLayoutEffect, useRef } from 'react'

import { DepthIcon } from '@/components/DepthIcon'
import { FloorMapPreview } from '@/components/FloorMapPreview'
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
import type {
  FloorReport,
  IdentityEntry,
  IdentityMaps,
  SeedReport,
} from '@/lib/spd-wasm'
import { cn } from '@/lib/utils'
import {
  $activeSeedId,
  $analyzing,
  $formError,
  $identitySpoilers,
  $mapSpoilers,
  $meta,
  $seedInput,
  $sessions,
  analyzeDraftSeed,
  closeSeedSession,
  loadSpdMeta,
  MAX_SAVED_SEEDS,
  normalizeSeedInput,
  type SeedSession,
  setActiveSeed,
  setIdentitySpoilers,
  setMapSpoilers,
  setSeedInput,
  startSessionRehydrate,
  tabLabel,
} from '@/stores/app'

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

/** Drop "Potion of " / "Scroll of " / "Ring of " — tab already names the category. */
function shortIdentityName(
  name: string,
  category: 'potion' | 'scroll' | 'ring'
): string {
  const prefix =
    category === 'potion'
      ? 'Potion of '
      : category === 'scroll'
        ? 'Scroll of '
        : 'Ring of '
  return name.startsWith(prefix) ? name.slice(prefix.length) : name
}

function IdentityGrid({
  entries,
  category,
}: {
  entries: IdentityEntry[]
  category: 'potion' | 'scroll' | 'ring'
}) {
  return (
    <div className="grid grid-cols-2 gap-x-3 gap-y-2">
      {entries.map((e) => {
        const shortName = shortIdentityName(e.name, category)
        return (
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
              <div className="truncate text-sm font-medium" title={e.name}>
                {shortName}
              </div>
              <div className="text-muted-foreground truncate text-xs capitalize">
                {appearanceDescription(category, e.appearance)}
              </div>
            </div>
          </div>
        )
      })}
    </div>
  )
}

function IdentitiesPanel({ identities }: { identities: IdentityMaps }) {
  // Sticky under seed tabs (same offset as region tabs). Not using Card
  // overflow-hidden so position:sticky on the outer wrapper stays valid.
  return (
    <section className="bg-card text-card-foreground ring-1 ring-foreground/10">
      <div className="space-y-1 px-4 pt-4">
        <h2 className="font-heading text-sm font-medium">Identities</h2>
        <p className="text-muted-foreground text-xs leading-relaxed">
          Unidentified appearances for this seed (from run init RNG).
        </p>
      </div>
      <div className="px-4 pt-3 pb-4">
        <Tabs defaultValue="potions">
          <TabsList className="h-auto w-full flex-wrap">
            <TabsTrigger value="potions">Potions</TabsTrigger>
            <TabsTrigger value="scrolls">Scrolls</TabsTrigger>
            <TabsTrigger value="rings">Rings</TabsTrigger>
          </TabsList>
          <TabsContent value="potions" className="mt-3">
            <IdentityGrid entries={identities.potions} category="potion" />
          </TabsContent>
          <TabsContent value="scrolls" className="mt-3">
            <IdentityGrid entries={identities.scrolls} category="scroll" />
          </TabsContent>
          <TabsContent value="rings" className="mt-3">
            <IdentityGrid entries={identities.rings} category="ring" />
          </TabsContent>
        </Tabs>
      </div>
    </section>
  )
}

function SeedInfoPanel({ report }: { report: SeedReport }) {
  return (
    <Card>
      <CardHeader>
        <CardTitle className="font-mono">
          {report.seed.code ?? report.seed.formatted}
        </CardTitle>
        <CardDescription>
          Numeric:{' '}
          <span className="text-foreground font-mono">
            {report.seed.numeric}
          </span>
        </CardDescription>
      </CardHeader>
      {report.message && (
        <CardContent>
          <p className="text-muted-foreground text-xs leading-relaxed">
            {report.message}
          </p>
        </CardContent>
      )}
    </Card>
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
  const hasQuest = (floor.quests?.length ?? 0) > 0
  const showMap = mapSpoilers && !!floor.map

  const details = (
    <div className="min-w-0 flex-1 space-y-3">
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
            <span className="ml-1.5 font-mono font-normal tabular-nums normal-case">
              ({floor.rooms.length})
            </span>
          </p>
          <p className="text-sm leading-relaxed">
            {floor.rooms.map((r) => r.replace(/Room$/, '')).join(' · ')}
          </p>
        </div>
      )}

      <div className="space-y-1">
        <p className="text-muted-foreground text-xs font-medium tracking-wide uppercase">
          Items
          <span className="ml-1.5 font-mono font-normal tabular-nums normal-case">
            ({floor.items.length})
          </span>
        </p>
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
    </div>
  )

  return (
    <section className="space-y-3 border-b py-6 first:pt-0 last:border-b-0 last:pb-0">
      <div className="flex flex-wrap items-center gap-2">
        <DepthIcon feeling={floor.feeling} size={20} />
        <span className="font-mono text-sm font-medium tabular-nums">
          Floor {floor.depth}
        </span>
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
        {hasQuest && (
          <Badge variant="default" className="text-xs">
            Quest
          </Badge>
        )}
        {showMap && floor.map && (
          <Badge variant="outline" className="font-mono text-xs">
            {floor.map.width}×{floor.map.height}
          </Badge>
        )}
      </div>

      <div className="flex items-start gap-3">
        {details}
        {showMap && floor.map && (
          <FloorMapPreview map={floor.map} depth={floor.depth} />
        )}
      </div>
    </section>
  )
}

function FloorsSection({
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

  // Not using Card: its default overflow-hidden kills position:sticky.
  // Region tabs stick under the seed session bar (--seed-tabs-height from App).
  return (
    <section className="bg-card text-card-foreground ring-1 ring-foreground/10">
      <div className="space-y-1 px-4 pt-4">
        <h2 className="font-heading text-sm font-medium">Floors</h2>
        <p className="text-muted-foreground text-xs leading-relaxed">
          Partial levelgen: layout, special/secret rooms, shops, crystal rooms,
          and quest rewards when reported. Boss floors (5 / 10 / 15 / 20 / 25)
          and Last Level (26) are hidden.
          {mapSpoilers
            ? ' Map thumbnails are 128×128 — click to expand.'
            : ' Enable Floor maps to view layout thumbnails.'}
        </p>
      </div>

      <Tabs
        defaultValue={defaultRegion}
        className="block gap-0 overflow-visible"
      >
        <div
          className={cn(
            'sticky z-10 border-b px-4 py-2',
            'bg-card/95 backdrop-blur supports-backdrop-filter:bg-card/90'
          )}
          style={{ top: 'var(--seed-tabs-height, 3rem)' }}
        >
          <TabsList className="h-auto w-full flex-wrap sm:w-auto">
            {groups.map(({ region, floors: regionFloors }) => {
              const lo = regionFloors[0].depth
              const hi = regionFloors[regionFloors.length - 1].depth
              return (
                <TabsTrigger key={region.id} value={region.id}>
                  {region.label}
                  <span className="text-muted-foreground ml-1 hidden text-xs tabular-nums sm:inline">
                    {lo}
                    {hi !== lo ? `–${hi}` : ''}
                  </span>
                </TabsTrigger>
              )
            })}
          </TabsList>
        </div>

        {groups.map(({ region, floors: regionFloors }) => (
          <TabsContent
            key={region.id}
            value={region.id}
            className="space-y-0 px-4 py-4"
          >
            {regionFloors.map((floor) => (
              <FloorDetail
                key={floor.depth}
                floor={floor}
                identities={identities}
                mapSpoilers={mapSpoilers}
              />
            ))}
          </TabsContent>
        ))}
      </Tabs>
    </section>
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
      <h2 className="font-heading text-base font-medium">
        No seeds analyzed yet
      </h2>
      <p className="text-muted-foreground max-w-sm text-sm leading-relaxed">
        Enter a seed in the left panel and click Analyze. Open seeds stay as
        tabs until you close them (max {MAX_SAVED_SEEDS}), and are restored
        after a refresh.
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
  const hasFloors = report.floors.length > 0

  return (
    <div className="space-y-4">
      <SeedInfoPanel report={report} />

      {/* Floors + optional Identities right column (w-80 matches main menu). */}
      <div
        className={cn(
          'flex flex-col gap-4',
          identitySpoilers && 'lg:flex-row lg:items-start'
        )}
      >
        {hasFloors && (
          <div className="min-w-0 flex-1">
            <FloorsSection
              floors={report.floors}
              identities={report.identities}
              mapSpoilers={mapSpoilers}
            />
          </div>
        )}

        {identitySpoilers && (
          <aside
            className={cn(
              // Same width as left main menu (`lg:w-80`).
              'w-full shrink-0 lg:w-80',
              // Stick under seed session tabs while floors scroll.
              'lg:sticky lg:self-start lg:max-h-[calc(100svh-var(--seed-tabs-height,3rem))] lg:overflow-y-auto',
              // Keep identities accessible when there are no floors.
              !hasFloors && 'flex-1'
            )}
            style={{ top: 'var(--seed-tabs-height, 3rem)' }}
          >
            <IdentitiesPanel identities={report.identities} />
          </aside>
        )}
      </div>
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

/**
 * Publish seed-tab bar height so region tabs stick beneath it.
 * Re-runs when `active` flips (sessions mount/unmount the bar).
 */
function useSeedTabsHeight(
  ref: { current: HTMLElement | null },
  active: boolean
) {
  useLayoutEffect(() => {
    if (!active) {
      document.documentElement.style.removeProperty('--seed-tabs-height')
      return
    }
    const el = ref.current
    if (!el) return

    const publish = () => {
      document.documentElement.style.setProperty(
        '--seed-tabs-height',
        `${el.offsetHeight}px`
      )
    }
    publish()
    const ro = new ResizeObserver(publish)
    ro.observe(el)
    return () => {
      ro.disconnect()
      document.documentElement.style.removeProperty('--seed-tabs-height')
    }
  }, [ref, active])
}

export default function App() {
  const seedInput = useStore($seedInput)
  const sessions = useStore($sessions)
  const activeId = useStore($activeSeedId)
  const analyzing = useStore($analyzing)
  const formError = useStore($formError)
  const meta = useStore($meta)
  const mapSpoilers = useStore($mapSpoilers)
  const identitySpoilers = useStore($identitySpoilers)
  const seedTabsRef = useRef<HTMLDivElement>(null)
  useSeedTabsHeight(seedTabsRef, sessions.length > 0)

  useEffect(() => {
    loadSpdMeta()
  }, [])

  useEffect(() => startSessionRehydrate(), [])

  async function onAnalyze(e: FormEvent) {
    e.preventDefault()
    await analyzeDraftSeed()
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
                  Partial seed analysis — layout, loot, and quest rewards (not
                  full game parity).
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
                  Codes, numeric seeds, or free-text fun seeds. Up to{' '}
                  {MAX_SAVED_SEEDS} open seeds are kept (oldest dropped).
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
                onCheckedChange={setIdentitySpoilers}
              />
              <SpoilerToggle
                id="map-spoilers"
                label="Floor maps"
                info="Shows 128×128 floor map thumbnails (click to expand). Heavily spoils layout before you play."
                checked={mapSpoilers}
                onCheckedChange={setMapSpoilers}
              />
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
              onValueChange={setActiveSeed}
              className="gap-0 overflow-visible"
            >
              <div
                ref={seedTabsRef}
                className="border-border bg-background/95 sticky top-0 z-20 border-b px-3 pt-3 pb-0 backdrop-blur supports-backdrop-filter:bg-background/80"
              >
                <TabsList
                  variant="line"
                  className="h-auto w-full flex-wrap justify-start gap-1"
                >
                  {sessions.map((s) => (
                    // Close control is a sibling of TabsTrigger (not nested —
                    // TabsTrigger renders as <button>).
                    <div
                      key={s.id}
                      className="group/seed-tab flex max-w-[14rem] items-center"
                    >
                      <TabsTrigger
                        value={s.id}
                        className="max-w-[12rem] gap-1 pr-1"
                      >
                        {(s.status === 'loading' || s.status === 'pending') && (
                          <Loader2 className="size-3 shrink-0 animate-spin" />
                        )}
                        <span className="truncate font-mono text-xs">
                          {tabLabel(s)}
                        </span>
                      </TabsTrigger>
                      <button
                        type="button"
                        className="text-muted-foreground hover:text-foreground hover:bg-muted inline-flex size-5 shrink-0 items-center justify-center rounded-none opacity-60 group-hover/seed-tab:opacity-100"
                        aria-label={`Close ${tabLabel(s)}`}
                        onClick={() => closeSeedSession(s.id)}
                      >
                        <X className="size-3" />
                      </button>
                    </div>
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
