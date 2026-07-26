# SPD Seed Analyzer — Next Phase

Pinned target: SPD v3.3.8 @ `7b8b845a7`; accuracy remains `partial`.

## Broaden regular Halls painter coverage

Depth 22 now has Java-backed `paintDoors` parity for AAA-AAA-AAA and the
contrasting ABC-DEF-GHI replay. The pinned `RuinsRoom.canMerge` override keeps
solid patch edges open, matching the post-door RNG boundary and affected door
terrain/discoverability; AAA transitions remain pinned.

1. Record a fixture-first depth-23 or depth-24 Halls painter trace, replaying
   all prior floors to preserve run state.
2. Compare pre-paint, room callbacks, post-door RNG, and affected structural
   cells before porting any newly exposed painter behavior.
3. Keep public Halls layout coverage partial until multiple regular-floor
   histories are verified.
