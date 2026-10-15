import {
  type ReactNode,
  useEffect,
  useRef,
  useState,
  useSyncExternalStore,
} from 'react'
import { Badge } from '@/components/ui/badge'
import type { ItemEnhancement } from '@/lib/item-kind'
import { gameItemName, itemGameText } from '@/lib/item-text'

const HIDE_MS = 160
const PANEL_WIDTH = 320
const PANEL_GAP = 6

type TextPart = {
  text: string
  highlight: boolean
}

/** SPD `RenderedTextBlock` toggles highlight on `_` and `**`. */
function gameTextParts(text: string): TextPart[] {
  const parts: TextPart[] = []
  let highlight = false
  let buffer = ''
  const push = () => {
    if (!buffer) return
    parts.push({ text: buffer, highlight })
    buffer = ''
  }
  for (let index = 0; index < text.length; index += 1) {
    if (text.startsWith('**', index)) {
      push()
      highlight = !highlight
      index += 1
      continue
    }
    if (text[index] === '_') {
      push()
      highlight = !highlight
      continue
    }
    buffer += text[index]
  }
  push()
  return parts
}

function GameText({ text }: { text: string }) {
  const parts = gameTextParts(text.replaceAll('%%', '%'))
  return (
    <>
      {parts.map((part, index) =>
        part.highlight ? (
          <span key={index} className="font-medium text-foreground">
            {part.text}
          </span>
        ) : (
          <span key={index}>{part.text}</span>
        )
      )}
    </>
  )
}

export type ItemDetailProps = {
  title?: string
  className?: string | null
  enhancement?: ItemEnhancement | null
  cursed?: boolean
  location?: string | null
  details?: ReactNode
}

export function itemDetailAvailable({
  className,
  enhancement,
  location,
  details,
}: ItemDetailProps): boolean {
  if (details != null && details !== false && details !== '') return true
  if (location) return true
  if (itemGameText(className)) return true
  if (itemGameText(enhancement)) return true
  return false
}

export function ItemDetail({
  title,
  className,
  enhancement,
  cursed = false,
  location,
  details,
}: ItemDetailProps) {
  const item = itemGameText(className)
  const enchantment = itemGameText(enhancement)
  const heading =
    title ??
    (item
      ? gameItemName(item.name)
      : enchantment
        ? gameItemName(enchantment.name)
        : 'Item')
  const descriptions = [item?.desc, enchantment?.desc].filter(
    (desc): desc is string => Boolean(desc)
  )

  return (
    <div className="flex flex-col gap-2">
      <div className="flex flex-wrap items-center gap-x-2 gap-y-1">
        <p className="font-semibold text-popover-foreground">{heading}</p>
        {cursed ? <Badge variant="destructive">Cursed</Badge> : null}
      </div>
      {descriptions.map((desc) => (
        <p
          key={desc}
          className="whitespace-pre-wrap text-sm text-muted-foreground"
        >
          <GameText text={desc} />
        </p>
      ))}
      {location ? (
        <div className="flex flex-col gap-1 border-t pt-2">
          <p className="text-sm font-semibold text-popover-foreground">
            Where to find
          </p>
          <p className="text-sm text-muted-foreground">{location}</p>
        </div>
      ) : null}
      {details != null && details !== false && details !== '' ? (
        <div className="flex flex-col gap-1 border-t pt-2">
          <p className="text-sm font-semibold text-popover-foreground">Notes</p>
          <div className="text-sm text-muted-foreground">{details}</div>
        </div>
      ) : null}
    </div>
  )
}

type ItemDetailPanel = {
  anchor: HTMLElement
  detail: ItemDetailProps
  pinned: boolean
}

let panel: ItemDetailPanel | null = null
let hideTimer: number | null = null
const listeners = new Set<() => void>()

function emitItemDetail() {
  for (const listener of listeners) listener()
}

function clearHideTimer() {
  if (hideTimer == null) return
  window.clearTimeout(hideTimer)
  hideTimer = null
}

function subscribeItemDetail(listener: () => void) {
  listeners.add(listener)
  return () => listeners.delete(listener)
}

function getItemDetailPanel() {
  return panel
}

