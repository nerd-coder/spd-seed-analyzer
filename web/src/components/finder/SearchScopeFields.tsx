import { InfoIcon, PlantIcon, ShuffleAngularIcon } from '@phosphor-icons/react'
import { Field, FieldError, FieldLabel } from '@/components/ui/field'
import {
  InputGroup,
  InputGroupAddon,
  InputGroupButton,
  InputGroupInput,
} from '@/components/ui/input-group'
import { NativeSelect, NativeSelectOption } from '@/components/ui/native-select'
import {
  Popover,
  PopoverContent,
  PopoverDescription,
  PopoverHeader,
  PopoverTitle,
  PopoverTrigger,
} from '@/components/ui/popover'
import {
  type FinderNumericInput,
  isIntegerInRange,
  MAX_CANDIDATES,
  MAX_FLOORS,
  MAX_RESULTS,
  MIN_CANDIDATES,
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

function randomSeed(): number {
  const values = new Uint32Array(2)
  crypto.getRandomValues(values)
  const random53 = (BigInt(values[0] & 0x1fffff) << 32n) | BigInt(values[1])
  return Number(random53 % BigInt(TOTAL_SEEDS))
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
    MIN_CANDIDATES,
    MAX_CANDIDATES
  )
  const maxMatchesInvalid = !isIntegerInRange(maxMatches, 1, MAX_RESULTS)

  return (
    <div className="flex flex-col gap-2">
      <Field
        data-invalid={attempted && startSeedInvalid ? true : undefined}
        data-disabled={running ? true : undefined}
      >
        <FieldLabel htmlFor="finder-start-seed">Start seed</FieldLabel>
        <InputGroup>
          <InputGroupAddon align="inline-start" aria-hidden>
            <PlantIcon />
          </InputGroupAddon>
          <InputGroupInput
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
          <Popover>
            <PopoverTrigger asChild>
              <InputGroupButton
                size="icon-xs"
                aria-label="About numeric start seeds"
              >
                <InfoIcon />
              </InputGroupButton>
            </PopoverTrigger>
            <PopoverContent align="start">
              <PopoverHeader>
                <PopoverTitle>Why a numeric seed?</PopoverTitle>
                <PopoverDescription>
                  The finder scans seeds in numeric order, starting here. SPD
                  seed codes and text seeds resolve to a number, so using that
                  number makes each bounded search resumable and unambiguous.
                </PopoverDescription>
              </PopoverHeader>
              <PopoverDescription>
                Available range: 0–{(TOTAL_SEEDS - 1).toLocaleString()}. The
                finder checks this value first, then the following numeric seeds
                until the candidate or result limit is reached.
              </PopoverDescription>
            </PopoverContent>
          </Popover>
          <InputGroupButton
            size="icon-sm"
            disabled={running}
            onClick={() => onStartSeedChange(randomSeed())}
            aria-label="Choose a random start seed"
            title="Random start seed"
          >
            <ShuffleAngularIcon />
          </InputGroupButton>
        </InputGroup>
        {attempted && startSeedInvalid ? (
          <FieldError>Use an integer from 0 to 5,429,503,678,975.</FieldError>
        ) : null}
      </Field>
      <div className="grid grid-cols-3 items-start gap-2">
        <Field
          data-invalid={attempted && candidateCountInvalid ? true : undefined}
          data-disabled={running ? true : undefined}
        >
          <FieldLabel htmlFor="finder-candidates">Candidates</FieldLabel>
          <InputGroup>
            <InputGroupInput
              id="finder-candidates"
              type="number"
              min={MIN_CANDIDATES}
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
          </InputGroup>
          {attempted && candidateCountInvalid ? (
            <FieldError>
              Choose {MIN_CANDIDATES.toLocaleString()}–
              {MAX_CANDIDATES.toLocaleString()} candidates.
            </FieldError>
          ) : null}
        </Field>
        <Field data-disabled={running ? true : undefined}>
          <FieldLabel htmlFor="finder-floors">Depth</FieldLabel>
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
                  {depth}
                </NativeSelectOption>
              )
            )}
          </NativeSelect>
        </Field>
        <Field
          data-invalid={attempted && maxMatchesInvalid ? true : undefined}
          data-disabled={running ? true : undefined}
        >
          <FieldLabel htmlFor="finder-max-results">Results</FieldLabel>
          <InputGroup>
            <InputGroupInput
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
          </InputGroup>
          {attempted && maxMatchesInvalid ? (
            <FieldError>Choose 1–{MAX_RESULTS} results.</FieldError>
          ) : null}
        </Field>
      </div>
    </div>
  )
}
