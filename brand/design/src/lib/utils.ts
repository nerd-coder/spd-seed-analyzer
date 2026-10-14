import { type ClassValue, clsx } from 'clsx'
import { extendTailwindMerge } from 'tailwind-merge'

/*
 * Custom theme scales tailwind-merge cannot infer. Unknown `text-*` and
 * `inset-shadow-*` values read as colors, so without these groups
 * `cn('text-px-1', 'text-tier-3')` drops the size and
 * `cn('inset-shadow-recessed', 'inset-shadow-none')` keeps both.
 */
const twMerge = extendTailwindMerge({
  extend: {
    classGroups: {
      'font-size': [{ text: ['px-1', 'px-2', 'px-3', 'px-4'] }],
      'inset-shadow': [{ 'inset-shadow': ['raised', 'pressed', 'recessed'] }],
      shadow: [{ shadow: ['window'] }],
    },
  },
})

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}
