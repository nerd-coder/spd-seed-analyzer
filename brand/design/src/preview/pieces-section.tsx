import { FloorBanner } from '@/components/spd/floor-banner'
import { ItemSlot } from '@/components/spd/item-slot'
import type { SampleItem } from '@/lib/sample-seed'
import { Section, Subsection } from '@/preview/section'

const SLOT_STATES: { item: SampleItem; caption: string }[] = [
  {
    item: { sprite: 'hornOfPlenty', name: 'Horn of Plenty' },
    caption: 'Plain',
  },
  {
    item: { sprite: 'wandLightning', name: 'Wand of Lightning', level: 2 },
    caption: 'Upgraded',
  },
  {
    item: {
      sprite: 'longsword',
      name: 'Longsword of Blazing',
      level: 3,
      glow: '#ff4400',
    },
    caption: 'Enchanted',
  },
  {
    item: { sprite: 'mailArmor', name: 'Mail Armor', level: -1, cursed: true },
    caption: 'Cursed',
  },
  { item: { sprite: 'gold', name: 'Gold', quantity: 41 }, caption: 'Stack' },
]

const POTIONS: { sprite: SampleItem['sprite']; color: string }[] = [
  { sprite: 'potionCrimson', color: 'Crimson' },
  { sprite: 'potionAmber', color: 'Amber' },
  { sprite: 'potionGolden', color: 'Golden' },
  { sprite: 'potionJade', color: 'Jade' },
  { sprite: 'potionTurquoise', color: 'Turquoise' },
  { sprite: 'potionAzure', color: 'Azure' },
  { sprite: 'potionIndigo', color: 'Indigo' },
  { sprite: 'potionMagenta', color: 'Magenta' },
]

const BANNER_DEPTHS = [3, 8, 13, 20, 24] as const

export function PiecesSection() {
  return (
    <Section
      id="pieces"
      intro="Parts drawn from the game itself. They sit on the pixel grid, keep square corners, and use the game's own art at whole-number scales."
    >
      <Subsection
        title="Item slot"
        note="A 17px journal cell at 2×. The +N and stack size use the pixel face with the game's dark outline so they read over any sprite."
      >
        <div className="flex flex-wrap gap-4 border-y py-6">
          {SLOT_STATES.map((state) => (
            <ItemSlot
              key={state.caption}
              item={state.item}
              caption={state.caption}
            />
          ))}
        </div>
      </Subsection>
      <Subsection
        title="Identities"
        note="Unidentified potions, scrolls and rings carry their appearance as a caption, so the seed's color-to-potion mapping reads at a glance."
      >
        <div className="flex flex-wrap gap-2 border-y py-6">
          {POTIONS.map((potion) => (
            <ItemSlot
              key={potion.color}
              item={{ sprite: potion.sprite, name: `${potion.color} potion` }}
              caption={potion.color}
            />
          ))}
        </div>
      </Subsection>
      <Subsection
        title="Floor banner"
        note="The depth block takes the region fill; the strip behind the title is the region's own floor tiles, darkened so outlined white text reads in either theme."
      >
        <div className="flex flex-col gap-3">
          {BANNER_DEPTHS.map((depth) => (
            <FloorBanner key={depth} depth={depth} />
          ))}
        </div>
      </Subsection>
      <Subsection
        title="Descent rail"
        note="The strip at the top of this page. One cell per floor, grouped by region. A skull marks a boss, a square marks a quest, pips count notable items. It is the report's table of contents."
      >
        <p className="text-sm text-muted-foreground">
          <a
            href="#top"
            className="font-semibold text-foreground underline decoration-primary decoration-2 underline-offset-4"
          >
            Back to the descent rail
          </a>
        </p>
      </Subsection>
    </Section>
  )
}
