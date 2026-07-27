import { WarningIcon } from '@phosphor-icons/react'
import { CircleQuestionMark } from 'lucide-react'
import { finderItemLabel } from '@/components/finder/finder-items'
import { ItemIcon } from '@/components/ItemIcon'
import { ItemName } from '@/components/ItemName'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import {
  Popover,
  PopoverContent,
  PopoverDescription,
  PopoverHeader,
  PopoverTitle,
  PopoverTrigger,
} from '@/components/ui/popover'
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

function ConditionalNotes({ notes }: { notes?: string[] }) {
  if (!notes?.length) return null

  return (
    <Popover>
      <PopoverTrigger asChild>
        <Button
          variant="ghost"
          size="icon-xs"
          aria-label="Show conditions"
          className="text-warning"
        >
          <WarningIcon weight="fill" />
        </Button>
      </PopoverTrigger>
      <PopoverContent align="start">
        <PopoverHeader>
          <PopoverTitle>Conditions</PopoverTitle>
          <PopoverDescription className="flex flex-col gap-1.5">
            {notes.map((note) => (
              <span key={note}>{note}</span>
            ))}
          </PopoverDescription>
        </PopoverHeader>
      </PopoverContent>
    </Popover>
  )
}

function ImpRingOptions({
  item,
  identities,
  identitySpoilers,
}: {
  item: ItemEntry
  identities: IdentityMaps
  identitySpoilers: boolean
}) {
  return (
    <div className="flex min-w-0 flex-1 flex-col gap-1">
      {item.candidate_classes?.map((className, index) => {
        const candidate = { category: 'ring', class_name: className }
        const label = finderItemLabel(className)
        return (
          <div key={className} className="flex flex-col gap-1">
            {index > 0 ? (
              <span className="pl-6 text-xs font-medium text-muted-foreground">
                OR
              </span>
            ) : null}
            <div className="flex items-start gap-2">
              <ItemIcon
                classNameItem={className}
                category="ring"
                appearance={
                  identitySpoilers
                    ? itemAppearance(candidate, identities)
                    : undefined
                }
                size={16}
                title={label}
                className="mt-0.5"
              />
              <span className="flex min-w-0 flex-wrap items-baseline gap-1.5">
                <ItemName
                  name={`${label}${item.level != null ? ` +${item.level}` : ''}`}
                />
                {item.cursed === true ? (
                  <Badge variant="destructive">cursed</Badge>
                ) : null}
              </span>
            </div>
          </div>
        )
      })}
    </div>
  )
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
        const sourceLabel =
          itemGroup(item) === 'shop' ||
          item.source === 'Blacksmith.Quest' ||
          item.source === 'Imp.Quest'
            ? null
            : formatItemSource(item.source)
        return (
          <li key={`${depth}-${index}`} className="flex items-start gap-2">
            {item.source === 'Imp.Quest' && item.candidate_classes?.length ? (
              <ImpRingOptions
                item={item}
                identities={identities}
                identitySpoilers={identitySpoilers}
              />
            ) : (
              <>
                {item.name === 'artifact or ring' ? (
                  <CircleQuestionMark
                    aria-label="Unknown artifact or ring"
                    className="mt-0.5 size-4 shrink-0 text-muted-foreground"
                  />
                ) : (
                  <ItemIcon
                    classNameItem={item.class_name}
                    category={item.category}
                    appearance={
                      identitySpoilers
                        ? itemAppearance(item, identities)
                        : undefined
                    }
                    size={16}
                    title={item.name}
                    className="mt-0.5"
                  />
                )}
                <span className="flex min-w-0 flex-wrap items-baseline gap-x-1.5 gap-y-0.5">
                  <ItemName name={item.name} />
                  {item.quantity > 1 ? (
                    <span className="font-mono text-muted-foreground tabular-nums">
                      x{item.quantity}
                    </span>
                  ) : null}
                  {item.tier != null ? (
                    <Badge variant="outline">tier {item.tier}</Badge>
                  ) : null}
                  {item.tier_range ? (
                    <Badge variant="outline">
                      tier {item.tier_range.min}–{item.tier_range.max}
                    </Badge>
                  ) : null}
                  {item.level_range && !item.name.includes('…') ? (
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
                  <ConditionalNotes notes={item.conditional_notes} />
                </span>
              </>
            )}
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
