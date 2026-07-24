import {
  MagnifyingGlassIcon,
  SpinnerGapIcon,
  StopIcon,
} from '@phosphor-icons/react'
import {
  type FormEvent,
  type MouseEvent,
  useEffect,
  useRef,
  useState,
} from 'react'
import { Button } from '@/components/ui/button'
import { Field, FieldGroup, FieldLabel } from '@/components/ui/field'
import { Switch } from '@/components/ui/switch'
import type { SeedSearchMatchMode } from '@/lib/spd-wasm'
import { ConstraintEditor } from './ConstraintEditor'
import {
  type FinderConfig,
  type FinderConstraint,
  type FinderNumericInput,
  isIntegerInRange,
  MAX_CANDIDATES,
  MAX_CONSTRAINTS,
  MAX_RESULTS,
  MIN_CANDIDATES,
  TOTAL_SEEDS,
} from './finder-types'
import { SearchScopeFields } from './SearchScopeFields'

const INITIAL_FLOORS = 10
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
  const nextConstraintId = useRef(2)
  const suppressNextSubmit = useRef(false)
  const [attempted, setAttempted] = useState(false)
  const [cancelCooldown, setCancelCooldown] = useState(false)
  const [startSeed, setStartSeed] = useState<FinderNumericInput>(0)
  const [candidateCount, setCandidateCount] = useState<FinderNumericInput>(100)
  const [floors, setFloors] = useState(INITIAL_FLOORS)
  const [maxMatches, setMaxMatches] = useState<FinderNumericInput>(10)
  const [matchMode, setMatchMode] = useState<SeedSearchMatchMode>('all')
  const [constraints, setConstraints] = useState<FinderConstraint[]>([
    {
      id: 1,
      className: 'RingOfWealth',
      minLevel: null,
      minDepth: 1,
      maxDepth: INITIAL_FLOORS,
    },
  ])

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
      () => setCancelCooldown(false),
      CANCEL_COOLDOWN_MS
    )
    return () => window.clearTimeout(timer)
  }, [cancelCooldown])

  function updateFloors(value: number) {
    setFloors(value)
    setConstraints((current) =>
      current.map((constraint) => ({ ...constraint, maxDepth: value }))
    )
  }

  function updateConstraint(
    id: number,
    patch: Partial<Omit<FinderConstraint, 'id'>>
  ) {
    setConstraints((current) =>
      current.map((constraint) =>
        constraint.id === id ? { ...constraint, ...patch } : constraint
      )
    )
  }

  function addConstraint() {
    setConstraints((current) => {
      if (current.length >= MAX_CONSTRAINTS) return current
      return [
        ...current,
        {
          id: nextConstraintId.current++,
          className: 'RingOfWealth',
          minLevel: null,
          minDepth: 1,
          maxDepth: floors,
        },
      ]
    })
  }

  function removeConstraint(id: number) {
    setConstraints((current) =>
      current.length === 1
        ? current
        : current.filter((constraint) => constraint.id !== id)
    )
  }

  function submit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault()
    if (suppressNextSubmit.current || cancelCooldown) {
      suppressNextSubmit.current = false
      return
    }
    setAttempted(true)
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
    })
  }

  function cancel(event: MouseEvent<HTMLButtonElement>) {
    event.preventDefault()
    event.stopPropagation()
    suppressNextSubmit.current = true
    setCancelCooldown(true)
    onCancel()
    window.setTimeout(() => {
      suppressNextSubmit.current = false
    }, 0)
  }

  return (
    <form onSubmit={submit} noValidate className="flex flex-col gap-3">
      <FieldGroup className="gap-2">
        <SearchScopeFields
          startSeed={startSeed}
          candidateCount={candidateCount}
          floors={floors}
          maxMatches={maxMatches}
          running={running}
          attempted={attempted}
          onStartSeedChange={setStartSeed}
          onCandidateCountChange={setCandidateCount}
          onFloorsChange={updateFloors}
          onMaxMatchesChange={setMaxMatches}
        />

        <ConstraintEditor
          constraints={constraints}
          running={running}
          onAdd={addConstraint}
          onRemove={removeConstraint}
          onUpdate={updateConstraint}
        />
      </FieldGroup>
      <div className="flex items-center justify-between gap-2">
        <Field orientation="horizontal" className="w-auto gap-2">
          <FieldLabel htmlFor="finder-match-rule">
            {matchMode === 'all' ? 'Match all' : 'Match any'}
          </FieldLabel>
          <Switch
            id="finder-match-rule"
            size="sm"
            checked={matchMode === 'all'}
            disabled={running}
            onCheckedChange={(checked) => setMatchMode(checked ? 'all' : 'any')}
            aria-label={matchMode === 'all' ? 'Match all' : 'Match any'}
          />
        </Field>
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
          <Button type="submit" disabled={cancelCooldown}>
            <MagnifyingGlassIcon data-icon="inline-start" />
            Find seeds
          </Button>
        )}
      </div>
    </form>
  )
}
