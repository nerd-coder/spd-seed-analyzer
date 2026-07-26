# SPD Seed Analyzer — Next Phase

Pinned target: SPD v3.3.8 @ `7b8b845a7`; accuracy remains `partial`.

## Fix depth-23 Halls branch placement

ABC-DEF-GHI now matches Java through both FigureEight loops, including the
RuinsExitRoom override. Divergence begins in `RegularBuilder.createBranches`.

1. Capture Java/Rust checkpoints for each branch source, retry, angle, and
   connection-room placement until the first mismatch is isolated.
2. Port that proven branch behavior and promote the ABC depth-23 builder,
   painter, and post-door assertions only when all match.
3. Replay AAA and ABC through depth 24 before broadening public coverage.
