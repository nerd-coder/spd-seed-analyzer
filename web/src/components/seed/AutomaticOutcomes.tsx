import { FloorsSection } from '@/components/seed/FloorsSection'
import type { TrinketChoice } from '@/components/seed/trinket-offers'
import type { SeedReport } from '@/lib/spd-wasm'

export function AutomaticOutcomes({
  report,
  trinketChoice,
}: {
  report: SeedReport
  trinketChoice: TrinketChoice
}) {
  return (
    <FloorsSection
      floors={report.floors}
      identities={report.identities}
      trinketSelection={report.trinket_selection}
      trinketChoice={trinketChoice}
    />
  )
}
