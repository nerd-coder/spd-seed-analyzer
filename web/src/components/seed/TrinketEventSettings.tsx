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
  MapProfile,
  TrinketEvent,
  TrinketKind,
  TrinketSelectionReport,
} from '@/lib/spd-wasm'
import { TrinketCombobox } from './TrinketCombobox'
import { TRINKET_OPTIONS, trinketKindFromClassName } from './trinket-options'

function floorRange(min: number, max: number): number[] {
  return Array.from({ length: Math.max(0, max - min + 1) }, (_, index) =>
    Math.min(26, min + index)
  )
}

type ActiveTrinket = {
  trinket: TrinketKind
  level: number
}

function applyTrinketEvent(
  held: ActiveTrinket | null,
  event: TrinketEvent
): ActiveTrinket | null {
  if (event.kind === 'acquired' || event.kind === 'transmuted') {
    return { trinket: event.trinket, level: held?.level ?? 0 }
  }
  return held && { trinket: held.trinket, level: held.level + 1 }
}

function activeTrinket(events: TrinketEvent[]): ActiveTrinket | null {
  let held: ActiveTrinket | null = null
  for (const event of events) {
    held = applyTrinketEvent(held, event)
  }
  return held
}

function defaultTrinket(selection: TrinketSelectionReport): TrinketKind {
  const offered = selection.catalyst_options
    .map(trinketKindFromClassName)
    .find((trinket): trinket is TrinketKind => trinket !== null)
  return offered ?? TRINKET_OPTIONS[0].value
}

function nextEvent(
  events: TrinketEvent[],
  selection: TrinketSelectionReport
): TrinketEvent {
  const before_depth =
    events.at(-1)?.before_depth ?? selection.first_effective_depth
  const held = activeTrinket(events)
  if (!held) {
    return {
      before_depth,
      kind: 'acquired',
      trinket: defaultTrinket(selection),
    }
  }
  if (held.level < 3) return { before_depth, kind: 'upgraded' }
  return {
    before_depth,
    kind: 'transmuted',
    trinket:
      TRINKET_OPTIONS.find((option) => option.value !== held.trinket)?.value ??
      held.trinket,
  }
}

function eventWithKind(
  event: TrinketEvent,
  kind: TrinketEvent['kind']
): TrinketEvent {
  if (kind === 'upgraded') return { before_depth: event.before_depth, kind }
  const trinket = 'trinket' in event ? event.trinket : TRINKET_OPTIONS[0].value
  return { before_depth: event.before_depth, kind, trinket }
}

export function TrinketEventSettings({
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
  const events = profile.trinket_events

  function setEvents(trinket_events: TrinketEvent[]) {
    onChange({ ...profile, trinket_events })
  }

  function updateEvent(index: number, next: TrinketEvent) {
    setEvents(
      events.map((event, eventIndex) => (eventIndex === index ? next : event))
    )
  }

  return (
    <Field>
      <FieldLabel>
        Trinket history
        {disabled ? (
          <SpinnerGapIcon className="animate-spin" aria-label="Regenerating" />
        ) : null}
      </FieldLabel>
      <Alert>
        <AlertTitle>
          First possible effect: floor {selection.first_effective_depth}
        </AlertTitle>
        <AlertDescription className="flex flex-col gap-2">
          <p>
            The Catalyst appears on floor {selection.catalyst_depth}; the first
            generated alchemy pot is on floor{' '}
            {selection.first_alchemy_pot_depth}
            {selection.first_alchemy_pot_is_secret ? ' (secret)' : ''}. Record
            the actions in the order they happened before each generated floor.
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
        </AlertDescription>
      </Alert>
      {events.length > 0 ? (
        <FieldGroup className="gap-2">
          {events.map((event, index) => {
            const previous = events[index - 1]
            const next = events[index + 1]
            const minDepth = Math.max(
              selection.first_effective_depth,
              previous?.before_depth ?? 1
            )
            const maxDepth = next?.before_depth ?? 26
            return (
              <Field
                key={`${index}-${event.before_depth}-${event.kind}`}
                orientation="horizontal"
                className="min-w-0"
                data-disabled={disabled ? true : undefined}
              >
                <InputGroup className="w-32 shrink-0">
                  <InputGroupAddon>
                    <InputGroupText>Action</InputGroupText>
                  </InputGroupAddon>
                  <InputGroupSelect
                    aria-label={`Trinket event ${index + 1} action`}
                    value={event.kind}
                    disabled={disabled}
                    onChange={(change) =>
                      updateEvent(
                        index,
                        eventWithKind(
                          event,
                          change.target.value as TrinketEvent['kind']
                        )
                      )
                    }
                  >
                    <option value="acquired">Acquire</option>
                    <option value="upgraded">Upgrade</option>
                    <option value="transmuted">Transmute</option>
                  </InputGroupSelect>
                </InputGroup>
                {event.kind === 'upgraded' ? (
                  <FieldDescription className="min-w-0 flex-1">
                    Raises the held trinket by one level.
                  </FieldDescription>
                ) : (
                  <TrinketCombobox
                    id={`${sessionId}-trinket-event-${index}`}
                    value={event.trinket}
                    selection={selection}
                    disabled={disabled}
                    label={`Trinket event ${index + 1} trinket`}
                    onChange={(trinket) =>
                      updateEvent(index, { ...event, trinket })
                    }
                  />
                )}
                <InputGroup className="w-28 shrink-0">
                  <InputGroupAddon>
                    <InputGroupText>Before</InputGroupText>
                  </InputGroupAddon>
                  <InputGroupSelect
                    aria-label={`Trinket event ${index + 1} effective floor`}
                    value={event.before_depth}
                    disabled={disabled}
                    onChange={(change) =>
                      updateEvent(index, {
                        ...event,
                        before_depth: Number(change.target.value),
                      })
                    }
                  >
                    {floorRange(minDepth, maxDepth).map((depth) => (
                      <option key={depth} value={depth}>
                        floor {depth}
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
                    setEvents(
                      events.filter((_, eventIndex) => eventIndex !== index)
                    )
                  }
                  aria-label={`Remove trinket event ${index + 1}`}
                  className="text-destructive hover:bg-destructive/10 hover:text-destructive"
                >
                  <TrashIcon />
                </Button>
              </Field>
            )
          })}
        </FieldGroup>
      ) : (
        <FieldDescription>No trinket action is recorded.</FieldDescription>
      )}
      <Button
        type="button"
        size="sm"
        variant="outline"
        disabled={disabled}
        onClick={() => setEvents([...events, nextEvent(events, selection)])}
      >
        <PlusIcon data-icon="inline-start" />
        Add trinket event
      </Button>
      <FieldDescription>
        Acquiring or transmuting starts a fresh trinket instance. An upgrade
        keeps the active instance and its prior level.
      </FieldDescription>
    </Field>
  )
}
