# SPD Seed Analyzer — Next Implementation Phase

Pinned target: SPD v3.3.8 @ `7b8b845a7`. Accuracy remains `partial`; see
`specs/accuracy.json`. Public maps contain painter-complete deterministic
structure only, never population or item placement.

Dedicated boss layouts at depths 5, 10, 15, 20, 25, and 26 are
fixture-backed. Regular-floor coverage remains fixture-specific.

## Next phase: depth 22 `HallsLevel` structural parity

1. Capture a pinned Java-oracle fixture for depth 22 through the full
   preceding-floor lifecycle, including terrain, discoverability, transitions,
   and RNG checkpoints.
2. Compare its builder, room selection, Halls painters, doors, and structural
   map projection with `spd-core`; port only the first divergence while
   preserving Java RNG call order.
3. Add focused Rust coverage, update `specs/accuracy.json`, rebuild WASM, and
   pass CI parity before committing.
