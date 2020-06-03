/*
 * FILE: Solver.js
 * 
 * DESCRIPTION:
 * This class is used to calculate temperature given a 
 * specific volume and pressure.
 * Converted from Obj-C code 11/9/2019
 * Converted from C# code 6/2/2020
 * 
 */

const M_Water = 0.01801528; // Molar mass of water expressed in kg/mol
const T_CR    = 647.0;      // Critical temperature in K
const P_CR    = 22100.0;    // Critical pressure in kPA
const R_S     = 0.48;       // Gas constant in kJ/(kg.K)

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

function CalculatePressure(specVol,
                           temp)
{
    let Vm = M_Water * specVol; // Molar volume in m³/mol

    let a = 0.4278 * Math.pow(R_S, 2) * Math.pow(T_CR, 2.5) / P_CR;

    let b = 0.0867 * R_S * T_CR / P_CR;

    let pressure = (R_S * temp / (Vm - b)) - (a / (Vm * (Vm + b) * Math.Sqrt(temp)));

    return pressure;
}

// Solved using the Relich-Kwong equation of state
function CalculateTemperature(specVol,
                              pressure)
{
    let Vm = M_Water * specVol; // Molar volume in m³/mol

    let a = 0.4275 * Math.pow(R_S, 2) * Math.pow(T_CR, 2.5) / P_CR;

    let b = 0.08664 * R_S * T_CR / P_CR;

    let g = R_S / (Vm - b);

    let h = a / (Vm * (Vm + b));

    let c = Math.pow(27.0 * Math.pow(g, 4.0) * Math.pow(h, 2.0) - 2.0 * Math.pow(g, 3.0) * Math.pow(pressure, 3.0) + 3.0 * Math.Sqrt(3.0) * Math.Sqrt(27.0 * Math.pow(g, 8.0) * Math.pow(h, 4.0) - 4.0 * Math.pow(g, 7.0) * Math.pow(h, 2.0) * Math.pow(pressure, 3.0)), 1.0 / 3.0);

    let temp = (1.0 / 3.0) * (Math.pow(2.0, 1.0 / 3.0) * Math.pow(pressure, 2.0) / c + c / (Math.pow(2.0, 1.0 / 3.0) * Math.pow(g, 2.0)) + 2.0 * pressure / g);

    return temp;
}