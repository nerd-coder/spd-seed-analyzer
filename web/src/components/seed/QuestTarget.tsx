import { InfoIcon } from '@phosphor-icons/react'
import {
  filmFrames,
  GameSprite,
  type SpriteSheet,
} from '@/components/GameSprite'
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

/** Longer edge of a quest-mob sprite, in CSS pixels. */
const LONG_EDGE = 36

/** Idle clip copied from the mob's `*Sprite` class. Repeated indices hold a pose. */
type MobClip = {
  sheet: SpriteSheet
  frames: ReturnType<typeof filmFrames>
  fps: number
  textKey: string
}

function mobClip(
  file: string,
  sheetWidth: number,
  sheetHeight: number,
  frameWidth: number,
  frameHeight: number,
  indices: readonly number[],
  fps: number
) {
  return {
    sheet: {
      url: `/assets/sprites/${file}`,
      width: sheetWidth,
      height: sheetHeight,
    },
    frames: filmFrames(sheetWidth, frameWidth, frameHeight, indices),
    fps,
  }
}

/** `FetidRatSprite` idle, 2 fps. */
const FETID_RAT: MobClip = {
  ...mobClip('rat.png', 256, 64, 16, 15, [32, 32, 32, 33], 2),
  textKey: 'FetidRat',
}

/** `GnollTricksterSprite` idle, column 42, 2 fps. */
const GNOLL_TRICKSTER: MobClip = {
  ...mobClip('gnoll.png', 256, 64, 12, 15, [42, 42, 42, 43, 42, 42, 43, 43], 2),
  textKey: 'GnollTrickster',
}

/** `GreatCrabSprite` idle, column 32, 5 fps. */
const GREAT_CRAB: MobClip = {
  ...mobClip('crab.png', 256, 64, 16, 16, [32, 33, 32, 34], 5),
  textKey: 'GreatCrab',
}

/** `GnollGeomancerSprite` idle, 2 fps. The statue offset is not used here. */
const GNOLL_GEOMANCER: MobClip = {
  ...mobClip(
    'gnoll_geomancer.png',
    256,
    32,
    12,
    16,
    [0, 0, 0, 1, 0, 0, 1, 1],
    2
  ),
  textKey: 'GnollGeomancer',
}

/**
 * Blue full-health cell. The spire's color is `Random.Int(3)` at spawn and
 * is not on the quest report, so the clip stays the blue frame.
 * `CrystalSpireSprite` idle is that one cell at 1 fps.
 */
const CRYSTAL_SPIRE: MobClip = {
  ...mobClip('crystal_spire.png', 128, 128, 29, 40, [0], 1),
  textKey: 'CrystalSpire',
}

/** `ImpSprite` idle, 10 fps, 12×14 film on `sprites/imp.png`. */
const IMP: MobClip = {
  ...mobClip(
    'imp.png',
    64,
    16,
    12,
    14,
    [
      0, 1, 2, 3, 0, 1, 2, 3, 0, 0, 0, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 0, 1, 2, 3,
      0, 1, 2, 3, 0, 1, 3, 0, 0, 0, 4, 4, 4, 4, 4, 4, 4, 4, 0, 0, 0, 4, 4, 4, 4,
      4, 4, 4, 4,
    ],
    10
  ),
  textKey: 'Imp',
}

const GHOST_FRAMES: Record<GhostTarget, MobClip> = {
  fetid_rat: FETID_RAT,
  gnoll_trickster: GNOLL_TRICKSTER,
  great_crab: GREAT_CRAB,
}

const GHOST_LABELS: Record<GhostTarget, string> = {
  fetid_rat: 'Fetid Rat',
  gnoll_trickster: 'Gnoll Trickster',
  great_crab: 'Great Crab',
}

const WANDMAKER_LABELS: Record<WandmakerObjective, string> = {
  corpse_dust: 'Corpse Dust',
  elemental_embers: 'Elemental Embers',
  rotberry: 'Rotberry',
}

const BLACKSMITH_LABELS: Record<BlacksmithObjective, string> = {
  crystal: 'Crystal',
  gnoll: 'Gnoll',
}

/** `ItemSpriteSheet.QUEST = xy(1, 30) = 464`. Dust is +1, ember is +3. */
const CORPSE_DUST_INDEX = 465
const ELEMENTAL_EMBERS_INDEX = 467

function MobSprite({ clip, label }: { clip: MobClip; label: string }) {
  return (
    <GameSprite
      sheet={clip.sheet}
      frames={clip.frames}
      fps={clip.fps}
      size={LONG_EDGE}
      label={label}
      title={label}
      textKey={clip.textKey}
    />
  )
}

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
  return (
    <MobSprite
      clip={objective === 'crystal' ? CRYSTAL_SPIRE : GNOLL_GEOMANCER}
      label={BLACKSMITH_LABELS[objective]}
    />
  )
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

function ImpTarget() {
  return (
    <span className="inline-flex items-center gap-1">
      <MobSprite clip={IMP} label="Imp" />
      <ImpStatueNote />
    </span>
  )
}

/** Quest target image. */
export function QuestTarget({ quest }: { quest: QuestReport }) {
  switch (quest.type) {
    case 'sad_ghost':
      return (
        <MobSprite
          clip={GHOST_FRAMES[quest.baseline.target]}
          label={GHOST_LABELS[quest.baseline.target]}
        />
      )
    case 'old_wandmaker':
      return <WandmakerTarget objective={quest.baseline.objective} />
    case 'troll_blacksmith':
      return <BlacksmithTarget objective={quest.baseline.objective} />
    case 'ambitious_imp':
      return <ImpTarget />
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
