import {
  MagnifyingGlass,
  MagnifyingGlassPlus,
  TreasureChest,
  UsersThree,
} from '@phosphor-icons/react'

import { Toggle } from '@/components/ui/toggle'
import { ToggleGroup, ToggleGroupItem } from '@/components/ui/toggle-group'
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
  return (
    <div
      className="dark absolute top-2 left-2 z-10 flex items-center gap-1 bg-background/90 p-1 text-foreground shadow-md ring-1 ring-foreground/10 backdrop-blur-sm"
      data-testid="map-settings-panel"
    >
      <ToggleGroup
        type="single"
        variant="outline"
        size="sm"
        spacing={0}
        value={zoom}
        onValueChange={(value) => {
          if (value) onZoomChange(value)
        }}
        aria-label="Map zoom"
      >
        <Tooltip>
          <TooltipTrigger asChild>
            <ToggleGroupItem value="1" aria-label="Zoom map to 1x">
              <MagnifyingGlass />
            </ToggleGroupItem>
          </TooltipTrigger>
          <TooltipContent className="dark">1× zoom</TooltipContent>
        </Tooltip>
        <Tooltip>
          <TooltipTrigger asChild>
            <ToggleGroupItem value="2" aria-label="Zoom map to 2x">
              <MagnifyingGlassPlus />
            </ToggleGroupItem>
          </TooltipTrigger>
          <TooltipContent className="dark">2× zoom</TooltipContent>
        </Tooltip>
      </ToggleGroup>
      {itemMarkers > 0 && (
        <Tooltip>
          <TooltipTrigger asChild>
            <Toggle
              variant="outline"
              size="sm"
              pressed={showItems}
              onPressedChange={onShowItemsChange}
              aria-label={`Show items (${itemMarkers})`}
            >
              <TreasureChest />
            </Toggle>
          </TooltipTrigger>
          <TooltipContent className="dark">
            Items ({itemMarkers}) · engine-confirmed cells only
          </TooltipContent>
        </Tooltip>
      )}
      {mobMarkers > 0 && (
        <Tooltip>
          <TooltipTrigger asChild>
            <Toggle
              variant="outline"
              size="sm"
              pressed={showMobs}
              onPressedChange={onShowMobsChange}
              aria-label={`Show known mobs (${mobMarkers})`}
            >
              <UsersThree />
            </Toggle>
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
