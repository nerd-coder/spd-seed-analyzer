import { Store } from '@tanstack/store'

export class AppStore<T> extends Store<T> {
  set(value: T) {
    this.setState(() => value)
  }
}

export function persistentStore<T>(
  key: string,
  initialValue: T,
  codec: { encode: (value: T) => string; decode: (value: string) => T }
) {
  let value = initialValue
  try {
    const stored = localStorage.getItem(key)
    if (stored !== null) value = codec.decode(stored)
  } catch {
    // Storage can be unavailable in privacy modes and non-browser tests.
  }
  const store = new AppStore(value)
  store.subscribe((next) => {
    try {
      localStorage.setItem(key, codec.encode(next))
    } catch {
      // Keep the in-memory preference when persistence is unavailable.
    }
  })
  return store
}

export function derivedStore<Sources extends readonly AppStore<unknown>[], T>(
  sources: Sources,
  derive: () => T
) {
  const store = new AppStore(derive())
  for (const source of sources) {
    source.subscribe(() => store.set(derive()))
  }
  return store
}
