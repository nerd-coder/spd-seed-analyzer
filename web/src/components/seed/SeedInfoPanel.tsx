import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card'
import type { SeedReport } from '@/lib/spd-wasm'

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

export function SeedInfoPanel({ report }: { report: SeedReport }) {
  const { input, code, formatted, numeric } = report.seed
  const showCustomAndCanonical = customDiffersFromCanonical(input, code)

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
      {report.message && (
        <CardContent>
          <p className="text-muted-foreground text-xs leading-relaxed">
            {report.message}
          </p>
        </CardContent>
      )}
    </Card>
  )
}
