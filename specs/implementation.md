# SPD Seed Analyzer — Next Phase

Pinned target: SPD v3.3.8 @ `7b8b845a7`; accuracy remains `partial`.

## Align depth-22 Halls painter order

The pinned `AAA-AAA-AAA` trace now shows that Rust starts with `RuinsRoom`,
while Java starts with `StatueRoom`; the offset is before any room callback.

1. Port the exact Java room-list order at the `RegularPainter` shuffle boundary.
2. Use the checkpoints to port only the first divergent callback or doors tail.
3. Once AAA is exact, prove main RNG, terrain, discoverability, and transitions,
   then add one contrasting depth-22 fixture.
