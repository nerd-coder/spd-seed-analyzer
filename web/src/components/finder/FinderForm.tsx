import {
  InfoIcon,
  MagnifyingGlassIcon,
  SpinnerGapIcon,
  StopIcon,
} from '@phosphor-icons/react'
import { useStore } from '@tanstack/react-store'
import {
  type FormEvent,
  type MouseEvent,
  useCallback,
  useEffect,
  useRef,
} from 'react'
import { Button } from '@/components/ui/button'
import { Field, FieldGroup, FieldLabel } from '@/components/ui/field'
import { Kbd } from '@/components/ui/kbd'
import { Switch } from '@/components/ui/switch'
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from '@/components/ui/tooltip'
import { $finderForm } from '@/stores/app'
import { ConstraintEditor } from './ConstraintEditor'
import {
  type FinderConfig,
  type FinderConstraint,
  isIntegerInRange,
  MAX_CANDIDATES,
  MAX_CONSTRAINTS,
  MAX_RESULTS,
  MIN_CANDIDATES,
  randomStartSeed,
  TOTAL_SEEDS,
} from './finder-types'
import { SearchScopeFields } from './SearchScopeFields'

const CANCEL_COOLDOWN_MS = 1_000

type FinderFormProps = {
  running: boolean
  cancelRequested: boolean
  onSearch: (config: FinderConfig) => void
  onCancel: () => void
}

