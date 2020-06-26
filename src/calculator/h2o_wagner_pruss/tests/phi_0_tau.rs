use super::*;
use super::super::H2OWagnerPruss;

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

//const TOP_LEFT_TS_DIAGRAM_PHI_R_TAU: f32 = ;
//const TOP_RIGHT_TS_DIAGRAM_PHI_R_TAU: f32 = ;
//const BOTTOM_RIGHT_TS_DIAGRAM_PHI_R_TAU: f32 = ;
const SUPERHEATED_POINT_1_PHI_0_TAU: f32 = 10.887587170062671;

#[test]
#[wasm_bindgen_test]
fn superheated_point_1()
{
    test_phi_0_tau(SUPERHEATED_POINT_1_T,
                   SUPERHEATED_POINT_1_D,
                   SUPERHEATED_POINT_1_PHI_0_TAU);
}

fn test_phi_0_tau(temperature: f32,
                  density: f32,
                  test_value: f32)
{
    let calc = H2OWagnerPruss::calculate_phi_0_tau(temperature, density);
    assert!((calc - test_value).abs() < std::f32::EPSILON);
}