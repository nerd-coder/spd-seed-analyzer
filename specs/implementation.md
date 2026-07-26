# SPD Seed Analyzer — Next Phase

Pinned target: SPD v3.3.8 @ `7b8b845a7`; accuracy remains `partial`.

## Locate the depth-23 Halls build divergence

AAA-AAA-AAA matches Java immediately before depth-23 `RegularLevel.build`.
The divergence begins during build, before room painting; depth-22 population
is not the cause. The existing depth-23 trace remains diagnostic only.

1. Add Java/Rust checkpoints inside the Halls build path: room counts, special
   room selection, builder choice, and every retry boundary through `initRooms`.
2. Port the first proven mismatch and promote the depth-23 trace to exact
   pre-shuffle, callback, and post-door assertions only when it passes.
3. Then add a contrasting regular-Halls preserved run before expanding public
   Halls layout coverage.
