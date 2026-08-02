use spd_core::analyze_seed;

fn main() {
    let seed = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "RZN-LKU-EFS".into());
    let floors = std::env::args()
        .nth(2)
        .and_then(|value| value.parse().ok())
        .unwrap_or(20);
    let report = analyze_seed(&seed, floors).expect("analyze seed");
    println!("{}", report.compact_text());
}
