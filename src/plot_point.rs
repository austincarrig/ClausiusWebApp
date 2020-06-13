/*
 * FILE ABSTRACT: plot_point.rs
 * 
 * DESCRIPTION:
 * This class represents the data on a specific point on a thermodynamic chart.
 * 
 */

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct PlotPoint {
    t: f32, // Temperature (C)
    p: f32, // Pressure (TODO)
    v: f32, // Specific Volume (TODO)
    u: f32, // Internal Energy (TODO)
    h: f32, // Enthalpy (TODO)
    s: f32, // Entropy (TODO)
    x: f32, // Quality (%)
}

#[wasm_bindgen]
impl PlotPoint {
    pub fn new (t_in: f32,
                p_in: f32,
                v_in: f32,
                u_in: f32,
                h_in: f32,
                s_in: f32,
                x_in: f32) -> PlotPoint {
        PlotPoint {
            t: t_in,
            p: p_in,
            v: v_in,
            u: u_in,
            h: h_in,
            s: s_in,
            x: x_in
        }
    }

    pub fn get_t(&self) -> f32 {
        self.t
    }

    pub fn get_p(&self) -> f32 {
        self.p
    }

    pub fn get_v(&self) -> f32 {
        self.v
    }

    pub fn get_u(&self) -> f32 {
        self.u
    }

    pub fn get_h(&self) -> f32 {
        self.h
    }

    pub fn get_s(&self) -> f32 {
        self.s
    }

    pub fn get_x(&self) -> f32 {
        self.x
    }
}