import { finderItemLabel } from '@/components/finder/finder-items'
import { ItemBox } from '@/components/ItemBox'
import { appearanceDescription } from '@/lib/identity'
import { ItemCategory, itemSprite } from '@/lib/item-kind'
import type { IdentityEntry, IdentityMaps } from '@/lib/spd-wasm'

/** Journal cell at scale 3: 17×3 + 8px padding. */
const IDENTITY_SCALE = 3

function IdentityGrid({
  entries,
  category,
}: {
  entries: IdentityEntry[]
  category: ItemCategory
}) {
  return (
    <div className="flex flex-wrap gap-0.5">
      {entries.map((e) => {
        const name = finderItemLabel(e.item)
        const appearance = appearanceDescription(e.appearance)
        return (
          <ItemBox
            key={e.item}
            sprites={itemSprite({
              className: e.item,
              category,
              appearance: e.appearance,
            })}
            scale={IDENTITY_SCALE}
            title={`${name} (${appearance})`}
            caption={appearance}
          />
        )
      })}
    </div>
  )
}

function IdentityGroup({
  title,
  entries,
  category,
}: {
  title: string
  entries: IdentityEntry[]
  category: ItemCategory
}) {
  return (
    <div className="flex flex-col gap-1">
      <h3 className="text-muted-foreground text-xs font-medium tracking-wide uppercase">
        {title}
      </h3>
      <IdentityGrid entries={entries} category={category} />
    </div>
  )
}

export function IdentitiesPanel({ identities }: { identities: IdentityMaps }) {
  return (
    <section className="bg-card text-card-foreground ring-1 ring-foreground/10">
      <div className="space-y-1 px-4 pt-4">
        <h2 className="font-heading text-sm font-medium">Identities</h2>
        <p className="text-muted-foreground text-xs leading-relaxed">
          Unidentified appearances for this seed
        </p>
      </div>
      <div className="flex flex-col gap-3 px-4 pt-3 pb-4">
        <IdentityGroup
          title="Potions"
          entries={identities.potions}
          category={ItemCategory.Potion}
        />
        <IdentityGroup
          title="Scrolls"
          entries={identities.scrolls}
          category={ItemCategory.Scroll}
        />
        <IdentityGroup
          title="Rings"
          entries={identities.rings}
          category={ItemCategory.Ring}
        />
      </div>
    </section>
  )
}
