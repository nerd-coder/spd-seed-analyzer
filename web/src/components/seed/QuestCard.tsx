import { GameObject } from '@/components/GameObject'
import { FloorItemList } from '@/components/seed/FloorItemSections'
import { ImpRewardDistribution } from '@/components/seed/ImpRewardDistribution'
import { QuestBanner } from '@/components/seed/QuestBanner'
import { QuestTarget } from '@/components/seed/QuestTarget'
import { questNpc, questTitle } from '@/components/seed/quest-branch'
import { WandmakerRewardDistribution } from '@/components/seed/WandmakerRewardDistribution'
import type {
  BranchFloorReport,
  IdentityMaps,
  ItemGroup,
  QuestReport,
} from '@/lib/spd-wasm'

function prefersVariableRewards(quest: QuestReport) {
  return quest.type === 'old_wandmaker' || quest.type === 'ambitious_imp'
}

function rewardsHeading(quest: QuestReport, isVariable: boolean) {
  if (quest.type === 'ambitious_imp' || !isVariable) return 'Rewards'
  return 'Rewards (vary with gameplay)'
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
  const title = questTitle(quest.type)
  const impSlots =
    quest.type === 'ambitious_imp' ? (quest.contract.slots ?? []) : []
  const wandSlots =
    quest.type === 'old_wandmaker' ? (quest.contract.slots ?? []) : []
  const variableRewards = prefersVariableRewards(quest)
    ? groupsWithPrediction(rewards, 'variable')
    : []
  const displayedRewards = variableRewards.length
    ? variableRewards
    : withoutPrediction(rewards, 'variable')

  const hasRewardBlock =
    displayedRewards.length > 0 || impSlots.length > 0 || wandSlots.length > 0

  return (
    // Quests share one band: the lantern rule marks them out of the item
    // lists without spending a hue the regions and loot tiers already own.
    <div
      data-quest-type={quest.type}
      className="flex flex-col overflow-hidden rounded-md border border-l-4 border-l-primary bg-accent/40"
    >
      <QuestBanner
        depth={depth}
        title={title}
        npc={<GameObject id={questNpc(quest.type)} />}
        target={<QuestTarget quest={quest} />}
        branch={branch}
        identities={identities}
      />
      {hasRewardBlock ? (
        <div className="flex flex-col gap-2 border-t px-3 py-2.5">
          <div className="flex items-center gap-1">
            <p className="text-sm font-semibold">
              {rewardsHeading(quest, variableRewards.length > 0)}
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

function groupsWithPrediction(groups: ItemGroup[], prediction: 'variable') {
  return groups
    .map((group) => ({
      ...group,
      variants: group.variants.filter(
        (variant) => variant.prediction === prediction
      ),
    }))
    .filter((group) => group.variants.length > 0)
}

function withoutPrediction(groups: ItemGroup[], prediction: 'variable') {
  return groups
    .map((group) => ({
      ...group,
      variants: group.variants.filter(
        (variant) => variant.prediction !== prediction
      ),
    }))
    .filter((group) => group.variants.length > 0)
}
