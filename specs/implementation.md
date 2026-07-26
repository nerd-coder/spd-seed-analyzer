# SPD Seed Analyzer — Next Phase

Pinned target: SPD v3.3.8 @ `7b8b845a7`; accuracy remains `partial`.

## Broaden Halls preserved-run parity

AAA-AAA-AAA depth 23 now matches Java through FigureEight build and room paint.
Coverage remains fixture-specific and overall accuracy remains `partial`.

1. Capture a contrasting regular-Halls depth-23 Java trace, including builder
   attempts, pre-shuffle bounds, painter callbacks, and post-door RNG.
2. Compare it against Rust, port the first proven mismatch if any, and retain
   both fixtures as exact parity evidence.
3. Continue the same fixture-first replay to depth 24, then audit remaining
   regular-floor gaps before expanding public coverage.
