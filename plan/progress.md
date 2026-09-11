# v4 close-out progress

Index: [README.md](README.md). Facts: `specs/analysis/`. Decks: `specs/generator-decks.md` (header/§4/§11 retargeted; Imp/vault sites still 01 remaining).

## Done

- **00 closed.** Pin, Daily clamp, java-oracle compile + `custom_terrain`, fixtures. Ref [00-pin-and-tooling.md](00-pin-and-tooling.md). `1ae8dd7`, `acc9fb0`, `073d423`.
- 01 PR 1+2 — `random(ARTIFACT)` miss uses ring defaults; `RING.dropped` unchanged; WEP_T3 probs pinned. Ref [01-generator-decks.md](01-generator-decks.md). `72b4a81`.
- 01 PR 3 — v4 weapon enchant/curse tables. `aebfd8d`.

Generation otherwise still v3; goldens red except pin/identity.

## Now

01 PR 4 — Imp spawn + vault `usingDefaults` deck oracles. Ref [01-generator-decks.md](01-generator-decks.md) PR 4. Do not port vault/Imp quest (02).

## Next

01 PR 5 (decks.md Imp measurements) → 02.

## Remaining

01 PR 4–5, then 02–06. 07 deny-list. Do not claim v4 analyzer completeness.
