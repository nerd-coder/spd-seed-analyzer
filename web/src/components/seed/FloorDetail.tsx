import { AlchemyPotIcon } from '@/components/AlchemyPotIcon'
import { DepthIcon } from '@/components/DepthIcon'
import { FloorMapDialog } from '@/components/FloorMapDialog'
import { finderItemLabel } from '@/components/finder/finder-items'
import { ItemBox } from '@/components/ItemBox'
import { FloorBanner } from '@/components/seed/FloorBanner'
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
    <section data-floor-depth={floor.depth}>
      {/* Sticky below the seed tabs and the floor strip; z-10 keeps it under both. */}
      <div
        className="sticky z-10"
        style={{
          top: 'calc(var(--seed-tabs-height, 3rem) + var(--descent-rail-height, 0px))',
        }}
      >
        <FloorBanner
          depth={floor.depth}
          tileset={displayedMap?.tileset}
          action={
            displayedMap ? (
              <FloorMapDialog
                map={displayedMap}
                identities={identities}
                depth={floor.depth}
              />
            ) : null
          }
        >
          {floor.feeling && floor.feeling !== 'none' ? (
            <DepthIcon feeling={floor.feeling} scale={4} />
          ) : null}
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
        </FloorBanner>
      </div>

      <div className="flex flex-col gap-4 px-4 py-4">
        {floor.quests && floor.quests.length > 0 && (
          <div className="space-y-2">
            <h4 className="text-sm font-semibold">Quests</h4>
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
