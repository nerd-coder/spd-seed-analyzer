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
import { Switch } from '@/components/ui/switch'
import type { MapProfile, SeedReport } from '@/lib/spd-wasm'
import { changeSeedMapProfile } from '@/stores/app'
import { HeldTrinketSettings } from './HeldTrinketSettings'

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
            <HeldTrinketSettings
              sessionId={sessionId}
              profile={mapProfile}
              selection={report.trinket_selection}
              disabled={refreshingLayout}
              onChange={(profile) =>
                void changeSeedMapProfile(sessionId, profile)
              }
            />
          </FieldGroup>
        </FieldSet>
      </CardContent>
    </Card>
  )
}
