use super::*;
use super::super::H2OWagnerPruss;

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

//const TOP_LEFT_TS_DIAGRAM_PHI_R_TAU: f32 = ;
//const TOP_RIGHT_TS_DIAGRAM_PHI_R_TAU: f32 = ;
//const BOTTOM_RIGHT_TS_DIAGRAM_PHI_R_TAU: f32 = ;
const SUPERHEATED_POINT_1_ENTHALPY: f32 = 3607.9074378844493;

#[test]
#[wasm_bindgen_test]
fn superheated_point_1()
{
    test_enthalpy(SUPERHEATED_POINT_1_T,
                  SUPERHEATED_POINT_1_D,
                  SUPERHEATED_POINT_1_ENTHALPY);
}

fn test_enthalpy(temperature: f32,
                 density: f32,
                 test_value: f32)
{
    let calc = H2OWagnerPruss::calculate_enthalpy(temperature, density);
    assert!((calc - test_value).abs() < std::f32::EPSILON);

    let u = H2OWagnerPruss::calculate_internal_energy(temperature, density);
    let calc2 = H2OWagnerPruss::calculate_enthalpy_with_u(temperature, density, u);
    assert!((calc2 - test_value).abs() < std::f32::EPSILON);
}