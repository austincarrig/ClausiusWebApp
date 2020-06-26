mod constants;

use constants::Constants;

pub struct SuperheatedRegionCalculator;
impl SuperheatedRegionCalculator {
    pub fn calculate_pressure(temperature: f32,
                              entropy: f32) -> f32
    {
        let clamped_temp = SuperheatedRegionCalculator::clamp_temperature(temperature);

        let mut i: u16 = 0;
        for t in Constants::TEMPERATURE_T_S.iter()
        {
            if (temperature - t).abs() < std::f32::EPSILON
            {
                break;
            }
            else if t > &temperature
            {
                i -= 1;
                break;
            }

            i += 1;
        }

        let mut j: u16 = 0;
        let entropy_row: [f32; 58] = Constants::ENTROPY_T_S[i as usize];
        for s in entropy_row.iter()
        {
            if s <= &entropy
            {
                if j != 0
                {
                    j -= 1;
                }
                break;
            }
            
            j += 1;
        }

        if j == 0 || j == Constants::PRESSURE_T_S.len() as u16 - 1
        {
            Constants::PRESSURE_T_S[j as usize]
        }
        else
        {
            let high_entropy = entropy_row[j as usize];
            let low_entropy  = entropy_row[(j - 1) as usize];

            let weight = (entropy - low_entropy) / (high_entropy - low_entropy);

            let high_pressure = Constants::PRESSURE_T_S[j as usize];
            let low_pressure  = Constants::PRESSURE_T_S[(j - 1) as usize];

            low_pressure + weight * (high_pressure - low_pressure)
        }
    }

    fn clamp_temperature(temperature: f32) -> f32
    {
        if temperature < Constants::TEMPERATURE_T_S[0]
        {
            Constants::TEMPERATURE_T_S[0]
        }
        else if temperature > Constants::TEMPERATURE_T_S[Constants::TEMPERATURE_T_S.len() - 1]
        {
            Constants::TEMPERATURE_T_S[Constants::TEMPERATURE_T_S.len() - 1]
        }
        else
        {
            temperature
        }
    }
}

#[cfg(test)]
mod tests;