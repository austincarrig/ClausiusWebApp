// The wasm-pack uses wasm-bindgen to build and generate JavaScript binding file.
// Import the wasm-bindgen crate. 
use wasm_bindgen::prelude::*;

mod solver;
mod plot_point;

use plot_point::PlotPoint;

// This exports an add function.
// It takes in two 32-bit integer values
// And returns a 32-bit integer value.
#[wasm_bindgen]
pub fn call_me_from_javascript() -> PlotPoint {
    const M_WATER: f32 = solver::Solver::M_WATER; // Molar mass of water expressed in kg/mol
    const CM2M: f32    = 1.0 / 100.0;    // Conversion from centimeters to meters, 1/100
    let temp: f32    = 294.4;          // T in Kelvin
    let molar_volume: f32  = 50.0;           // Molar volume in cm³/mol

    // Specific volume in m³/kg
    let specific_volume: f32 = (molar_volume / M_WATER) * f32::powf(CM2M, 3.0);

    // Pressure output should be kPA
    let pressure: f32 = solver::Solver::calculate_pressure(specific_volume, temp);

    let temperature: f32 = solver::Solver::calculate_temperature(specific_volume, pressure);

    PlotPoint::new(
        1.0,
        1.0,
        1.0,
        0.0,
        0.0,
        0.0,
        -1.0
    )
}

// A NOT exported constant
// Rust does not support exporting constants
// for Wasm (that I know of).
// const ADD_CONSTANT: i32 = 24;

// A NOT exported function
// It takes in two 32-bit integer values
// And returns a 32-bit integer value.
//fn add_integer_with_constant(a: i32, b: i32) -> i32 {
//    return a + b + ADD_CONSTANT;
//}