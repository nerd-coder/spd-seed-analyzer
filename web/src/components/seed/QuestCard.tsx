import { FloorItemList } from '@/components/seed/FloorItemSections'
import { ImpRewardDistribution } from '@/components/seed/ImpRewardDistribution'
import { QuestBranchMap } from '@/components/seed/QuestBranchMap'
import { QuestTarget } from '@/components/seed/QuestTarget'
import { WandmakerRewardDistribution } from '@/components/seed/WandmakerRewardDistribution'
import type {
  BranchFloorReport,
  IdentityMaps,
  ItemGroup,
  QuestReport,
} from '@/lib/spd-wasm'
import { cn } from '@/lib/utils'

const QUEST_TITLES: Record<QuestReport['type'], string> = {
  sad_ghost: 'Sad Ghost',
  old_wandmaker: 'Old Wandmaker',
  troll_blacksmith: 'Troll Blacksmith',
  ambitious_imp: 'Ambitious Imp',
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
  const title = QUEST_TITLES[quest.type]
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
    // Quests share one band: the lantern rule marks them out of the item
    // lists without spending a hue the regions and loot tiers already own.
    <div
      data-quest-type={quest.type}
      className={cn(
        'flex flex-col overflow-hidden rounded-md border border-l-4 border-l-primary bg-accent/40',
        branch ? null : 'gap-2 px-3 py-2.5'
      )}
    >
      {branch ? (
        <QuestBranchMap
          branch={branch}
          identities={identities}
          title={title}
          target={<QuestTarget quest={quest} />}
        />
      ) : (
        <>
          <h4 className="font-semibold">{title}</h4>
          <QuestTarget quest={quest} />
        </>
      )}
      {hasRewardBlock ? (
        <div
          className={cn(
            'flex flex-col gap-2',
            branch ? 'border-t px-3 py-2.5' : 'border-t pt-2.5'
          )}
        >
          <div className="flex items-center gap-1">
            <p className="text-sm font-semibold">
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
