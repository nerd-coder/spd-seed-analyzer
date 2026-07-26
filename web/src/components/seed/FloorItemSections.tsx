import { ItemIcon } from '@/components/ItemIcon'
import { ItemName } from '@/components/ItemName'
import { Badge } from '@/components/ui/badge'
import { itemAppearance } from '@/lib/identity'
import { formatItemSource, isHighlightSource } from '@/lib/labels'
import type { FloorReport, IdentityMaps, ItemEntry } from '@/lib/spd-wasm'

function itemGroup(item: ItemEntry): 'shop' | 'quest' | 'general' {
  const source = item.source ?? ''
  if (source.includes('ShopRoom')) return 'shop'
  if (
    /Ghost\.Quest|Wandmaker\.Quest|Blacksmith\.Quest|Imp\.Quest/.test(source)
  ) {
    return 'quest'
  }
  return 'general'
}

export function FloorItemList({
  items,
  identities,
  identitySpoilers,
  depth,
}: {
  items: ItemEntry[]
  identities: IdentityMaps
  identitySpoilers: boolean
  depth: number
}) {
  return (
    <ul className="flex flex-col gap-1.5 text-sm">
      {items.map((item, index) => {
        const sourceLabel = formatItemSource(item.source)
        return (
          <li key={`${depth}-${index}`} className="flex items-start gap-2">
            <ItemIcon
              classNameItem={item.class_name}
              category={item.category}
              appearance={
                identitySpoilers ? itemAppearance(item, identities) : undefined
              }
              size={16}
              title={item.name}
              className="mt-0.5"
            />
            <span className="flex min-w-0 flex-wrap items-baseline gap-x-1.5 gap-y-0.5">
              <ItemName name={item.name} />
              {item.tier != null ? (
                <Badge variant="outline">tier {item.tier}</Badge>
              ) : null}
              {item.tier_range ? (
                <Badge variant="outline">
                  tier {item.tier_range.min}–{item.tier_range.max}
                </Badge>
              ) : null}
              {item.level_range ? (
                <Badge variant="outline">
                  +{item.level_range.min}…+{item.level_range.max}
                </Badge>
              ) : null}
              {item.cursed === true ? (
                <Badge variant="destructive">cursed</Badge>
              ) : null}
              {sourceLabel ? (
                <Badge
                  variant={
                    isHighlightSource(item.source) ? 'secondary' : 'outline'
                  }
                  title={item.source ?? undefined}
                >
                  {sourceLabel}
                </Badge>
              ) : null}
              {item.conditional_notes?.map((note) => (
                <Badge
                  key={note}
                  variant="secondary"
                  className="h-auto basis-full justify-start whitespace-normal py-1 font-normal"
                >
                  {note}
                </Badge>
              ))}
            </span>
          </li>
        )
      })}
    </ul>
  )
}

export function FloorItemSections({
  floor,
  identities,
  identitySpoilers,
}: {
  floor: FloorReport
  identities: IdentityMaps
  identitySpoilers: boolean
}) {
  const groups = partitionFloorItems(floor.items)
  const sections = [
    { key: 'general', label: 'Items', items: groups.general },
    { key: 'shop', label: 'Shop', items: groups.shop },
  ].filter(({ key, items }) => key === 'general' || items.length > 0)

  return (
    <>
      {sections.map(({ key, label, items }) => (
        <div key={key} className="flex flex-col gap-1">
          <p className="text-muted-foreground text-xs font-medium tracking-wide uppercase">
            {label}
            <span className="ml-1.5 font-mono font-normal tabular-nums normal-case">
              ({items.length})
            </span>
          </p>
          {items.length > 0 ? (
            <FloorItemList
              items={items}
              identities={identities}
              identitySpoilers={identitySpoilers}
              depth={floor.depth}
            />
          ) : (
            <p className="text-muted-foreground text-sm">
              No general items listed.
            </p>
          )}
        </div>
      ))}
    </>
  )
}

export function partitionFloorItems(items: ItemEntry[]) {
  return {
    general: items.filter((item) => itemGroup(item) === 'general'),
    shop: items.filter((item) => itemGroup(item) === 'shop'),
    quest: items.filter((item) => itemGroup(item) === 'quest'),
  }
}
