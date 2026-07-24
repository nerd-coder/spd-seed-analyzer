import {
  MagnifyingGlassMinus,
  MagnifyingGlassPlus,
  TreasureChest,
  UsersThree,
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
  itemMarkers: number
  showItems: boolean
  onShowItemsChange: (show: boolean) => void
  mobMarkers: number
  showMobs: boolean
  onShowMobsChange: (show: boolean) => void
}

export function MapSettingsPanel({
  zoom,
  onZoomChange,
  itemMarkers,
  showItems,
  onShowItemsChange,
  mobMarkers,
  showMobs,
  onShowMobsChange,
}: Props) {
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
      {itemMarkers > 0 && (
        <Tooltip>
          <TooltipTrigger asChild>
            <Button
              variant="ghost"
              size="icon-sm"
              aria-pressed={showItems}
              onClick={() => onShowItemsChange(!showItems)}
              aria-label={`Show items (${itemMarkers})`}
              className="aria-pressed:bg-muted"
            >
              <TreasureChest />
            </Button>
          </TooltipTrigger>
          <TooltipContent className="dark">
            Items ({itemMarkers}) · engine-confirmed cells only
          </TooltipContent>
        </Tooltip>
      )}
      {mobMarkers > 0 && (
        <Tooltip>
          <TooltipTrigger asChild>
            <Button
              variant="ghost"
              size="icon-sm"
              aria-pressed={showMobs}
              onClick={() => onShowMobsChange(!showMobs)}
              aria-label={`Show known mobs (${mobMarkers})`}
              className="aria-pressed:bg-muted"
            >
              <UsersThree />
            </Button>
          </TooltipTrigger>
          <TooltipContent className="dark">
            Known mobs ({mobMarkers}) · exact on depth 1, partial on some later
            floors
          </TooltipContent>
        </Tooltip>
      )}
    </div>
  )
}
