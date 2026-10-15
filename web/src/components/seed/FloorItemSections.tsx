import { InfoIcon, QuestionIcon } from '@phosphor-icons/react'
import { ItemBox } from '@/components/ItemBox'
import { floorItemGroups } from '@/components/seed/floor-items'
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
import { itemSprite, parseItemEnhancement } from '@/lib/item-kind'
import {
  enchantmentName,
  itemLabel,
  variantCandidateClasses,
  variantClassName,
  variantTitle,
} from '@/lib/item-title'
import type {
  FloorReport,
  IdentityMaps,
  ItemGroup,
  ItemVariant,
} from '@/lib/spd-wasm'

const CURSED_SLOT_BACKGROUND =
  'color-mix(in oklch, var(--degraded) 22%, var(--muted))'

function VariableItemsPopover() {
  return (
    <Popover>
      <PopoverTrigger asChild>
        <Button
          variant="ghost"
          size="icon-xs"
          aria-label="About items that vary with gameplay"
        >
          <InfoIcon />
        </Button>
      </PopoverTrigger>
      <PopoverContent align="start" className="w-80">
        <PopoverHeader>
          <PopoverTitle>Varies with gameplay</PopoverTitle>
          <PopoverDescription>
            These are the items this seed rolls when gameplay has not shifted
            generation. Player choices, trinkets, challenges, or prior
            generation can change them, so they are not seed-wide guarantees.
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
    enchantmentName(item.enchantment?.type) ?? null,
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
    enchantment: enchantmentName(item.enchantment?.type),
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
  // Catalyst offers render beside the first alchemy pot, not in this list.
  const className = variantClassName(item)
  if (className === 'TrinketCatalyst') {
    return [
      slotFrom(
        item,
        `${keyPrefix}-catalyst`,
        describedTitle(variantTitle(item), item),
        className,
        identities
      ),
    ]
  }

  const candidates = variantCandidateClasses(item)
  if (candidates.length > 0) {
    const slots: FloorItemSlotModel[] = []
    for (const [index, candidate] of candidates.entries()) {
      slots.push(
        slotFrom(
          item,
          `${keyPrefix}-${index}-${candidate}`,
          describedTitle(itemLabel(candidate), item),
          candidate,
          identities
        )
      )
    }
    return slots
  }

  if (item.label === 'artifact or ring') {
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
      describedTitle(variantTitle(item), item),
      className,
      identities
    ),
  ]
}

function FloorItemSlot({ slot }: { slot: FloorItemSlotModel }) {
  const enhancement = parseItemEnhancement(slot.enchantment)
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
          <QuestionIcon
            aria-hidden
            weight="bold"
            className="text-foreground pointer-events-none absolute inset-0 m-auto size-7"
          />
        ) : null}
        {slot.quantity > 1 ? (
          <span className="ink-outline font-pixel text-px-1 pointer-events-none absolute -top-0.5 left-0.5 leading-[20px] text-[#f0c040]">
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
    <ul className="flex flex-wrap gap-1">
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
  const variable = items.some((group) =>
    group.variants.some((item) => item.prediction === 'variable')
  )
  return (
    <div className="flex flex-col gap-2">
      <div className="flex items-center gap-1">
        <p className="text-sm font-semibold">
          {label}
          {items.length > 0 ? (
            <>
              {' '}
              <span className="text-muted-foreground ml-1 font-normal tabular-nums">
                {items.length}
              </span>
            </>
          ) : null}
        </p>
        {variable ? <VariableItemsPopover /> : null}
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
  const { notable, other } = floorItemGroups(floor)

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

export {
  partitionFloorItems,
  visibleItemGroups,
} from '@/components/seed/floor-items'
