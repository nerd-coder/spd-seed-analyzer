import { usePublishedHeight } from '@/hooks/usePublishedHeight'

/**
 * Publish seed-tab bar height so floor banners stick beneath it. Re-runs when
 * `active` flips (sessions mount/unmount the bar).
 */
export function useSeedTabsHeight(
  ref: { current: HTMLElement | null },
  active: boolean
) {
  usePublishedHeight(ref, '--seed-tabs-height', active)
}
