import { WarningIcon } from '@phosphor-icons/react'
import { CircleQuestionMark, ListFilterIcon } from 'lucide-react'
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
import type {
  FloorReport,
  IdentityMaps,
  ItemCondition,
  ItemDependencyCondition,
  ItemEntry,
  ItemSpawnCondition,
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

function typedConditionLabel(condition: ItemCondition): string {
  switch (condition.type) {
    case 'challenge':
      return `${humanize(condition.challenge)} ${condition.enabled ? 'enabled' : 'disabled'}`
    case 'trinket':
      return condition.events.length
        ? `Trinket state (${condition.events.length} events)`
        : 'Trinket state'
    case 'artifact':
      return condition.events.length
        ? `Artifact history (${condition.events.length} events)`
        : 'Artifact history'
    case 'quest':
      return `Quest: ${humanize(condition.quest_id)}${condition.depth ? ` before floor ${condition.depth}` : ''}`
    case 'choice':
      return `${humanize(condition.group_id)}: choose ${condition.selected_count} of ${condition.option_count}${condition.favor_requirement ? ` (${condition.favor_requirement} favor)` : ''}`
    case 'inventory':
      return `Inventory: ${humanize(condition.requirement_id)}`
    case 'runtime':
      return `Runtime state: ${humanize(condition.state_id)}`
  }
}

function ItemConditions({
  conditions,
  title = 'Conditions',
}: {
  conditions?: ItemCondition[]
  title?: string
}) {
  if (!conditions?.length) return null

  return (
    <Popover>
      <PopoverTrigger asChild>
        <Button
          variant="ghost"
          size="icon-xs"
          aria-label={`Show ${title.toLowerCase()}`}
        >
          <WarningIcon weight="fill" />
        </Button>
      </PopoverTrigger>
      <PopoverContent align="start">
        <PopoverHeader>
          <PopoverTitle>{title}</PopoverTitle>
          <PopoverDescription className="flex flex-col gap-1.5">
            {conditions.map((condition, index) => (
              <span key={`${condition.type}-${index}`}>
                {typedConditionLabel(condition)}
              </span>
            ))}
          </PopoverDescription>
        </PopoverHeader>
      </PopoverContent>
    </Popover>
  )
}

function humanize(value: string): string {
  return value
    .split('_')
    .filter(Boolean)
    .map((word) => word.charAt(0).toUpperCase() + word.slice(1))
    .join(' ')
}

function conditionLabel(condition: ItemDependencyCondition): string {
  if (condition.type === 'challenge') {
    return `${humanize(condition.challenge)} ${condition.enabled ? 'enabled' : 'disabled'}`
  }
  if (condition.type === 'artifact') {
    const events = condition.events ?? []
    if (events.length === 0) return 'No external artifact history'
    return events
      .map(
        (event) =>
          `${humanize(event.artifact)} ${event.kind} before floor ${event.before_depth}`
      )
      .join('; ')
  }
  const events = condition.events ?? []
  if (events.length === 0) return 'No generation-affecting trinket'

  let level = 0
  return events
    .map((event) => {
      if (event.kind === 'upgraded') {
        level += 1
        return `upgrade to +${level} before floor ${event.before_depth}`
      }
      level = 0
      return `${event.kind === 'acquired' ? 'get' : 'transmute to'} ${humanize(
        event.trinket ?? 'trinket'
      )} before floor ${event.before_depth}`
    })
    .join('; ')
}

function shortConditionLabel(conditions: ItemSpawnCondition[]): string {
  const conditionTypes = [
    ...new Set(
      conditions.flatMap((condition) =>
        (condition.all_of ?? []).map((dependency) => dependency.type)
      )
    ),
  ]
  if (conditions.length !== 1) {
    return `${conditionTypes.map(humanize).join(' + ')} conditions`
  }
  const dependencies = conditions[0].all_of ?? []
  if (dependencies.length !== 1) {
    return `${conditionTypes.map(humanize).join(' + ')} requirements`
  }
  return conditionLabel(dependencies[0])
}

function SpawnConditions({
  conditions,
}: {
  conditions?: ItemSpawnCondition[]
}) {
  if (!conditions?.length) return null

  return (
    <Popover>
      <PopoverTrigger asChild>
        <Button variant="outline" size="xs">
          <ListFilterIcon data-icon="inline-start" />
          {shortConditionLabel(conditions)}
        </Button>
      </PopoverTrigger>
      <PopoverContent align="start" className="w-80">
        <PopoverHeader>
          <PopoverTitle>Spawn conditions</PopoverTitle>
          <PopoverDescription>
            Any one of these modeled routes may produce this item and its shown
            properties.
          </PopoverDescription>
        </PopoverHeader>
        <ol className="mt-3 flex list-decimal flex-col gap-2 pl-4 text-sm">
          {conditions.map((condition, index) => (
            <li key={`${index}-${JSON.stringify(condition)}`}>
              {(condition.all_of ?? []).map(conditionLabel).join(' · ')}
            </li>
          ))}
        </ol>
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
                <ItemConditions conditions={item.conditions} />
                <ItemConditions
                  conditions={item.enchantment?.conditions}
                  title="Enchantment conditions"
                />
                <SpawnConditions conditions={item.spawn_conditions} />
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
}: {
  item: ItemEntry
  identities: IdentityMaps
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
        <span className="font-medium text-muted-foreground">
          Trinket offers
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
}: {
  items: ItemEntry[]
  identities: IdentityMaps
  depth: number
}) {
  return (
    <ul className="flex flex-col gap-1.5 text-sm">
      {items.map((item, index) => {
        const sourceLabel =
          itemGroup(item) === 'shop' ||
          item.source === 'Ghost.Quest' ||
          item.source === 'Wandmaker.Quest' ||
          item.source === 'Blacksmith.Quest' ||
          item.source === 'Imp.Quest'
            ? null
            : formatItemSource(item.source)
        return (
          <li key={`${depth}-${index}`} className="flex items-center gap-2">
            {item.candidate_classes?.length ? (
              item.name === 'Trinket Catalyst' ? (
                <CatalystOffers item={item} identities={identities} />
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
                  <SpawnConditions conditions={item.spawn_conditions} />
                  <ItemConditions conditions={item.conditions} />
                  <ItemConditions
                    conditions={item.enchantment?.conditions}
                    title="Enchantment conditions"
                  />
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
}: {
  floor: FloorReport
  identities: IdentityMaps
}) {
  const groups = partitionFloorItems(floor.items)
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
      {sections.map(({ key, label, items }) => (
        <div key={key} className="flex flex-col gap-1">
          <div className="flex items-center gap-1">
            <p className="text-muted-foreground text-xs font-medium tracking-wide uppercase">
              {label}
              <span className="ml-1.5 font-mono font-normal tabular-nums normal-case">
                ({items.length})
              </span>
            </p>
          </div>
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

export function partitionFloorItems(items: ItemEntry[]) {
  return {
    general: items.filter((item) => itemGroup(item) === 'general'),
    guaranteed: items.filter((item) => itemGroup(item) === 'guaranteed'),
    loot: items.filter((item) => itemGroup(item) === 'loot'),
    shop: items.filter((item) => itemGroup(item) === 'shop'),
    quest: items.filter((item) => itemGroup(item) === 'quest'),
  }
}
