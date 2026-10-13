import { GameSprite } from '@/components/GameSprite'
import { ALCHEMY_ICON, UI_ICONS } from '@/lib/ui-icons'

const ICONS = {
  url: UI_ICONS.url,
  width: UI_ICONS.sheetW,
  height: UI_ICONS.sheetH,
} as const

type Props = {
  /** Target display size in CSS px. Native sprite is 16×16. */
  size?: number
  label?: string
  className?: string
}

/**
 * SPD alchemy pot from `/assets/interfaces/icons.png` (`Icons.ALCHEMY`).
 */
export function AlchemyPotIcon({
  size = 32,
  label = 'Alchemy pot',
  className,
}: Props) {
  return (
    <GameSprite
      sheet={ICONS}
      frames={[ALCHEMY_ICON]}
      size={size}
      label={label}
      title={label}
      textKey="AlchemyPot"
      className={className}
    />
  )
}
