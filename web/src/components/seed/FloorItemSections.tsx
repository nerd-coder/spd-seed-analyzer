import { CircleQuestionMark, InfoIcon } from 'lucide-react'
import { finderItemLabel } from '@/components/finder/finder-items'
import { ItemIcon } from '@/components/ItemIcon'
import { ItemName } from '@/components/ItemName'
import {
  EnchantmentConditionDetails,
  SpawnConditionDetails,
  UpgradeConditionDetails,
} from '@/components/seed/ItemConditionDetails'
import { TrinketRotationPopover } from '@/components/seed/TrinketRotationPopover'
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
import type {
  FloorReport,
  IdentityMaps,
  ItemEntry,
  TrinketSelectionReport,
} from '@/lib/spd-wasm'

function itemGroup(
  item: ItemEntry
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

function OtherItemsBaselinePopover() {
  return (
    <Popover>
      <PopoverTrigger asChild>
        <Button
          variant="ghost"
          size="icon-xs"
          aria-label="About baseline other-item rewards"
        >
          <InfoIcon />
        </Button>
      </PopoverTrigger>
      <PopoverContent align="start" className="w-80">
        <PopoverHeader>
          <PopoverTitle>Fresh baseline rewards</PopoverTitle>
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

function CandidateOptions({
  item,
  identities,
}: {
  item: ItemEntry
  identities: IdentityMaps
}) {
  return (
    <div className="flex min-w-0 flex-1 flex-col gap-1">
      {item.name === 'Trinket Catalyst' ? (
        <span className="font-medium">Trinket Catalyst — choose one</span>
      ) : null}
      {item.candidate_classes?.map((className, index) => {
        const candidate = { category: item.category, class_name: className }
        const label = finderItemLabel(className)
        return (
          <div key={`${index}-${className}`} className="flex flex-col gap-1">
            {index > 0 ? (
              <span className="pl-6 text-xs font-medium text-muted-foreground">
                OR
              </span>
            ) : null}
            <div className="flex items-start gap-2">
              <ItemIcon
                classNameItem={className}
                category={item.category}
                appearance={itemAppearance(candidate, identities)}
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
                {item.enchantment ? (
                  <Badge variant="secondary">{item.enchantment.type}</Badge>
                ) : null}
                <UpgradeConditionDetails
                  levelRange={item.level_range}
                  conditions={item.conditions}
                />
                <EnchantmentConditionDetails enchantment={item.enchantment} />
                <SpawnConditionDetails conditions={item.spawn_conditions} />
              </span>
            </div>
          </div>
        )
      })}
    </div>
  )
}

function CatalystOffers({
  item,
  identities,
  trinketSelection,
}: {
  item: ItemEntry
  identities: IdentityMaps
  trinketSelection?: TrinketSelectionReport
}) {
  return (
    <div className="flex min-w-0 flex-1 flex-col gap-1.5">
      <div className="flex items-center gap-2">
        <ItemIcon
          classNameItem={item.class_name}
          category={item.category}
          appearance={itemAppearance(item, identities)}
          size={16}
          title={item.name}
          className="mt-0.5"
        />
        <ItemName name={item.name} />
      </div>
      <div className="flex flex-col gap-1 pl-6">
        <span className="flex items-center gap-1 font-medium text-muted-foreground">
          Trinket offers
          <TrinketRotationPopover
            sequence={trinketSelection?.transmutation_sequence ?? []}
          />
        </span>
        {item.candidate_classes?.map((className) => {
          const candidate = { category: item.category, class_name: className }
          return (
            <div key={className} className="flex items-center gap-2">
              <ItemIcon
                classNameItem={className}
                category={item.category}
                appearance={itemAppearance(candidate, identities)}
                size={16}
                title={finderItemLabel(className)}
              />
              <ItemName name={finderItemLabel(className)} />
            </div>
          )
        })}
      </div>
    </div>
  )
}

export function FloorItemList({
  items,
  identities,
  depth,
  trinketSelection,
  showQuestSources = false,
}: {
  items: ItemEntry[]
  identities: IdentityMaps
  depth: number
  trinketSelection?: TrinketSelectionReport
  showQuestSources?: boolean
}) {
  return (
    <ul className="flex flex-col gap-1.5 text-sm">
      {items.map((item, index) => {
        const sourceLabel =
          itemGroup(item) === 'shop' ||
          (!showQuestSources &&
            (item.source === 'Ghost.Quest' ||
              item.source === 'Wandmaker.Quest' ||
              item.source === 'Blacksmith.Quest' ||
              item.source === 'Imp.Quest'))
            ? null
            : formatItemSource(item.source)
        return (
          <li key={`${depth}-${index}`} className="flex items-center gap-2">
            {item.candidate_classes?.length ? (
              item.name === 'Trinket Catalyst' ? (
                <CatalystOffers
                  item={item}
                  identities={identities}
                  trinketSelection={trinketSelection}
                />
              ) : (
                <CandidateOptions item={item} identities={identities} />
              )
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
                    appearance={itemAppearance(item, identities)}
                    size={16}
                    title={item.name}
                    className="mt-0.5"
                  />
                )}
                <span className="flex min-w-0 flex-wrap items-center gap-x-1.5 gap-y-0.5">
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
                  {item.enchantment ? (
                    <Badge variant="secondary">{item.enchantment.type}</Badge>
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
                  <SpawnConditionDetails conditions={item.spawn_conditions} />
                  <UpgradeConditionDetails
                    levelRange={item.level_range}
                    conditions={item.conditions}
                  />
                  <EnchantmentConditionDetails enchantment={item.enchantment} />
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
  trinketSelection,
}: {
  floor: FloorReport
  identities: IdentityMaps
  trinketSelection?: TrinketSelectionReport
}) {
  const reportItems = floor.items.filter(
    (item) => item.prediction !== 'baseline'
  )
  const groups = partitionFloorItems(reportItems)
  const baselineItems = floor.items.filter(
    (baseline) =>
      baseline.prediction === 'baseline' &&
      !reportItems.some(
        (item) =>
          item.prediction === 'exact' &&
          item.class_name === baseline.class_name &&
          item.level === baseline.level &&
          item.source === baseline.source
      )
  )
  const sections = [
    { key: 'guaranteed', label: 'Guaranteed spawns', items: groups.guaranteed },
    { key: 'loot', label: 'Floor loot', items: groups.loot },
    { key: 'general', label: 'Other items', items: groups.general },
    {
      key: 'shop',
      label: 'Shop',
      items: groups.shop,
    },
  ].filter(({ items }) => items.length > 0)

  return (
    <>
      {baselineItems.length > 0 ? (
        <div className="flex flex-col gap-1">
          <div className="flex flex-wrap items-center gap-1">
            <p className="text-muted-foreground text-xs font-medium tracking-wide uppercase">
              Fresh baseline highlights
              <span className="ml-1.5 font-mono font-normal tabular-nums normal-case">
                ({baselineItems.length})
              </span>
            </p>
            <Badge variant="outline">planning only</Badge>
            <OtherItemsBaselinePopover />
          </div>
          <p className="text-muted-foreground text-xs leading-relaxed">
            This is baseline analysis only. Player choices, trinkets,
            challenges, or prior generation can change these items.
          </p>
          <FloorItemList
            items={baselineItems}
            identities={identities}
            depth={floor.depth}
            trinketSelection={trinketSelection}
            showQuestSources
          />
        </div>
      ) : null}
      {sections.map(({ key, label, items }) => (
        <div key={key} className="flex flex-col gap-1">
          <div className="flex items-center gap-1">
            <p className="text-muted-foreground text-xs font-medium tracking-wide uppercase">
              {label}
              <span className="ml-1.5 font-mono font-normal tabular-nums normal-case">
                ({items.length})
              </span>
            </p>
            {key === 'general' ? <OtherItemsBaselinePopover /> : null}
          </div>
          <FloorItemList
            items={items}
            identities={identities}
            depth={floor.depth}
            trinketSelection={trinketSelection}
          />
        </div>
      ))}
    </>
  )
}

export function partitionFloorItems(items: ItemEntry[]) {
  return {
    general: items.filter((item) => itemGroup(item) === 'general'),
    guaranteed: items.filter((item) => itemGroup(item) === 'guaranteed'),
    loot: items.filter((item) => itemGroup(item) === 'loot'),
    shop: items.filter((item) => itemGroup(item) === 'shop'),
    quest: items.filter((item) => itemGroup(item) === 'quest'),
  }
}
