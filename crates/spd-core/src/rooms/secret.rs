//! Port of `SecretRoom.initForRun` / `secretsForFloor` / `createRoom`.

use crate::random::Random;

use super::types::RoomSpec;

const ALL_SECRETS: &[&str] = &[
    "SecretGardenRoom",
    "SecretLaboratoryRoom",
    "SecretLibraryRoom",
    "SecretLarderRoom",
    "SecretWellRoom",
    "SecretRunestoneRoom",
    "SecretArtilleryRoom",
    "SecretChestChasmRoom",
    "SecretHoneypotRoom",
    "SecretHoardRoom",
    "SecretMazeRoom",
    "SecretSummoningRoom",
];

const BASE_REGION_SECRETS: [f32; 5] = [2.0, 2.25, 2.5, 2.75, 3.0];

/// Returns (`runSecrets` order, `regionSecretsThisRun`).
pub fn init_for_run() -> (Vec<&'static str>, [i32; 5]) {
    let mut region_secrets = [0i32; 5];
    for i in 0..5 {
        let chance = BASE_REGION_SECRETS[i];
        region_secrets[i] = chance as i32;
        if Random::float() < (chance % 1.0) {
            region_secrets[i] += 1;
        }
    }

    let mut run_secrets: Vec<&str> = ALL_SECRETS.to_vec();
    Random::shuffle(&mut run_secrets);
    (run_secrets, region_secrets)
}

/// `SecretRoom.secretsForFloor` — mutates region counters.
pub fn secrets_for_floor(depth: i32, region_secrets: &mut [i32; 5]) -> i32 {
    if depth == 1 {
        return 0;
    }
    let region = (depth / 5) as usize;
    let floor = depth % 5;
    let floors_left = 5 - floor;

    let secrets = if floors_left == 0 {
        region_secrets[region] as f32
    } else {
        let mut secrets = region_secrets[region] as f32 / floors_left as f32;
        if Random::float() < (secrets % 1.0) {
            secrets = secrets.ceil();
        } else {
            secrets = secrets.floor();
        }
        secrets
    };

    region_secrets[region] -= secrets as i32;
    secrets as i32
}

/// `SecretRoom.createRoom`.
pub fn create_room(run_secrets: &mut Vec<&'static str>) -> RoomSpec {
    let mut index = Random::chances(&[6., 3., 1.]);
    while index >= run_secrets.len() as i32 {
        index -= 1;
    }
    if index < 0 {
        index = 0;
    }
    let name = run_secrets.remove(index as usize);
    run_secrets.push(name);
    RoomSpec::secret(name)
}
