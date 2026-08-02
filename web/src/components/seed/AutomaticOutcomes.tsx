import { FloorsSection } from '@/components/seed/FloorsSection'
import type { SeedReport } from '@/lib/spd-wasm'

export function AutomaticOutcomes({ report }: { report: SeedReport }) {
  return (
    <FloorsSection
      floors={report.floors}
      identities={report.identities}
      trinketSelection={report.trinket_selection}
    />
  )
}
