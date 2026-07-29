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
import type { MapProfile, SeedReport } from '@/lib/spd-wasm'
import { changeSeedMapProfile } from '@/stores/app'
import { ArtifactEventSettings } from './ArtifactEventSettings'
import { ChallengeSettings } from './ChallengeSettings'
import { TrinketEventSettings } from './TrinketEventSettings'

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
            <Field>
              <FieldContent>
                <FieldLabel>First-generation main-path profile</FieldLabel>
                <FieldDescription>
                  Record the run state known before each floor is first
                  generated. These settings never infer combat or other runtime
                  actions.
                </FieldDescription>
              </FieldContent>
            </Field>
            <ChallengeSettings
              sessionId={sessionId}
              profile={mapProfile}
              disabled={refreshingLayout}
              onChange={(profile) =>
                void changeSeedMapProfile(sessionId, profile)
              }
            />
            <TrinketEventSettings
              sessionId={sessionId}
              profile={mapProfile}
              selection={report.trinket_selection}
              disabled={refreshingLayout}
              onChange={(profile) =>
                void changeSeedMapProfile(sessionId, profile)
              }
            />
            <ArtifactEventSettings
              profile={mapProfile}
              disabled={refreshingLayout}
              onChange={(profile) =>
                void changeSeedMapProfile(sessionId, profile)
              }
            />
            <Field orientation="horizontal">
              <FieldContent>
                <FieldLabel htmlFor={`${sessionId}-parchment-scrap`}>
                  Parchment Scrap when claiming Ghost reward
                </FieldLabel>
                <FieldDescription>
                  This claim-only state does not change floor generation.
                </FieldDescription>
              </FieldContent>
              <InputGroup className="w-36 shrink-0">
                <InputGroupAddon>
                  <InputGroupText>Level</InputGroupText>
                </InputGroupAddon>
                <InputGroupSelect
                  id={`${sessionId}-parchment-scrap`}
                  aria-label="Parchment Scrap level when claiming Ghost reward"
                  value={mapProfile.claim_state.parchment_scrap_level ?? ''}
                  disabled={refreshingLayout}
                  onChange={(event) =>
                    updateMapProfile({
                      claim_state: {
                        parchment_scrap_level:
                          event.target.value === ''
                            ? undefined
                            : Number(event.target.value),
                      },
                    })
                  }
                >
                  <option value="">Not held</option>
                  <option value={0}>+0</option>
                  <option value={1}>+1</option>
                  <option value={2}>+2</option>
                  <option value={3}>+3</option>
                </InputGroupSelect>
              </InputGroup>
            </Field>
          </FieldGroup>
        </FieldSet>
      </CardContent>
    </Card>
  )
}
