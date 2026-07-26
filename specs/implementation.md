# SPD Seed Analyzer — Next Phase

Pinned target: SPD v3.3.8 @ `7b8b845a7`; accuracy remains `partial`.

## Align depth-22 Halls painter order

`AAA-AAA-AAA` matches Java through the pre-paint RNG checkpoint and room
facts, but Rust's first painter callback is `RuinsRoom` rather than Java's
`StatueRoom`. The divergence is the builder's returned room-list order at the
`RegularPainter` list shuffle boundary.

1. Extend the Java Halls trace with the pre-shuffle room list (including stable
   room bounds/identity) and regenerate the fixture.
2. Port that exact FigureEight insertion order; keep the pre-paint checkpoint
   and full callback order/RNG as regression assertions.
3. Once AAA is exact, prove terrain, discoverability, transitions, and a
   contrasting depth-22 fixture.
