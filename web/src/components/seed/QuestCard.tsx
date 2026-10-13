import { FloorItemList } from '@/components/seed/FloorItemSections'
import { ImpRewardDistribution } from '@/components/seed/ImpRewardDistribution'
import { QuestBranchMap } from '@/components/seed/QuestBranchMap'
import { QuestTarget } from '@/components/seed/QuestTarget'
import { WandmakerRewardDistribution } from '@/components/seed/WandmakerRewardDistribution'
import { Badge } from '@/components/ui/badge'
import type {
  BranchFloorReport,
  IdentityMaps,
  ItemGroup,
  QuestReport,
} from '@/lib/spd-wasm'
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

function prefersBaselineRewards(quest: QuestReport) {
  return quest.type === 'old_wandmaker' || quest.type === 'ambitious_imp'
}

function rewardsHeading(quest: QuestReport, isBaseline: boolean) {
  if (quest.type === 'ambitious_imp' || !isBaseline) return 'Rewards'
  return 'Baseline rewards'
}

export function QuestCard({
  quest,
  rewards,
  identities,
  depth,
  branch,
}: {
  quest: QuestReport
  rewards: ItemGroup[]
  identities: IdentityMaps
  depth: number
  branch?: BranchFloorReport | null
}) {
  const styles = QUEST_STYLES[quest.type]
  const impSlots =
    quest.type === 'ambitious_imp' ? (quest.contract.slots ?? []) : []
  const wandSlots =
    quest.type === 'old_wandmaker' ? (quest.contract.slots ?? []) : []
  const baselineRewards = prefersBaselineRewards(quest)
    ? groupsWithPrediction(rewards, 'baseline')
    : []
  const displayedRewards = baselineRewards.length
    ? baselineRewards
    : withoutPrediction(rewards, 'baseline')

  const hasRewardBlock =
    displayedRewards.length > 0 || impSlots.length > 0 || wandSlots.length > 0

  return (
    <div
      data-quest-type={quest.type}
      className={cn(
        'flex flex-col overflow-hidden rounded-none border',
        styles.border,
        branch ? null : 'gap-1.5 px-3 py-2.5'
      )}
    >
      {branch ? (
        <QuestBranchMap
          branch={branch}
          identities={identities}
          title={styles.title}
          target={<QuestTarget quest={quest} />}
        />
      ) : (
        <>
          <div className="flex flex-wrap items-center gap-1.5">
            <Badge
              variant="outline"
              className={cn('font-medium', styles.badge)}
            >
              {styles.title}
            </Badge>
          </div>
          <QuestTarget quest={quest} />
        </>
      )}
      {hasRewardBlock ? (
        <div
          className={cn(
            'flex flex-col gap-1',
            branch ? 'border-t px-3 py-2.5' : 'border-t pt-2'
          )}
        >
          <div className="flex items-center gap-1">
            <p className="text-muted-foreground text-xs font-medium tracking-wide uppercase">
              {rewardsHeading(quest, baselineRewards.length > 0)}
            </p>
            {impSlots.length > 0 ? (
              <ImpRewardDistribution
                slots={impSlots}
                upgradesPinned={
                  quest.type === 'ambitious_imp'
                    ? (quest.contract.upgrades_pinned ?? true)
                    : true
                }
                totalUpgradeValue={
                  quest.type === 'ambitious_imp'
                    ? quest.contract.total_upgrade_value
                    : undefined
                }
              />
            ) : null}
            {wandSlots.length > 0 ? (
              <WandmakerRewardDistribution slots={wandSlots} />
            ) : null}
          </div>
          {displayedRewards.length > 0 ? (
            <FloorItemList
              items={displayedRewards}
              identities={identities}
              depth={depth}
            />
          ) : null}
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
