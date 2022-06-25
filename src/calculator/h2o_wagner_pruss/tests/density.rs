use super::*;
use super::super::H2OWagnerPruss;

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

// MPa
//const TOP_LEFT_TS_DIAGRAM_PHI_R_TAU: f32 = ;
//const TOP_RIGHT_TS_DIAGRAM_PHI_R_TAU: f32 = ;
//const BOTTOM_RIGHT_TS_DIAGRAM_PHI_R_TAU: f32 = ;
const SUPERHEATED_POINT_1_PRESSURE: f32 = 3.796165343768067;

#[test]
#[wasm_bindgen_test]
fn superheated_point_1()
{
    test_delta(SUPERHEATED_POINT_1_T,
               SUPERHEATED_POINT_1_PRESSURE,
               SUPERHEATED_POINT_1_D);
}

#[test]
#[wasm_bindgen_test]
fn superheated_point_2()
{
    let calc = H2OWagnerPruss::calculate_density(599.62558, 34.2);
    println!("Supeheated Point 2 Caluclated Density: {}", calc);
    assert!(calc > 0.0);
}

fn test_delta(temperature: f32,
              pressure: f32,
              test_value: f32)
{
    let calc = H2OWagnerPruss::calculate_density(temperature, pressure);
    assert!((calc - test_value).abs() < std::f32::EPSILON);
}