import { PlusIcon, SpinnerGapIcon, TrashIcon } from '@phosphor-icons/react'
import { finderItemLabel } from '@/components/finder/finder-items'
import { ItemIcon } from '@/components/ItemIcon'
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
  InputGroupSelect,
  InputGroupText,
} from '@/components/ui/input-group'
import type {
  HeldTrinketProfile,
  MapProfile,
  TrinketSelectionReport,
} from '@/lib/spd-wasm'
import { TrinketCombobox } from './TrinketCombobox'
import { TRINKET_OPTIONS } from './trinket-options'

const MAX_TRINKET_STATES = 12

function nextInitialState(
  states: HeldTrinketProfile[],
  selection: TrinketSelectionReport
): HeldTrinketProfile {
  const previous = states.at(-1)
  const directOption = TRINKET_OPTIONS.find(
    (option) =>
      option.className && selection.catalyst_options.includes(option.className)
  )
  return {
    trinket: previous?.trinket ?? directOption?.value ?? 'mossy_clump',
    level: previous?.level ?? 0,
    start_depth: previous
      ? Math.min(26, previous.start_depth + 1)
      : selection.first_effective_depth,
  }
}

function floorRange(min: number, max: number): number[] {
  return Array.from({ length: Math.max(0, max - min + 1) }, (_, index) =>
    Math.min(26, min + index)
  )
}

export function HeldTrinketSettings({
  sessionId,
  profile,
  selection,
  disabled,
  onChange,
}: {
  sessionId: string
  profile: MapProfile
  selection: TrinketSelectionReport
  disabled: boolean
  onChange: (profile: MapProfile) => void
}) {
  const states = profile.held_trinkets
  const lastState = states.at(-1)
  const canAdd =
    !disabled &&
    states.length < MAX_TRINKET_STATES &&
    (!lastState || lastState.start_depth < 26)

  function setStates(held_trinkets: HeldTrinketProfile[]) {
    onChange({ ...profile, held_trinkets })
  }

  function updateState(index: number, patch: Partial<HeldTrinketProfile>) {
    const held_trinkets = states.map((state, stateIndex) =>
      stateIndex === index ? { ...state, ...patch } : { ...state }
    )
    if (patch.level !== undefined) {
      for (let later = index + 1; later < held_trinkets.length; later += 1) {
        held_trinkets[later].level = Math.max(
          held_trinkets[later].level,
          patch.level
        )
      }
    }
    setStates(held_trinkets)
  }

  return (
    <Field>
      <FieldLabel>
        Held trinket history
        {disabled ? (
          <SpinnerGapIcon className="animate-spin" aria-label="Regenerating" />
        ) : null}
      </FieldLabel>
      <Alert>
        <AlertTitle>
          Earliest effect: floor {selection.first_effective_depth}
        </AlertTitle>
        <AlertDescription className="flex flex-col gap-2">
          <p>
            The Catalyst appears on floor {selection.catalyst_depth}; the first
            generated alchemy pot is on floor{' '}
            {selection.first_alchemy_pot_depth}
            {selection.first_alchemy_pot_is_secret ? ' (secret)' : ''}. The
            choice happens after that floor is generated, so it starts affecting
            the next floor.
          </p>
          <div className="flex flex-wrap items-center gap-x-3 gap-y-1">
            <span className="font-medium text-foreground">
              Catalyst offers:
            </span>
            {selection.catalyst_options.map((className) => (
              <span key={className} className="flex items-center gap-1">
                <ItemIcon
                  classNameItem={className}
                  category="trinket"
                  size={16}
                  title={finderItemLabel(className)}
                />
                {finderItemLabel(className)}
              </span>
            ))}
          </div>
          <p>
            Combobox labels show when each modeled trinket occurs in this seed’s
            Catalyst or transmutation order. You choose when upgrades or
            transmutations happen.
          </p>
        </AlertDescription>
      </Alert>
      {states.length > 0 ? (
        <FieldGroup className="gap-2">
          {states.map((state, index) => {
            const previous = states[index - 1]
            const next = states[index + 1]
            const minDepth = previous
              ? previous.start_depth + 1
              : selection.first_effective_depth
            const maxDepth = next ? next.start_depth - 1 : 26
            const minLevel = previous?.level ?? 0
            return (
              <Field
                key={`${index}-${state.start_depth}`}
                orientation="horizontal"
                className="min-w-0"
                data-disabled={disabled ? true : undefined}
              >
                <TrinketCombobox
                  id={`${sessionId}-trinket-${index}`}
                  value={state.trinket}
                  selection={selection}
                  disabled={disabled}
                  label={`Trinket history entry ${index + 1}`}
                  onChange={(trinket) =>
                    updateState(index, {
                      trinket,
                      level: minLevel,
                      start_depth: minDepth,
                    })
                  }
                />
                <InputGroup className="w-28 shrink-0">
                  <InputGroupAddon>
                    <InputGroupText>Upgrade</InputGroupText>
                  </InputGroupAddon>
                  <InputGroupSelect
                    aria-label={`Trinket history entry ${index + 1} upgrade level`}
                    value={state.level}
                    disabled={disabled}
                    onChange={(event) =>
                      updateState(index, { level: Number(event.target.value) })
                    }
                  >
                    {floorRange(minLevel, 3).map((level) => (
                      <option key={level} value={level}>
                        +{level}
                      </option>
                    ))}
                  </InputGroupSelect>
                </InputGroup>
                <InputGroup className="w-24 shrink-0">
                  <InputGroupAddon>
                    <InputGroupText>Floor</InputGroupText>
                  </InputGroupAddon>
                  <InputGroupSelect
                    aria-label={`Trinket history entry ${index + 1} starting floor`}
                    value={state.start_depth}
                    disabled={disabled}
                    onChange={(event) =>
                      updateState(index, {
                        start_depth: Number(event.target.value),
                      })
                    }
                  >
                    {floorRange(minDepth, maxDepth).map((depth) => (
                      <option key={depth} value={depth}>
                        {depth}
                      </option>
                    ))}
                  </InputGroupSelect>
                </InputGroup>
                <Button
                  type="button"
                  size="icon-sm"
                  variant="ghost"
                  disabled={disabled}
                  onClick={() =>
                    setStates(
                      states.filter((_, stateIndex) => stateIndex !== index)
                    )
                  }
                  aria-label={`Remove trinket history entry ${index + 1}`}
                  className="text-destructive hover:bg-destructive/10 hover:text-destructive"
                >
                  <TrashIcon />
                </Button>
              </Field>
            )
          })}
        </FieldGroup>
      ) : (
        <FieldDescription>
          No supported held-trinket effect is applied to generated floors.
        </FieldDescription>
      )}
      <Button
        type="button"
        size="sm"
        variant="outline"
        className="w-fit"
        disabled={!canAdd}
        onClick={() =>
          setStates([...states, nextInitialState(states, selection)])
        }
      >
        <PlusIcon data-icon="inline-start" />
        Add trinket change
      </Button>
      <FieldDescription>
        Levels can only stay the same or increase. A transmutation keeps the
        current level; add another row on the first floor generated after the
        change.
      </FieldDescription>
    </Field>
  )
}
