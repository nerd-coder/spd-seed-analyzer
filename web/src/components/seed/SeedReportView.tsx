import { FloorsSection } from '@/components/seed/FloorsSection'
import { IdentitiesPanel } from '@/components/seed/IdentitiesPanel'
import { SeedInfoPanel } from '@/components/seed/SeedInfoPanel'
import type { MapProfile, SeedReport } from '@/lib/spd-wasm'

export function SeedReportView({
  report,
  sessionId,
  mapProfile,
  identitySpoilers,
  mapSpoilers,
}: {
  report: SeedReport
  sessionId: string
  mapProfile: MapProfile | undefined
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
          sessionId={sessionId}
          mapProfile={mapProfile}
          identities={report.identities}
          identitySpoilers={identitySpoilers}
          mapSpoilers={mapSpoilers}
        />
      )}
    </div>
  )
}
