# SPD Seed Analyzer — Handoff

**Updated:** 2026-07-25

**Pinned SPD:** v3.3.8 @ `7b8b845a7`

**Status:** `partial` — `specs/accuracy.json` is the coverage source of truth.

## Next phase

Close the AAA-AAA-AAA floor-12 FigureEightBuilder layout gap. Instrument the
first build attempt on both sides after the aligned `initRooms` boundary, then
compare in order: shuffled room order, main-loop split/tunnel classes, each
`placeRoom` result, closing stitches, and branch retries. Fix only the first
pinned-source semantic difference; then promote floor 12 into the exact replay
and continue with pre-paint, pre-mobs, and pre-items boundaries.

## Current checkpoint

- Floor-12 Java fixture captured. Non-connection room selection matches.
- Java and Rust both choose FigureEightBuilder with intensity `0.5489903`; the
  active RNG also matches at the end of `initRooms`.
- First known gap is builder output: Rust has 22 rooms, Java 19. Painter and
  lifecycle parity are not claimed for floor 12.
- Floor 11 remains exact at its documented deterministic boundaries.

Run the complete CI `check` sequence from `AGENTS.md` before committing.
