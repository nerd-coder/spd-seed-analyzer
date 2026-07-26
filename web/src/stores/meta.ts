/**
 * SPD build metadata (version / commit) loaded from wasm.
 */

import { getSpdMeta } from '@/lib/spd-wasm'

import { $formError } from './sessions'
import { AppStore } from './store-utils'

export type SpdMeta = { version: string; commit: string }

export const $meta = new AppStore<SpdMeta | null>(null)

export function loadSpdMeta() {
  getSpdMeta()
    .then((m) => $meta.set(m))
    .catch((e: unknown) => {
      $formError.set(e instanceof Error ? e.message : String(e))
    })
}
