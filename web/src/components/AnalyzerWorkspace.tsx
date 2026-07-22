import { useStore } from '@nanostores/react'
import { SpinnerGapIcon, XIcon } from '@phosphor-icons/react'
import { useRef } from 'react'
import { SettingsButton } from '@/components/SettingsButton'
import { EmptyAnalysisPlaceholder } from '@/components/seed/EmptyAnalysisPlaceholder'
import { SessionPane } from '@/components/seed/SessionPane'
import { ThemeToggle } from '@/components/ThemeToggle'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { useSeedTabsHeight } from '@/hooks/useSeedTabsHeight'
import {
  $activeSeedId,
  $identitySpoilers,
  $mapSpoilers,
  $sessions,
  closeSeedSession,
  setActiveSeed,
  tabLabel,
} from '@/stores/app'

export function AnalyzerWorkspace() {
  const sessions = useStore($sessions)
  const activeId = useStore($activeSeedId)
  const mapSpoilers = useStore($mapSpoilers)
  const identitySpoilers = useStore($identitySpoilers)
  const seedTabsRef = useRef<HTMLDivElement>(null)
  useSeedTabsHeight(seedTabsRef, sessions.length > 0)

  if (sessions.length === 0) {
    return (
      <>
        <div className="absolute top-3 right-3 z-30 hidden items-center gap-1.5 lg:flex">
          <SettingsButton />
          <ThemeToggle />
        </div>
        <EmptyAnalysisPlaceholder />
      </>
    )
  }

  return (
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
          {sessions.map((session) => (
            <div
              key={session.id}
              className="group/seed-tab flex max-w-[14rem] items-center"
            >
              <TabsTrigger
                value={session.id}
                className="max-w-[12rem] gap-1 pr-1"
              >
                {session.status === 'loading' ||
                session.status === 'pending' ? (
                  <SpinnerGapIcon className="shrink-0 animate-spin" />
                ) : null}
                <span className="truncate font-mono text-xs">
                  {tabLabel(session)}
                </span>
              </TabsTrigger>
              <button
                type="button"
                className="text-muted-foreground hover:text-foreground hover:bg-muted inline-flex size-5 shrink-0 items-center justify-center rounded-none opacity-60 group-hover/seed-tab:opacity-100"
                aria-label={`Close ${tabLabel(session)}`}
                onClick={() => closeSeedSession(session.id)}
              >
                <XIcon className="size-3" />
              </button>
            </div>
          ))}
        </TabsList>
        <div className="mt-0.5 mb-1.5 hidden shrink-0 items-center gap-1.5 lg:flex">
          <SettingsButton />
          <ThemeToggle />
        </div>
      </div>

      <div className="flex flex-col gap-4 p-4 md:p-6">
        {sessions.map((session) => (
          <TabsContent key={session.id} value={session.id} className="mt-0">
            <SessionPane
              session={session}
              identitySpoilers={identitySpoilers}
              mapSpoilers={mapSpoilers}
            />
          </TabsContent>
        ))}
      </div>
    </Tabs>
  )
}
