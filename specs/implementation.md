# SPD Seed Analyzer — Next Implementation Phase

Pinned target: SPD v3.3.8 @ `7b8b845a7`. Accuracy remains `partial`; see
`specs/accuracy.json`. Public maps contain painter-complete deterministic
structure only, never population or item placement.

Depth 5 `SewerBossLevel`, depth 15 `CavesBossLevel`, and fixed boss layouts
at depths 10, 20, and 26 are fixture-backed. Regular-floor coverage remains
fixture-specific.

## Next phase: depth 25 `HallsBossLevel`

1. Port its pinned Java builder, terrain, transitions, and retry/RNG lifecycle
   into `spd-core`; preserve Java call order.
2. Capture multi-seed Java-oracle fixtures and compare normalized structural
   terrain, discoverability, transitions, and the post-build RNG boundary.
3. Add focused Rust tests, update `specs/accuracy.json`, rebuild WASM, and run
   CI parity before committing.
