#![allow(dead_code)]
#![allow(unused_variables)]

// The wasm-pack uses wasm-bindgen to build and generate JavaScript binding file.
// Import the wasm-bindgen crate. 
use wasm_bindgen::prelude::*;

mod plot_point;
mod graph;
mod calculator;

use graph::chart::{Chart, ChartType};
use plot_point::PlotPoint;
use calculator::thermodynamic_calculator::ThermodynamicCalculator;

// This exports an add function.
// It takes in two 32-bit integer values
// And returns a 32-bit integer value.
#[wasm_bindgen]
pub fn calculate_thermo_properties(canvas_width: f32,
                                   canvas_height: f32,
                                   canvas_pos_x: f32,
                                   canvas_pos_y: f32) -> PlotPoint
{
    let chart = Chart::new(ChartType::Ts);

    let x_scale = (chart.x_axis.max - chart.x_axis.min) / canvas_width;
    let y_scale = (chart.y_axis.max - chart.y_axis.min) / canvas_height;

    let x_value = chart.x_axis.min + x_scale * canvas_pos_x;
    let y_value = chart.y_axis.min + y_scale * (canvas_height - canvas_pos_y);

    ThermodynamicCalculator::calculate(x_value,
                                       y_value,
                                       ChartType::Ts)
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