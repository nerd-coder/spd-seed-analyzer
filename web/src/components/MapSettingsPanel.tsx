import {
  MagnifyingGlassMinus,
  MagnifyingGlassPlus,
} from '@phosphor-icons/react'
import type { ReactNode } from 'react'

import { Button } from '@/components/ui/button'
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from '@/components/ui/tooltip'

type Props = {
  canZoomIn: boolean
  canZoomOut: boolean
  onZoomIn: () => void
  onZoomOut: () => void
}

export function MapSettingsPanel({
  canZoomIn,
  canZoomOut,
  onZoomIn,
  onZoomOut,
}: Props) {
  return (
    <div
      className="dark absolute top-2 left-2 z-10 flex items-center gap-0.5 bg-background/30 p-1 text-foreground shadow-sm ring-1 ring-foreground/15 backdrop-blur-[2px]"
      data-testid="map-settings-panel"
    >
      <ZoomButton
        label="Zoom map out"
        tooltip="Zoom out"
        enabled={canZoomOut}
        onClick={onZoomOut}
      >
        <MagnifyingGlassMinus />
      </ZoomButton>
      <ZoomButton
        label="Zoom map in"
        tooltip="Zoom in"
        enabled={canZoomIn}
        onClick={onZoomIn}
      >
        <MagnifyingGlassPlus />
      </ZoomButton>
    </div>
  )
}

function ZoomButton({
  label,
  tooltip,
  enabled,
  onClick,
  children,
}: {
  label: string
  tooltip: string
  enabled: boolean
  onClick: () => void
  children: ReactNode
}) {
  return (
    <Tooltip>
      <TooltipTrigger asChild>
        <Button
          variant="ghost"
          size="icon-sm"
          aria-label={label}
          aria-disabled={!enabled}
          className="aria-disabled:opacity-40"
          onClick={() => {
            if (enabled) onClick()
          }}
        >
          {children}
        </Button>
      </TooltipTrigger>
      <TooltipContent className="dark">{tooltip}</TooltipContent>
    </Tooltip>
  )
}
