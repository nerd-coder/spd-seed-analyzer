use spd_core::analyze_seed;

fn main() {
    let seed = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "GFX-PZH-DCH".into());
    let floors: u32 = std::env::args()
        .nth(2)
        .and_then(|s| s.parse().ok())
        .unwrap_or(6);
    let r = analyze_seed(&seed, floors).expect("analyze");
    println!(
        "status={} msg={}",
        r.status,
        r.message.as_deref().unwrap_or("")
    );
    for f in r.floors {
        println!(
            "--- floor {} feeling={:?} builder={:?}",
            f.depth, f.feeling, f.builder
        );
        println!("  rooms: {}", f.rooms.join(", "));
        for it in f.items {
            println!(
                "  [{:<10}] {:<40} class={:?} source={:?}",
                it.category, it.name, it.class_name, it.source
            );
        }
    }
}
