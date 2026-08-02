# Implementation resume

Current state: the public report uses typed item conditions, name-less identity
entries, and an `{type, conditions}` enchantment object. Spawn conditions remain
OR-of-AND clauses and serialize without empty trinket clauses. Rust owns all
condition derivation; WASM and React consume the contract and render reusable
typed-condition popovers. `floors.items` contains exact, constrained, and
fresh/no-history baseline entries, distinguished by `prediction`; there is no
parallel baseline-items floor field. The analyzer keeps baseline items in their
ordinary item or quest group instead of rendering a separate baseline section;
Wandmaker and Imp cards show only their concrete fresh baseline rewards with a
visible route-independent reward contract, never a full-category candidate
expansion. The finder shows only matching evidence, and baseline items never
become finder evidence. Exact claims remain limited to seed-safe paths. Public
accuracy is `partial`.

Next steps:

1. Extend typed condition coverage as additional player-state routes are
   replayed (Rat Skull, Cracked Spyglass, Barren Land, and artifact events).
2. Add Java-oracle fixtures for newly covered stateful branches and verify the
   serialized contract when the pinned SPD version changes.
