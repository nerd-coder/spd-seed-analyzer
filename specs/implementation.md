# SPD Seed Analyzer — Next Phase

Pinned target: SPD v3.3.8 @ `7b8b845a7`; accuracy remains `partial`.

The `AAA-AAA-AFU` retry-heavy replay now passes on Halls 21–24 plus the
floor-16 City shop trace that unblocked it.

## Close the SecretGardenRoom map gap

`secret_garden_prizes` matches the pinned RNG stream but throws the `Patch`
result away, so the room contributes no terrain and no plants to the map.

1. Paint the room the way `SecretGardenRoom.paint` does — wall frame, grass
   interior, high-grass patch — and record the four plant cells, applying
   `Level.plant`'s high-grass-to-grass conversion.
2. Pin it with a reference floor that actually contains the room, then extend
   the accuracy manifest if the map claim moves.

## Then broaden the LoopBuilder shop evidence

Only one pinned floor currently exercises `LoopBuilder`'s narrower shop
collision list. Capture a City or Prison shop trace for a second seed so the
path has more than a single regression case.
