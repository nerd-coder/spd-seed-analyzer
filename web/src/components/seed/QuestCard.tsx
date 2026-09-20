import { FloorItemList } from '@/components/seed/FloorItemSections'
import { ImpRewardDistribution } from '@/components/seed/ImpRewardDistribution'
import { QuestBranchMap } from '@/components/seed/QuestBranchMap'
import { Alert, AlertDescription, AlertTitle } from '@/components/ui/alert'
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
      return `Spawned on depth ${quest.baseline.spawn_depth}`
  }
}

function baselineContract(quest: QuestReport) {
  switch (quest.type) {
    case 'old_wandmaker':
      return 'Reward contract: two distinct uncursed +1…+3 wands; complete the quest and choose one.'
    case 'ambitious_imp':
      return 'Reward contract: six vault take-out options; the player keeps one.'
    default:
      return null
  }
}

function shopGateCopy(quest: QuestReport) {
  if (quest.type !== 'ambitious_imp') return null
  return 'The floor 20/21 Imp shop is not guaranteed. It appears only after completing the vault with score > 2000.'
}

/**
 * Every option is +2…+5, and `EscapeCrystal` only lets an item above +1 leave
 * at score >= 4000. The best non-statue total is 3000, so the statue is the
 * only way to keep one of the six.
 */
function extractionGateCopy(quest: QuestReport) {
  if (quest.type !== 'ambitious_imp') return null
  return 'Keeping one of these six requires leaving with the Imp Statue: every option is +2 or better, and lower vault scores can only take out an item at +0/+1.'
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
  const contract = baselineContract(quest)
  const shopGate = shopGateCopy(quest)
  const extractionGate = extractionGateCopy(quest)
  const impSlots =
    quest.type === 'ambitious_imp' ? (quest.contract.slots ?? []) : []
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
      {shopGate ? (
        <p className="text-muted-foreground text-xs leading-relaxed">
          {shopGate}
        </p>
      ) : null}
      {extractionGate ? (
        <p className="text-muted-foreground text-xs leading-relaxed">
          {extractionGate}
        </p>
      ) : null}
      {contract ? (
        <Alert variant="warning">
          <AlertTitle>Fresh/no-history baseline</AlertTitle>
          <AlertDescription>
            Player choices, trinkets, challenges, or prior generation can change
            this target and reward. {contract}
          </AlertDescription>
        </Alert>
      ) : null}
      {displayedRewards.length > 0 || impSlots.length > 0 ? (
        <div className="flex flex-col gap-1 border-t pt-2">
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
      {branch ? (
        <div className="border-t pt-2">
          <QuestBranchMap branch={branch} identities={identities} />
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
