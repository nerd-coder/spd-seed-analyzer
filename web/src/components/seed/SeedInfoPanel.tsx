import { SpinnerGapIcon } from '@phosphor-icons/react'
import { ItemIcon } from '@/components/ItemIcon'
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card'
import {
  Field,
  FieldContent,
  FieldDescription,
  FieldGroup,
  FieldLabel,
  FieldLegend,
  FieldSet,
} from '@/components/ui/field'
import {
  InputGroup,
  InputGroupAddon,
  InputGroupSelect,
  InputGroupText,
} from '@/components/ui/input-group'
import { Switch } from '@/components/ui/switch'
import type { MapProfile, MapTrinketProfile, SeedReport } from '@/lib/spd-wasm'
import { changeSeedMapProfile } from '@/stores/app'

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

type TrinketKind = 'none' | 'mossy_clump' | 'trap_mechanism' | 'mimic_tooth'

function trinketParts(profile: MapTrinketProfile): {
  kind: TrinketKind
  level: number
} {
  if (profile === 'no_map_affecting_trinkets') return { kind: 'none', level: 0 }
  const match = profile.match(
    /^(mossy_clump|trap_mechanism|mimic_tooth)([0-3])$/
  )
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
}: {
  report: SeedReport
  sessionId: string
  mapProfile: MapProfile
  refreshingLayout: boolean
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
      <CardContent>
        <FieldSet>
          <FieldLegend>Run settings</FieldLegend>
          <FieldGroup>
            <Field orientation="horizontal">
              <FieldContent>
                <FieldLabel htmlFor={`${sessionId}-forbidden-runes`}>
                  Forbidden Runes
                </FieldLabel>
                <FieldDescription>
                  Removes every second guaranteed Scroll of Upgrade and
                  recalculates later generation.
                </FieldDescription>
              </FieldContent>
              <Switch
                id={`${sessionId}-forbidden-runes`}
                checked={mapProfile.forbidden_runes}
                disabled={refreshingLayout}
                onCheckedChange={(forbidden_runes) =>
                  updateMapProfile({ forbidden_runes })
                }
              />
            </Field>
            <Field>
              <FieldLabel htmlFor={`${sessionId}-trinket`}>
                Held trinket
              </FieldLabel>
              <div className="flex flex-col gap-2 sm:flex-row sm:items-center">
                <InputGroup className="sm:w-52 sm:flex-none">
                  <InputGroupSelect
                    id={`${sessionId}-trinket`}
                    aria-label="Trinket"
                    value={kind}
                    disabled={refreshingLayout}
                    startContent={
                      kind !== 'none' ? (
                        <ItemIcon
                          classNameItem={
                            kind === 'mossy_clump'
                              ? 'MossyClump'
                              : kind === 'trap_mechanism'
                                ? 'TrapMechanism'
                                : 'MimicTooth'
                          }
                          size={20}
                          title={
                            kind === 'mossy_clump'
                              ? 'Mossy Clump'
                              : kind === 'trap_mechanism'
                                ? 'Trap Mechanism'
                                : 'Mimic Tooth'
                          }
                        />
                      ) : undefined
                    }
                    onChange={(event) => {
                      const nextKind = event.target.value as TrinketKind
                      updateMapProfile({
                        trinket: profileTrinket(nextKind, level),
                      })
                    }}
                  >
                    <option value="none">None</option>
                    <option value="mossy_clump">Mossy Clump</option>
                    <option value="trap_mechanism">Trap Mechanism</option>
                    <option value="mimic_tooth">Mimic Tooth</option>
                  </InputGroupSelect>
                  {refreshingLayout ? (
                    <InputGroupAddon align="inline-end" role="status">
                      <SpinnerGapIcon className="animate-spin" aria-hidden />
                      <span className="sr-only">Regenerating analysis…</span>
                    </InputGroupAddon>
                  ) : null}
                </InputGroup>
                <InputGroup
                  className="sm:w-56 sm:flex-none"
                  data-disabled={kind === 'none'}
                >
                  <InputGroupAddon>
                    <InputGroupText>Upgrade</InputGroupText>
                  </InputGroupAddon>
                  <InputGroupSelect
                    aria-label="Upgrade level"
                    value={level}
                    disabled={refreshingLayout || kind === 'none'}
                    onChange={(event) =>
                      updateMapProfile({
                        trinket: profileTrinket(
                          kind,
                          Number(event.target.value)
                        ),
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
                    disabled={refreshingLayout || kind === 'none'}
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
                The trinket is held starting when the selected floor is
                generated. Mimic Tooth also recalculates Sad Ghost rewards;
                changing any run setting regenerates this seed’s analysis.
              </FieldDescription>
            </Field>
          </FieldGroup>
        </FieldSet>
      </CardContent>
    </Card>
  )
}
