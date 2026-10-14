# Design system

The SPD Seed Analyzer's design system: shadcn/ui components and Tailwind CSS
v4 tokens, with a React + Vite preview app that shows every rule in use.

- [`DESIGN.md`](DESIGN.md): the spec (principles, color, type, shape, motion,
  components, voice, adoption steps).
- [`src/theme/theme.css`](src/theme/theme.css): the tokens. Importable as
  `@spd-seed-analyzer/design/theme.css`.
- `src/components/ui/`: shadcn components (`radix-lyra`) restyled to the
  tokens.
- `src/components/spd/`: game pieces (item slot, floor banner, descent rail).
- `src/preview/`: the preview page sections. Sample data only; the preview
  never runs the analyzer engine.

## Run the preview

From the repo root:

```bash
bun install
bun run design        # http://localhost:5174
bun run build:design  # type-check and build to brand/design/dist
```

The theme switch in the top bar flips between Limestone (light) and Torchlit
(dark) and is remembered in `localStorage`.

## Assets

`public/sprites/` and `src/theme/fonts/pixel_font.ttf` are copies of
Shattered Pixel Dungeon assets (GPL-3.0 game, Apache-2.0 font) from
`web/public/assets/`. `public/textures/` holds 128×32 floor strips cut from
each region's `tiles_*.png` (floor tile plus its two decorated variants).
