import { Field, FieldError, FieldLabel } from '@/components/ui/field'
import { Input } from '@/components/ui/input'
import { NativeSelect, NativeSelectOption } from '@/components/ui/native-select'
import {
  type FinderNumericInput,
  isIntegerInRange,
  MAX_CANDIDATES,
  MAX_FLOORS,
  MAX_RESULTS,
  TOTAL_SEEDS,
} from './finder-types'

type SearchScopeFieldsProps = {
  startSeed: FinderNumericInput
  candidateCount: FinderNumericInput
  floors: number
  maxMatches: FinderNumericInput
  running: boolean
  attempted: boolean
  onStartSeedChange: (value: FinderNumericInput) => void
  onCandidateCountChange: (value: FinderNumericInput) => void
  onFloorsChange: (value: number) => void
  onMaxMatchesChange: (value: FinderNumericInput) => void
}

function inputNumber(value: string, valueAsNumber: number): FinderNumericInput {
  return value === '' ? '' : valueAsNumber
}

export function SearchScopeFields({
  startSeed,
  candidateCount,
  floors,
  maxMatches,
  running,
  attempted,
  onStartSeedChange,
  onCandidateCountChange,
  onFloorsChange,
  onMaxMatchesChange,
}: SearchScopeFieldsProps) {
  const startSeedInvalid = !isIntegerInRange(startSeed, 0, TOTAL_SEEDS - 1)
  const candidateCountInvalid = !isIntegerInRange(
    candidateCount,
    1,
    MAX_CANDIDATES
  )
  const maxMatchesInvalid = !isIntegerInRange(maxMatches, 1, MAX_RESULTS)

  return (
    <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-4">
      <Field
        data-invalid={attempted && startSeedInvalid ? true : undefined}
        data-disabled={running ? true : undefined}
      >
        <FieldLabel htmlFor="finder-start-seed">Start seed</FieldLabel>
        <Input
          id="finder-start-seed"
          type="number"
          min={0}
          max={TOTAL_SEEDS - 1}
          step={1}
          value={startSeed}
          disabled={running}
          aria-invalid={attempted && startSeedInvalid}
          onChange={(event) =>
            onStartSeedChange(
              inputNumber(
                event.currentTarget.value,
                event.currentTarget.valueAsNumber
              )
            )
          }
          className="font-mono"
        />
        {attempted && startSeedInvalid ? (
          <FieldError>Use an integer from 0 to 5,429,503,678,975.</FieldError>
        ) : null}
      </Field>
      <Field
        data-invalid={attempted && candidateCountInvalid ? true : undefined}
        data-disabled={running ? true : undefined}
      >
        <FieldLabel htmlFor="finder-candidates">Candidates</FieldLabel>
        <Input
          id="finder-candidates"
          type="number"
          min={1}
          max={MAX_CANDIDATES}
          step={1}
          value={candidateCount}
          disabled={running}
          aria-invalid={attempted && candidateCountInvalid}
          onChange={(event) =>
            onCandidateCountChange(
              inputNumber(
                event.currentTarget.value,
                event.currentTarget.valueAsNumber
              )
            )
          }
        />
        {attempted && candidateCountInvalid ? (
          <FieldError>Choose 1–{MAX_CANDIDATES} candidates.</FieldError>
        ) : null}
      </Field>
      <Field data-disabled={running ? true : undefined}>
        <FieldLabel htmlFor="finder-floors">Analysis depth</FieldLabel>
        <NativeSelect
          id="finder-floors"
          value={String(floors)}
          disabled={running}
          onChange={(event) => onFloorsChange(Number(event.target.value))}
          className="w-full"
        >
          {Array.from({ length: MAX_FLOORS }, (_, index) => index + 1).map(
            (depth) => (
              <NativeSelectOption key={depth} value={depth}>
                Through floor {depth}
              </NativeSelectOption>
            )
          )}
        </NativeSelect>
      </Field>
      <Field
        data-invalid={attempted && maxMatchesInvalid ? true : undefined}
        data-disabled={running ? true : undefined}
      >
        <FieldLabel htmlFor="finder-max-results">Max results</FieldLabel>
        <Input
          id="finder-max-results"
          type="number"
          min={1}
          max={MAX_RESULTS}
          step={1}
          value={maxMatches}
          disabled={running}
          aria-invalid={attempted && maxMatchesInvalid}
          onChange={(event) =>
            onMaxMatchesChange(
              inputNumber(
                event.currentTarget.value,
                event.currentTarget.valueAsNumber
              )
            )
          }
        />
        {attempted && maxMatchesInvalid ? (
          <FieldError>Choose 1–{MAX_RESULTS} results.</FieldError>
        ) : null}
      </Field>
    </div>
  )
}
