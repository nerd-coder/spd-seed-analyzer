import {
  MagnifyingGlassMinus,
  MagnifyingGlassPlus,
} from '@phosphor-icons/react'

import { Button } from '@/components/ui/button'
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from '@/components/ui/tooltip'

type Props = {
  zoom: string
  onZoomChange: (zoom: string) => void
}

export function MapSettingsPanel({ zoom, onZoomChange }: Props) {
  const isZoomed = zoom === '2'

  return (
    <div
      className="dark absolute top-2 left-2 z-10 flex items-center gap-0.5 bg-background/30 p-1 text-foreground shadow-sm ring-1 ring-foreground/15 backdrop-blur-[2px]"
      data-testid="map-settings-panel"
    >
      <Tooltip>
        <TooltipTrigger asChild>
          <Button
            variant="ghost"
            size="icon-sm"
            onClick={() => onZoomChange(isZoomed ? '1' : '2')}
            aria-label={`Switch map to ${isZoomed ? '1x' : '2x'} zoom`}
          >
            {isZoomed ? <MagnifyingGlassMinus /> : <MagnifyingGlassPlus />}
          </Button>
        </TooltipTrigger>
        <TooltipContent className="dark">
          Switch to {isZoomed ? '1×' : '2×'} zoom
        </TooltipContent>
      </Tooltip>
    </div>
  )
}
