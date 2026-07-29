import { PlusIcon, TrashIcon } from '@phosphor-icons/react'
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
import type { ArtifactEvent, ArtifactKind, MapProfile } from '@/lib/spd-wasm'

const ARTIFACTS: { value: ArtifactKind; label: string }[] = [
  { value: 'alchemists_toolkit', label: "Alchemist's Toolkit" },
  { value: 'chalice_of_blood', label: 'Chalice of Blood' },
  { value: 'cloak_of_shadows', label: 'Cloak of Shadows' },
  { value: 'dried_rose', label: 'Dried Rose' },
  { value: 'ethereal_chains', label: 'Ethereal Chains' },
  { value: 'holy_tome', label: 'Holy Tome' },
  { value: 'horn_of_plenty', label: 'Horn of Plenty' },
  { value: 'master_thieves_armband', label: "Master Thieves' Armband" },
  { value: 'sandals_of_nature', label: 'Sandals of Nature' },
  { value: 'skeleton_key', label: 'Skeleton Key' },
  { value: 'talisman_of_foresight', label: 'Talisman of Foresight' },
  { value: 'timekeepers_hourglass', label: "Timekeeper's Hourglass" },
  { value: 'unstable_spellbook', label: 'Unstable Spellbook' },
]

function floorRange(min: number, max: number): number[] {
  return Array.from({ length: Math.max(0, max - min + 1) }, (_, index) =>
    Math.min(26, min + index)
  )
}

export function ArtifactEventSettings({
  profile,
  disabled,
  onChange,
}: {
  profile: MapProfile
  disabled: boolean
  onChange: (profile: MapProfile) => void
}) {
  const events = profile.artifact_events

  function setEvents(artifact_events: ArtifactEvent[]) {
    onChange({ ...profile, artifact_events })
  }

  function updateEvent(index: number, next: ArtifactEvent) {
    setEvents(
      events.map((event, eventIndex) => (eventIndex === index ? next : event))
    )
  }

  return (
    <Field>
      <FieldLabel>External artifact history</FieldLabel>
      <FieldDescription>
        Record artifacts obtained or transmuted outside floor generation before
        a floor is generated. For a transmutation, choose the artifact changed
        from. These events are saved now and will drive the replay once
        artifact-event application lands.
      </FieldDescription>
      {events.length > 0 ? (
        <FieldGroup className="gap-2">
          {events.map((event, index) => {
            const previous = events[index - 1]
            const next = events[index + 1]
            const minDepth = previous?.before_depth ?? 2
            const maxDepth = next?.before_depth ?? 26
            return (
              <Field
                key={`${index}-${event.before_depth}-${event.kind}`}
                orientation="horizontal"
                className="min-w-0"
                data-disabled={disabled ? true : undefined}
              >
                <InputGroup className="w-36 shrink-0">
                  <InputGroupAddon>
                    <InputGroupText>Action</InputGroupText>
                  </InputGroupAddon>
                  <InputGroupSelect
                    aria-label={`Artifact event ${index + 1} action`}
                    value={event.kind}
                    disabled={disabled}
                    onChange={(change) =>
                      updateEvent(index, {
                        ...event,
                        kind: change.target.value as ArtifactEvent['kind'],
                      })
                    }
                  >
                    <option value="obtained">Obtained</option>
                    <option value="transmuted">Transmuted</option>
                  </InputGroupSelect>
                </InputGroup>
                <InputGroup className="min-w-0 flex-1">
                  <InputGroupAddon>
                    <InputGroupText>
                      {event.kind === 'transmuted' ? 'From' : 'Artifact'}
                    </InputGroupText>
                  </InputGroupAddon>
                  <InputGroupSelect
                    aria-label={`Artifact event ${index + 1} artifact`}
                    value={event.artifact}
                    disabled={disabled}
                    onChange={(change) =>
                      updateEvent(index, {
                        ...event,
                        artifact: change.target.value as ArtifactKind,
                      })
                    }
                  >
                    {ARTIFACTS.map((artifact) => (
                      <option key={artifact.value} value={artifact.value}>
                        {artifact.label}
                      </option>
                    ))}
                  </InputGroupSelect>
                </InputGroup>
                <InputGroup className="w-28 shrink-0">
                  <InputGroupAddon>
                    <InputGroupText>Before</InputGroupText>
                  </InputGroupAddon>
                  <InputGroupSelect
                    aria-label={`Artifact event ${index + 1} effective floor`}
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
                  aria-label={`Remove artifact event ${index + 1}`}
                  className="text-destructive hover:bg-destructive/10 hover:text-destructive"
                >
                  <TrashIcon />
                </Button>
              </Field>
            )
          })}
        </FieldGroup>
      ) : null}
      <Button
        type="button"
        size="sm"
        variant="outline"
        disabled={disabled}
        onClick={() =>
          setEvents([
            ...events,
            {
              before_depth: events.at(-1)?.before_depth ?? 2,
              kind: 'obtained',
              artifact: 'alchemists_toolkit',
            },
          ])
        }
      >
        <PlusIcon data-icon="inline-start" />
        Add artifact event
      </Button>
    </Field>
  )
}
