import { SpinnerGapIcon } from '@phosphor-icons/react'
import { SpoilerToggle } from '@/components/seed/SpoilerToggle'
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card'
import { Field, FieldDescription, FieldLabel } from '@/components/ui/field'
import {
  InputGroup,
  InputGroupAddon,
  InputGroupSelect,
  InputGroupText,
} from '@/components/ui/input-group'
import { NativeSelect, NativeSelectOption } from '@/components/ui/native-select'
import type { MapProfile, MapTrinketProfile, SeedReport } from '@/lib/spd-wasm'
import {
  changeSeedMapProfile,
  setSeedIdentitySpoilers,
  setSeedMapSpoilers,
} from '@/stores/app'

/**
 * True when the user's input is not the same seed-code presentation as
 * the canonical `ABC-DEF-GHI` form (fun text, bare numbers, etc.).
 */
function customDiffersFromCanonical(
  input: string,
  code: string | null | undefined
): code is string {
  if (!code) return false
  return input !== code
}

type TrinketKind = 'none' | 'mossy_clump' | 'trap_mechanism'

function trinketParts(profile: MapTrinketProfile): {
  kind: TrinketKind
  level: number
} {
  if (profile === 'no_map_affecting_trinkets') return { kind: 'none', level: 0 }
  const match = profile.match(/^(mossy_clump|trap_mechanism)([0-3])$/)
  return {
    kind: (match?.[1] as TrinketKind | undefined) ?? 'none',
    level: Number(match?.[2] ?? 0),
  }
}

function profileTrinket(kind: TrinketKind, level: number): MapTrinketProfile {
  if (kind === 'none') return 'no_map_affecting_trinkets'
  return `${kind}${Math.max(0, Math.min(3, level))}` as MapTrinketProfile
}

export function SeedInfoPanel({
  report,
  sessionId,
  mapProfile,
  refreshingLayout,
  identitySpoilers,
  mapSpoilers,
}: {
  report: SeedReport
  sessionId: string
  mapProfile: MapProfile
  refreshingLayout: boolean
  identitySpoilers: boolean
  mapSpoilers: boolean
}) {
  const { input, code, formatted, numeric } = report.seed
  const showCustomAndCanonical = customDiffersFromCanonical(input, code)
  const { kind, level } = trinketParts(mapProfile.trinket)

  function updateMapProfile(patch: Partial<MapProfile>) {
    void changeSeedMapProfile(sessionId, { ...mapProfile, ...patch })
  }

  return (
    <Card>
      <CardHeader className="gap-3">
        <CardTitle className="font-mono">{input}</CardTitle>
        <CardDescription>
          {showCustomAndCanonical && (
            <p>
              Canonical:{' '}
              <span className="text-foreground font-mono">
                {code ?? formatted}
              </span>
            </p>
          )}
          <p>
            Numeric:{' '}
            <span className="text-foreground font-mono">{numeric}</span>
          </p>
        </CardDescription>
      </CardHeader>
      <CardContent className="flex flex-col gap-4">
        <SpoilerToggle
          id={`${sessionId}-identity-spoilers`}
          label="Identities"
          info="Reveal potion, scroll, and ring appearance mappings for this seed."
          checked={identitySpoilers}
          onCheckedChange={(value) => setSeedIdentitySpoilers(sessionId, value)}
        />
        <Field>
          <SpoilerToggle
            id={`${sessionId}-map-spoilers`}
            label="Floor maps"
            info="Reveal painter-complete layouts for this seed."
            checked={mapSpoilers}
            onCheckedChange={(value) => setSeedMapSpoilers(sessionId, value)}
          />
          <FieldLabel className="sr-only" htmlFor={`${sessionId}-trinket`}>
            Floor layout trinket
          </FieldLabel>
          <div className="flex flex-col gap-2 sm:flex-row">
            <NativeSelect
              className="w-full sm:w-auto"
              id={`${sessionId}-trinket`}
              aria-label="Trinket"
              value={kind}
              disabled={refreshingLayout || !mapSpoilers}
              onChange={(event) => {
                const nextKind = event.target.value as TrinketKind
                updateMapProfile({
                  trinket: profileTrinket(nextKind, level),
                })
              }}
            >
              <NativeSelectOption value="none">None</NativeSelectOption>
              <NativeSelectOption value="mossy_clump">
                Mossy Clump
              </NativeSelectOption>
              <NativeSelectOption value="trap_mechanism">
                Trap Mechanism
              </NativeSelectOption>
            </NativeSelect>
            <InputGroup data-disabled={kind === 'none'}>
              <InputGroupAddon>
                <InputGroupText>Upgrade</InputGroupText>
              </InputGroupAddon>
              <InputGroupSelect
                aria-label="Upgrade level"
                value={level}
                disabled={refreshingLayout || !mapSpoilers || kind === 'none'}
                onChange={(event) =>
                  updateMapProfile({
                    trinket: profileTrinket(kind, Number(event.target.value)),
                  })
                }
              >
                {[0, 1, 2, 3].map((upgrade) => (
                  <option key={upgrade} value={upgrade}>
                    +{upgrade}
                  </option>
                ))}
              </InputGroupSelect>
              <InputGroupSelect
                aria-label="Held starting on floor"
                value={mapProfile.trinket_start_depth}
                disabled={refreshingLayout || !mapSpoilers || kind === 'none'}
                onChange={(event) =>
                  updateMapProfile({
                    trinket_start_depth: Number(event.target.value),
                  })
                }
              >
                {Array.from({ length: 26 }, (_, index) => index + 1).map(
                  (depth) => (
                    <option key={depth} value={depth}>
                      Floor {depth}
                    </option>
                  )
                )}
              </InputGroupSelect>
            </InputGroup>
          </div>
          <FieldDescription>
            The trinket is held starting when the selected floor is generated.
            Changing any value regenerates every map for this seed.
          </FieldDescription>
          {refreshingLayout ? (
            <p className="flex items-center gap-1 text-xs text-muted-foreground">
              <SpinnerGapIcon className="animate-spin" aria-hidden />
              Regenerating maps…
            </p>
          ) : null}
        </Field>
      </CardContent>
    </Card>
  )
}
