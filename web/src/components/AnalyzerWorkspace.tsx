import { useStore } from '@nanostores/react'
import { SpinnerGapIcon, XIcon } from '@phosphor-icons/react'
import { useRef } from 'react'
import { ScrollableSessionTabs } from '@/components/ScrollableSessionTabs'
import { SessionPane } from '@/components/seed/SessionPane'
import { Tabs, TabsContent, TabsTrigger } from '@/components/ui/tabs'
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
import { EmptyAnalysisPlaceholder } from './seed/EmptyAnalysisPlaceholder'

export function AnalyzerWorkspace() {
  const sessions = useStore($sessions)
  const activeId = useStore($activeSeedId)
  const mapSpoilers = useStore($mapSpoilers)
  const identitySpoilers = useStore($identitySpoilers)
  const seedTabsRef = useRef<HTMLDivElement>(null)
  useSeedTabsHeight(seedTabsRef, sessions.length > 0)

  if (sessions.length === 0) return <EmptyAnalysisPlaceholder />

  return (
    <Tabs
      value={activeId ?? sessions[0].id}
      onValueChange={setActiveSeed}
      className="gap-0 overflow-visible"
    >
      <ScrollableSessionTabs ref={seedTabsRef}>
        {sessions.map((session) => (
          <div
            key={session.id}
            className="group/seed-tab flex min-w-0 max-w-[14rem] shrink-0 items-center"
          >
            <TabsTrigger
              value={session.id}
              className="min-w-0 max-w-[12rem] gap-1 pr-1"
            >
              {session.status === 'loading' || session.status === 'pending' ? (
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
      </ScrollableSessionTabs>

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
