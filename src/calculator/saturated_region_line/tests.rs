use super::SaturatedRegionLine;

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

const TEMPERATURE: f32 = 200.5; // C
const PRESSURE: f32 = 1571.0645; // kPa
const PRESSURE_SIG_DIG: f32 = 0.001; // kPA
const SPEC_VOL_FLUID: f32 = 0.0011573; // m3/kg
const SPEC_VOL_FLUID_SIG_DIG: f32 = 0.00001; // m3/kg
const SPEC_VOL_GAS: f32 = 0.125943; // m3/kg
const SPEC_VOL_GAS_SIG_DIG: f32 = 0.0001; // m3/kg
const INT_ENERGY_FLUID: f32 = 852.82913712805; // kJ/kg
const INT_ENERGY_FLUID_SIG_DIG: f32 = 0.1; // kJ/kg
const INT_ENERGY_GAS: f32 = 2594.5058609568; // kJ/kg
const INT_ENERGY_GAS_SIG_DIG: f32 = 0.1; // kJ/kg
const ENTHALPY_FLUID: f32 = 854.647; // kJ/kg
const ENTHALPY_FLUID_SIG_DIG: f32 = 0.01; // kJ/kg
const ENTHALPY_GAS: f32 = 2792.36; // kJ/kg
const ENTHALPY_GAS_SIG_DIG: f32 = 0.01; // kJ/kg
const ENTROPY_FLUID: f32 = 2.3355224868653; // kJ/kg.K
const ENTROPY_FLUID_SIG_DIG: f32 = 0.0001; // kJ/kg.K
const ENTROPY_GAS: f32 = 6.4265656433297; // kJ/kg.K
const ENTROPY_GAS_SIG_DIG: f32 = 0.0001; // kJ/kg.K

#[test]
#[wasm_bindgen_test]
fn pressure()
{
    let sat_line = SaturatedRegionLine::new(200.5);
    assert!((PRESSURE - sat_line.get_p()).abs() < PRESSURE_SIG_DIG)
}

#[test]
#[wasm_bindgen_test]
fn spec_vol_fluid()
{
    let sat_line = SaturatedRegionLine::new(TEMPERATURE);
    assert!((SPEC_VOL_FLUID - sat_line.get_v_f()).abs() < SPEC_VOL_FLUID_SIG_DIG)
}

#[test]
#[wasm_bindgen_test]
fn spec_vol_gas()
{
    let sat_line = SaturatedRegionLine::new(TEMPERATURE);
    assert!((SPEC_VOL_GAS - sat_line.get_v_g()).abs() < SPEC_VOL_GAS_SIG_DIG)
}

#[test]
#[wasm_bindgen_test]
fn int_energy_fluid()
{
    let sat_line = SaturatedRegionLine::new(TEMPERATURE);
    assert!((INT_ENERGY_FLUID - sat_line.get_u_f()).abs() < INT_ENERGY_FLUID_SIG_DIG)
}

#[test]
#[wasm_bindgen_test]
fn int_energy_gas()
{
    let sat_line = SaturatedRegionLine::new(TEMPERATURE);
    assert!((INT_ENERGY_GAS - sat_line.get_u_g()).abs() < INT_ENERGY_GAS_SIG_DIG)
}

#[test]
#[wasm_bindgen_test]
fn enthalpy_fluid()
{
    let sat_line = SaturatedRegionLine::new(TEMPERATURE);
    assert!((ENTHALPY_FLUID - sat_line.get_h_f()).abs() < ENTHALPY_FLUID_SIG_DIG)
}

#[test]
#[wasm_bindgen_test]
fn enthalpy_gas()
{
    let sat_line = SaturatedRegionLine::new(TEMPERATURE);
    assert!((ENTHALPY_GAS - sat_line.get_h_g()).abs() < ENTHALPY_GAS_SIG_DIG)
}

#[test]
#[wasm_bindgen_test]
fn entropy_fluid()
{
    let sat_line = SaturatedRegionLine::new(TEMPERATURE);
    assert!((ENTROPY_FLUID - sat_line.get_s_f()).abs() < ENTROPY_FLUID_SIG_DIG)
}

#[test]
#[wasm_bindgen_test]
fn entropy_gas()
{
    let sat_line = SaturatedRegionLine::new(TEMPERATURE);
    assert!((ENTROPY_GAS - sat_line.get_s_g()).abs() < ENTROPY_GAS_SIG_DIG)
}