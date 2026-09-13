import { InfoIcon, XIcon } from '@phosphor-icons/react'
import { useStore } from '@tanstack/react-store'
import { Alert, AlertAction, AlertTitle } from '@/components/ui/alert'
import { Button } from '@/components/ui/button'
import {
  $dismissedVersionAlert,
  $meta,
  dismissVersionAlert,
} from '@/stores/app'

const SPD_REPO = 'https://github.com/00-Evan/shattered-pixel-dungeon'

export function SupportedVersionAlert() {
  const meta = useStore($meta)
  const dismissed = useStore($dismissedVersionAlert)
  if (!meta || dismissed === meta.version) return null

  return (
    <Alert variant="info" className="items-center py-1.5 pr-8" role="status">
      <InfoIcon />
      <AlertTitle className="whitespace-nowrap">
        Supports{' '}
        <a
          href={`${SPD_REPO}/releases/tag/${meta.version}`}
          target="_blank"
          rel="noreferrer"
        >
          SPD {meta.version}
        </a>
      </AlertTitle>
      <AlertAction className="top-1/2 right-1 -translate-y-1/2">
        <Button
          type="button"
          variant="ghost"
          size="icon-xs"
          aria-label="Dismiss supported version"
          onClick={() => dismissVersionAlert(meta.version)}
        >
          <XIcon />
        </Button>
      </AlertAction>
    </Alert>
  )
}
