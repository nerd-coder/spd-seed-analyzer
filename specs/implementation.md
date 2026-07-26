# SPD Seed Analyzer — Next Phase

Pinned target: SPD v3.3.8 @ `7b8b845a7`; accuracy remains `partial`.

## Capture exact HallsBoss visual state

Depth-25 normalized terrain, discoverability, transitions, and pre-items RNG
match Java for AAA-AAA-AAA and ABC-DEF-GHI. Exact raw decorations, tile
variance, and center-piece visual/wall layers are not yet asserted.

1. Extend the Java pre-items oracle with raw terrain, tile variance, and custom
   visual/wall facts for both preserved runs.
2. Model and assert those layers in `spd-core`, including accepted-attempt RNG
   boundaries where needed.
3. Replay the LastLevel at depth 26 after HallsBoss is exact.
