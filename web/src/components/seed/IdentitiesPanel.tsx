import { useStore } from '@tanstack/react-store'
import { finderItemLabel } from '@/components/finder/finder-items'
import { ItemBox } from '@/components/ItemBox'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { appearanceDescription } from '@/lib/identity'
import { ItemCategory, itemSprite } from '@/lib/item-kind'
import type { IdentityEntry, IdentityMaps } from '@/lib/spd-wasm'
import { $reportNavigation, setIdentitiesTab } from '@/stores/app'

function IdentityGrid({
  entries,
  category,
}: {
  entries: IdentityEntry[]
  category: ItemCategory
}) {
  return (
    <div className="flex flex-wrap gap-1">
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
            scale={3}
            title={`${name} (${appearance})`}
            caption={appearance}
          />
        )
      })}
    </div>
  )
}

export function IdentitiesPanel({ identities }: { identities: IdentityMaps }) {
  const identitiesTab = useStore(
    $reportNavigation,
    (state) => state.identitiesTab
  )

  return (
    <section className="bg-card text-card-foreground ring-1 ring-foreground/10">
      <div className="space-y-1 px-4 pt-4">
        <h2 className="font-heading text-sm font-medium">Identities</h2>
        <p className="text-muted-foreground text-xs leading-relaxed">
          Unidentified appearances for this seed (from run init RNG).
        </p>
      </div>
      <div className="px-4 pt-3 pb-4">
        <Tabs
          value={identitiesTab}
          onValueChange={(value) => {
            if (
              value === 'potions' ||
              value === 'scrolls' ||
              value === 'rings'
            ) {
              setIdentitiesTab(value)
            }
          }}
        >
          <TabsList className="h-auto w-full flex-wrap">
            <TabsTrigger value="potions">Potions</TabsTrigger>
            <TabsTrigger value="scrolls">Scrolls</TabsTrigger>
            <TabsTrigger value="rings">Rings</TabsTrigger>
          </TabsList>
          <TabsContent value="potions" className="mt-3">
            <IdentityGrid
              entries={identities.potions}
              category={ItemCategory.Potion}
            />
          </TabsContent>
          <TabsContent value="scrolls" className="mt-3">
            <IdentityGrid
              entries={identities.scrolls}
              category={ItemCategory.Scroll}
            />
          </TabsContent>
          <TabsContent value="rings" className="mt-3">
            <IdentityGrid
              entries={identities.rings}
              category={ItemCategory.Ring}
            />
          </TabsContent>
        </Tabs>
      </div>
    </section>
  )
}
