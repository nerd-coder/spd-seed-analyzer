# SPD Seed Analyzer — Next Phase

Pinned target: SPD v3.3.8 @ `7b8b845a7`; accuracy remains `partial`.

## Complete the GFX Halls replay

`GFX-PZH-DCH` is not yet an exact Halls 21–24 replay. The depth-19 City
layout now matches through room painting, doors, and the pre-mob boundary, but
its later item-population stream leaves the persistent Generator category deck
wrong. That upstream state makes the first depth-21 mismatch appear in
`SecretSummoningRoom`; the room's local Java call order already matches.
Depths 22, 23, and 24 match their GFX Halls painter traces; depth 21 remains
blocked by the inherited deck state.

1. Identify and port the remaining depth-19 post-mob item-population lifecycle
   so the Generator category deck matches the pinned Java state.
2. With the restored deck, replay GFX depth 21 and match every callback and
   the post-door boundary without a local workaround.
3. Once all four GFX Halls floors pass naturally, retain focused oracle tests,
   update `accuracy.json`, run the complete CI check job, and commit the phase.
