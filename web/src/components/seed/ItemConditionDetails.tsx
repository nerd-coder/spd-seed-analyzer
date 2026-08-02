import { ListFilterIcon, SparklesIcon } from 'lucide-react'
import { Button } from '@/components/ui/button'
import {
  Popover,
  PopoverContent,
  PopoverDescription,
  PopoverHeader,
  PopoverTitle,
  PopoverTrigger,
} from '@/components/ui/popover'
import type {
  ItemCondition,
  ItemDependencyCondition,
  ItemEnchantment,
  ItemSpawnCondition,
} from '@/lib/spd-wasm'

function humanize(value: string): string {
  return value
    .split('_')
    .filter(Boolean)
    .map((word) => word.charAt(0).toUpperCase() + word.slice(1))
    .join(' ')
}

function dependencyDescription(condition: ItemDependencyCondition): string {
  if (condition.type === 'challenge') {
    return `${humanize(condition.challenge)} is ${condition.enabled ? 'enabled' : 'disabled'}`
  }

  if (condition.type === 'artifact') {
    const events = condition.events ?? []
    return events.length
      ? events
          .map(
            (event) =>
              `${humanize(event.artifact)} is ${event.kind} before floor ${event.before_depth}`
          )
          .join(' and ')
      : 'no artifact history changes the result'
  }

  const events = condition.events ?? []
  if (!events.length) return 'no generation-affecting trinket is held'

  let level = 0
  return events
    .map((event) => {
      if (event.kind === 'upgraded') {
        level += 1
        return `the held trinket reaches +${level} before floor ${event.before_depth}`
      }
      level = 0
      const trinket = humanize(event.trinket ?? 'trinket')
      const upgradeRequirement = event.min_upgrades
        ? ` at +${event.min_upgrades} or better`
        : ''
      return `${trinket} is ${event.kind === 'acquired' ? 'acquired' : 'transmuted'}${upgradeRequirement} before floor ${event.before_depth}`
    })
    .join(' and ')
}

function conditionDescription(condition: ItemCondition): string {
  if (condition.type === 'challenge') {
    return `${humanize(condition.challenge)} is ${condition.enabled ? 'enabled' : 'disabled'}`
  }
  if (condition.type === 'trinket' || condition.type === 'artifact') {
    return dependencyDescription(condition)
  }
  if (condition.type === 'quest') {
    return `${humanize(condition.quest_id)} is completed${condition.depth ? ` before floor ${condition.depth}` : ''}`
  }
  if (condition.type === 'choice') {
    return `choose ${condition.selected_count} of ${condition.option_count} ${humanize(condition.group_id)} options${condition.favor_requirement ? ` with ${condition.favor_requirement} favor` : ''}`
  }
  if (condition.type === 'inventory') {
    return `have ${humanize(condition.requirement_id)} in your inventory`
  }
  return `${humanize(condition.state_id)} runtime state applies`
}

function spawnSummary(conditions: ItemSpawnCondition[]): string {
  if (conditions.length === 1) {
    const requirements = conditions[0].all_of ?? []
    if (requirements.length === 1) {
      const [requirement] = requirements
      if (requirement.type === 'challenge') {
        return `${humanize(requirement.challenge)} ${requirement.enabled ? 'enabled' : 'disabled'}`
      }
      return dependencyDescription(requirement)
    }
  }
  return `${conditions.length} possible spawn routes`
}

export function SpawnConditionDetails({
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
          {spawnSummary(conditions)}
        </Button>
      </PopoverTrigger>
      <PopoverContent align="start" className="w-80">
        <PopoverHeader>
          <PopoverTitle>Spawn conditions</PopoverTitle>
          <PopoverDescription>
            This item appears when any one of these routes applies.
          </PopoverDescription>
        </PopoverHeader>
        <ol className="flex list-decimal flex-col gap-2 pl-4 text-sm leading-relaxed">
          {conditions.map((condition, index) => (
            <li key={`${index}-${JSON.stringify(condition)}`}>
              {(condition.all_of ?? [])
                .map(dependencyDescription)
                .join(' and ')}
            </li>
          ))}
        </ol>
      </PopoverContent>
    </Popover>
  )
}

export function UpgradeConditionDetails({
  levelRange,
  conditions,
}: {
  levelRange?: { min: number; max: number } | null
  conditions?: ItemCondition[]
}) {
  if (!levelRange) return null

  return (
    <Popover>
      <PopoverTrigger asChild>
        <Button
          variant="ghost"
          size="icon-xs"
          aria-label="Show upgrade conditions"
        >
          <SparklesIcon />
        </Button>
      </PopoverTrigger>
      <PopoverContent align="start" className="w-80">
        <PopoverHeader>
          <PopoverTitle>Upgrade conditions</PopoverTitle>
          <PopoverDescription>
            The starting upgrade is not fixed by the seed alone.
          </PopoverDescription>
        </PopoverHeader>
        <div className="flex flex-col gap-2 text-sm leading-relaxed">
          <p>
            Starts between +{levelRange.min} and +{levelRange.max}.
          </p>
          {conditions?.length ? (
            <ul className="flex list-disc flex-col gap-1 pl-4">
              {conditions.map((condition, index) => (
                <li key={`${condition.type}-${index}`}>
                  {conditionDescription(condition)}
                </li>
              ))}
            </ul>
          ) : null}
        </div>
      </PopoverContent>
    </Popover>
  )
}

export function EnchantmentConditionDetails({
  enchantment,
}: {
  enchantment?: ItemEnchantment | null
}) {
  if (!enchantment?.conditions.length) return null

  return (
    <Popover>
      <PopoverTrigger asChild>
        <Button
          variant="ghost"
          size="icon-xs"
          aria-label="Show enchantment conditions"
        >
          <SparklesIcon />
        </Button>
      </PopoverTrigger>
      <PopoverContent align="start" className="w-80">
        <PopoverHeader>
          <PopoverTitle>Enchantment conditions</PopoverTitle>
          <PopoverDescription>
            {humanize(enchantment.type)} is retained when these requirements are
            met.
          </PopoverDescription>
        </PopoverHeader>
        <ul className="flex list-disc flex-col gap-1 pl-4 text-sm leading-relaxed">
          {enchantment.conditions.map((condition, index) => (
            <li key={`${condition.type}-${index}`}>
              {conditionDescription(condition)}
            </li>
          ))}
        </ul>
      </PopoverContent>
    </Popover>
  )
}
