import { InfoIcon } from '@phosphor-icons/react'
import { GameObject, type GameObjectId } from '@/components/GameObject'
import { ItemBox } from '@/components/ItemBox'
import { Button } from '@/components/ui/button'
import {
  Popover,
  PopoverContent,
  PopoverDescription,
  PopoverHeader,
  PopoverTitle,
  PopoverTrigger,
} from '@/components/ui/popover'
import { itemSprite } from '@/lib/item-kind'
import type {
  BlacksmithObjective,
  BranchFloorReport,
  GhostTarget,
  QuestReport,
  WandmakerObjective,
} from '@/lib/spd-wasm'

const GHOST_TARGETS: Record<GhostTarget, GameObjectId> = {
  fetid_rat: 'FetidRat',
  gnoll_trickster: 'GnollTrickster',
  great_crab: 'GreatCrab',
}

const WANDMAKER_LABELS: Record<WandmakerObjective, string> = {
  corpse_dust: 'Corpse Dust',
  elemental_embers: 'Elemental Embers',
  rotberry: 'Rotberry',
}

const BLACKSMITH_TARGETS: Record<
  BlacksmithObjective,
  { id: GameObjectId; label: string }
> = {
  crystal: { id: 'CrystalSpire', label: 'Crystal' },
  gnoll: { id: 'GnollGeomancer', label: 'Gnoll' },
}

/** `ItemSpriteSheet.QUEST = xy(1, 30) = 464`. Dust is +1, ember is +3. */
const CORPSE_DUST_INDEX = 465
const ELEMENTAL_EMBERS_INDEX = 467

function WandmakerTarget({ objective }: { objective: WandmakerObjective }) {
  const label = WANDMAKER_LABELS[objective]
  if (objective === 'rotberry') {
    return (
      <ItemBox
        scale={2}
        title={label}
        sprites={itemSprite({ className: 'RotberrySeed' })}
      />
    )
  }
  return (
    <ItemBox
      scale={2}
      title={label}
      textKey={objective === 'corpse_dust' ? 'CorpseDust' : 'Embers'}
      sprites={{
        index:
          objective === 'corpse_dust'
            ? CORPSE_DUST_INDEX
            : ELEMENTAL_EMBERS_INDEX,
      }}
    />
  )
}

function BlacksmithTarget({ objective }: { objective: BlacksmithObjective }) {
  const target = BLACKSMITH_TARGETS[objective]
  return <GameObject id={target.id} label={target.label} />
}

/**
 * Every option is +2…+5, and `EscapeCrystal` only lets an item above +1 leave
 * at score >= 4000. The best non-statue total is 3000, so the statue is the
 * only way to keep one of the six.
 */
function ImpStatueNote() {
  return (
    <Popover>
      <PopoverTrigger asChild>
        <Button
          variant="ghost"
          size="icon-xs"
          aria-label="About the Imp Statue"
          className="text-white hover:text-white"
        >
          <InfoIcon />
        </Button>
      </PopoverTrigger>
      <PopoverContent align="start" className="w-80">
        <PopoverHeader>
          <PopoverTitle>Imp Statue</PopoverTitle>
          <PopoverDescription>
            Keeping one of these six requires leaving with the Imp Statue: every
            option is +2 or better, and lower vault scores can only take out an
            item at +0/+1.
          </PopoverDescription>
        </PopoverHeader>
      </PopoverContent>
    </Popover>
  )
}

/** Quest target image. The Imp quest has none; its banner carries the statue note. */
export function QuestTarget({ quest }: { quest: QuestReport }) {
  switch (quest.type) {
    case 'sad_ghost':
      return <GameObject id={GHOST_TARGETS[quest.variable.target]} />
    case 'old_wandmaker':
      return <WandmakerTarget objective={quest.variable.objective} />
    case 'troll_blacksmith':
      return <BlacksmithTarget objective={quest.variable.objective} />
    case 'ambitious_imp':
      return <ImpStatueNote />
  }
}

/** Target image for a branch that is not rendered inside its quest card. */
export function branchObjectiveTarget(branch: BranchFloorReport) {
  if (branch.kind !== 'blacksmith_mine') return null
  if (branch.objective === 'Crystal') {
    return <BlacksmithTarget objective="crystal" />
  }
  if (branch.objective === 'Gnoll') {
    return <BlacksmithTarget objective="gnoll" />
  }
  return null
}
