import { FloorsSection } from '@/components/seed/FloorsSection'
import { IdentitiesPanel } from '@/components/seed/IdentitiesPanel'
import { SeedInfoPanel } from '@/components/seed/SeedInfoPanel'
import type { SeedReport } from '@/lib/spd-wasm'

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
      {identitySpoilers && <IdentitiesPanel identities={report.identities} />}
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
