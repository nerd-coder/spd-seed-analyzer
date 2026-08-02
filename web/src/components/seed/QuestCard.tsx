import { FloorItemList } from '@/components/seed/FloorItemSections'
import { Badge } from '@/components/ui/badge'
import type { IdentityMaps, ItemEntry, QuestReport } from '@/lib/spd-wasm'
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
      return `Target: ${label(quest.baseline.objective)}`
    case 'troll_blacksmith':
      return `Target: ${label(quest.baseline.objective)}`
    case 'ambitious_imp':
      return `Target: ${label(quest.baseline.target)} (${quest.baseline.required_tokens} tokens)`
  }
}

export function QuestCard({
  quest,
  rewards,
  identities,
  depth,
}: {
  quest: QuestReport
  rewards: ItemEntry[]
  identities: IdentityMaps
  depth: number
}) {
  const styles = QUEST_STYLES[quest.type]

  return (
    <div
      className={cn(
        'space-y-1.5 rounded-none border px-3 py-2.5',
        styles.border
      )}
    >
      <Badge variant="outline" className={cn('font-medium', styles.badge)}>
        {styles.title}
      </Badge>
      <p className="text-muted-foreground text-xs leading-relaxed">
        {targetSummary(quest)}
      </p>
      {rewards.length > 0 ? (
        <div className="flex flex-col gap-1 border-t pt-2">
          <p className="text-muted-foreground text-xs font-medium tracking-wide uppercase">
            Rewards
          </p>
          <FloorItemList
            items={rewards}
            identities={identities}
            depth={depth}
          />
        </div>
      ) : null}
    </div>
  )
}
