import { FloorItemList } from '@/components/seed/FloorItemSections'
import { Alert, AlertDescription, AlertTitle } from '@/components/ui/alert'
import { Badge } from '@/components/ui/badge'
import type { IdentityMaps, ItemGroup, QuestReport } from '@/lib/spd-wasm'
import { cn } from '@/lib/utils'

const QUEST_STYLES: Record<
  QuestReport['type'],
  { badge: string; border: string; title: string }
> = {
  sad_ghost: {
    badge: 'bg-sky-500/15 text-sky-800 dark:text-sky-200 border-sky-500/30',
    border: 'border-sky-500/25 bg-sky-500/5',
    title: 'Sad Ghost',
  },
  old_wandmaker: {
    badge:
      'bg-violet-500/15 text-violet-800 dark:text-violet-200 border-violet-500/30',
    border: 'border-violet-500/25 bg-violet-500/5',
    title: 'Old Wandmaker',
  },
  troll_blacksmith: {
    badge:
      'bg-amber-500/15 text-amber-900 dark:text-amber-200 border-amber-500/30',
    border: 'border-amber-500/25 bg-amber-500/5',
    title: 'Troll Blacksmith',
  },
  ambitious_imp: {
    badge: 'bg-rose-500/15 text-rose-800 dark:text-rose-200 border-rose-500/30',
    border: 'border-rose-500/25 bg-rose-500/5',
    title: 'Ambitious Imp',
  },
}

function label(value: string) {
  return value
    .split('_')
    .map((part) => part[0].toUpperCase() + part.slice(1))
    .join(' ')
}

function targetSummary(quest: QuestReport) {
  switch (quest.type) {
    case 'sad_ghost':
      return `Target: ${label(quest.baseline.target)}`
    case 'old_wandmaker':
      return `Baseline target: ${label(quest.baseline.objective)}`
    case 'troll_blacksmith':
      return `Target: ${label(quest.baseline.objective)}`
    case 'ambitious_imp':
      return `Baseline target: ${label(quest.baseline.target)} (${quest.baseline.required_tokens} tokens)`
  }
}

function baselineContract(quest: QuestReport) {
  switch (quest.type) {
    case 'old_wandmaker':
      return 'Reward contract: two distinct uncursed +1…+3 wands; complete the quest and choose one.'
    case 'ambitious_imp':
      return 'Reward contract: one cursed +2…+4 ring after completing the quest.'
    default:
      return null
  }
}

export function QuestCard({
  quest,
  rewards,
  identities,
  depth,
}: {
  quest: QuestReport
  rewards: ItemGroup[]
  identities: IdentityMaps
  depth: number
}) {
  const styles = QUEST_STYLES[quest.type]
  const contract = baselineContract(quest)
  const baselineRewards = contract
    ? groupsWithPrediction(rewards, 'baseline')
    : []
  const displayedRewards = baselineRewards.length
    ? baselineRewards
    : withoutPrediction(rewards, 'baseline')

  return (
    <div
      data-quest-type={quest.type}
      className={cn(
        'flex flex-col gap-1.5 rounded-none border px-3 py-2.5',
        styles.border
      )}
    >
      <div className="flex flex-wrap items-center gap-1.5">
        <Badge variant="outline" className={cn('font-medium', styles.badge)}>
          {styles.title}
        </Badge>
        {contract ? <Badge variant="outline">Fresh baseline</Badge> : null}
      </div>
      <p className="text-muted-foreground text-xs leading-relaxed">
        {targetSummary(quest)}
      </p>
      {contract ? (
        <Alert variant="warning">
          <AlertTitle>Fresh/no-history baseline</AlertTitle>
          <AlertDescription>
            Player choices, trinkets, challenges, or prior generation can change
            this target and reward. {contract}
          </AlertDescription>
        </Alert>
      ) : null}
      {displayedRewards.length > 0 ? (
        <div className="flex flex-col gap-1 border-t pt-2">
          <p className="text-muted-foreground text-xs font-medium tracking-wide uppercase">
            {baselineRewards.length ? 'Baseline rewards' : 'Rewards'}
          </p>
          <FloorItemList
            items={displayedRewards}
            identities={identities}
            depth={depth}
          />
        </div>
      ) : null}
    </div>
  )
}

function groupsWithPrediction(groups: ItemGroup[], prediction: 'baseline') {
  return groups
    .map((group) => ({
      ...group,
      variants: group.variants.filter(
        (variant) => variant.prediction === prediction
      ),
    }))
    .filter((group) => group.variants.length > 0)
}

function withoutPrediction(groups: ItemGroup[], prediction: 'baseline') {
  return groups
    .map((group) => ({
      ...group,
      variants: group.variants.filter(
        (variant) => variant.prediction !== prediction
      ),
    }))
    .filter((group) => group.variants.length > 0)
}
