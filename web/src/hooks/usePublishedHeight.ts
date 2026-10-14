import { useLayoutEffect } from 'react'

/**
 * Publish an element's height as a CSS custom property on <html>, so sticky
 * layers below it can offset themselves. ResizeObserver keeps it current when
 * the element wraps; the property is removed while `active` is false.
 */
export function usePublishedHeight(
  ref: { current: HTMLElement | null },
  property: `--${string}`,
  active: boolean
) {
  useLayoutEffect(() => {
    const root = document.documentElement
    if (!active) {
      root.style.removeProperty(property)
      return
    }
    const el = ref.current
    if (!el) return

    const publish = () => {
      root.style.setProperty(property, `${el.offsetHeight}px`)
    }
    publish()
    const ro = new ResizeObserver(publish)
    ro.observe(el)
    return () => {
      ro.disconnect()
      root.style.removeProperty(property)
    }
  }, [ref, property, active])
}
