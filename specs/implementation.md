# SPD Seed Analyzer — Next Phase

Pinned target: SPD v3.3.8 @ `7b8b845a7`; accuracy remains `partial`.

## Expand contrasting Halls replay coverage

AAA-AAA-AAA and ABC-DEF-GHI now replay Java's public layout through depth 26,
including LastLevel decoration RNG, variance, transitions, and custom layers.
GFX-PZH-DCH independently matches Halls depth 23's Loop builder and full
painter/RNG trace.

1. Replay GFX-PZH-DCH through Halls depths 21, 22, and 24.
2. Add a contrasting retry-heavy Halls history and capture Java checkpoints.
3. Promote coverage only when every replay preserves layout and seed-determined
   spawn evidence.
