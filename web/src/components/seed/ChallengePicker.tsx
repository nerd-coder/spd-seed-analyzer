import { GameSprite } from '@/components/GameSprite'
import { ItemBox } from '@/components/ItemBox'
import { ToggleGroup, ToggleGroupItem } from '@/components/ui/toggle-group'
import { itemClassName } from '@/generated/game-ids'
import {
  type ChallengeArt,
  challengeName,
  GENERATION_CHALLENGES,
  PLAY_ONLY_CHALLENGES,
} from '@/lib/challenges'
import { filmFrames, GAME_OBJECTS } from '@/lib/game-objects'
import { itemSprite } from '@/lib/item-kind'
import type { Challenge, IdentityMaps } from '@/lib/spd-wasm'
import { CHALLENGE_ICONS, UI_ICONS } from '@/lib/ui-icons'

/** The player's challenge settings for one seed session. */
export type ChallengeChoice = {
  enabled: readonly Challenge[]
  /** A regenerated report is on its way. */
  busy: boolean
  select: (challenges: Challenge[]) => void
}

const ICON_SHEET = {
  url: UI_ICONS.url,
  width: UI_ICONS.sheetW,
  height: UI_ICONS.sheetH,
}

/** `RatSprite` first idle frame. */
const RAT = {
  sheet: { url: '/assets/sprites/rat.png', width: 256, height: 64 },
  frames: filmFrames(256, 16, 15, [0]),
}

function listNames(names: readonly string[]): string {
  if (names.length <= 1) return names.join('')
  return `${names.slice(0, -1).join(', ')} and ${names.at(-1)}`
}

function ChallengeArtwork({
  art,
  label,
  identities,
}: {
  art: ChallengeArt
  label: string
  identities: IdentityMaps
}) {
  if (art.kind === 'item') {
    const appearance = identities.scrolls.find(
      (entry) => itemClassName(entry.item) === art.className
    )?.appearance
    return (
      <ItemBox
        scale={3}
        panel={false}
        border={false}
        bg={false}
        sprites={itemSprite({
          className: art.className,
          category: art.category,
          appearance,
        })}
      />
    )
  }
  const clip = art.id === 'Rat' ? RAT : GAME_OBJECTS[art.id]
  return (
    <GameSprite
      sheet={clip.sheet}
      frames={clip.frames.slice(0, 1)}
      scale={art.id === 'Rat' ? 3 : 2}
      label={label}
    />
  )
}

function statusNote(enabled: readonly Challenge[]): string {
  if (enabled.length === 0) {
    return 'Floors are generated without challenges. Turn on the ones your run uses.'
  }
  return `Floors are generated with ${listNames(enabled.map(challengeName))}.`
}

/** Challenges that change generated floors; each toggle regenerates the report. */
export function ChallengePicker({
  choice,
  identities,
}: {
  choice: ChallengeChoice
  identities: IdentityMaps
}) {
  const champions = choice.enabled.includes('champion_enemies')
  const count = choice.enabled.length
  return (
    <section
      data-challenge-picker
      aria-labelledby="challenge-picker-title"
      aria-busy={choice.busy}
      className="flex flex-col gap-3"
    >
      <div className="flex items-center gap-2">
        <GameSprite
          sheet={ICON_SHEET}
          frames={[count > 0 ? CHALLENGE_ICONS.on : CHALLENGE_ICONS.off]}
          scale={2}
          label={count > 0 ? 'Challenges on' : 'No challenges'}
        />
        <h3 id="challenge-picker-title" className="font-pixel text-px-1">
          Challenges
        </h3>
        <span className="text-muted-foreground text-sm tabular-nums">
          {count} of {GENERATION_CHALLENGES.length} on
        </span>
      </div>

      <ToggleGroup
        type="multiple"
        variant="outline"
        spacing={2}
        value={[...choice.enabled]}
        disabled={choice.busy}
        aria-label="Challenges that change generated floors"
        className="grid w-full grid-cols-1 items-stretch sm:grid-cols-2 lg:grid-cols-5"
        onValueChange={(value) => choice.select(value as Challenge[])}
      >
        {GENERATION_CHALLENGES.map((challenge) => (
          <ToggleGroupItem
            key={challenge.id}
            value={challenge.id}
            aria-label={challenge.name}
            className="h-auto w-full cursor-pointer justify-start gap-3 px-3 py-2 text-left whitespace-normal lg:flex-col lg:items-center lg:justify-start lg:gap-2 lg:py-3 lg:text-center"
          >
            <span className="grid size-13 shrink-0 place-items-center">
              <ChallengeArtwork
                art={challenge.art}
                label={challenge.name}
                identities={identities}
              />
            </span>
            <span className="flex min-w-0 flex-col gap-0.5">
              <span className="text-foreground leading-tight">
                {challenge.name}
              </span>
              <span className="text-muted-foreground text-xs font-normal leading-tight">
                {challenge.effect}
              </span>
            </span>
          </ToggleGroupItem>
        ))}
      </ToggleGroup>

      <div className="flex flex-col gap-1 text-sm" aria-live="polite">
        <p className="text-foreground">
          {choice.busy ? 'Regenerating floors…' : statusNote(choice.enabled)}
        </p>
        {champions && !choice.busy ? (
          <p className="text-muted-foreground">
            Floor 1's champion is set by the seed. Deeper champions depend on
            how many monsters spawned before, so a giant one can move monsters
            and loot; those spots are marked "May differ".
          </p>
        ) : null}
        <p className="text-muted-foreground">
          {listNames(PLAY_ONLY_CHALLENGES)} only change play, so they leave
          floors as they are.
        </p>
      </div>
    </section>
  )
}
