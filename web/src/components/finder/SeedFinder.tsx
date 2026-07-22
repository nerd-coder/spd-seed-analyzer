import { WarningIcon } from '@phosphor-icons/react'
import { Alert, AlertDescription, AlertTitle } from '@/components/ui/alert'
import type { SeedSearchMatch } from '@/lib/spd-wasm'
import { analyzeSeedInput } from '@/stores/app'
import { FinderForm } from './FinderForm'
import { FinderResults } from './FinderResults'
import { useSeedFinder } from './useSeedFinder'

const PARTIAL_MESSAGE =
  'The finder uses the partial analyzer. Generated loot is incomplete and results may not match the pinned game.'

export function SeedFinder({ onOpenAnalyze }: { onOpenAnalyze: () => void }) {
  const { run, start, cancel } = useSeedFinder()

  function analyzeMatch(match: SeedSearchMatch) {
    const input = match.seed.code ?? String(match.seed.numeric)
    void analyzeSeedInput(input)
    onOpenAnalyze()
  }

  return (
    <div className="flex flex-col gap-4 p-4 pt-0 md:p-6 md:pt-0">
      <div className="sticky top-0 z-20 -mx-4 md:-mx-6 bg-background/95 border-b px-4 py-2 md:px-6 md:py-4 backdrop-blur supports-backdrop-filter:bg-background/80">
        <div className="flex items-start justify-between gap-4">
          <div className="flex flex-col gap-1">
            <h1 className="font-heading text-base font-medium">Seed finder</h1>
            <p className="text-muted-foreground max-w-2xl text-xs/relaxed">
              Search a deliberate numeric range for exact generated items, then
              open promising seeds in the analyzer.
            </p>
          </div>
        </div>

        <div className="absolute bottom-0 left-4 right-4 h-4 bg-linear-to-t from-background/95 to-transparent pointer-events-none md:left-6 md:right-6" />
      </div>

      <Alert>
        <WarningIcon />
        <AlertTitle>Partial-accuracy search</AlertTitle>
        <AlertDescription>{run.message ?? PARTIAL_MESSAGE}</AlertDescription>
      </Alert>

      <FinderForm
        running={run.status === 'running'}
        cancelRequested={run.cancelRequested}
        onSearch={(config) => void start(config)}
        onCancel={cancel}
      />
      <FinderResults run={run} onAnalyze={analyzeMatch} />
    </div>
  )
}
