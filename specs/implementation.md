# SPD Seed Analyzer — Handoff

**Pinned SPD:** v3.3.8 @ `7b8b845a7`

**Accuracy:** `partial`; `specs/accuracy.json` is authoritative.

## Next phase

Add the matching per-attempt trace to the pinned Java FigureEightBuilder oracle,
then compare it with Rust's test-only trace for AAA-AAA-AAA floor 21. Locate the
first differing attempt boundary or failure stage and make only the narrow,
source-backed call-order, placement, or collection-order fix. Require exact
room classes, normalized bounds, six TunnelRooms, and pre-paint RNG parity
before carrying the floor through painting and final map facts.

## Current checkpoint

- Rust now has non-advancing FigureEight attempt traces: attempt index, RNG
  probes, typed failure stage, insertion-order room bounds, and success.
- Trace regression coverage proves diagnostics do not alter the RNG stream.
- Static audits rejected retry persistence and pending-room ID remapping as the
  floor-21 cause; do not suppress or force retries without matched trace proof.
- Rust succeeds on attempt 1 with 17 rooms/7 tunnels; Java has 16 rooms/6
  tunnels. Overall accuracy remains `partial`.
