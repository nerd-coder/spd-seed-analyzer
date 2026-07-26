# SPD Seed Analyzer — Next Phase

Pinned target: SPD v3.3.8 @ `7b8b845a7`; accuracy remains `partial`.

## Fix the first depth-23 Halls branch placement mismatch

ABC-DEF-GHI matches Java through branch entry and 248 target branch-placement
calls. It first diverges immediately after `placeRoom` from tunnel
`[3,22,8,29]` at `198.03421°`.

1. Trace `placeRoom` at that boundary: free-space selection, room sizing,
   coordinate adjustment, and `connect`/neighbour effects.
2. Port the proven behavior and promote ABC depth-23 builder, painter, and
   post-door assertions only when all match.
3. Replay AAA and ABC through depth 24 before broadening public coverage.
