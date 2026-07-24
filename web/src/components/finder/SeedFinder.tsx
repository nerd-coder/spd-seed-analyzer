import { useStore } from '@nanostores/react'
import {
  BinocularsIcon,
  SpinnerGapIcon,
  WarningIcon,
  XIcon,
} from '@phosphor-icons/react'
import { useRef } from 'react'
import { ScrollableSessionTabs } from '@/components/ScrollableSessionTabs'
import { Alert, AlertDescription, AlertTitle } from '@/components/ui/alert'
import { Tabs, TabsContent, TabsTrigger } from '@/components/ui/tabs'
import { WorkspaceEmptyPlaceholder } from '@/components/WorkspaceEmptyPlaceholder'
import { useSeedTabsHeight } from '@/hooks/useSeedTabsHeight'
import type { SeedSearchMatch } from '@/lib/spd-wasm'
import {
  $activeFinderId,
  $finderSessions,
  analyzeSeedInput,
  closeFinderSession,
  setActiveFinder,
} from '@/stores/app'
import { FinderResults } from './FinderResults'

const PARTIAL_MESSAGE =
  'The finder uses the partial analyzer. Generated loot is incomplete and results may not match the pinned game.'

export function SeedFinder({ onOpenAnalyze }: { onOpenAnalyze: () => void }) {
  const sessions = useStore($finderSessions)
  const activeId = useStore($activeFinderId)
  const tabsRef = useRef<HTMLDivElement>(null)
  useSeedTabsHeight(tabsRef, sessions.length > 0)

  function analyzeMatch(match: SeedSearchMatch) {
    const input = match.seed.code ?? String(match.seed.numeric)
    void analyzeSeedInput(input)
    onOpenAnalyze()
  }

  if (sessions.length === 0) {
    return (
      <WorkspaceEmptyPlaceholder
        icon={BinocularsIcon}
        title="No searches yet"
        description="Configure item constraints in the sidebar to start a search."
      />
    )
  }

  return (
    <Tabs
      value={activeId ?? sessions[0].id}
      onValueChange={setActiveFinder}
      className="gap-0 overflow-visible"
    >
      <ScrollableSessionTabs ref={tabsRef}>
        {sessions.map((session) => (
          <div
            key={session.id}
            className="group/finder-tab flex min-w-0 max-w-[14rem] shrink-0 items-center"
          >
            <TabsTrigger
              value={session.id}
              className="min-w-0 max-w-[12rem] gap-1 pr-1"
            >
              {session.run.status === 'running' ? (
                <SpinnerGapIcon className="shrink-0 animate-spin" />
              ) : null}
              <span className="truncate text-xs">{session.name}</span>
            </TabsTrigger>
            <button
              type="button"
              className="text-muted-foreground hover:text-foreground hover:bg-muted inline-flex size-5 shrink-0 items-center justify-center rounded-none opacity-60 group-hover/finder-tab:opacity-100"
              aria-label={`Close ${session.name}`}
              onClick={() => closeFinderSession(session.id)}
            >
              <XIcon className="size-3" />
            </button>
          </div>
        ))}
      </ScrollableSessionTabs>

      <div className="flex flex-col gap-4 p-4 md:p-6">
        {sessions.map((session) => (
          <TabsContent key={session.id} value={session.id} className="mt-0">
            <div className="flex flex-col gap-4">
              <Alert>
                <WarningIcon />
                <AlertTitle>Partial-accuracy search</AlertTitle>
                <AlertDescription>
                  {session.run.message ?? PARTIAL_MESSAGE}
                </AlertDescription>
              </Alert>
              <FinderResults run={session.run} onAnalyze={analyzeMatch} />
            </div>
          </TabsContent>
        ))}
      </div>
    </Tabs>
  )
}
