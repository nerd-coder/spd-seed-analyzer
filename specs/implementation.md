# SPD Seed Analyzer — Next Phase

Pinned target: SPD v3.3.8 @ `7b8b845a7`; accuracy remains `partial`.

## Replay Halls parity through depth 24

AAA-AAA-AAA and ABC-DEF-GHI now both match Java at depth 23 through the
FigureEight builder, pre-shuffle rooms, painter callbacks, and post-door RNG.

1. Capture and compare both preserved runs through depth 24.
2. Fix any newly exposed boundary before promoting coverage.
3. Continue phase-by-phase replay of later regular floors.
