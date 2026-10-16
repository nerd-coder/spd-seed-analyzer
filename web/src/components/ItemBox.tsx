import {
  type CSSProperties,
  type MouseEvent,
  type ReactNode,
  type Ref,
  useRef,
} from 'react'
import {
  itemDetailAvailable,
  scheduleHideItemDetail,
  showItemDetail,
  togglePinItemDetail,
} from '@/components/item-detail'
import type { ItemContractSummary } from '@/lib/item-contract'
import {
  curseBorderColor,
  enhancementBorderColor,
  enhancementColor,
} from '@/lib/item-glow'
import {
  ITEM_ICON_SHEET,
  ITEM_SHEET,
  isGenericItemIconIndex,
  itemIconStyle,
  itemTypeIconIndex,
  resolveItemIconIndex,
  type SpriteSheet,
} from '@/lib/item-icons'
import {
  type ItemEnhancement,
  ItemSheetKind,
  type ItemSpriteSpec,
} from '@/lib/item-kind'
import { itemIconSpriteSize, itemSpriteSize } from '@/lib/item-sprite-rects'
import { itemGameText } from '@/lib/item-text'
import { upgradeLevelColorClass } from '@/lib/upgrade'
import { cn } from '@/lib/utils'

/** SPD `ScrollingGridPane` cell, in native pixels. */
const CELL = 17
/** Extra pixels on each side so the frame sits off the sprite. */
const PAD = 4

type ItemBoxProps = {
  sprites?: ItemSpriteSpec | ItemSpriteSpec[]
  /** Journal second icon. Omit to use the potion, scroll, or ring type mark. */
  subSprite?: ItemSpriteSpec | null
  /** Enchantment, glyph, or curse. Tints the main sprite and the frame. */
  enhancement?: ItemEnhancement | null
  /** Cursed equipment uses a red frame when no enhancement color is set. */
  cursed?: boolean
  /** Upgrade or degrade. Draws a colored `+N` chip. `0` draws nothing. */
  level?: number | null
  /** Draw the slot frame. Default true; off when the slot sits inside a control. */
  border?: boolean
  /** Fill the slot. Default true. */
  bg?: boolean
  /** Replaces the slot fill when `bg` is set. */
  bgColor?: string
  /** One line, drawn inside the bottom of the square. */
  caption?: ReactNode
  /** Multiplier of the 17px journal cell. Default 2. */
  scale?: number
  title?: string
  /**
   * Item-window class when the sprite itself has no class name.
   * Used for quest items drawn by sprite index, such as corpse dust.
   */
  textKey?: string
  /**
   * Analyzer notes, shown under the game description.
   * Omit to hide that section.
   */
  details?: ReactNode
  /** Constrained sibling facts for a gameplay-variable sample. */
  contract?: ItemContractSummary | null
  /** Where the item spawns, e.g. `Floor 3, Crystal vault`. */
  location?: string | null
  /**
   * Hover and click open the game description. Default true.
   * Turn off when another control covers the slot.
   */
  panel?: boolean
  className?: string
  /** Drawn inside the slot, above the sprites. Quantity lives here. */
  children?: ReactNode
}

function sheetOf(spec: ItemSpriteSpec): SpriteSheet {
  return spec.sheet === ItemSheetKind.ItemIcons ? ITEM_ICON_SHEET : ITEM_SHEET
}

function frameOf(spec: ItemSpriteSpec, index: number, sheet: SpriteSheet) {
  if (spec.sourceWidth != null || spec.sourceHeight != null) {
    return {
      sourceWidth: spec.sourceWidth ?? sheet.size,
      sourceHeight: spec.sourceHeight ?? sheet.size,
    }
  }
  const [sourceWidth, sourceHeight] =
    sheet === ITEM_ICON_SHEET
      ? itemIconSpriteSize(index)
      : itemSpriteSize(index)
  return { sourceWidth, sourceHeight }
}

function spriteStyle(spec: ItemSpriteSpec, scale: number) {
  const sheet = sheetOf(spec)
  const index =
    spec.index ??
    resolveItemIconIndex(spec.className, {
      category: spec.category,
      appearance: spec.appearance,
    })
  const frame = frameOf(spec, index, sheet)
  const style = itemIconStyle(
    index,
    scale * Math.max(frame.sourceWidth, frame.sourceHeight),
    frame.sourceWidth,
    frame.sourceHeight,
    true,
    sheet
  )
  return {
    style,
    invert: sheet === ITEM_SHEET && isGenericItemIconIndex(index),
  }
}

