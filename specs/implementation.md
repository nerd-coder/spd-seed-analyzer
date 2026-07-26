# SPD Seed Analyzer — Next Implementation Phase

Pinned target: SPD v3.3.8 @ `7b8b845a7`. Accuracy is `partial`; see
`specs/accuracy.json`. Public maps are painter-complete structure only.

## Next phase: isolate the depth-22 Halls main-RNG offset

1. Add a pinned Java oracle trace for `AAA-AAA-AAA` after each depth-22 room
   paint callback and after `paintDoors`.
2. Compare those non-advancing checkpoints with the Rust replay, then port only
   the first divergent callback or door lifecycle.
3. Prove exact post-paint main RNG, terrain, discoverability, and transitions;
   add one contrasting fixture only after the AAA replay is exact.
