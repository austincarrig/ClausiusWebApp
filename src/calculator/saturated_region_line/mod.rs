mod constants;

use constants::Constants;

pub struct SaturatedRegionLine
{
    t: f32,
    p: f32,
    v_f: f32,
    v_g: f32,
    u_f: f32,
    u_g: f32,
    h_f: f32,
    h_g: f32,
    s_f: f32,
    s_g: f32,
}

impl SaturatedRegionLine
{
    pub fn new(t_in: f32) -> SaturatedRegionLine
    {
        let mut i: u16 = 0;
        for t in Constants::TEMPERATURE.iter()
        {
            if t > &t_in
            {
                break;
            }
            i += 1;
        }

        let low_temp = Constants::TEMPERATURE[(i - 1) as usize];
        let high_temp = Constants::TEMPERATURE[i as usize];

        let weight = (t_in - low_temp) / (high_temp - low_temp);

        SaturatedRegionLine {
            t: t_in,
            p: SaturatedRegionLine::interpolate(Constants::PRESSURE, weight, i),
            v_f: SaturatedRegionLine::interpolate(Constants::SPECIFIC_VOLUME_FLUID, weight, i),
            v_g: SaturatedRegionLine::interpolate(Constants::SPECIFIC_VOLUME_GAS, weight, i),
            u_f: SaturatedRegionLine::interpolate(Constants::INTERNAL_ENERGY_FLUID, weight, i),
            u_g: SaturatedRegionLine::interpolate(Constants::INTERNAL_ENERGY_GAS, weight, i),
            h_f: SaturatedRegionLine::interpolate(Constants::ENTHALPY_FLUID, weight, i),
            h_g: SaturatedRegionLine::interpolate(Constants::ENTHALPY_GAS, weight, i),
            s_f: SaturatedRegionLine::interpolate(Constants::ENTROPY_FLUID, weight, i),
            s_g: SaturatedRegionLine::interpolate(Constants::ENTROPY_GAS, weight, i)
        }
    }

    pub fn get_t(&self) -> f32
    {
        self.t
    }

    pub fn get_p(&self) -> f32
    {
        self.p
    }

    pub fn get_v_f(&self) -> f32
    {
        self.v_f
    }

    pub fn get_v_g(&self) -> f32
    {
        self.v_g
    }

    pub fn get_u_f(&self) -> f32
    {
        self.u_f
    }

    pub fn get_u_g(&self) -> f32
    {
        self.u_g
    }

    pub fn get_h_f(&self) -> f32
    {
        self.h_f
    }

    pub fn get_h_g(&self) -> f32
    {
        self.h_g
    }

    pub fn get_s_f(&self) -> f32
    {
        self.s_f
    }

    pub fn get_s_g(&self) -> f32
    {
        self.s_g
    }

    pub fn calculate_temperature(p_in: f32) -> f32
    {
        let mut i: u16 = 0;
        for p in Constants::PRESSURE.iter()
        {
            if p > &p_in
            {
                break;
            }
            i += 1;
        }

        let low_p  = Constants::PRESSURE[(i - 1) as usize];
        let high_p = Constants::PRESSURE[i as usize];
        let low_t  = Constants::TEMPERATURE[(i - 1) as usize];
        let high_t = Constants::TEMPERATURE[i as usize];

        let weight = (p_in - low_p) / (high_p - low_p);

        low_t + weight * (high_t - low_t)
    }

    fn interpolate(values: [f32; 375],
                   weight: f32,
                   index: u16) -> f32
    {
        let low: f32  = values[(index - 1) as usize];
        let high: f32 = values[index as usize];

        low + weight * (high - low)
    }
}