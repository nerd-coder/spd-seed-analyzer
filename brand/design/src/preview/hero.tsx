import { useStore } from '@tanstack/react-store'
import { DescentRail } from '@/components/spd/descent-rail'
import { FloorReport } from '@/components/spd/floor-report'
import { $floor, selectFloor } from '@/lib/preview-store'
import { SAMPLE_FLOORS, SAMPLE_SEED, sampleFloor } from '@/lib/sample-seed'

/**
 * The system in use before it is explained: a seed, the run as a strip of
 * floors, and the floor you pick.
 */
export function Hero() {
  const depth = useStore($floor)

  return (
    <section className="mx-auto flex max-w-6xl flex-col gap-10 px-5 pt-12 pb-20 lg:px-8 lg:pt-16">
      <div className="flex flex-col gap-4">
        <h1 className="font-pixel text-px-2 sm:text-px-3 lg:text-px-4">
          <span className="sr-only">
            SPD Seed Analyzer design system, sample seed{' '}
          </span>
          {SAMPLE_SEED}
        </h1>
        <p className="max-w-[60ch] text-lg text-muted-foreground">
          The SPD Seed Analyzer design system, shown on a sample seed. Each
          floor sits in its region's color. Pick a floor to see what it holds.
        </p>
      </div>
      <DescentRail
        floors={SAMPLE_FLOORS}
        selected={depth}
        onSelect={selectFloor}
      />
      <div className="overflow-hidden rounded-lg border bg-card">
        <FloorReport floor={sampleFloor(depth)} />
      </div>
      <p className="-mt-6 text-sm text-muted-foreground">
        Sample data. The preview does not run the analyzer.
      </p>
    </section>
  )
}
