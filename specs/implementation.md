# SPD Seed Analyzer — Next Phase

Pinned target: SPD v3.3.8 @ `7b8b845a7`; accuracy remains `partial`.

## Expand replay coverage beyond two preserved runs

AAA-AAA-AAA and ABC-DEF-GHI now replay Java's public layout through depth 26,
including LastLevel decoration RNG, variance, transitions, and custom layers.

1. Add contrasting preserved runs for unproven Halls builder histories.
2. Capture their Java checkpoints before changing core generation.
3. Promote coverage only when every new replay preserves layout and
   seed-determined spawn evidence.
