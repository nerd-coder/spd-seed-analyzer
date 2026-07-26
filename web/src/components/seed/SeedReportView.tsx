import { FloorsSection } from '@/components/seed/FloorsSection'
import { IdentitiesPanel } from '@/components/seed/IdentitiesPanel'
import { SeedInfoPanel } from '@/components/seed/SeedInfoPanel'
import type { MapProfile, SeedReport } from '@/lib/spd-wasm'

export function SeedReportView({
  report,
  sessionId,
  mapProfile,
  refreshingLayout,
  identitySpoilers,
  mapSpoilers,
}: {
  report: SeedReport
  sessionId: string
  mapProfile: MapProfile
  refreshingLayout: boolean
  identitySpoilers: boolean
  mapSpoilers: boolean
}) {
  const hasFloors = report.floors.length > 0

  return (
    <div className="space-y-4">
      <SeedInfoPanel
        report={report}
        sessionId={sessionId}
        mapProfile={mapProfile}
        refreshingLayout={refreshingLayout}
        identitySpoilers={identitySpoilers}
        mapSpoilers={mapSpoilers}
      />
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
