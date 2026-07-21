import { FloorsSection } from '@/components/seed/FloorsSection'
import { IdentitiesPanel } from '@/components/seed/IdentitiesPanel'
import { SeedInfoPanel } from '@/components/seed/SeedInfoPanel'
import type { SeedReport } from '@/lib/spd-wasm'
import { cn } from '@/lib/utils'

export function SeedReportView({
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
