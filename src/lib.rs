#![allow(dead_code)]
#![allow(unused_variables)]

// The wasm-pack uses wasm-bindgen to build and generate JavaScript binding file.
// Import the wasm-bindgen crate. 
use wasm_bindgen::prelude::*;

mod solver;
mod plot_point;
mod graph;

use plot_point::PlotPoint;

// This exports an add function.
// It takes in two 32-bit integer values
// And returns a 32-bit integer value.
#[wasm_bindgen]
pub fn calculate_thermo_properties(canvas_width: f32,
                                   canvas_height: f32,
                                   canvas_pos_x: f32,
                                   canvas_pos_y: f32) -> PlotPoint {
    
    PlotPoint::new(
        12.0,
        1.0,
        1.0,
        0.0,
        0.0,
        0.0,
        -1.0
    )
}


// TEST CODE FOR LATER!!!
/*
    const M_WATER: f32 = solver::Solver::M_WATER; // Molar mass of water expressed in kg/mol
    const CM2M: f32    = 1.0 / 100.0;    // Conversion from centimeters to meters, 1/100
    let temp: f32    = 294.4;          // T in Kelvin
    let molar_volume: f32  = 50.0;           // Molar volume in cm³/mol

    // Specific volume in m³/kg
    let specific_volume: f32 = (molar_volume / M_WATER) * f32::powf(CM2M, 3.0);

    // Pressure output should be kPA
    let pressure: f32 = solver::Solver::calculate_pressure(specific_volume, temp);

    let temperature: f32 = solver::Solver::calculate_temperature(specific_volume, pressure);
    */