/** Point the single description panel at this slot. A pinned panel stays put. */
export function showItemDetail(anchor: HTMLElement, detail: ItemDetailProps) {
  clearHideTimer()
  if (panel?.pinned) return
  if (panel?.anchor === anchor) return
  panel = { anchor, detail, pinned: false }
  emitItemDetail()
}

/** Pin the panel on this slot, or close it when it is already pinned there. */
export function togglePinItemDetail(
  anchor: HTMLElement,
  detail: ItemDetailProps
) {
  clearHideTimer()
  if (panel?.pinned && panel.anchor === anchor) {
    panel = null
  } else {
    panel = { anchor, detail, pinned: true }
  }
  emitItemDetail()
}

/** Close after a short delay unless the pointer enters another slot or the panel. */
export function scheduleHideItemDetail(anchor: HTMLElement) {
  if (panel?.pinned) return
  if (panel && panel.anchor !== anchor) return
  clearHideTimer()
  hideTimer = window.setTimeout(() => {
    hideTimer = null
    if (panel?.pinned) return
    panel = null
    emitItemDetail()
  }, HIDE_MS)
}

export function cancelHideItemDetail() {
  clearHideTimer()
}

export function hideItemDetail() {
  clearHideTimer()
  if (!panel) return
  panel = null
  emitItemDetail()
}

function panelPosition(anchor: DOMRect) {
  const margin = 8
  const left = Math.min(
    Math.max(margin, anchor.left),
    Math.max(margin, window.innerWidth - PANEL_WIDTH - margin)
  )
  const below = anchor.bottom + PANEL_GAP
  const placeAbove = below + 240 > window.innerHeight && anchor.top > 240
  return {
    left,
    top: placeAbove ? undefined : below,
    bottom: placeAbove
      ? window.innerHeight - anchor.top + PANEL_GAP
      : undefined,
  }
}

/** One description panel for every item slot. */
export function ItemDetailLayer() {
  const current = useSyncExternalStore(
    subscribeItemDetail,
    getItemDetailPanel,
    () => null
  )
  const layerRef = useRef<HTMLDivElement>(null)
  const [frame, setFrame] = useState(0)
  const place =
    current && frame >= 0
      ? panelPosition(current.anchor.getBoundingClientRect())
      : null

  useEffect(() => {
    if (!current) return
    const update = () => {
      if (!current.anchor.isConnected) {
        hideItemDetail()
        return
      }
      setFrame((value) => value + 1)
    }
    window.addEventListener('scroll', update, true)
    window.addEventListener('resize', update)
    return () => {
      window.removeEventListener('scroll', update, true)
      window.removeEventListener('resize', update)
    }
  }, [current])

  useEffect(() => {
    if (!current) return
    const onKey = (event: KeyboardEvent) => {
      if (event.key === 'Escape') hideItemDetail()
    }
    const onPointerDown = (event: PointerEvent) => {
      const target = event.target
      if (!(target instanceof Node)) return
      if (current.anchor.contains(target)) return
      if (layerRef.current?.contains(target)) return
      if (
        target instanceof Element &&
        (target.closest('[data-slot="item-box"]') instanceof
          HTMLButtonElement ||
          target.closest('[data-slot="game-sprite"]') instanceof
            HTMLButtonElement)
      ) {
        return
      }
      hideItemDetail()
    }
    document.addEventListener('keydown', onKey)
    document.addEventListener('pointerdown', onPointerDown)
    return () => {
      document.removeEventListener('keydown', onKey)
      document.removeEventListener('pointerdown', onPointerDown)
    }
  }, [current])

  if (!current || !place) return null

  return (
    <div
      ref={layerRef}
      data-slot="item-detail"
      role="dialog"
      className="fixed z-50 max-h-80 overflow-y-auto rounded-lg border bg-popover p-3.5 text-sm text-popover-foreground shadow-window inset-shadow-raised"
      style={{
        width: PANEL_WIDTH,
        left: place.left,
        top: place.top,
        bottom: place.bottom,
      }}
      onPointerEnter={cancelHideItemDetail}
      onPointerLeave={() => scheduleHideItemDetail(current.anchor)}
    >
      <ItemDetail {...current.detail} />
    </div>
  )
}
