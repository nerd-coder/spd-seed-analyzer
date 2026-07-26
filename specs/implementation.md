# SPD Seed Analyzer — Next Phase

Pinned target: SPD v3.3.8 @ `7b8b845a7`; accuracy remains `partial`.

## Validate depth-22 Halls painter output

`AAA-AAA-AAA` now matches Java at the FigureEight pre-shuffle room-list
boundary and through every room-paint callback. The remaining focused boundary
is `RegularPainter.paintDoors`: Rust is two main-RNG draws ahead immediately
after it.

1. Add a Java oracle checkpoint around the Halls door merge and identify the
   exact connected-edge traversal or merge condition causing the two draws.
2. Port the behavior and assert the post-door RNG boundary, terrain,
   discoverability, and transitions for AAA.
3. Add a contrasting depth-22 fixture before broadening Halls coverage.
