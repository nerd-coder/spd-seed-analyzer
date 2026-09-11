# v4 close-out progress

Index: [README.md](README.md). Facts: `specs/analysis/`. Decks: `specs/generator-decks.md` (stale until 01).

## Done

- 00 PR 1 — pin bump to v4.0.0 @ `2bb34a4e9`, Daily `2026-04-01` / epoch `20544`. Ref [00-pin-and-tooling.md](00-pin-and-tooling.md) PR 1. `1ae8dd7`.
- 00 PR 2 — java-oracle compiles on v4; captures `custom_terrain`. Ref [00-pin-and-tooling.md](00-pin-and-tooling.md) PR 2. `acc9fb0`. Generation still v3.3.8.

## Now

00 PR 3 — regenerate v4 java-oracle fixtures. Ref [00-pin-and-tooling.md](00-pin-and-tooling.md) PR 3 and `tools/java-oracle/README.md` command list. Leave Rust goldens red.

## Next

00 PR 4 (oracle README pin) → 01.

## Remaining

00 PR 3–4, then 01–06. 07 is a deny-list. Do not claim v4 analyzer completeness.
