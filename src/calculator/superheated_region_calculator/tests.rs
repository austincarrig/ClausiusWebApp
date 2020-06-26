use super::SuperheatedRegionCalculator;
use super::Constants;

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[test]
#[wasm_bindgen_test]
fn clamp_high()
{
    let max_val = Constants::TEMPERATURE_T_S[Constants::TEMPERATURE_T_S.len() - 1];
    let clamped_val = SuperheatedRegionCalculator::clamp_temperature(max_val + 1.0);
    assert_eq!(clamped_val, max_val)
}

#[test]
#[wasm_bindgen_test]
fn clamp_low()
{
    let min_val = Constants::TEMPERATURE_T_S[0];
    let clamped_val = SuperheatedRegionCalculator::clamp_temperature(min_val - 1.0);
    assert_eq!(clamped_val, min_val)
}

#[test]
#[wasm_bindgen_test]
fn clamp_middle()
{
    let mid_val = Constants::TEMPERATURE_T_S[Constants::TEMPERATURE_T_S.len() / 2];
    let clamped_val = SuperheatedRegionCalculator::clamp_temperature(mid_val);
    assert_eq!(clamped_val, mid_val)
}