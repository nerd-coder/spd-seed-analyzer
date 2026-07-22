use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;
use spd_core::items::identities::IdentityEntry;
use spd_core::{analyze_seed, init_run, parse_seed, SPD_COMMIT, SPD_VERSION};

const IDENTITY_SCHEMA_VERSION: u32 = 1;
const FLOOR_SCHEMA_VERSION: u32 = 2;
const FINAL_HEAPS_SCHEMA_VERSION: u32 = 3;

#[path = "java_oracle_goldens/final_heaps.rs"]
mod final_heaps;

#[derive(Debug, Deserialize)]
struct OracleFixture {
    schema_version: u32,
    #[serde(default)]
    contract: Option<String>,
    spd: SpdPin,
    input: OracleInput,
    identities: OracleIdentities,
    #[serde(default)]
    floors: Vec<OracleFloor>,
}

#[derive(Debug, Deserialize)]
struct SpdPin {
    version: String,
    commit: String,
}

#[derive(Debug, Deserialize)]
struct OracleInput {
    seed: String,
    numeric: i64,
    #[serde(default)]
    depths: Vec<u32>,
}

#[derive(Debug, Deserialize)]
struct OracleIdentities {
    potions: Vec<OracleIdentity>,
    scrolls: Vec<OracleIdentity>,
    rings: Vec<OracleIdentity>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
struct OracleIdentity {
    item: String,
    appearance: String,
}

#[derive(Debug, Deserialize)]
struct OracleFloor {
    depth: u32,
    #[serde(default)]
    forced_items: Vec<OracleItem>,
    #[serde(default)]
    final_heaps: Vec<OracleHeap>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
struct OracleItem {
    #[serde(rename = "class")]
    class_name: String,
    quantity: i32,
    level: i32,
    cursed: bool,
}

#[derive(Debug, Deserialize)]
struct OracleHeap {
    cell: u32,
    heap_type: String,
    items: Vec<OracleItem>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct ComparableItem {
    class_name: String,
    cursed: bool,
}

fn fixture_paths() -> Vec<PathBuf> {
    let fixture_dir =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tools/java-oracle/fixtures");
    let mut paths: Vec<_> = fs::read_dir(&fixture_dir)
        .unwrap_or_else(|error| {
            panic!(
                "failed to read Java oracle fixtures at {}: {error}",
                fixture_dir.display()
            )
        })
        .map(|entry| entry.expect("read fixture directory entry").path())
        .filter(|path| {
            path.extension()
                .is_some_and(|extension| extension == "json")
        })
        .collect();
    paths.sort();
    assert!(
        !paths.is_empty(),
        "no Java oracle JSON fixtures found at {}",
        fixture_dir.display()
    );
    paths
}

fn read_fixture(path: &Path) -> OracleFixture {
    let json = fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()));
    serde_json::from_str(&json)
        .unwrap_or_else(|error| panic!("failed to parse {}: {error}", path.display()))
}

fn identity_pairs(entries: &[IdentityEntry]) -> Vec<OracleIdentity> {
    entries
        .iter()
        .map(|entry| OracleIdentity {
            item: entry.item.clone(),
            appearance: entry.appearance.clone(),
        })
        .collect()
}

#[test]
fn run_identities_match_pinned_java_oracle() {
    for path in fixture_paths() {
        let fixture = read_fixture(&path);
        let context = path.display();

        assert!(
            matches!(
                fixture.schema_version,
                IDENTITY_SCHEMA_VERSION | FLOOR_SCHEMA_VERSION | FINAL_HEAPS_SCHEMA_VERSION
            ),
            "supported schema version in {context}"
        );
        assert_eq!(fixture.spd.version, SPD_VERSION, "SPD version in {context}");
        assert_eq!(fixture.spd.commit, SPD_COMMIT, "SPD commit in {context}");
        if fixture.schema_version == IDENTITY_SCHEMA_VERSION {
            assert!(
                fixture.contract.is_none()
                    && fixture.input.depths.is_empty()
                    && fixture.floors.is_empty(),
                "schema v1 identity fixture must not contain floors in {context}"
            );
        }

        let parsed = parse_seed(&fixture.input.seed)
            .unwrap_or_else(|error| panic!("invalid seed in {context}: {error}"));
        assert_eq!(
            parsed.numeric, fixture.input.numeric,
            "numeric seed in {context}"
        );

        let actual = init_run(fixture.input.numeric).identities;
        assert_eq!(
            identity_pairs(&actual.potions),
            fixture.identities.potions,
            "potion identities in {context}"
        );
        assert_eq!(
            identity_pairs(&actual.scrolls),
            fixture.identities.scrolls,
            "scroll identities in {context}"
        );
        assert_eq!(
            identity_pairs(&actual.rings),
            fixture.identities.rings,
            "ring identities in {context}"
        );
    }
}

#[test]
fn depth_one_forced_items_match_pinned_java_oracle() {
    let mut compared = 0;
    for path in fixture_paths() {
        let fixture = read_fixture(&path);
        if fixture.schema_version != FLOOR_SCHEMA_VERSION {
            continue;
        }
        let context = path.display();
        assert_eq!(fixture.input.depths, [1], "requested depths in {context}");
        assert_eq!(fixture.floors.len(), 1, "floor count in {context}");
        let expected_floor = &fixture.floors[0];
        assert_eq!(expected_floor.depth, 1, "floor depth in {context}");
        assert!(
            expected_floor
                .forced_items
                .iter()
                .all(|item| item.quantity == 1 && item.level == 0),
            "the first forced-item contract covers unit, level-zero items in {context}"
        );

        let expected: Vec<_> = expected_floor
            .forced_items
            .iter()
            .map(|item| ComparableItem {
                class_name: item.class_name.clone(),
                cursed: item.cursed,
            })
            .collect();
        let report = analyze_seed(&fixture.input.seed, 1)
            .unwrap_or_else(|error| panic!("failed to analyze seed in {context}: {error}"));
        let actual: Vec<_> = report.floors[0]
            .items
            .iter()
            .filter(|item| item.source.as_deref() == Some("forced"))
            .map(|item| ComparableItem {
                class_name: item
                    .class_name
                    .clone()
                    .expect("all analyzed items have a Java class name"),
                cursed: item.cursed,
            })
            .collect();
        assert_eq!(actual, expected, "ordered forced items in {context}");
        assert_eq!(report.status, "partial", "accuracy status in {context}");
        compared += 1;
    }
    assert!(compared > 0, "no schema v2 floor fixtures were compared");
}
