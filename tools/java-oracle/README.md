# Java run oracle

This tool produces deterministic JSON directly from the pinned Shattered Pixel
Dungeon Java implementation. Schema v1 covers run-level potion, scroll, and
ring identity mappings. Schema v2 adds an intentionally narrow floor contract:
the ordered depth-one `itemsToSpawn` queue at the exact pre-`build()` boundary.
It does **not** claim room generation, final heap contents, or full seed-finder
parity.

## Requirements

- A local Shattered Pixel Dungeon checkout at commit
  `7b8b845a76fe76c6b7c031ae9e570852411f56db` (v3.3.8).
- `git`, `tar`, and JDK 17 or newer (the pinned checkout's Gradle wrapper
  requires Java 17, while the oracle bytecode targets Java 11).
- Network access on the first run if Gradle dependencies are not cached.

The runner validates the checkout's exact `HEAD`, exports the pinned commit to
a temporary directory, builds there, and removes the directory afterward. It
does not change the external checkout or use uncommitted source from it.

## Usage

From the analyzer repository root:

```bash
./tools/java-oracle/run AAA-AAA-AAA
./tools/java-oracle/run --depth 1 AAA-AAA-AAA
```

If an older Java is the machine default, select a JDK 17 installation for the
command:

```bash
JAVA_HOME=/path/to/jdk-17 PATH=/path/to/jdk-17/bin:$PATH \
  ./tools/java-oracle/run AAA-AAA-AAA
```

The usual SPD clone path is the default. Override it with either form:

```bash
SPD_SOURCE=/path/to/shattered-pixel-dungeon ./tools/java-oracle/run AAA-AAA-AAA
./tools/java-oracle/run --source /path/to/shattered-pixel-dungeon AAA-AAA-AAA
```

The committed fixtures are regenerated with these exact commands (stdout is
the default when `--output` is omitted):

```bash
./tools/java-oracle/run --output tools/java-oracle/fixtures/aaa-aaa-aaa.json AAA-AAA-AAA
./tools/java-oracle/run --output tools/java-oracle/fixtures/abc-def-ghi.json ABC-DEF-GHI
./tools/java-oracle/run --output tools/java-oracle/fixtures/gfx-pzh-dch.json GFX-PZH-DCH
./tools/java-oracle/run --output tools/java-oracle/fixtures/hello.json hello
./tools/java-oracle/run --depth 1 \
  --output tools/java-oracle/fixtures/aaa-aaa-aaa-floor-1.json AAA-AAA-AAA
```

The Rust golden consumer validates every `fixtures/*.json` file:

```bash
cargo test -p spd-core --test java_oracle_goldens
```

## JSON contracts

### Schema version 1: identities

```json
{
  "schema_version": 1,
  "spd": { "version": "v3.3.8", "commit": "7b8b845a7" },
  "input": { "seed": "AAA-AAA-AAA", "numeric": 0, "depths": [] },
  "identities": {
    "potions": [{ "item": "PotionOfStrength", "appearance": "..." }],
    "scrolls": [{ "item": "ScrollOfUpgrade", "appearance": "..." }],
    "rings": [{ "item": "RingOfAccuracy", "appearance": "..." }]
  }
}
```

Arrays preserve `Generator.Category` class order from SPD. `item` is the Java
simple class name and `appearance` is the exact internal color, rune, or gem
label serialized by SPD's own identity handlers, so neither field depends on
localization. `input.depths` is reserved and must remain empty while the oracle
is identity-only. Additive fields may be introduced without changing
`schema_version`; changing existing field meaning or ordering requires a new
version.

### Schema version 2: depth-one forced items

Passing `--depth 1` emits the same identities plus:

```json
{
  "schema_version": 2,
  "input": { "seed": "AAA-AAA-AAA", "numeric": 0, "depths": [1] },
  "floors": [{
    "depth": 1,
    "forced_items": [
      { "class": "Food", "quantity": 1, "level": 0, "cursed": false }
    ]
  }]
}
```

Only depth 1 is accepted. A recording `SewerLevel` snapshots the protected
queue when the pinned `Level.create()` first enters `build()`, then stops via an
internal sentinel before builder or painter RNG. This is the pre-room forced
spawn queue, not a post-paint observation. The injected runtime uses SPD's own
`gdxVersion` for the desktop native needed by icon-backed item constructors;
no asset files or graphics context are loaded.

The Rust golden compares this fixture with the surviving `source="forced"`
slice in `analyze_seed(..., 1)`. It deliberately does not compare every placed
floor item.
