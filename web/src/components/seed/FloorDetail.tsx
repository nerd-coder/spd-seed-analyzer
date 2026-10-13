import { AlchemyPotIcon } from '@/components/AlchemyPotIcon'
import { DepthIcon } from '@/components/DepthIcon'
import { FloorMapPreview } from '@/components/FloorMapPreview'
import { finderItemLabel } from '@/components/finder/finder-items'
import { ItemBox } from '@/components/ItemBox'
import {
  FloorItemSections,
  visibleItemGroups,
} from '@/components/seed/FloorItemSections'
import { QuestBranchMap } from '@/components/seed/QuestBranchMap'
import { QuestCard } from '@/components/seed/QuestCard'
import { branchObjectiveTarget } from '@/components/seed/QuestTarget'
import {
  branchTitle,
  matchingQuestBranch,
  unclaimedBranches,
} from '@/components/seed/quest-branch'
import {
  isFirstAlchemyPot,
  trinketOfferDetails,
} from '@/components/seed/trinket-offers'
import { itemSprite } from '@/lib/item-kind'
import { formatItemSource } from '@/lib/labels'
import type {
  FloorReport,
  GuaranteedAppearance,
  IdentityMaps,
  TrinketSelectionReport,
} from '@/lib/spd-wasm'
import { cn } from '@/lib/utils'

function alchemyPotLabel(appearance: GuaranteedAppearance) {
  const source = formatItemSource(appearance.source)
  return source ? `${appearance.name}, ${source}` : appearance.name
}

export function FloorDetail({
  floor,
  identities,
  trinketSelection,
  offerClasses,
}: {
  floor: FloorReport
  identities: IdentityMaps
  trinketSelection: TrinketSelectionReport
  offerClasses: readonly string[]
}) {
  const displayedMap = floor.map ?? floor.assumed_map ?? null
  const visibleItems = visibleItemGroups(floor.items)
  const leftoverBranches = unclaimedBranches(floor.branches, floor.quests)
  const alchemyPots = (floor.guaranteed_appearances ?? []).filter(
    (appearance) => appearance.kind === 'alchemy_pot'
  )
  const offerPotIndex = alchemyPots.findIndex((appearance) =>
    isFirstAlchemyPot(appearance, floor.depth, trinketSelection)
  )

  return (
    <section>
      {/* Sticky below the seed tabs. z-10 keeps the banner's own layers under that bar. */}
      <div
        className="sticky z-10"
        style={{ top: 'var(--seed-tabs-height, 3rem)' }}
      >
        <div className="relative isolate w-full overflow-hidden border-y border-border bg-black">
          {displayedMap ? (
            <FloorMapPreview
              variant="banner"
              map={displayedMap}
              identities={identities}
              depth={floor.depth}
            />
          ) : null}
          <div className="pointer-events-none absolute inset-0 z-10 bg-white/70 dark:bg-black/50" />
          <div
            className={cn(
              'relative z-20 flex w-full flex-wrap items-center gap-2 px-3 py-2',
              displayedMap && 'pr-12'
            )}
          >
            <DepthIcon feeling={floor.feeling} size={32} />
            <h3 className="spd-floor-label font-mono text-sm font-bold text-white tabular-nums">
              Floor {floor.depth}
            </h3>
            {alchemyPots.map((appearance, index) => (
              <span
                key={`${appearance.source ?? appearance.kind}-${index}`}
                className="inline-flex items-center gap-1"
              >
                <AlchemyPotIcon label={alchemyPotLabel(appearance)} />
                {index === offerPotIndex ? (
                  <span className="inline-flex items-center gap-0">
                    {offerClasses.map((className, offerIndex) => (
                      <ItemBox
                        key={className}
                        scale={1}
                        border
                        bg
                        title={finderItemLabel(className)}
                        sprites={itemSprite({
                          className,
                          category: 'trinket',
                        })}
                        details={trinketOfferDetails(
                          offerIndex,
                          offerClasses.length,
                          trinketSelection.transmutation_sequence
                        )}
                      />
                    ))}
                  </span>
                ) : null}
              </span>
            ))}
          </div>
        </div>
      </div>

      <div className="space-y-3 px-4 py-4">
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

        <FloorItemSections floor={floor} identities={identities} />

        {leftoverBranches.length > 0 && (
          <div className="space-y-2">
            {leftoverBranches.map((branch) => (
              <QuestBranchMap
                key={`${branch.id.depth}-${branch.id.branch}`}
                branch={branch}
                identities={identities}
                title={branchTitle(branch.kind)}
                target={branchObjectiveTarget(branch)}
              />
            ))}
          </div>
        )}
      </div>
    </section>
  )
}
