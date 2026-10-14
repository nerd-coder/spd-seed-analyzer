import { type ClassValue, clsx } from 'clsx'
import { extendTailwindMerge } from 'tailwind-merge'

/*
 * The pixel type scale is a custom `text-*` size. Without this, tailwind-merge
 * reads `text-px-1` as a color and drops it next to `text-tier-3`.
 */
const twMerge = extendTailwindMerge({
  extend: {
    classGroups: {
      'font-size': [{ text: ['px-1', 'px-2', 'px-3', 'px-4'] }],
    },
  },
})

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}
