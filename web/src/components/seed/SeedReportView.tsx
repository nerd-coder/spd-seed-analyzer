import { FloorsSection } from '@/components/seed/FloorsSection'
import { IdentitiesPanel } from '@/components/seed/IdentitiesPanel'
import { SeedInfoPanel } from '@/components/seed/SeedInfoPanel'
import type { SeedReport } from '@/lib/spd-wasm'

export function SeedReportView({
  report,
  refreshingLayout,
  identitySpoilers,
  mapSpoilers,
}: {
  report: SeedReport
  refreshingLayout: boolean
  identitySpoilers: boolean
  mapSpoilers: boolean
}) {
  const hasFloors = report.floors.length > 0

  return (
    <div className="space-y-4">
      <SeedInfoPanel report={report} />
      {identitySpoilers && <IdentitiesPanel identities={report.identities} />}
      {hasFloors && (
        <FloorsSection
          floors={report.floors}
          refreshingLayout={refreshingLayout}
          identities={report.identities}
          identitySpoilers={identitySpoilers}
          mapSpoilers={mapSpoilers}
        />
      )}
    </div>
  )
}
