/**
 * Color theme preference (light / dark / system), persisted.
 */

import { persistentAtom } from '@nanostores/persistent'

export type Theme = 'light' | 'dark' | 'system'

const THEME_KEY = 'spd-analyzer-theme'
const THEMES: Theme[] = ['light', 'dark', 'system']

const themeCodec = {
  encode: (v: Theme) => v,
  decode: (v: string): Theme =>
    v === 'light' || v === 'dark' || v === 'system' ? v : 'system',
}

export const $theme = persistentAtom<Theme>(THEME_KEY, 'system', themeCodec)

export function setTheme(value: Theme) {
  $theme.set(value)
}

/** Cycle light → dark → system → light. */
export function cycleTheme() {
  const current = $theme.get()
  const i = THEMES.indexOf(current)
  $theme.set(THEMES[(i + 1) % THEMES.length] ?? 'system')
}

export function resolvedTheme(theme: Theme): 'light' | 'dark' {
  if (theme === 'light' || theme === 'dark') return theme
  if (typeof window === 'undefined') return 'light'
  return window.matchMedia('(prefers-color-scheme: dark)').matches
    ? 'dark'
    : 'light'
}

/** Apply `.dark` on `<html>` for the given preference. */
export function applyTheme(theme: Theme) {
  const resolved = resolvedTheme(theme)
  const root = document.documentElement
  root.classList.toggle('dark', resolved === 'dark')
  root.style.colorScheme = resolved
}

/**
 * Apply current theme and keep it in sync with store + system preference.
 * Returns a cleanup function.
 */
export function initTheme(): () => void {
  applyTheme($theme.get())

  const unsub = $theme.subscribe((theme) => {
    applyTheme(theme)
  })

  const mql = window.matchMedia('(prefers-color-scheme: dark)')
  const onSystemChange = () => {
    if ($theme.get() === 'system') applyTheme('system')
  }
  mql.addEventListener('change', onSystemChange)

  return () => {
    unsub()
    mql.removeEventListener('change', onSystemChange)
  }
}
