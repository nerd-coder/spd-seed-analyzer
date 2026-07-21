import { ItemIcon } from '@/components/ItemIcon'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { appearanceDescription, shortIdentityName } from '@/lib/identity'
import type { IdentityEntry, IdentityMaps } from '@/lib/spd-wasm'

function IdentityGrid({
  entries,
  category,
}: {
  entries: IdentityEntry[]
  category: 'potion' | 'scroll' | 'ring'
}) {
  return (
    <div className="grid grid-cols-2 gap-x-3 gap-y-2">
      {entries.map((e) => {
        const shortName = shortIdentityName(e.name, category)
        return (
          <div key={e.item} className="flex min-w-0 items-center gap-2">
            <ItemIcon
              classNameItem={e.item}
              category={category}
              appearance={e.appearance}
              size={24}
              title={e.name}
              className="shrink-0"
            />
            <div className="min-w-0 leading-tight">
              <div className="truncate text-sm font-medium" title={e.name}>
                {shortName}
              </div>
              <div className="text-muted-foreground truncate text-xs capitalize">
                {appearanceDescription(category, e.appearance)}
              </div>
            </div>
          </div>
        )
      })}
    </div>
  )
}

export function IdentitiesPanel({ identities }: { identities: IdentityMaps }) {
  // Sticky under seed tabs (same offset as region tabs). Not using Card
  // overflow-hidden so position:sticky on the outer wrapper stays valid.
  return (
    <section className="bg-card text-card-foreground ring-1 ring-foreground/10">
      <div className="space-y-1 px-4 pt-4">
        <h2 className="font-heading text-sm font-medium">Identities</h2>
        <p className="text-muted-foreground text-xs leading-relaxed">
          Unidentified appearances for this seed (from run init RNG).
        </p>
      </div>
      <div className="px-4 pt-3 pb-4">
        <Tabs defaultValue="potions">
          <TabsList className="h-auto w-full flex-wrap">
            <TabsTrigger value="potions">Potions</TabsTrigger>
            <TabsTrigger value="scrolls">Scrolls</TabsTrigger>
            <TabsTrigger value="rings">Rings</TabsTrigger>
          </TabsList>
          <TabsContent value="potions" className="mt-3">
            <IdentityGrid entries={identities.potions} category="potion" />
          </TabsContent>
          <TabsContent value="scrolls" className="mt-3">
            <IdentityGrid entries={identities.scrolls} category="scroll" />
          </TabsContent>
          <TabsContent value="rings" className="mt-3">
            <IdentityGrid entries={identities.rings} category="ring" />
          </TabsContent>
        </Tabs>
      </div>
    </section>
  )
}
