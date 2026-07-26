# SPD Seed Analyzer — Next Implementation Phase

Pinned target: SPD v3.3.8 @ `7b8b845a7`. Accuracy is `partial`; see
`specs/accuracy.json`. Public maps are painter-complete structure only.

## Next phase: trace the depth-22 Halls painter boundary

1. Add a pinned Java oracle trace after each depth-22 room painter and door
   pass for `AAA-AAA-AAA`; Rust is currently two main-stream draws ahead at
   `createMobs`.
2. Port only the first divergent Halls room-painter/door lifecycle and prove
   post-paint RNG, terrain, discoverability, and transitions for that fixture.
3. Add one contrasting seed only after the first fixture is exact; update the
   accuracy manifest and run CI parity before committing behavior changes.
