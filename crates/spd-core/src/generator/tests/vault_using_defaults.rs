use serde::Deserialize;

const ORACLE_JSON: &str =
    include_str!("../../../../../tools/java-oracle/fixtures/vault/aaa-aaa-aaa-floor-17.json");

#[derive(Debug, Deserialize)]
struct Fixture {
    schema_version: u32,
    contract: String,
    spd: SpdPin,
    input: Input,
    depth: i32,
    branch: i32,
    dropped: Dropped,
}

#[derive(Debug, Deserialize)]
struct SpdPin {
    version: String,
    commit: String,
}

#[derive(Debug, Deserialize)]
struct Input {
    seed: String,
}

#[derive(Debug, Deserialize)]
struct Dropped {
    #[serde(rename = "RING")]
    ring: Counter,
    #[serde(rename = "WAND")]
    wand: Counter,
    #[serde(rename = "ARTIFACT")]
    artifact: Counter,
}

#[derive(Debug, Deserialize)]
struct Counter {
    before: i32,
    after: i32,
}

#[test]
fn vault_using_defaults_leaves_ring_dropped_flat() {
    let fixture: Fixture = serde_json::from_str(ORACLE_JSON).expect("vault usingDefaults fixture");
    assert_eq!(fixture.schema_version, 1);
    assert_eq!(fixture.contract, "vault_level_using_defaults");
    assert_eq!(fixture.spd.version, crate::SPD_VERSION);
    assert_eq!(fixture.spd.commit, crate::SPD_COMMIT);
    assert_eq!(fixture.input.seed, "AAA-AAA-AAA");
    assert_eq!(fixture.depth, 17);
    assert_eq!(fixture.branch, 1);

    assert_eq!(
        fixture.dropped.ring.before, fixture.dropped.ring.after,
        "VaultLevel must not advance RING.dropped"
    );
    assert_eq!(
        fixture.dropped.artifact.before, fixture.dropped.artifact.after,
        "VaultLevel does not draw ARTIFACT"
    );
    assert_eq!(
        fixture.dropped.wand.before, fixture.dropped.wand.after,
        "VaultLevel wand draws use defaults, not WAND.dropped"
    );
}
