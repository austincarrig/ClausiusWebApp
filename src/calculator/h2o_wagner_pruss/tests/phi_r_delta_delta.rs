use super::*;
use super::super::H2OWagnerPruss;

//const TOP_LEFT_TS_DIAGRAM_PHI_R_TAU: f32 = ;
//const TOP_RIGHT_TS_DIAGRAM_PHI_R_TAU: f32 = ;
//const BOTTOM_RIGHT_TS_DIAGRAM_PHI_R_TAU: f32 = ;
const SUPERHEATED_POINT_1_PHI_R_DELTA_DELTA: f32 = 0.127586340316874;

#[test]
fn superheated_point_1()
{
    test_phi_r_delta_delta(SUPERHEATED_POINT_1_T,
                           SUPERHEATED_POINT_1_D,
                           SUPERHEATED_POINT_1_PHI_R_DELTA_DELTA);
}

fn test_phi_r_delta_delta(temperature: f32,
                          density: f32,
                          test_value: f32)
{
    let calc = H2OWagnerPruss::calculate_phi_r_delta_delta(temperature, density);
    assert!((calc - test_value).abs() < std::f32::EPSILON);
}