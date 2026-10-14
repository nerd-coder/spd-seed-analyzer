import { useStore } from '@tanstack/react-store'
import { oklchToHex, readToken } from '@/lib/color'
import { $theme } from '@/lib/preview-store'
import { cn } from '@/lib/utils'

/** Reads tokens on every theme change, so values match what is on screen. */
export function useTokens(tokens: readonly string[]) {
  useStore($theme)
  return tokens.map((token) => {
    const value = readToken(token)
    return { token, value, hex: oklchToHex(value) }
  })
}

export type SwatchSpec = { token: string; name: string; role: string }

export function SwatchGrid({
  swatches,
  className,
}: {
  swatches: readonly SwatchSpec[]
  className?: string
}) {
  const values = useTokens(swatches.map((swatch) => swatch.token))
  return (
    <ul
      className={cn(
        'grid grid-cols-2 gap-x-4 gap-y-6 sm:grid-cols-3 xl:grid-cols-5',
        className
      )}
    >
      {swatches.map((swatch, i) => (
        <li key={swatch.token} className="flex flex-col gap-2">
          <div
            className="h-14 border border-foreground/15"
            style={{ background: `var(${swatch.token})` }}
          />
          <div className="flex flex-col gap-0.5 text-sm">
            <span className="font-semibold">{swatch.name}</span>
            <code className="font-mono text-xs text-muted-foreground">
              {swatch.token}
            </code>
            <span className="text-xs text-muted-foreground tabular-nums">
              {values[i]?.hex ?? values[i]?.value}
            </span>
            <span className="text-muted-foreground">{swatch.role}</span>
          </div>
        </li>
      ))}
    </ul>
  )
}
