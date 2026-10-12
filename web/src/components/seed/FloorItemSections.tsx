import { CircleQuestionMark, InfoIcon } from 'lucide-react'
import { finderItemLabel } from '@/components/finder/finder-items'
import { ItemIcon } from '@/components/ItemIcon'
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
import type {
  FloorReport,
  IdentityMaps,
  ItemGroup,
  ItemVariant,
} from '@/lib/spd-wasm'

/** SPD `InventorySlot` NORMAL, ARGB `0x9953564D`. */
const SLOT_BACKGROUND = 'rgba(83, 86, 77, 0.6)'
const CURSED_SLOT_BACKGROUND = 'rgba(150, 64, 58, 0.75)'

function itemSection(
  item: ItemGroup
): 'shop' | 'quest' | 'guaranteed' | 'loot' | 'general' {
  const source = item.source ?? ''
  if (source.includes('ShopRoom')) return 'shop'
  if (
    /Ghost\.Quest|Wandmaker\.Quest|Blacksmith\.Quest|Imp\.Quest/.test(source)
  ) {
    return 'quest'
  }
  if (source === 'guaranteed floor spawn' || source === 'forced') {
    return 'guaranteed'
  }
  if (source === 'heap') return 'loot'
  return 'general'
}

function BaselineItemsPopover() {
  return (
    <Popover>
      <PopoverTrigger asChild>
        <Button
          variant="ghost"
          size="icon-xs"
          aria-label="About fresh baseline items"
        >
          <InfoIcon />
        </Button>
      </PopoverTrigger>
      <PopoverContent align="start" className="w-80">
        <PopoverHeader>
          <PopoverTitle>Fresh baseline items</PopoverTitle>
          <PopoverDescription>
            This is baseline analysis only. Player choices, trinkets,
            challenges, or prior generation can change these items, so they are
            not seed-wide guarantees.
          </PopoverDescription>
        </PopoverHeader>
      </PopoverContent>
    </Popover>
  )
}

type FloorItemSlotModel = {
  key: string
  label: string
  quantity: number
  level: number | null
  cursed: boolean
  unknown: boolean
  className?: string | null
  category?: string
  appearance?: string
}

function describedTitle(name: string, item: ItemVariant) {
  const extras = [
    item.cursed === true ? 'cursed' : null,
    item.enchantment?.type ?? null,
  ].filter((part): part is string => part != null)
  return extras.length > 0 ? `${name}, ${extras.join(', ')}` : name
}

function slotFrom(
  item: ItemVariant,
  key: string,
  label: string,
  className: string | null | undefined,
  identities: IdentityMaps,
  unknown = false
): FloorItemSlotModel {
  const subject = { category: item.category, class_name: className }
  return {
    key,
    label,
    quantity: item.quantity,
    level: item.level ?? null,
    cursed: item.cursed === true,
    unknown,
    className,
    category: item.category,
    appearance: unknown ? undefined : itemAppearance(subject, identities),
  }
}

function slotsForVariant(
  item: ItemVariant,
  identities: IdentityMaps,
  keyPrefix: string
) {
  if (item.candidate_classes?.length) {
    const slots: FloorItemSlotModel[] = []
    if (item.class_name === 'TrinketCatalyst') {
      slots.push(
        slotFrom(
          item,
          `${keyPrefix}-catalyst`,
          describedTitle(item.name, item),
          item.class_name,
          identities
        )
      )
    }
    for (const [index, className] of item.candidate_classes.entries()) {
      slots.push(
        slotFrom(
          item,
          `${keyPrefix}-${index}-${className}`,
          describedTitle(finderItemLabel(className), item),
          className,
          identities
        )
      )
    }
    return slots
  }

  if (item.name === 'artifact or ring') {
    return [
      slotFrom(
        item,
        keyPrefix,
        'Unknown artifact or ring',
        null,
        identities,
        true
      ),
    ]
  }

  return [
    slotFrom(
      item,
      keyPrefix,
      describedTitle(item.name, item),
      item.class_name,
      identities
    ),
  ]
}

