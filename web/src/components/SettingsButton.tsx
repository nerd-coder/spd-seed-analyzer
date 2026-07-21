import { useStore } from '@nanostores/react'
import { Settings } from 'lucide-react'
import { useId } from 'react'

import { SpoilerToggle } from '@/components/seed/SpoilerToggle'
import { Button } from '@/components/ui/button'
import {
  Popover,
  PopoverContent,
  PopoverDescription,
  PopoverHeader,
  PopoverTitle,
  PopoverTrigger,
} from '@/components/ui/popover'
import { cn } from '@/lib/utils'
import {
  $identitySpoilers,
  $mapSpoilers,
  setIdentitySpoilers,
  setMapSpoilers,
} from '@/stores/app'

type SettingsButtonProps = {
  className?: string
}

export function SettingsButton({ className }: SettingsButtonProps) {
  const uid = useId()
  const mapSpoilers = useStore($mapSpoilers)
  const identitySpoilers = useStore($identitySpoilers)

  return (
    <Popover>
      <PopoverTrigger asChild>
        <Button
          type="button"
          variant="outline"
          size="icon-sm"
          className={cn('bg-background/90 shadow-sm backdrop-blur', className)}
          aria-label="Settings"
        >
          <Settings />
        </Button>
      </PopoverTrigger>
      <PopoverContent align="end" className="w-64">
        <PopoverHeader>
          <PopoverTitle>Settings</PopoverTitle>
          <PopoverDescription>
            These options reveal seed secrets. Leave them off if you want to
            keep exploration surprises.
          </PopoverDescription>
        </PopoverHeader>
        <div className="space-y-3">
          <SpoilerToggle
            id={`${uid}-identity-spoilers`}
            label="Identities"
            info="Reveals potion, scroll, and ring color/rune/gem → type mappings for the active seed."
            checked={identitySpoilers}
            onCheckedChange={setIdentitySpoilers}
          />
          <SpoilerToggle
            id={`${uid}-map-spoilers`}
            label="Floor maps"
            info="Shows 128×128 floor map thumbnails (click to expand). Heavily spoils layout before you play."
            checked={mapSpoilers}
            onCheckedChange={setMapSpoilers}
          />
        </div>
      </PopoverContent>
    </Popover>
  )
}
