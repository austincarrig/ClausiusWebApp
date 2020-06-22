const C_TO_K: f32 = 273.15;

const TOP_LEFT_TS_DIAGRAM_T: f32 = 699.0;
const TOP_LEFT_TS_DIAGRAM_D: f32 = 250.0;

const TOP_RIGHT_TS_DIAGRAM_T: f32 = 700.0;
const TOP_RIGHT_TS_DIAGRAM_D: f32 = 0.24143509017;

const BOTTOM_RIGHT_TS_DIAGRAM_T: f32 = 1.0;
const BOTTOM_RIGHT_TS_DIAGRAM_D: f32 = 0.00487925074;

const SUPERHEATED_POINT_1_T: f32 = 570.01 + C_TO_K;
const SUPERHEATED_POINT_1_D: f32 = 9.96015936255;

const SUPERHEATED_POINT_2_T: f32 = 420.38;
const SUPERHEATED_POINT_2_D: f32 = 270.27027027;

const SUPERHEATED_POINT_3_T: f32 = 393.16;
const SUPERHEATED_POINT_3_D: f32 = 0.82155767334;

const SUPERHEATED_POINT_4_T: f32 = 334.99;
const SUPERHEATED_POINT_4_D: f32 = 27.7777777778;

const SUPERHEATED_POINT_5_T: f32 = 245.84;
const SUPERHEATED_POINT_5_D: f32 = 0.07614812332;

const SUPERHEATED_POINT_6_T: f32 = 160.46;
const SUPERHEATED_POINT_6_D: f32 = 2.75406224181;

const SUPERHEATED_POINT_7_T: f32 = 160.46;
const SUPERHEATED_POINT_7_D: f32 = 2.75406224181;

// Above Critical Point
const SUPERHEATED_POINT_8_T: f32 = 389.41;
const SUPERHEATED_POINT_8_D: f32 = 625.0;

mod phi_0_tau;
mod phi_r_delta;
mod phi_r_delta_delta;
mod phi_r_tau;

/*
// Near Critical Point
#[test]
#[ignore]
fn saturated_point_1()
{
    let temperature: f32 = 363.14;
    let density: f32 = 250.0;
}

#[test]
#[ignore]
fn saturated_point_2()
{
    let temperature: f32 = 289.01;
    let density: f32 = 285.714285714;
}

#[test]
#[ignore]
fn saturated_point_3()
{
    let temperature: f32 = 191.42;
    let density: f32 = 98.0392156863;
}

#[test]
#[ignore]
fn saturated_point_4()
{
    let temperature: f32 = 101.34;
    let density: f32 = 26.8817204301;
}

#[test]
#[ignore]
fn saturated_point_5()
{
    let temperature: f32 = 4.69;
    let density: f32 = 0.41618112202;
}

#[test]
#[ignore]
fn saturated_point_6()
{
    let temperature: f32 = 8.45;
    let density: f32 = 0.01750562368;
}

#[test]
#[ignore]
fn saturated_point_7()
{
    let temperature: f32 = 6.57;
    let density: f32 = 0.00774261083;
}

#[test]
#[ignore]
fn saturated_point_8()
{
    let temperature: f32 = 99.46;
    let density: f32 = 0.60368246302;
}

#[test]
#[ignore]
fn saturated_point_9()
{
    let temperature: f32 = 198.93;
    let density: f32 = 7.92393026941;
}

#[test]
#[ignore]
fn saturated_point_10()
{
    let temperature: f32 = 301.21;
    let density: f32 = 48.5436893204;
}

#[test]
#[ignore]
fn saturated_point_11()
{
    let temperature: f32 = 180.16;
    let density: f32 = 10.0704934542;
}

*/
