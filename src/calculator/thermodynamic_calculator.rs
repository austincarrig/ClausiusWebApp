use super::super::graph::chart::ChartType;
use super::super::plot_point::PlotPoint;

use super::superheated_region_calculator::SuperheatedRegionCalculator;
use super::saturated_region_line::SaturatedRegionLine;
use super::h2o_wagner_pruss::H2OWagnerPruss;

pub struct ThermodynamicCalculator;
impl ThermodynamicCalculator {
    // Critical temperature of water (C)
    const T_CRITICAL: f32 = 373.9;

    // Critical pressure of water (kPA)
    // private static readonly double P_CRITICAL = 22100.0;

    // Minimum temperature to display on the ts diagram (C)
    const T_SAT_MIN: f32 = 1.0;

    pub fn calculate(x_value: f32,
                     y_value: f32,
                     chart_type: ChartType) -> PlotPoint
    {
        match chart_type {
            ChartType::Ts => {
                ThermodynamicCalculator::calculate_t_s(y_value, x_value)
            }
            ChartType::Ph | ChartType::Pv => {
                panic!("AHHH")
            }
        }
    }

    fn calculate_t_s(temperature: f32,
                     entropy: f32) -> PlotPoint
    {
        let mut clamped_temp = temperature;
        
        if clamped_temp < ThermodynamicCalculator::T_SAT_MIN
        {
            clamped_temp = ThermodynamicCalculator::T_SAT_MIN;
        }

        if clamped_temp > ThermodynamicCalculator::T_CRITICAL
        {
            ThermodynamicCalculator::calculate_superheated(clamped_temp,
                                                           entropy)
        }
        else
        {
            if clamped_temp == ThermodynamicCalculator::T_CRITICAL
            {
                PlotPoint::new(
                    clamped_temp,
                    -1.0,
                    -1.0,
                    -1.0,
                    -1.0,
                    entropy,
                    -1.0
                )
            }
            else
            {
                let sat_region_line = SaturatedRegionLine::new(temperature);

                if entropy < sat_region_line.get_s_f()
                {
                    ThermodynamicCalculator::calculate_compressed(sat_region_line,
                                                                  entropy)
                }
                else if entropy > sat_region_line.get_s_f() && entropy < sat_region_line.get_s_g()
                {
                    ThermodynamicCalculator::calculate_saturated(sat_region_line,
                                                                 entropy)
                }
                else
                {
                    ThermodynamicCalculator::calculate_superheated(clamped_temp,
                                                                   entropy)
                }
            }
        }
    }

    fn calculate_superheated(temperature: f32,
                             entropy: f32) -> PlotPoint
    {
        let pressure = SuperheatedRegionCalculator::calculate_pressure(temperature,
                                                                       entropy);

        let temperature_kelvin = temperature + 273.15; // temperature (C -> K)
        let pressure_mpa = pressure / 1000.0; // pressure (kPA -> MPa)

        let density = H2OWagnerPruss::calculate_density(temperature_kelvin,
                                                        pressure_mpa);

        let spec_volume = 1.0 / density;

        let int_energy = H2OWagnerPruss::calculate_internal_energy(temperature_kelvin,
                                                                   density);

        let enthalpy = H2OWagnerPruss::calculate_enthalpy_with_u(temperature_kelvin,
                                                                 density,
                                                                 int_energy);

        PlotPoint::new(
            temperature,
            pressure,
            spec_volume,
            int_energy,
            enthalpy,
            entropy,
            -1.0
        )
    }

    fn calculate_compressed(sat_region_line: SaturatedRegionLine,
                            entropy: f32) -> PlotPoint
    {
        PlotPoint::new(
            sat_region_line.get_t(),
            sat_region_line.get_p(),
            sat_region_line.get_v_f(),
            sat_region_line.get_u_f(),
            sat_region_line.get_h_f(),
            entropy,
            -1.0
        )
    }

    fn calculate_saturated(sat_region_line: SaturatedRegionLine,
                           entropy: f32) -> PlotPoint
    {
        let pressure    = sat_region_line.get_p();
        let quality     = (entropy - sat_region_line.get_s_f()) / (sat_region_line.get_s_g() - sat_region_line.get_s_f());
        let spec_volume = sat_region_line.get_v_f() + quality * (sat_region_line.get_v_g() - sat_region_line.get_v_f());
        let int_energy  = sat_region_line.get_u_f() + quality * (sat_region_line.get_u_g() - sat_region_line.get_u_f());
        let enthalpy    = sat_region_line.get_h_f() + quality * (sat_region_line.get_h_g() - sat_region_line.get_h_f());

        PlotPoint::new(
            sat_region_line.get_t(),
            pressure,
            spec_volume,
            int_energy,
            enthalpy,
            entropy,
            quality * 100.0
        )
    }
}