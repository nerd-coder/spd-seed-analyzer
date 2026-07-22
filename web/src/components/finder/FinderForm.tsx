import {
  MagnifyingGlassIcon,
  SpinnerGapIcon,
  StopIcon,
} from '@phosphor-icons/react'
import { type FormEvent, useRef, useState } from 'react'
import { Button } from '@/components/ui/button'
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from '@/components/ui/card'
import {
  FieldDescription,
  FieldGroup,
  FieldLegend,
  FieldSet,
} from '@/components/ui/field'
import { ToggleGroup, ToggleGroupItem } from '@/components/ui/toggle-group'
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
  TOTAL_SEEDS,
} from './finder-types'
import { SearchScopeFields } from './SearchScopeFields'

const INITIAL_FLOORS = 10

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
  const [attempted, setAttempted] = useState(false)
  const [startSeed, setStartSeed] = useState<FinderNumericInput>(0)
  const [candidateCount, setCandidateCount] = useState<FinderNumericInput>(25)
  const [floors, setFloors] = useState(INITIAL_FLOORS)
  const [maxMatches, setMaxMatches] = useState<FinderNumericInput>(10)
  const [matchMode, setMatchMode] = useState<SeedSearchMatchMode>('all')
  const [constraints, setConstraints] = useState<FinderConstraint[]>([
    {
      id: 1,
      className: 'PotionOfHealing',
      minDepth: 1,
      maxDepth: INITIAL_FLOORS,
    },
  ])

  const startSeedInvalid = !isIntegerInRange(startSeed, 0, TOTAL_SEEDS - 1)
  const candidateCountInvalid = !isIntegerInRange(
    candidateCount,
    1,
    MAX_CANDIDATES
  )
  const maxMatchesInvalid = !isIntegerInRange(maxMatches, 1, MAX_RESULTS)
  const constraintsInvalid = constraints.some(
    (constraint) =>
      !constraint.className ||
      !isIntegerInRange(constraint.minDepth, 1, floors) ||
      !isIntegerInRange(
        constraint.maxDepth,
        Number(constraint.minDepth),
        floors
      )
  )
  const invalid =
    startSeedInvalid ||
    candidateCountInvalid ||
    maxMatchesInvalid ||
    constraintsInvalid

  function updateFloors(value: number) {
    setFloors(value)
    setConstraints((current) =>
      current.map((constraint) => {
        if (constraint.minDepth === '' || constraint.maxDepth === '') {
          return constraint
        }
        const maxDepth = Math.min(constraint.maxDepth, value)
        return {
          ...constraint,
          maxDepth,
          minDepth: Math.min(constraint.minDepth, maxDepth),
        }
      })
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
          className: 'PotionOfHealing',
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
    setAttempted(true)
    if (invalid) return
    onSearch({
      startSeed: Number(startSeed),
      candidateCount: Number(candidateCount),
      floors,
      constraints: constraints.map(({ className, minDepth, maxDepth }) => ({
        className,
        minDepth: Number(minDepth),
        maxDepth: Number(maxDepth),
      })),
      matchMode,
      maxMatches: Number(maxMatches),
    })
  }

  return (
    <form onSubmit={submit} noValidate>
      <Card>
        <CardHeader>
          <CardTitle>Search parameters</CardTitle>
          <CardDescription>
            Scan a fixed numeric range. Each item must appear within its
            inclusive floor range.
          </CardDescription>
        </CardHeader>
        <CardContent>
          <FieldGroup>
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

            <FieldSet data-disabled={running ? true : undefined}>
              <FieldLegend variant="label">Match rule</FieldLegend>
              <ToggleGroup
                type="single"
                variant="outline"
                value={matchMode}
                disabled={running}
                aria-label="Item constraint match rule"
                onValueChange={(value) => {
                  if (value === 'any' || value === 'all') setMatchMode(value)
                }}
                spacing={0}
              >
                <ToggleGroupItem value="all">All items</ToggleGroupItem>
                <ToggleGroupItem value="any">Any item</ToggleGroupItem>
              </ToggleGroup>
              <FieldDescription>
                {matchMode === 'all'
                  ? 'Every configured item constraint must match.'
                  : 'At least one configured item constraint must match.'}
              </FieldDescription>
            </FieldSet>

            <ConstraintEditor
              constraints={constraints}
              floors={floors}
              running={running}
              attempted={attempted}
              onAdd={addConstraint}
              onRemove={removeConstraint}
              onUpdate={updateConstraint}
            />
          </FieldGroup>
        </CardContent>
        <CardFooter className="justify-end">
          {running ? (
            <Button
              type="button"
              variant="destructive"
              disabled={cancelRequested}
              onClick={onCancel}
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
            <Button type="submit">
              <MagnifyingGlassIcon data-icon="inline-start" />
              Find seeds
            </Button>
          )}
        </CardFooter>
      </Card>
    </form>
  )
}
