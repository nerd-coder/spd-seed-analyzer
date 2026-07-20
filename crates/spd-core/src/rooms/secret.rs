//! Port of `SecretRoom.initForRun`.

use crate::random::Random;

/// `ALL_SECRETS` declaration order.
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
        region_secrets[i] = chance as i32; // truncate toward zero like (int)float
        if Random::float() < (chance % 1.0) {
            region_secrets[i] += 1;
        }
    }

    let mut run_secrets: Vec<&str> = ALL_SECRETS.to_vec();
    Random::shuffle(&mut run_secrets);
    (run_secrets, region_secrets)
}
