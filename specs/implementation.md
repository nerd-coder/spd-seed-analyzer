# SPD Seed Analyzer — Next Phase

Pinned target: SPD v3.3.8 @ `7b8b845a7`; accuracy remains `partial`.

## Locate the first depth-23 Halls branch RNG mismatch

ABC-DEF-GHI still reaches the same SentryRoom bounds from tunnel `[3,22,8,29]`
at `198.03421°`, but Java is two RNG draws ahead afterward. The target
`findFreeSpace` call has no equal-axis draw; the first 19 Java/Rust tie events
match exactly.

1. Trace all RNG-consuming operations around the preceding branch placement
   (angle selection, room sizing, and connection retries) to locate the two
   missing Java draws.
2. Port only the proven behavior, then promote ABC depth-23 builder, painter,
   and post-door assertions when they all match.
3. Replay AAA and ABC through depth 24 before broadening public coverage.
