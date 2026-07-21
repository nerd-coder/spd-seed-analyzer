use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;
use spd_core::items::identities::IdentityEntry;
use spd_core::{init_run, parse_seed, SPD_COMMIT, SPD_VERSION};

const ORACLE_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Deserialize)]
struct OracleFixture {
    schema_version: u32,
    spd: SpdPin,
    input: OracleInput,
    identities: OracleIdentities,
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

        assert_eq!(
            fixture.schema_version, ORACLE_SCHEMA_VERSION,
            "schema version in {context}"
        );
        assert_eq!(fixture.spd.version, SPD_VERSION, "SPD version in {context}");
        assert_eq!(fixture.spd.commit, SPD_COMMIT, "SPD commit in {context}");
        assert!(
            fixture.input.depths.is_empty(),
            "schema v1 identity fixture must not request floor depths in {context}"
        );

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
