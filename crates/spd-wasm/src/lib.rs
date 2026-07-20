//! WASM bindings for `spd-core`.

use spd_core::{analyze_seed as core_analyze, parse_seed as core_parse, AnalyzeError, SeedError};
use wasm_bindgen::prelude::*;

fn seed_err(e: SeedError) -> JsValue {
    JsValue::from_str(&e.to_string())
}

fn analyze_err(e: AnalyzeError) -> JsValue {
    JsValue::from_str(&e.to_string())
}

/// Initialize panic hook for better console errors in the browser.
#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}

/// Parse a seed string into `{ input, numeric, code, formatted }`.
#[wasm_bindgen]
pub fn parse_seed(input: &str) -> Result<JsValue, JsValue> {
    let info = core_parse(input).map_err(seed_err)?;
    serde_wasm_bindgen::to_value(&info).map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Analyze a seed for `floors` depths. Returns a `SeedReport` object.
#[wasm_bindgen]
pub fn analyze_seed(input: &str, floors: u32) -> Result<JsValue, JsValue> {
    let report = core_analyze(input, floors).map_err(analyze_err)?;
    serde_wasm_bindgen::to_value(&report).map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Pinned SPD version string.
#[wasm_bindgen]
pub fn spd_version() -> String {
    spd_core::SPD_VERSION.to_string()
}

/// Pinned SPD commit short hash.
#[wasm_bindgen]
pub fn spd_commit() -> String {
    spd_core::SPD_COMMIT.to_string()
}