export function FinderForm({
  running,
  cancelRequested,
  onSearch,
  onCancel,
}: FinderFormProps) {
  const formRef = useRef<HTMLFormElement>(null)
  const suppressNextSubmit = useRef(false)
  const {
    attempted,
    cancelCooldown,
    startSeed,
    candidateCount,
    floors,
    maxMatches,
    matchMode,
    nonStop,
    constraints,
  } = useStore($finderForm)

  const startSeedInvalid = !isIntegerInRange(startSeed, 0, TOTAL_SEEDS - 1)
  const candidateCountInvalid = !isIntegerInRange(
    candidateCount,
    MIN_CANDIDATES,
    MAX_CANDIDATES
  )
  const maxMatchesInvalid = !isIntegerInRange(maxMatches, 1, MAX_RESULTS)
  const constraintsInvalid = constraints.some(
    (constraint) => !constraint.className
  )
  const invalid =
    startSeedInvalid ||
    candidateCountInvalid ||
    maxMatchesInvalid ||
    constraintsInvalid

  useEffect(() => {
    if (!cancelCooldown) return
    const timer = window.setTimeout(
      () => $finderForm.set({ ...$finderForm.get(), cancelCooldown: false }),
      CANCEL_COOLDOWN_MS
    )
    return () => window.clearTimeout(timer)
  }, [cancelCooldown])

  const updateState = useCallback(function updateState(
    patch: Partial<ReturnType<typeof $finderForm.get>>
  ) {
    $finderForm.set({ ...$finderForm.get(), ...patch })
  }, [])

  const randomizeStartSeed = useCallback(() => {
    updateState({ startSeed: randomStartSeed() })
  }, [updateState])

  useEffect(() => {
    function handleShortcut(event: KeyboardEvent) {
      if (
        !event.ctrlKey ||
        event.altKey ||
        event.metaKey ||
        event.shiftKey ||
        event.repeat
      ) {
        return
      }

      switch (event.key.toLowerCase()) {
        case 'r':
          event.preventDefault()
          if (!running) randomizeStartSeed()
          break
        case 'f':
          event.preventDefault()
          if (!running && !cancelCooldown) formRef.current?.requestSubmit()
          break
      }
    }

    window.addEventListener('keydown', handleShortcut)
    return () => window.removeEventListener('keydown', handleShortcut)
  }, [cancelCooldown, randomizeStartSeed, running])

  function updateFloors(value: number) {
    updateState({
      floors: value,
      constraints: constraints.map((constraint) => ({
        ...constraint,
        maxDepth: value,
      })),
    })
  }

  function updateConstraint(
    id: number,
    patch: Partial<Omit<FinderConstraint, 'id'>>
  ) {
    updateState({
      constraints: constraints.map((constraint) =>
        constraint.id === id ? { ...constraint, ...patch } : constraint
      ),
    })
  }

  function addConstraint() {
    if (constraints.length >= MAX_CONSTRAINTS) return
    const nextConstraintId =
      constraints.reduce((highest, constraint) => {
        return Math.max(highest, constraint.id)
      }, 0) + 1
    updateState({
      constraints: [
        ...constraints,
        {
          id: nextConstraintId,
          className: 'RingOfWealth',
          minLevel: null,
          minDepth: 1,
          maxDepth: floors,
        },
      ],
    })
  }

  function removeConstraint(id: number) {
    if (constraints.length === 1) return
    updateState({
      constraints: constraints.filter((constraint) => constraint.id !== id),
    })
  }

  function submit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault()
    if (suppressNextSubmit.current || cancelCooldown) {
      suppressNextSubmit.current = false
      return
    }
    updateState({ attempted: true })
    if (invalid) return
    onSearch({
      startSeed: Number(startSeed),
      candidateCount: Number(candidateCount),
      floors,
      constraints: constraints.map(
        ({ className, minLevel, minDepth, maxDepth }) => ({
          className,
          minLevel,
          minDepth: Number(minDepth),
          maxDepth: Number(maxDepth),
        })
      ),
      matchMode,
      maxMatches: Number(maxMatches),
      nonStop,
    })
  }

  function cancel(event: MouseEvent<HTMLButtonElement>) {
    event.preventDefault()
    event.stopPropagation()
    suppressNextSubmit.current = true
    updateState({ cancelCooldown: true })
    onCancel()
    window.setTimeout(() => {
      suppressNextSubmit.current = false
    }, 0)
  }

  return (
    <form
      ref={formRef}
      onSubmit={submit}
      noValidate
      className="flex flex-col gap-3"
    >
      <FieldGroup className="gap-2">
        <SearchScopeFields
          startSeed={startSeed}
          candidateCount={candidateCount}
          floors={floors}
          maxMatches={maxMatches}
          running={running}
          attempted={attempted}
          onStartSeedChange={(value) => updateState({ startSeed: value })}
          onCandidateCountChange={(value) =>
            updateState({ candidateCount: value })
          }
          onFloorsChange={updateFloors}
          onMaxMatchesChange={(value) => updateState({ maxMatches: value })}
          onRandomStartSeed={randomizeStartSeed}
        />

        <ConstraintEditor
          constraints={constraints}
          running={running}
          onAdd={addConstraint}
          onRemove={removeConstraint}
          onUpdate={updateConstraint}
        />
      </FieldGroup>
      <div className="flex items-end justify-between gap-2">
        <div className="flex flex-col gap-2">
          <Field orientation="horizontal" className="w-auto gap-2">
            <FieldLabel htmlFor="finder-match-rule">
              {matchMode === 'all' ? 'Match all' : 'Match any'}
            </FieldLabel>
            <Switch
              id="finder-match-rule"
              size="sm"
              checked={matchMode === 'all'}
              disabled={running}
              onCheckedChange={(checked) =>
                updateState({ matchMode: checked ? 'all' : 'any' })
              }
              aria-label={matchMode === 'all' ? 'Match all' : 'Match any'}
            />
          </Field>
          <Field orientation="horizontal" className="w-auto gap-2">
            <div className="flex flex-1 items-center gap-1">
              <FieldLabel htmlFor="finder-non-stop">
                Don&apos;t let me down
              </FieldLabel>
              <Tooltip>
                <TooltipTrigger asChild>
                  <button
                    type="button"
                    className="text-muted-foreground hover:text-foreground focus-visible:ring-ring rounded-sm focus-visible:ring-2 focus-visible:outline-none"
                    aria-label="About Don't let me down mode"
                  >
                    <InfoIcon className="size-3.5" />
                  </button>
                </TooltipTrigger>
                <TooltipContent>
                  Keep searching from new random seeds until the targeted result
                  count is reached.
                </TooltipContent>
              </Tooltip>
            </div>
            <Switch
              id="finder-non-stop"
              size="sm"
              checked={nonStop}
              disabled={running}
              onCheckedChange={(checked) => updateState({ nonStop: checked })}
              aria-label="Don't let me down"
            />
          </Field>
        </div>
        {running ? (
          <Button
            type="button"
            variant="destructive"
            disabled={cancelRequested}
            onClick={cancel}
          >
            {cancelRequested ? (
              <SpinnerGapIcon
                data-icon="inline-start"
                className="animate-spin"
              />
            ) : (
              <StopIcon data-icon="inline-start" />
            )}
            {cancelRequested ? 'Cancelling…' : 'Cancel'}
          </Button>
        ) : (
          <Tooltip>
            <TooltipTrigger asChild>
              <Button
                type="submit"
                disabled={cancelCooldown}
                aria-keyshortcuts="Control+F"
              >
                <MagnifyingGlassIcon data-icon="inline-start" />
                Find
              </Button>
            </TooltipTrigger>
            <TooltipContent>
              Find <Kbd>Ctrl + F</Kbd>
            </TooltipContent>
          </Tooltip>
        )}
      </div>
    </form>
  )
}