function ItemGlyph({
  spec,
  scale,
  glow,
}: {
  spec: ItemSpriteSpec
  scale: number
  glow?: string | null
}) {
  const { style, invert } = spriteStyle(spec, scale)
  const glowStyle: CSSProperties | undefined = glow
    ? {
        ...style,
        backgroundColor: glow,
        backgroundImage: 'none',
        mixBlendMode:
          glow.toLowerCase() === '#000000' ? 'multiply' : 'plus-lighter',
        WebkitMaskImage: style.backgroundImage,
        maskImage: style.backgroundImage,
        WebkitMaskSize: style.backgroundSize,
        maskSize: style.backgroundSize,
        WebkitMaskPosition: style.backgroundPosition,
        maskPosition: style.backgroundPosition,
        WebkitMaskRepeat: 'no-repeat',
        maskRepeat: 'no-repeat',
      }
    : undefined

  return (
    <span
      className="relative isolate"
      style={{ width: style.width, height: style.height }}
    >
      <span
        aria-hidden="true"
        className={cn(invert && 'dark:invert')}
        style={style}
      />
      {glowStyle ? (
        <span
          className="spd-item-glow pointer-events-none absolute top-0 left-0"
          style={glowStyle}
        />
      ) : null}
    </span>
  )
}

/** Film-frame clip. The slot centers this box. */
function MainSpriteClip({
  spec,
  scale,
  glow,
}: {
  spec: ItemSpriteSpec
  scale: number
  glow?: string | null
}) {
  const { style } = spriteStyle(spec, scale)
  return (
    <span
      className="col-start-1 row-start-1 overflow-hidden"
      style={{ width: style.width, height: style.height }}
    >
      <ItemGlyph spec={spec} scale={scale} glow={glow} />
    </span>
  )
}

function spriteList(sprites: ItemBoxProps['sprites']): ItemSpriteSpec[] {
  if (!sprites) return []
  return Array.isArray(sprites) ? sprites : [sprites]
}

function subjectClass(sprites: ItemSpriteSpec[]): string | null {
  for (const spec of sprites) {
    if (spec.className && itemGameText(spec.className)) return spec.className
  }
  return sprites.find((spec) => spec.className)?.className ?? null
}

function levelLabel(level: number): string {
  return level > 0 ? `+${level}` : `${level}`
}

type ItemFrameProps = {
  ref?: Ref<HTMLButtonElement | HTMLSpanElement>
  interactive: boolean
  size: number
  bg: boolean
  bgColor?: string
  border?: boolean
  /** Enchantment or curse frame; the theme border otherwise. */
  borderColor: string | null
  borderWidth: number
  label: string
  title?: string
  className?: string
  caption?: ReactNode
  level?: number | null
  mains: ItemSpriteSpec[]
  corner: ItemSpriteSpec | null
  glow: string | null
  scale: number
  children?: ReactNode
  onPointerEnter?: () => void
  onPointerLeave?: () => void
  onClick?: (event: MouseEvent<HTMLButtonElement>) => void
}

