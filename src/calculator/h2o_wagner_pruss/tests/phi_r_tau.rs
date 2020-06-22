use super::*;
use super::super::H2OWagnerPruss;

//const TOP_LEFT_TS_DIAGRAM_PHI_R_TAU: f32 = -1.525597801637211;
//const TOP_RIGHT_TS_DIAGRAM_PHI_R_TAU: f32 = -0.001659790043634;
//const BOTTOM_RIGHT_TS_DIAGRAM_PHI_R_TAU: f32 = -0.001527936462799;
const SUPERHEATED_POINT_1_PHI_R_TAU: f32 = -0.082931693508678;

#[test]
fn superheated_point_1()
{
    test_phi_r_tau(SUPERHEATED_POINT_1_T,
                   SUPERHEATED_POINT_1_D,
                   SUPERHEATED_POINT_1_PHI_R_TAU);
}

fn test_phi_r_tau(temperature: f32,
                  density: f32,
                  test_value: f32)
{
    let calc = H2OWagnerPruss::calculate_phi_r_tau(temperature, density);
    assert!((calc - test_value).abs() < std::f32::EPSILON);
}