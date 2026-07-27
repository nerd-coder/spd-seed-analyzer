# SPD Seed Analyzer — Next Phase

Pinned target: SPD v3.3.8 @ `7b8b845a7`; accuracy remains `partial`.

## Expand contrasting Halls replay coverage

GFX-PZH-DCH now replays Java Halls depths 21–24 through every room-paint
callback and post-door boundary. AAA Caves visual replay also matches through
depth 14. Coverage remains fixture-specific.

1. Capture a retry-heavy regular Halls history with Java builder, painter, and
   population checkpoints.
2. Replay it through depths 21–24 and retain only source-backed parity fixes.
3. Promote coverage only after the new history and all CI checks pass.
