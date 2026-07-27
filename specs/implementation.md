# SPD Seed Analyzer — Next Phase

Pinned target: SPD v3.3.8 @ `7b8b845a7`; accuracy remains `partial`.

## Finish the AFU retry-heavy Halls replay

Uncommitted `AAA-AAA-AFU` fixtures cover inner `RegularLevel` builder retries,
painter callbacks, post-door RNG, and pre-mob/pre-item boundaries on Halls
21–24. Depths 21, 22, and 24 pass. Depth 23 exposes an upstream persistent
artifact-deck mismatch: Rust consumes one extra artifact in the depth-16 shop;
Java's rare stock is a Stylus.

1. Trace the floor-16 ShopRoom RNG milestones in Java and Rust to locate the
   first source-backed divergent operation.
2. Fix the shop lifecycle, then make AFU depth 23 and its CrystalVault evidence
   pass without weakening the replay assertions.
3. Update `accuracy.json`, run CI parity, and commit the completed phase.
