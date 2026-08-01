import { AutomaticOutcomes } from '@/components/seed/AutomaticOutcomes'
import { IdentitiesPanel } from '@/components/seed/IdentitiesPanel'
import { SeedInfoPanel } from '@/components/seed/SeedInfoPanel'
import type { SeedReport } from '@/lib/spd-wasm'

export function SeedReportView({ report }: { report: SeedReport }) {
  return (
    <div className="space-y-4">
      <SeedInfoPanel report={report} />
      <IdentitiesPanel identities={report.identities} />
      <AutomaticOutcomes report={report} />
    </div>
  )
}
