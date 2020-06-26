use super::*;
use super::super::H2OWagnerPruss;

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

//const TOP_LEFT_TS_DIAGRAM_PHI_R_TAU: f32 = ;
//const TOP_RIGHT_TS_DIAGRAM_PHI_R_TAU: f32 = ;
//const BOTTOM_RIGHT_TS_DIAGRAM_PHI_R_TAU: f32 = ;
const SUPERHEATED_POINT_1_INT_ENERGY: f32 = 3226.7724234968214;

#[test]
#[wasm_bindgen_test]
fn superheated_point_1()
{
    test_int_energy(SUPERHEATED_POINT_1_T,
                    SUPERHEATED_POINT_1_D,
                    SUPERHEATED_POINT_1_INT_ENERGY);
}

fn test_int_energy(temperature: f32,
                   density: f32,
                   test_value: f32)
{
    let calc = H2OWagnerPruss::calculate_internal_energy(temperature, density);
    assert!((calc - test_value).abs() < std::f32::EPSILON);
}