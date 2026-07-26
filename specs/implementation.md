# SPD Seed Analyzer — Next Implementation Phase

Pinned target: SPD v3.3.8 @ `7b8b845a7`. Accuracy is `partial`; see
`specs/accuracy.json`. Public maps are painter-complete structure only.

## Next phase: depth 22 Halls painter boundary

1. Use the AAA-AAA-AAA depth-22 Java fixture to isolate the two-draw desync
   after room painting and before `createMobs`.
2. Port only that first Halls room-painter/door lifecycle difference, then
   assert post-paint RNG, structural terrain, discoverability, and transitions.
3. Extend to one contrasting seed only after the first fixture is exact; update
   the manifest and run CI parity before commit.
