# SPD Seed Analyzer — Next Phase

Pinned target: SPD v3.3.8 @ `7b8b845a7`; accuracy remains `partial`.

## Fix the first depth-23 Halls branch placement mismatch

ABC-DEF-GHI places the same SentryRoom bounds from tunnel `[3,22,8,29]` at
`198.03421°`, but Java is two RNG draws ahead after that call.

1. Trace collision-order-sensitive `findFreeSpace` tie breaks at that call and
   match their RNG consumption.
2. Port the proven behavior and promote ABC depth-23 builder, painter, and
   post-door assertions only when all match.
3. Replay AAA and ABC through depth 24 before broadening public coverage.
