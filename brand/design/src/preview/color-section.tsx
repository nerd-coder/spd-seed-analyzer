import { ItemSlot } from '@/components/spd/item-slot'
import { REGIONS, textureUrl } from '@/lib/regions'
import { cn } from '@/lib/utils'
import { Section, Subsection } from '@/preview/section'
import { SwatchGrid, type SwatchSpec, useTokens } from '@/preview/swatches'

const STONE: SwatchSpec[] = [
  { token: '--background', name: 'Ground', role: 'Page' },
  { token: '--card', name: 'Slab', role: 'Windows, report frame' },
  { token: '--popover', name: 'Raised slab', role: 'Dialogs, popovers' },
  { token: '--muted', name: 'Hollow', role: 'Slots, trays, wells' },
  { token: '--secondary', name: 'Stone', role: 'Secondary buttons' },
  { token: '--border', name: 'Mortar', role: 'Dividers, frames' },
  { token: '--input', name: 'Chisel', role: 'Control edges, 3:1' },
  { token: '--muted-foreground', name: 'Lichen', role: 'Secondary text' },
  { token: '--foreground', name: 'Ink', role: 'Body text' },
]

const LANTERN: SwatchSpec[] = [
  { token: '--primary', name: 'Lantern', role: 'Primary action, focus' },
  { token: '--accent', name: 'Lantern wash', role: 'Pressed, quest band' },
  { token: '--ring', name: 'Ring', role: 'Keyboard focus' },
]

const STATUS: SwatchSpec[] = [
  { token: '--destructive', name: 'Curse', role: 'Errors, removal' },
  { token: '--warning', name: 'Warning', role: 'Partial results' },
  { token: '--info', name: 'Info', role: 'Version notes' },
]

const TIERS = [1, 2, 3, 4, -1] as const

function RegionRows() {
  const values = useTokens(REGIONS.map((region) => region.token))
  return (
    <ul className="flex flex-col divide-y border-y">
      {REGIONS.map((region, i) => (
        <li
          key={region.id}
          className="grid grid-cols-[4rem_1fr] items-center gap-x-4 gap-y-2 py-3 sm:grid-cols-[4rem_10rem_1fr_12rem]"
        >
          <div
            className={cn(
              'grid h-14 place-items-center font-pixel text-px-1 text-region-foreground inset-shadow-raised',
              region.fill
            )}
          >
            {region.floors[0]}
          </div>
          <div className="flex flex-col">
            <span className="font-semibold">{region.name}</span>
            <span className="text-sm text-muted-foreground tabular-nums">
              Floors {region.floors[0]}–{region.floors[1]}
            </span>
          </div>
          <p className="col-span-2 text-sm text-muted-foreground sm:col-span-1">
            {region.source}{' '}
            <code className="font-mono text-xs">
              {region.token} {values[i]?.hex}
            </code>
          </p>
          <div
            aria-hidden
            className="pixelated col-span-2 h-10 border sm:col-span-1"
            style={{
              backgroundImage: `url(${textureUrl(region.id)})`,
              backgroundSize: '256px 64px',
            }}
          />
        </li>
      ))}
    </ul>
  )
}

export function ColorSection() {
  return (
    <Section
      id="color"
      intro="Stone for the interface, lantern light for action, and one hue per region of the dungeon. Hue always carries meaning: depth, loot tier, or status. Switch the theme to see both palettes."
    >
      <Subsection
        title="Stone"
        note="Neutrals sampled from the game's window chrome and floor tiles, with a faint green cast. Surfaces step lighter as they rise toward the player."
      >
        <SwatchGrid swatches={STONE} />
      </Subsection>
      <Subsection
        title="Lantern"
        note="The game titles every window in yellow. Here that yellow marks the one thing to do next: the primary button, the selected floor, keyboard focus."
      >
        <SwatchGrid swatches={LANTERN} />
      </Subsection>
      <Subsection
        title="Regions"
        note="Five hues, one per region, identical in both themes. They tint floor numbers, banners and the descent rail, and never decorate anything else. Text on a region fill is always ink-dark."
      >
        <RegionRows />
      </Subsection>
      <Subsection
        title="Loot tiers"
        note="The upgrade on a slot is drawn in the game's pixel face and colored by tier: +1, +2, +3, then +4 and above. A negative level takes the curse red, and so does the frame of cursed gear."
      >
        <div className="flex flex-wrap gap-3">
          {TIERS.map((level) => (
            <ItemSlot
              key={level}
              item={{
                sprite: level < 0 ? 'mailArmor' : 'ringSapphire',
                name: level < 0 ? 'Mail Armor' : 'Sapphire ring',
                level,
                cursed: level < 0,
              }}
              caption={level < 0 ? 'Degraded' : `Tier ${level}`}
            />
          ))}
        </div>
      </Subsection>
      <Subsection title="Status">
        <SwatchGrid swatches={STATUS} />
      </Subsection>
    </Section>
  )
}
