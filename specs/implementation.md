# SPD Seed Analyzer — Next Phase

Pinned target: SPD v3.3.8 @ `7b8b845a7`; accuracy remains `partial`.

## Restore Halls preserved-run continuity

The committed AAA-AAA-AAA depth-23 Java painter trace shows that Rust diverges
only after it must carry depth-22 Halls population into the next floor. Its
21 room callbacks and post-door RNG boundary are preserved as the fixture-first
target; no depth-23 layout parity is claimed.

1. Trace the pinned depth-22 Halls `create` population path after `paintDoors`,
   including forced Torch placement and every main-RNG consumer.
2. Port the missing state/RNG lifecycle in `spd-core`, then turn the depth-23
   diagnostic sentinel into exact pre-shuffle, callback, and post-door checks.
3. Record a contrasting regular-Halls run only after AAA depth-23 parity holds;
   keep public Halls layout coverage partial until both histories match.
