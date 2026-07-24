import { useStore } from '@nanostores/react'
import {
  BinocularsIcon,
  MagnifyingGlassIcon,
  PlantIcon,
  SpinnerGapIcon,
} from '@phosphor-icons/react'
import type { FormEvent } from 'react'
import { AppFloatingAction } from '@/components/AppFloatingAction'
import { FinderForm } from '@/components/finder/FinderForm'
import { AccuracyWarning } from '@/components/seed/AccuracyWarning'
import { Alert, AlertDescription, AlertTitle } from '@/components/ui/alert'
import { Button } from '@/components/ui/button'
import {
  Field,
  FieldDescription,
  FieldGroup,
  FieldLabel,
} from '@/components/ui/field'
import {
  InputGroup,
  InputGroupAddon,
  InputGroupInput,
} from '@/components/ui/input-group'
import { TabsList, TabsTrigger } from '@/components/ui/tabs'
import {
  $activeFinderSession,
  $analyzing,
  $formError,
  $seedInput,
  type AppMode,
  analyzeDraftSeed,
  cancelFinderSearch,
  MAX_SAVED_SEEDS,
  normalizeSeedInput,
  setSeedInput,
  startFinderSearch,
} from '@/stores/app'

export function AppSidebar({ mode }: { mode: AppMode }) {
  const seedInput = useStore($seedInput)
  const analyzing = useStore($analyzing)
  const formError = useStore($formError)
  const activeFinder = useStore($activeFinderSession)

  async function onAnalyze(event: FormEvent) {
    event.preventDefault()
    await analyzeDraftSeed()
  }

  return (
    <aside className="border-border text-sidebar-foreground lg:sticky lg:top-0 lg:max-h-svh lg:h-full lg:w-80 lg:shrink-0 lg:self-start lg:overflow-y-auto lg:border-r">
      <div
        className="relative w-full bg-black"
        style={{ aspectRatio: '616/200' }}
      >
        <img
          src="/assets/title.gif"
          alt="Shattered Pixel Dungeon"
          className="absolute inset-0 h-full w-full object-contain"
          style={{ imageRendering: 'pixelated' }}
        />
        <img
          src="/assets/title_overlay.png"
          alt="SEED Analyzer"
          className="absolute inset-0 h-full w-full object-contain"
          style={{ imageRendering: 'pixelated' }}
        />
        <AppFloatingAction />
      </div>
      <div className="flex flex-col gap-4 p-4 lg:h-full">
        <AccuracyWarning />

        <TabsList
          className="grid w-full grid-cols-2"
          aria-label="Analyzer mode"
        >
          <TabsTrigger value="analyze">
            <MagnifyingGlassIcon data-icon="inline-start" />
            Analyze
          </TabsTrigger>
          <TabsTrigger value="finder">
            <BinocularsIcon data-icon="inline-start" />
            Find
          </TabsTrigger>
        </TabsList>

        {mode === 'analyze' ? (
          <form onSubmit={onAnalyze}>
            <FieldGroup className="gap-2">
              <Field>
                <FieldLabel htmlFor="seed">Enter your seed</FieldLabel>
                <div className="flex w-full items-stretch">
                  <InputGroup className="min-w-0 flex-1 border-r-0">
                    <InputGroupAddon align="inline-start" aria-hidden>
                      <PlantIcon />
                    </InputGroupAddon>
                    <InputGroupInput
                      id="seed"
                      value={seedInput}
                      onChange={(event) => setSeedInput(event.target.value)}
                      placeholder="XXX-XXX-XXX"
                      autoComplete="off"
                      spellCheck={false}
                      className="font-mono uppercase"
                    />
                  </InputGroup>
                  <Button
                    type="submit"
                    size="default"
                    disabled={analyzing || !normalizeSeedInput(seedInput)}
                  >
                    {analyzing ? (
                      <SpinnerGapIcon
                        data-icon="inline-start"
                        className="animate-spin"
                      />
                    ) : (
                      <MagnifyingGlassIcon data-icon="inline-start" />
                    )}
                    Analyze
                  </Button>
                </div>
                <FieldDescription>
                  Codes, numeric seeds, or free-text fun seeds. Up to{' '}
                  {MAX_SAVED_SEEDS} open seeds are kept (oldest dropped).
                </FieldDescription>
              </Field>
            </FieldGroup>
          </form>
        ) : (
          <FinderForm
            running={activeFinder?.run.status === 'running'}
            cancelRequested={activeFinder?.run.cancelRequested ?? false}
            onSearch={(config) => void startFinderSearch(config)}
            onCancel={() => {
              if (activeFinder) cancelFinderSearch(activeFinder.id)
            }}
          />
        )}

        {mode === 'analyze' && formError ? (
          <Alert variant="destructive">
            <AlertTitle>Error</AlertTitle>
            <AlertDescription>{formError}</AlertDescription>
          </Alert>
        ) : null}
      </div>
    </aside>
  )
}
