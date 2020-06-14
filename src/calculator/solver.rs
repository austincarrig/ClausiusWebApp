/*
 * FILE: Solver.rs
 * 
 * DESCRIPTION:
 * This class is used to calculate temperature given a specific volume and pressure.
 * Converted from Obj-C to C# 11/9/2019.
 * Converted from C# to Rust 6/10/2019
 * 
 */

pub struct Solver;
impl Solver
{
    pub const M_WATER: f32 = 0.01801528; // Molar mass of water expressed in kg/mol
    const T_CR: f32 = 647.0;         // Critical temperature in K
    const P_CR: f32 = 22100.0;       // Critical pressure in kPA
    const R_S: f32 = 0.48;           // Gas constant in kJ/(kg.K)

    // Redlich-Kwong equation of state, where V is molar volume:
    //
    //      R * T            a
    // p = ——————— - ——————————————————
    //      V - b     V * (V + b) * √T
    //
    // a, b are constants:
    //
    //               R² * √(T_crit⁵)
    // a = 0.4278 * —————————————————
    //                   p_crit
    //
    //               R * T_crit
    // b = 0.0867 * ————————————
    //                 p_crit
    //
    // Units:
    // T expected in K
    // P expected in kPA
    // V expected in m³/kg
    pub fn calculate_pressure(specific_volume: f32,
                              temp: f32) -> f32
    {
        let v_m: f32 = Solver::M_WATER * specific_volume; // Molar volume in m³/mol

        let a: f32 = 0.4278 * f32::powf(Solver::R_S, 2.0) * f32::powf(Solver::T_CR, 2.5) / Solver::P_CR;

        let b: f32 = 0.0867 * Solver::R_S * Solver::T_CR / Solver::P_CR;

        let pressure: f32 = (Solver::R_S * temp / (v_m - b)) - (a / (v_m * (v_m + b) * temp.sqrt()));

        return pressure;
    }

    // Solved using the Relich-Kwong equation of state
    pub fn calculate_temperature(specific_volume: f32,
                                 pressure: f32) -> f32
    {
        let v_m: f32 = Solver::M_WATER * specific_volume; // Molar volume in m³/mol

        let a: f32 = 0.4275 * f32::powf(Solver::R_S, 2.0) * f32::powf(Solver::T_CR, 2.5) / Solver::P_CR;

        let b: f32 = 0.08664 * Solver::R_S * Solver::T_CR / Solver::P_CR;

        let g: f32 = Solver::R_S / (v_m - b);

        let h: f32 = a / (v_m * (v_m + b));

        let c: f32 = f32::powf(27.0 * f32::powf(g, 4.0) * f32::powf(h, 2.0) - 2.0 * f32::powf(g, 3.0) * f32::powf(pressure, 3.0) + 3.0 * (3.0 as f32).sqrt() * (27.0 * f32::powf(g, 8.0) * f32::powf(h, 4.0) - 4.0 * f32::powf(g, 7.0) * f32::powf(h, 2.0) * f32::powf(pressure, 3.0)).sqrt(), 1.0 / 3.0);

        let temp: f32 = (1.0 / 3.0) * (f32::powf(2.0, 1.0 / 3.0) * f32::powf(pressure, 2.0) / c + c / (f32::powf(2.0, 1.0 / 3.0) * f32::powf(g, 2.0)) + 2.0 * pressure / g);

        return temp;
    }
}