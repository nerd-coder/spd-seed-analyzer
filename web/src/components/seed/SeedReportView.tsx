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
      {/* Seed info + optional Identities share a 50/50 top row. */}
      <div
        className={cn(
          'grid gap-4',
          identitySpoilers && 'lg:grid-cols-2 lg:items-start'
        )}
      >
        <SeedInfoPanel report={report} />
        {identitySpoilers && <IdentitiesPanel identities={report.identities} />}
      </div>

      {hasFloors && (
        <FloorsSection
          floors={report.floors}
          identities={report.identities}
          mapSpoilers={mapSpoilers}
        />
      )}
    </div>
  )
}
