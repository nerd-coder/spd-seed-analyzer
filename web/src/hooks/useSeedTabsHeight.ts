import { useLayoutEffect } from 'react'

/**
 * Publish seed-tab bar height so region tabs stick beneath it.
 * Re-runs when `active` flips (sessions mount/unmount the bar).
 */
export function useSeedTabsHeight(
  ref: { current: HTMLElement | null },
  active: boolean
) {
  useLayoutEffect(() => {
    if (!active) {
      document.documentElement.style.removeProperty('--seed-tabs-height')
      return
    }
    const el = ref.current
    if (!el) return

    const publish = () => {
      document.documentElement.style.setProperty(
        '--seed-tabs-height',
        `${el.offsetHeight}px`
      )
    }
    publish()
    const ro = new ResizeObserver(publish)
    ro.observe(el)
    return () => {
      ro.disconnect()
      document.documentElement.style.removeProperty('--seed-tabs-height')
    }
  }, [ref, active])
}
