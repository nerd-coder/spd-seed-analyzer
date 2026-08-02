# Implementation status

Floor 1 now promotes painter-generated room rewards to exact seed facts when the
layout stream is stable. Its report also includes grouped, non-positional initial
encounters with fixed drops, runtime-chance drops, and carried rewards generated
with the floor. Public maps remain painter-only and the analyzer remains
`partial`.

Next steps:

1. Extend encounter summaries beyond Floor 1 only after prior-floor player and
   runtime state can be represented without treating baseline continuations as
   exact.
2. Add more combat-reward rules only with pinned v3.3.8 source evidence, keeping
   runtime RNG outcomes conditional rather than predicting an identity.
