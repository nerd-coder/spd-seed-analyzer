# SPD Seed Analyzer — Next Phase

Pinned target: SPD v3.3.8 @ `7b8b845a7`; accuracy remains `partial`.

## Fix the contrasting depth-23 Halls builder

AAA-AAA-AAA depth 23 has exact builder and painter parity. Contrasting
ABC-DEF-GHI matches Java at FigureEight entry but diverges at the first
builder-exit RNG boundary and its 26 pre-shuffle room bounds.

1. Trace that FigureEight attempt through room placement, connection creation,
   and retry checks to locate the first mismatched RNG consumer or geometry rule.
2. Port the proven behavior and promote the ABC depth-23 builder, painter, and
   post-door fixture comparisons to exact parity.
3. Then replay both histories to depth 24 before broadening public coverage.
