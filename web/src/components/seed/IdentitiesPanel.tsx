import { GameSprite } from '@/components/GameSprite'
import { ItemBox } from '@/components/ItemBox'
import { itemClassName } from '@/generated/game-ids'
import { appearanceDescription } from '@/lib/identity'
import { ItemCategory, itemSprite } from '@/lib/item-kind'
import { itemLabel } from '@/lib/item-title'
import type { IdentityEntry, IdentityMaps } from '@/lib/spd-wasm'
import { CATALOG_ICON, UI_ICONS } from '@/lib/ui-icons'

const ICON_SHEET = {
  url: UI_ICONS.url,
  width: UI_ICONS.sheetW,
  height: UI_ICONS.sheetH,
}

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
        const name = itemLabel(e.item)
        const appearance = appearanceDescription(e.appearance)
        return (
          <ItemBox
            key={e.item}
            sprites={itemSprite({
              className: itemClassName(e.item),
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
    <div className="flex flex-col gap-2">
      <h4 className="text-sm font-semibold">{title}</h4>
      <IdentityGrid entries={entries} category={category} />
    </div>
  )
}

export function IdentitiesPanel({ identities }: { identities: IdentityMaps }) {
  return (
    <section
      data-identities
      aria-labelledby="identities-title"
      className="flex flex-col gap-3"
    >
      <div className="flex items-center gap-2">
        <GameSprite
          sheet={ICON_SHEET}
          frames={[CATALOG_ICON]}
          scale={2}
          label="Item catalog"
        />
        <h3 id="identities-title" className="font-pixel text-px-1">
          Identities
        </h3>
        <span className="text-muted-foreground text-sm">
          Unidentified appearances for this seed
        </span>
      </div>
      <div className="flex flex-col gap-4">
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
