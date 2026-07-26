# SPD Seed Analyzer — Next Phase

Pinned target: SPD v3.3.8 @ `7b8b845a7`; accuracy remains `partial`.

## Replay LastLevel parity at depth 26

AAA-AAA-AAA and ABC-DEF-GHI now match Java's HallsBoss visible layout at
depth 25: raw terrain, tile variance, transitions, pre-items RNG, and both
center-piece custom layers.

1. Capture both preserved runs at LastLevel depth 26.
2. Compare terrain, decorations, transitions, and pre-items RNG.
3. Continue remaining coverage gaps only after the complete floor path is
   replayed.
