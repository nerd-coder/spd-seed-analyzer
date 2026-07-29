import {
  Field,
  FieldContent,
  FieldDescription,
  FieldGroup,
  FieldLabel,
} from '@/components/ui/field'
import { Switch } from '@/components/ui/switch'
import type { Challenge, MapProfile } from '@/lib/spd-wasm'

const CHALLENGES: {
  value: Challenge
  label: string
  description: string
}[] = [
  {
    value: 'champion_enemies',
    label: 'Champion Enemies',
    description: 'Recorded for this first-generation run.',
  },
  {
    value: 'badder_bosses',
    label: 'Badder Bosses',
    description:
      'Recorded now; floor-15 trap generation will be replayed in a later phase.',
  },
  {
    value: 'on_diet',
    label: 'On Diet',
    description: 'Recorded for this first-generation run.',
  },
  {
    value: 'faith_is_my_armor',
    label: 'Faith Is My Armor',
    description: 'Recorded for this first-generation run.',
  },
  {
    value: 'pharmacophobia',
    label: 'Pharmacophobia',
    description: 'Recorded for this first-generation run.',
  },
  {
    value: 'barren_land',
    label: 'Barren Land',
    description:
      'Recorded now; plant suppression will be replayed in a later phase.',
  },
  {
    value: 'swarm_intelligence',
    label: 'Swarm Intelligence',
    description: 'Recorded for this first-generation run.',
  },
  {
    value: 'into_darkness',
    label: 'Into Darkness',
    description: 'Recorded for this first-generation run.',
  },
  {
    value: 'forbidden_runes',
    label: 'Forbidden Runes',
    description: 'Removes every second guaranteed Scroll of Upgrade.',
  },
]

export function ChallengeSettings({
  sessionId,
  profile,
  disabled,
  onChange,
}: {
  sessionId: string
  profile: MapProfile
  disabled: boolean
  onChange: (profile: MapProfile) => void
}) {
  function toggle(challenge: Challenge, enabled: boolean) {
    const challenges = enabled
      ? [...profile.challenges, challenge]
      : profile.challenges.filter((value) => value !== challenge)
    onChange({ ...profile, challenges })
  }

  return (
    <FieldGroup>
      {CHALLENGES.map((challenge) => {
        const id = `${sessionId}-${challenge.value}`
        return (
          <Field
            key={challenge.value}
            orientation="horizontal"
            data-disabled={disabled ? true : undefined}
          >
            <FieldContent>
              <FieldLabel htmlFor={id}>{challenge.label}</FieldLabel>
              <FieldDescription>{challenge.description}</FieldDescription>
            </FieldContent>
            <Switch
              id={id}
              checked={profile.challenges.includes(challenge.value)}
              disabled={disabled}
              onCheckedChange={(enabled) => toggle(challenge.value, enabled)}
            />
          </Field>
        )
      })}
    </FieldGroup>
  )
}
