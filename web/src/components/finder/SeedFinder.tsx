import { WarningIcon } from '@phosphor-icons/react'
import { SettingsButton } from '@/components/SettingsButton'
import { ThemeToggle } from '@/components/ThemeToggle'
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
    <div className="flex flex-col gap-4 p-4 md:p-6">
      <div className="flex items-start justify-between gap-4">
        <div className="flex flex-col gap-1">
          <h1 className="font-heading text-base font-medium">Seed finder</h1>
          <p className="text-muted-foreground max-w-2xl text-xs/relaxed">
            Search a deliberate numeric range for exact generated items, then
            open promising seeds in the analyzer.
          </p>
        </div>
        <div className="hidden shrink-0 items-center gap-1.5 lg:flex">
          <SettingsButton />
          <ThemeToggle />
        </div>
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