function FloorItemSlot({ slot }: { slot: FloorItemSlotModel }) {
  return (
    <li
      title={slot.label}
      className="relative flex size-[56px] shrink-0 items-center justify-center"
      style={{
        backgroundColor: slot.cursed ? CURSED_SLOT_BACKGROUND : SLOT_BACKGROUND,
      }}
    >
      {slot.unknown ? (
        <CircleQuestionMark
          role="img"
          aria-label={slot.label}
          size={32}
          className="text-white"
        />
      ) : (
        <ItemIcon
          classNameItem={slot.className}
          category={slot.category}
          appearance={slot.appearance}
          size={32}
          title={slot.label}
        />
      )}
      {slot.quantity > 1 ? (
        <span className="pointer-events-none absolute top-0.5 left-0.5 font-mono text-[10px] leading-none text-white">
          {slot.quantity}
        </span>
      ) : null}
      {slot.level != null && slot.level !== 0 ? (
        <span className="pointer-events-none absolute right-0.5 bottom-0.5 font-mono text-[10px] leading-none text-white">
          {slot.level > 0 ? `+${slot.level}` : `${slot.level}`}
        </span>
      ) : null}
    </li>
  )
}

export function FloorItemList({
  items,
  identities,
  depth,
}: {
  items: ItemGroup[]
  identities: IdentityMaps
  depth: number
}) {
  const slots = items.flatMap((group, groupIndex) =>
    group.variants.flatMap((item, variantIndex) =>
      slotsForVariant(
        item,
        identities,
        `${depth}-${groupIndex}-${variantIndex}`
      )
    )
  )

  return (
    <ul className="flex flex-wrap gap-[2px]">
      {slots.map((slot) => (
        <FloorItemSlot key={slot.key} slot={slot} />
      ))}
    </ul>
  )
}

export function FloorItemSections({
  floor,
  identities,
}: {
  floor: FloorReport
  identities: IdentityMaps
}) {
  const groups = partitionFloorItems(visibleItemGroups(floor.items))
  const sections = [
    {
      key: 'guaranteed',
      label: 'Guaranteed spawns',
      caption: null,
      items: groups.guaranteed,
    },
    { key: 'loot', label: 'Floor loot', caption: null, items: groups.loot },
    {
      key: 'general',
      label: 'Other items',
      caption: null,
      items: groups.general,
    },
  ].filter(({ items }) => items.length > 0)

  return (
    <>
      {sections.map(({ key, label, caption, items }) => (
        <div key={key} className="flex flex-col gap-1">
          <div className="flex items-center gap-1">
            <p className="text-muted-foreground text-xs font-medium tracking-wide uppercase">
              {label}
              <span className="ml-1.5 font-mono font-normal tabular-nums normal-case">
                ({items.length})
              </span>
            </p>
            {items.some((group) =>
              group.variants.some((item) => item.prediction === 'baseline')
            ) ? (
              <BaselineItemsPopover />
            ) : null}
          </div>
          {caption ? (
            <p className="text-muted-foreground text-xs">{caption}</p>
          ) : null}
          <FloorItemList
            items={items}
            identities={identities}
            depth={floor.depth}
          />
        </div>
      ))}
    </>
  )
}

export function visibleItemGroups(items: ItemGroup[]) {
  const exactItems = new Set(
    items.flatMap((group) =>
      group.variants
        .filter((item) => item.prediction === 'exact')
        .map((item) => itemDisplayKey(item, group.source))
    )
  )
  return items
    .map((group) => ({
      ...group,
      variants: group.variants.filter(
        (item) =>
          item.prediction !== 'baseline' ||
          !exactItems.has(itemDisplayKey(item, group.source))
      ),
    }))
    .filter((group) => group.variants.length > 0)
}

function itemDisplayKey(item: ItemVariant, source?: string | null) {
  return JSON.stringify([item.class_name, item.level, source])
}

export function partitionFloorItems(items: ItemGroup[]) {
  return {
    general: items.filter((item) => itemSection(item) === 'general'),
    guaranteed: items.filter((item) => itemSection(item) === 'guaranteed'),
    loot: items.filter((item) => itemSection(item) === 'loot'),
    shop: items.filter((item) => itemSection(item) === 'shop'),
    quest: items.filter((item) => itemSection(item) === 'quest'),
  }
}
