/** Preview state: theme and the floor picked on the descent rail. */
import { Store } from '@tanstack/store'

export type ThemeMode = 'light' | 'dark'

const THEME_KEY = 'spd-design-theme'

function readTheme(): ThemeMode {
  try {
    return localStorage.getItem(THEME_KEY) === 'light' ? 'light' : 'dark'
  } catch {
    return 'dark'
  }
}

export const $theme = new Store<ThemeMode>(readTheme())
export const $floor = new Store<number>(4)

/** Toggle the class first so a re-render reads the new token values. */
export function setTheme(mode: ThemeMode) {
  const root = document.documentElement
  root.classList.toggle('dark', mode === 'dark')
  root.style.colorScheme = mode
  $theme.setState(() => mode)
  try {
    localStorage.setItem(THEME_KEY, mode)
  } catch {
    /* storage can be unavailable; the choice still applies */
  }
}

export function selectFloor(depth: number) {
  $floor.setState(() => depth)
}
