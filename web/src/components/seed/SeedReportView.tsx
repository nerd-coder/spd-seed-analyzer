import { useRef } from 'react'
import { AutomaticOutcomes } from '@/components/seed/AutomaticOutcomes'
import { DescentRail } from '@/components/seed/DescentRail'
import { IdentitiesPanel } from '@/components/seed/IdentitiesPanel'
import type { TrinketChoice } from '@/components/seed/trinket-offers'
import { stuckBottom, useCurrentFloor } from '@/hooks/useCurrentFloor'
import type { SeedReport } from '@/lib/spd-wasm'

export function SeedReportView({
  report,
  trinketChoice,
}: {
  report: SeedReport
  trinketChoice: TrinketChoice
}) {
  const containerRef = useRef<HTMLDivElement>(null)
  const railRef = useRef<HTMLElement>(null)
  const current = useCurrentFloor(containerRef, railRef)
  const floors = [...report.floors].sort((a, b) => a.depth - b.depth)

  function scrollToFloor(depth: number) {
    const section = containerRef.current?.querySelector<HTMLElement>(
      `[data-floor-depth="${depth}"]`
    )
    if (!section) return
    const top =
      window.scrollY +
      section.getBoundingClientRect().top -
      stuckBottom(railRef.current)
    const reduce = window.matchMedia('(prefers-reduced-motion: reduce)').matches
    window.scrollTo({ top, behavior: reduce ? 'auto' : 'smooth' })
  }

  // The strip is a sticky child of this wrapper, so it stays for the whole report.
  return (
    <div ref={containerRef} className="space-y-4">
      {floors.length > 0 ? (
        <DescentRail
          ref={railRef}
          floors={floors}
          current={current}
          onSelect={scrollToFloor}
        />
      ) : null}
      <IdentitiesPanel identities={report.identities} />
      <AutomaticOutcomes report={report} trinketChoice={trinketChoice} />
    </div>
  )
}
