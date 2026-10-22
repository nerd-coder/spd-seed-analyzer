import { CheckIcon, CopyIcon } from '@phosphor-icons/react'
import { useEffect, useState } from 'react'
import { GameWindow } from '@/components/GameWindow'
import { Button } from '@/components/ui/button'
import type { SeedInfo } from '@/lib/spd-wasm'

const COPIED_MS = 2000

/** The seed in the game's pixel font inside a game window, with a copy action. */
export function SeedTitle({ seed }: { seed: SeedInfo }) {
  const text = seed.code ?? seed.formatted
  const [copied, setCopied] = useState(false)
  const [failed, setFailed] = useState(false)

  useEffect(() => {
    if (!copied) return
    const timer = window.setTimeout(() => setCopied(false), COPIED_MS)
    return () => window.clearTimeout(timer)
  }, [copied])

  async function copy() {
    try {
      await navigator.clipboard.writeText(text)
      setFailed(false)
      setCopied(true)
    } catch {
      setFailed(true)
    }
  }

  return (
    <div className="flex flex-col items-start gap-2">
      <GameWindow className="max-w-full">
        <div className="flex flex-wrap items-center gap-x-4 gap-y-2 px-2 py-1">
          <h2
            data-seed-title
            className="font-pixel text-px-2 lg:text-px-3 min-w-0 break-all select-all"
          >
            {text}
          </h2>
          <Button
            type="button"
            variant="outline"
            size="sm"
            onClick={() => void copy()}
            aria-label={copied ? `Copied ${text}` : `Copy seed ${text}`}
          >
            {copied ? (
              <CheckIcon data-icon="inline-start" weight="bold" />
            ) : (
              <CopyIcon data-icon="inline-start" />
            )}
            {copied ? 'Copied' : 'Copy seed'}
          </Button>
        </div>
      </GameWindow>
      <span className="sr-only" aria-live="polite">
        {copied ? `Copied ${text} to the clipboard` : ''}
      </span>
      {failed ? (
        <p className="text-destructive w-full text-sm" role="alert">
          The clipboard is blocked here. Select the seed and copy it instead.
        </p>
      ) : null}
    </div>
  )
}
