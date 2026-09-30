import {
  BinocularsIcon,
  MagnifyingGlassIcon,
  PlantIcon,
  SpinnerGapIcon,
} from '@phosphor-icons/react'
import { useStore } from '@tanstack/react-store'
import {
  type ChangeEvent,
  type FormEvent,
  useLayoutEffect,
  useRef,
} from 'react'
import { AppFloatingAction } from '@/components/AppFloatingAction'
import { FinderForm } from '@/components/finder/FinderForm'
import { SupportedVersionAlert } from '@/components/SupportedVersionAlert'
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
  caretAfterSignificant,
  formatCanonicalSeedInput,
  isCompleteCanonicalSeed,
  significantCountBefore,
} from '@/lib/canonical-seed'
import {
  $activeFinderSession,
  $analyzing,
  $finderRunning,
  $formError,
  $seedInput,
  type AppMode,
  analyzeDraftSeed,
  cancelFinderSearch,
  MAX_SAVED_SEEDS,
  setSeedInput,
  startFinderSearch,
} from '@/stores/app'

export function AppSidebar({ mode }: { mode: AppMode }) {
  const seedInput = useStore($seedInput)
  const analyzing = useStore($analyzing)
  const formError = useStore($formError)
  const activeFinder = useStore($activeFinderSession)
  const finderRunning = useStore($finderRunning)
  const seedInputRef = useRef<HTMLInputElement>(null)
  const caretRef = useRef<number | null>(null)

  useLayoutEffect(() => {
    const input = seedInputRef.current
    const caret = caretRef.current
    if (!input || caret == null || input.value !== seedInput) return
    input.setSelectionRange(caret, caret)
  }, [seedInput])

  function onSeedChange(event: ChangeEvent<HTMLInputElement>) {
    const raw = event.target.value
    const cursor = event.target.selectionStart ?? raw.length
    const formatted = formatCanonicalSeedInput(raw)
    caretRef.current = caretAfterSignificant(
      formatted,
      significantCountBefore(raw, cursor)
    )
    if (formatted === seedInput) {
      event.target.value = formatted
      event.target.setSelectionRange(caretRef.current, caretRef.current)
      return
    }
    setSeedInput(formatted)
  }

  async function onAnalyze(event: FormEvent) {
    event.preventDefault()
    await analyzeDraftSeed()
  }

  return (
    <aside className="border-border text-sidebar-foreground lg:sticky lg:top-0 lg:max-h-svh lg:h-full lg:w-80 lg:shrink-0 lg:self-start lg:overflow-y-auto lg:border-r">
      <div className="flex flex-col gap-4 p-4 lg:h-full">
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

        <SupportedVersionAlert />

        <TabsList
          className="grid w-full grid-cols-2"
          aria-label="Analyzer mode"
        >
          <TabsTrigger value="analyze">
            {analyzing ? (
              <SpinnerGapIcon
                data-icon="inline-start"
                className="animate-spin"
              />
            ) : (
              <MagnifyingGlassIcon data-icon="inline-start" />
            )}
            Analyze
          </TabsTrigger>
          <TabsTrigger value="finder">
            {finderRunning ? (
              <SpinnerGapIcon
                data-icon="inline-start"
                className="animate-spin"
              />
            ) : (
              <BinocularsIcon data-icon="inline-start" />
            )}
            Find
          </TabsTrigger>
        </TabsList>

        <div className={mode === 'analyze' ? undefined : 'hidden'}>
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
                      ref={seedInputRef}
                      id="seed"
                      value={seedInput}
                      onChange={onSeedChange}
                      placeholder="XXX-XXX-XXX or YYYY-MM-DD"
                      autoCapitalize="characters"
                      autoComplete="off"
                      autoCorrect="off"
                      spellCheck={false}
                      maxLength={11}
                      className="font-mono"
                    />
                  </InputGroup>
                  <Button
                    type="submit"
                    size="default"
                    aria-label="Analyze"
                    disabled={analyzing || !isCompleteCanonicalSeed(seedInput)}
                  >
                    {analyzing ? (
                      <SpinnerGapIcon
                        data-icon="inline-start"
                        className="animate-spin"
                      />
                    ) : (
                      <MagnifyingGlassIcon data-icon="inline-start" />
                    )}
                  </Button>
                </div>
                <FieldDescription>
                  Seed codes are ABC-DEF-GHI. Daily Runs are YYYY-MM-DD. Up to{' '}
                  {MAX_SAVED_SEEDS} open seeds are kept (oldest dropped).
                </FieldDescription>
              </Field>
            </FieldGroup>
          </form>
          {formError ? (
            <Alert variant="destructive" className="mt-4">
              <AlertTitle>Error</AlertTitle>
              <AlertDescription>{formError}</AlertDescription>
            </Alert>
          ) : null}
        </div>

        <div className={mode === 'finder' ? undefined : 'hidden'}>
          <FinderForm
            running={activeFinder?.run.status === 'running'}
            cancelRequested={activeFinder?.run.cancelRequested ?? false}
            onSearch={(config) => void startFinderSearch(config)}
            onCancel={() => cancelFinderSearch()}
          />
        </div>
      </div>
    </aside>
  )
}