function ItemFrame({
  ref,
  interactive,
  size,
  bg,
  bgColor,
  border = true,
  borderColor,
  borderWidth,
  label,
  title,
  className,
  caption,
  level,
  mains,
  corner,
  glow,
  scale,
  children,
  onPointerEnter,
  onPointerLeave,
  onClick,
}: ItemFrameProps) {
  const hasCaption = caption != null && caption !== false && caption !== ''
  const showLevel = level != null && level !== 0
  const shared = {
    'aria-label': label,
    'data-slot': 'item-box',
    className: cn(
      'relative inline-block shrink-0 align-middle font-sans',
      border ? 'border border-border' : 'border-0',
      bg && 'inset-shadow-recessed',
      bg && !bgColor && 'bg-muted',
      interactive &&
        'cursor-pointer p-0 text-left font-inherit select-none focus-visible:outline-2 focus-visible:outline-offset-1 focus-visible:outline-ring',
      className
    ),
    style: {
      width: size,
      height: size,
      background: !bg ? 'transparent' : bgColor,
      borderColor: border ? (borderColor ?? undefined) : undefined,
      borderWidth: border && borderColor ? borderWidth : undefined,
    },
    onPointerEnter,
    onPointerLeave,
  }
  const body = (
    <>
      <span className="pointer-events-none absolute inset-0 grid place-items-center">
        {mains.map((spec, index) => (
          <MainSpriteClip
            key={`${spec.className ?? spec.index ?? 'sprite'}-${index}`}
            spec={spec}
            scale={scale}
            glow={glow}
          />
        ))}
      </span>
      {corner ? (
        <span className="pointer-events-none absolute top-0.5 right-0.5">
          <ItemGlyph spec={corner} scale={scale} />
        </span>
      ) : null}
      {children}
      {showLevel ? (
        // `dark` scopes the bright tier values: like the game, labels on art
        // are outlined in black in both themes.
        <span
          data-upgrade={level}
          className={cn(
            'dark ink-outline pointer-events-none absolute right-0.5 leading-[20px]',
            scale >= 2 ? 'font-pixel text-px-1' : 'font-sans text-xs font-bold',
            hasCaption ? 'bottom-4' : '-bottom-0.5',
            upgradeLevelColorClass(level)
          )}
        >
          {levelLabel(level)}
        </span>
      ) : null}
      {hasCaption ? (
        <span className="pointer-events-none absolute inset-x-0 bottom-0 truncate bg-black/60 px-1 py-0.5 text-center font-sans text-[11px] leading-none font-normal tracking-normal text-white capitalize">
          {caption}
        </span>
      ) : null}
    </>
  )

  if (interactive) {
    return (
      <button
        {...shared}
        ref={ref as Ref<HTMLButtonElement>}
        type="button"
        onClick={onClick}
      >
        {body}
      </button>
    )
  }

  return (
    <span {...shared} ref={ref as Ref<HTMLSpanElement>} title={title}>
      {body}
    </span>
  )
}

/**
 * Journal catalog cell (`ScrollingGridPane.GridItem`) plus an optional caption.
 * Each main sprite is its film crop, centered in the square. The sub-sprite
 * sits in the top-right. An enhancement tints the sprite and recolors the
 * frame. Hover or click opens the game description when one exists.
 */
export function ItemBox({
  sprites,
  subSprite,
  enhancement,
  cursed = false,
  level,
  border = true,
  bg = true,
  bgColor,
  caption,
  scale = 2,
  title,
  textKey,
  details,
  contract,
  location,
  panel = true,
  className,
  children,
}: ItemBoxProps) {
  const size = CELL * scale + PAD * 2
  const mains = spriteList(sprites)
  const glow = enhancementColor(enhancement)
  const coloredBorder =
    enhancementBorderColor(enhancement) ?? (cursed ? curseBorderColor() : null)
  const autoType =
    subSprite === undefined ? itemTypeIconIndex(mains[0]?.className) : null
  const corner =
    subSprite === undefined
      ? autoType == null
        ? null
        : { index: autoType, sheet: ItemSheetKind.ItemIcons }
      : subSprite
  const classNameForText = subjectClass(mains)
  const detail = {
    title,
    className: textKey ?? classNameForText,
    enhancement,
    cursed,
    location,
    contract,
    details,
  }
  const interactive = panel && itemDetailAvailable(detail)
  const name =
    title ?? (typeof caption === 'string' ? caption : undefined) ?? 'item'
  const label = cursed ? `${name}, cursed` : name
  const anchorRef = useRef<HTMLButtonElement | HTMLSpanElement>(null)

  return (
    <ItemFrame
      ref={anchorRef}
      interactive={interactive}
      size={size}
      bg={bg || cursed}
      bgColor={bgColor}
      border={border}
      borderColor={coloredBorder}
      borderWidth={coloredBorder ? 2 : 1}
      label={label}
      title={interactive ? undefined : title}
      className={className}
      caption={caption}
      level={level}
      mains={mains}
      corner={corner}
      glow={glow}
      scale={scale}
      onPointerEnter={
        interactive
          ? () => {
              const anchor = anchorRef.current
              if (anchor) showItemDetail(anchor, detail)
            }
          : undefined
      }
      onPointerLeave={
        interactive
          ? () => {
              const anchor = anchorRef.current
              if (anchor) scheduleHideItemDetail(anchor)
            }
          : undefined
      }
      onClick={
        interactive
          ? (event) => {
              event.preventDefault()
              const anchor = anchorRef.current
              if (anchor) togglePinItemDetail(anchor, detail)
            }
          : undefined
      }
    >
      {children}
    </ItemFrame>
  )
}

export type { ItemBoxProps, ItemSpriteSpec }
