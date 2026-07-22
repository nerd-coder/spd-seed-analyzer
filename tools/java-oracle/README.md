# Java run oracle

This tool produces deterministic JSON directly from the pinned Shattered Pixel
Dungeon Java implementation. Schema v1 covers run-level potion, scroll, and
ring identity mappings. Schema v2 adds an intentionally narrow floor contract:
the ordered depth-one `itemsToSpawn` queue at the exact pre-`build()` boundary.
Schema v3 is a separately scoped level contract that snapshots final heaps and
mobs after the real `Level.create()` lifecycle completes. Sewer depth 1 and
Prison depths 6 and 8 are supported. Depth 1 is generated directly; deeper
targets are generated after completing every prior floor so run-persistent state is
preserved. Additive visual fields capture final terrain, discoverability, tile
variance, transitions, traps, plants, and active blobs. It does **not** claim
full seed-finder parity.

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
./tools/java-oracle/run --final-heaps-depth 1 AAA-AAA-AAA
./tools/java-oracle/run --final-heaps-depth 6 HKT-JZN-XQQ
./tools/java-oracle/run --final-heaps-depth 8 HKT-JZN-XQQ
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
./tools/java-oracle/run --final-heaps-depth 1 \
  --output tools/java-oracle/fixtures/aaa-aaa-aaa-final-heaps-floor-1.json AAA-AAA-AAA
./tools/java-oracle/run --final-heaps-depth 1 \
  --output tools/java-oracle/fixtures/abc-def-ghi-final-heaps-floor-1.json ABC-DEF-GHI
./tools/java-oracle/run --final-heaps-depth 1 \
  --output tools/java-oracle/fixtures/gfx-pzh-dch-final-heaps-floor-1.json GFX-PZH-DCH
./tools/java-oracle/run --final-heaps-depth 1 \
  --output tools/java-oracle/fixtures/hello-final-heaps-floor-1.json hello
./tools/java-oracle/run --final-heaps-depth 1 \
  --output tools/java-oracle/fixtures/hkt-jzn-xqq-final-heaps-floor-1.json HKT-JZN-XQQ
./tools/java-oracle/run --final-heaps-depth 6 \
  --output tools/java-oracle/fixtures/hkt-jzn-xqq-final-heaps-floor-6.json HKT-JZN-XQQ
./tools/java-oracle/run --final-heaps-depth 8 \
  --output tools/java-oracle/fixtures/hkt-jzn-xqq-final-heaps-floor-8.json HKT-JZN-XQQ
```

The Rust golden consumer validates every `fixtures/*.json` file. The schema-v3
test requires all five committed depth-one fixtures to match lifecycle probes,
map bounds, heap cells, mob facts, and the report-visible item projection. The
HKT floor-one fixture also requires exact terrain, discoverability, tile
variance, transitions, traps, structured heaps/mobs, plants, and active blobs.
The floor-six and floor-eight fixtures pin full Java observations while Rust
asserts only the strongest exact subset implemented for each floor:

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

### Schema version 3: final placed heaps

Passing `--final-heaps-depth 1` runs the exact pinned initialization followed
by `Dungeon.newLevel()` for a depth-one `SewerLevel`. Passing depth 6 or 8
first runs `Dungeon.newLevel()` for every preceding floor, then creates the
target `PrisonLevel`. The oracle snapshots
`level.heaps` and `level.mobs` only after `Level.create()` returns (build,
painter, mob pass, and `createItems` have all run). The oracle uses a fresh
Warrior hero with no challenges and a fresh in-memory preference store. The
intro page is marked read and the intro prompt disabled because SPD
intentionally places its early Guidebook with an unseeded generator; that
meta/tutorial heap is outside this seed-deterministic contract. The oracle also
gates the pinned `Bones.get()` call as a daily run solely to prevent a machine's
external `bones.dat` from entering a committed seed fixture; no other generation
path consults that flag at this pin.

The output has `schema_version: 3` and `contract: "final_placed_heaps"`:

```json
{
  "floors": [{
    "depth": 1,
    "width": 40,
    "height": 30,
    "rooms": ["EntranceRoom", "ExitRoom", "PoolRoom"],
    "pre_paint_rng": [1993374861, -149591753],
    "pre_mobs_rng": [1726373121, -188171336],
    "pre_items_rng": [-339886649, -1704611306],
    "terrain": [4, 4, 1],
    "discoverable": [false, true, true],
    "tile_variance": [12, 68, 97],
    "transitions": [],
    "traps": [],
    "plants": [],
    "blobs": [],
    "final_heaps": [{
      "cell": 315,
      "heap_type": "chest",
      "items": [
        { "class": "ScaleArmor", "quantity": 1, "level": 0, "cursed": false }
      ]
    }],
    "final_mobs": [
      { "cell": 234, "class": "Rat" }
    ]
  }]
}
```

`rooms` is the sorted list of Java room simple-class names retained by the
builder, including connection rooms, and records the room-family coverage of
each fixture. `terrain`, `discoverable`, and `tile_variance` are row-major
arrays parallel to the floor bounds. Transitions, traps, plants, and non-empty
blob concentrations retain stable render-facing Java facts and are sorted for
deterministic comparison. Heaps are ordered by ascending row-major `cell`;
each `items` array keeps the Java `Heap.items` stack order. `heap_type` is the
lower-case stable form of SPD's `Heap.Type`. Item class, quantity, true level,
and curse state are kept without localization. Gold, keys, and every other
heap item generated within
the stated deterministic scope remain in this contract; nothing is filtered
for UI convenience. `final_mobs` is likewise cell-sorted and uses Java simple
class names, covering both room-painted mobs and the ambient `createMobs`
pass. The three eight-value RNG probes snapshot consecutive full-range
`Random.Int()` results from separate fresh runs before `RegularPainter.paint`
and at the `createMobs` and `createItems` entry boundaries; recording stops at
each boundary, so the probes do not perturb the final heap/mob run. They make
raw LCG draw-count comparison possible even while an earlier phase is
desynchronized. This is an exact-pin observation contract. The five committed
depth-one fixtures currently match their strongest honest Rust projection;
only HKT floor 1 opts into the additive render-fact assertions. HKT floors 6
and 8 are exact Java observations but not yet full Rust lifecycle matches.
These fixtures are not evidence that every room set, deeper floor, or full
heap fact matches.
