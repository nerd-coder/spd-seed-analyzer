# Playwright visual regression

This package performs strict pixel comparisons of the analyzer's deterministic
map canvas backing bitmap in Chromium. Capturing the backing bitmap avoids
viewport clipping when a full-resolution map is taller than its dialog. The
regular-floor cases come from `tools/visual/tests/map-render-fixtures.ts`;
browser-rendered baselines are committed in `tools/visual/snapshots/`.

The snapshots cover only the public structural-layout projection: room
geometry, walls, traversable floor, doors, and entrances/exits. Entity-rich
gameplay captures, quest branches, decoration, traps, plants, blobs, heaps,
mobs, item placement, explored FOV, and animation state are product non-goals
and are not visual-regression inputs.

## First-time setup

From the repository root:

```bash
bun install
bun run install:visual-browser
```

## Run comparisons

```bash
bun run test:map-render
bun run test:visual
```

`test:map-render` validates the structural snapshot registry. `test:visual`
builds the WASM and production web app, starts the Vite preview on
`127.0.0.1:4173`, and compares every registered map against its baseline. When
`web/dist` is already current, use `bun run test:visual:only`.

Failures write the actual image, expected image, pixel diff, and Playwright
trace to `tools/visual/test-results/` (gitignored).

## Intentionally update baselines

After reviewing a deliberate renderer change:

```bash
bun run test:visual:update
git diff -- tools/visual/snapshots
```

Snapshot updates should be reviewed as binary artifacts. Do not use them to
hide unexpected differences, and do not promote these cases to a full
seed-finder or gameplay-state parity claim while the engine status is
`partial`.
