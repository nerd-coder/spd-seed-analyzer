import { SpinnerGapIcon } from '@phosphor-icons/react'
import { useStore } from '@tanstack/react-store'
import { Settings } from 'lucide-react'
import { useId } from 'react'

import { SpoilerToggle } from '@/components/seed/SpoilerToggle'
import { Button } from '@/components/ui/button'
import { Field, FieldDescription, FieldLabel } from '@/components/ui/field'
import { NativeSelect, NativeSelectOption } from '@/components/ui/native-select'
import {
  Popover,
  PopoverContent,
  PopoverDescription,
  PopoverHeader,
  PopoverTitle,
  PopoverTrigger,
} from '@/components/ui/popover'
import type { MapTrinketProfile } from '@/lib/spd-wasm'
import { cn } from '@/lib/utils'
import {
  $analyzing,
  $identitySpoilers,
  $mapSpoilers,
  $mapTrinket,
  changeMapTrinket,
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
  const mapTrinket = useStore($mapTrinket)
  const analyzing = useStore($analyzing)

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
      <PopoverContent align="start" className="w-sm max-w-svw">
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
            info="Shows deterministic floor-layout thumbnails. Heavily spoils exploration."
            checked={mapSpoilers}
            onCheckedChange={setMapSpoilers}
          />
          <Field>
            <FieldLabel htmlFor={`${uid}-map-trinket`}>
              Floor-layout trinket
            </FieldLabel>
            <NativeSelect
              id={`${uid}-map-trinket`}
              className="w-full"
              value={mapTrinket}
              disabled={analyzing}
              onChange={(event) =>
                void changeMapTrinket(event.target.value as MapTrinketProfile)
              }
            >
              <NativeSelectOption value="no_map_affecting_trinkets">
                None (default)
              </NativeSelectOption>
              {[0, 1, 2, 3].map((level) => (
                <NativeSelectOption
                  key={`mossy-${level}`}
                  value={`mossy_clump${level}`}
                >
                  Mossy Clump +{level}
                </NativeSelectOption>
              ))}
              {[0, 1, 2, 3].map((level) => (
                <NativeSelectOption
                  key={`trap-${level}`}
                  value={`trap_mechanism${level}`}
                >
                  Trap Mechanism +{level}
                </NativeSelectOption>
              ))}
            </NativeSelect>
            <FieldDescription>
              Applies to the complete run. Changing it regenerates every open
              seed map.
            </FieldDescription>
            {analyzing ? (
              <p className="flex items-center gap-1 text-xs text-muted-foreground">
                <SpinnerGapIcon className="animate-spin" aria-hidden />
                Regenerating maps…
              </p>
            ) : null}
          </Field>
        </div>
      </PopoverContent>
    </Popover>
  )
}
