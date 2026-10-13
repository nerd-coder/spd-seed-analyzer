import { DepthIcon } from '@/components/DepthIcon'
import { FloorMapPreview } from '@/components/FloorMapPreview'
import { FloorAppearanceSection } from '@/components/seed/FloorAppearanceSection'
import {
  FloorItemSections,
  visibleItemGroups,
} from '@/components/seed/FloorItemSections'
import { QuestBranchMap } from '@/components/seed/QuestBranchMap'
import { QuestCard } from '@/components/seed/QuestCard'
import {
  branchTitle,
  matchingQuestBranch,
  unclaimedBranches,
} from '@/components/seed/quest-branch'
import type {
  BranchFloorReport,
  FloorReport,
  IdentityMaps,
} from '@/lib/spd-wasm'

function branchBorderClass(kind: BranchFloorReport['kind']) {
  switch (kind) {
    case 'blacksmith_mine':
      return 'border-amber-700/40'
    case 'imp_vault':
      return 'border-rose-700/40'
  }
}

function NestedBranchFloor({
  branch,
  identities,
}: {
  branch: BranchFloorReport
  identities: IdentityMaps
}) {
  return (
    <section
      className={`space-y-3 border-l-2 bg-muted/25 px-3 py-3 ${branchBorderClass(branch.kind)}`}
    >
      <h4 className="font-heading text-sm font-medium">
        {branchTitle(branch.kind)}
      </h4>
      <QuestBranchMap branch={branch} identities={identities} />
    </section>
  )
}

export function FloorDetail({
  floor,
  identities,
}: {
  floor: FloorReport
  identities: IdentityMaps
}) {
  const displayedMap = floor.map ?? floor.assumed_map ?? null
  const visibleItems = visibleItemGroups(floor.items)
  const leftoverBranches = unclaimedBranches(floor.branches, floor.quests)

  return (
    <section className="space-y-3 border-b py-6 first:pt-0 last:border-b-0 last:pb-0">
      <div className="relative h-32 w-full overflow-hidden bg-black focus-within:ring-2 focus-within:ring-ring">
        {displayedMap ? (
          <FloorMapPreview
            variant="banner"
            map={displayedMap}
            identities={identities}
            depth={floor.depth}
          />
        ) : null}
        <div className="pointer-events-none absolute inset-0 z-10 bg-black/50" />
        <div className="pointer-events-none absolute inset-0 z-20 flex items-center gap-2 px-3">
          <DepthIcon feeling={floor.feeling} size={32} />
          <h3 className="font-mono text-sm font-medium text-white tabular-nums">
            Floor {floor.depth}
          </h3>
        </div>
      </div>

      <div className="space-y-3">
        {floor.quests && floor.quests.length > 0 && (
          <div className="space-y-2">
            <p className="text-muted-foreground text-xs font-medium tracking-wide uppercase">
              Quests
            </p>
            <div className="space-y-2">
              {floor.quests.map((q, i) => (
                <QuestCard
                  key={`${floor.depth}-quest-${i}`}
                  quest={q}
                  rewards={visibleItems.filter(
                    (item) => item.source === q.contract.rewards.item_source
                  )}
                  identities={identities}
                  depth={floor.depth}
                  branch={matchingQuestBranch(q, floor.branches)}
                />
              ))}
            </div>
          </div>
        )}

        <FloorAppearanceSection appearances={floor.guaranteed_appearances} />
        <FloorItemSections floor={floor} identities={identities} />
      </div>

      {leftoverBranches.length > 0 && (
        <div className="space-y-2 pt-1">
          {leftoverBranches.map((branch) => (
            <NestedBranchFloor
              key={`${branch.id.depth}-${branch.id.branch}`}
              branch={branch}
              identities={identities}
            />
          ))}
        </div>
      )}
    </section>
  )
}
