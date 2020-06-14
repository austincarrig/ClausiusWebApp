use super::super::graph::chart::ChartType;
use super::super::plot_point::PlotPoint;

use super::superheated_region_calculator::SuperheatedRegionCalculator;

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
                panic!("AHH")
            }
            else
            {
                panic!("AH")
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

        // H2OWagnerPruss wagPruss = H2OWagnerPruss.Instance;

        let density = -1.0; //wagPruss.CalculateDensity(temperatureKelvin,
                                                   //pressureMPa);

        let spec_volume = 1.0 / density;

        let int_energy = -1.0; //wagPruss.CalculateInternalEnergy(temperatureKelvin,
                                                            //density);

        let enthalpy = -1.0; //wagPruss.CalculateEnthalpy(temperatureKelvin,
                                                     //density);

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
}