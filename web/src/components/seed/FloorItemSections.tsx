import { CircleQuestionMark, InfoIcon } from 'lucide-react'
import { finderItemLabel } from '@/components/finder/finder-items'
import { ItemBox } from '@/components/ItemBox'
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
import {
  enhancementInItemName,
  itemSprite,
  parseItemEnhancement,
} from '@/lib/item-kind'
import { isNotableSpawn } from '@/lib/item-notable'
import type {
  FloorReport,
  IdentityMaps,
  ItemGroup,
  ItemVariant,
} from '@/lib/spd-wasm'

const CURSED_SLOT_BACKGROUND = 'rgba(150, 64, 58, 0.75)'

function splitNotableItems(items: ItemGroup[]) {
  const notable: ItemGroup[] = []
  const other: ItemGroup[] = []
  for (const group of items) {
    const notableVariants = group.variants.filter(isNotableSpawn)
    const otherVariants = group.variants.filter((item) => !isNotableSpawn(item))
    if (notableVariants.length > 0) {
      notable.push({ ...group, variants: notableVariants })
    }
    if (otherVariants.length > 0) {
      other.push({ ...group, variants: otherVariants })
    }
  }
  return { notable, other }
}

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
  enchantment?: string | null
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
    enchantment: item.enchantment?.type,
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
  const namedEnhancement = enhancementInItemName(slot.label, slot.className)
  const enhancement = slot.cursed
    ? (namedEnhancement ?? parseItemEnhancement(slot.enchantment))
    : (parseItemEnhancement(slot.enchantment) ?? namedEnhancement)
  return (
    <li className="shrink-0">
      <ItemBox
        sprites={
          slot.unknown
            ? undefined
            : itemSprite({
                className: slot.className,
                category: slot.category,
                appearance: slot.appearance,
              })
        }
        enhancement={enhancement}
        cursed={slot.cursed}
        level={slot.level}
        title={slot.label}
        bgColor={slot.cursed ? CURSED_SLOT_BACKGROUND : undefined}
      >
        {slot.unknown ? (
          <CircleQuestionMark
            aria-hidden
            className="pointer-events-none absolute inset-0 m-auto size-8 text-neutral-900 dark:text-white"
          />
        ) : null}
        {slot.quantity > 1 ? (
          <span className="spd-slot-edge pointer-events-none absolute top-1 left-1 font-sans text-[12px] leading-none font-bold tracking-normal text-[#f0c040]">
            {slot.quantity}
          </span>
        ) : null}
      </ItemBox>
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

function ItemGroupSection({
  label,
  items,
  empty,
  identities,
  depth,
}: {
  label: string
  items: ItemGroup[]
  empty?: string
  identities: IdentityMaps
  depth: number
}) {
  const baseline = items.some((group) =>
    group.variants.some((item) => item.prediction === 'baseline')
  )
  return (
    <div className="flex flex-col gap-1">
      <div className="flex items-center gap-1">
        <p className="text-muted-foreground text-xs font-medium tracking-wide uppercase">
          {label}
          {items.length > 0 ? (
            <span className="ml-1.5 font-mono font-normal tabular-nums normal-case">
              ({items.length})
            </span>
          ) : null}
        </p>
        {baseline ? <BaselineItemsPopover /> : null}
      </div>
      {items.length > 0 ? (
        <FloorItemList items={items} identities={identities} depth={depth} />
      ) : empty ? (
        <p className="text-muted-foreground text-sm">{empty}</p>
      ) : null}
    </div>
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
  // Shop stays off the floor list. Quest rewards stay on the quest card.
  const shown = [...groups.guaranteed, ...groups.loot, ...groups.general]
  const { notable, other } = splitNotableItems(shown)

  return (
    <>
      <ItemGroupSection
        label="Notable"
        items={notable}
        empty="No notable items on this floor."
        identities={identities}
        depth={floor.depth}
      />
      {other.length > 0 ? (
        <ItemGroupSection
          label="Other items"
          items={other}
          identities={identities}
          depth={floor.depth}
        />
      ) : null}
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
