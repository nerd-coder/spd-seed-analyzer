//! Writes the generated TypeScript ids: `cargo run -p spd-objects --bin gen-ts`.

use std::path::Path;

fn main() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../web/src/generated/game-ids.ts");
    std::fs::create_dir_all(path.parent().expect("generated dir")).expect("create generated dir");
    std::fs::write(&path, spd_objects::typescript::render()).expect("write game-ids.ts");
    println!("wrote {}", path.display());
}
