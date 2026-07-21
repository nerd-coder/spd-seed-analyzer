import { useStore } from '@nanostores/react'
import { Info, Loader2, Search, X } from 'lucide-react'
import { type FormEvent, useEffect, useRef } from 'react'

import { EmptyAnalysisPlaceholder } from '@/components/seed/EmptyAnalysisPlaceholder'
import { SessionPane } from '@/components/seed/SessionPane'
import { SpoilerToggle } from '@/components/seed/SpoilerToggle'
import { ThemeToggle } from '@/components/ThemeToggle'
import { Alert, AlertDescription, AlertTitle } from '@/components/ui/alert'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import { Card, CardContent } from '@/components/ui/card'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { Separator } from '@/components/ui/separator'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from '@/components/ui/tooltip'
import { useSeedTabsHeight } from '@/hooks/useSeedTabsHeight'
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
  setActiveSeed,
  setIdentitySpoilers,
  setMapSpoilers,
  setSeedInput,
  startSessionRehydrate,
  tabLabel,
} from '@/stores/app'

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
            <Card size="sm" className="relative overflow-hidden py-0">
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
                {/* Mobile: theme control lives in the title panel */}
                <div className="absolute top-2 right-2 z-10 lg:hidden">
                  <ThemeToggle className="border-white/20 bg-black/55 text-white hover:bg-black/70 hover:text-white" />
                </div>
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
        <main className="relative min-w-0 flex-1">
          {/* Desktop: theme control at top-right of content panel */}
          {sessions.length === 0 && (
            <div className="absolute top-3 right-3 z-30 hidden lg:block">
              <ThemeToggle />
            </div>
          )}
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
                className="border-border bg-background/95 sticky top-0 z-20 flex items-start gap-2 border-b px-3 pt-3 pb-0 backdrop-blur supports-backdrop-filter:bg-background/80"
              >
                <TabsList
                  variant="line"
                  className="h-auto min-w-0 flex-1 flex-wrap justify-start gap-1"
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
                <ThemeToggle className="mt-0.5 mb-1.5 hidden shrink-0 lg:inline-flex" />
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
