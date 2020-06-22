/*
 * FILE ABSTRACT: H20WagnerPruss.cs
 * 
 * DESCRIPTION:
 * Wagner & Pruß (2002)
 * The IAPWS Formulation 1995 for the thermodynamic properties of ordinary water substance
 * for general and scientific use. J.Phys.Chem.Ref.Data., 31, 387-535
 * 
 * Code has been converted from Obj-C code written by Matthias Gottschalk, 8/19/2010.
 * Copyright for this product belongs to PhysChemGeo, 2010. All rights reserved.
 * Permission has been provided by the originator to reproduce and commercially use
 * this product.
 * 
 */

mod constants;

use constants::Constants;

pub struct H2OWagnerPruss;

impl H2OWagnerPruss
{
    pub fn calculate_density(temperature: f32,
                             pressure: f32) -> f32
    {
        let mut rho1: f32 = -1.0;
        let mut rho2: f32 = -1.0;
        
        let rho_min = pressure / (2.0 * (Constants::coVolume * pressure + Constants::R / 1000.0 * temperature));

        let rho_max = Constants::extrap_max_density;

        let log_rho_increment = (rho_max.ln() - rho_min.ln()) / 200.0;

        let mut log_rho = rho_min.ln();

        let mut old_rho = rho_min;

        let mut old_f_rho = rho_min / 1000.0 * Constants::R * temperature * (1.0 + rho_min / Constants::rho_c * H2OWagnerPruss::calculate_phi_r_delta(temperature, rho_min)) - pressure;
        
        while log_rho < rho_max.ln()
        {
            log_rho += log_rho_increment;

            let rho = f32::exp(log_rho);

            let f_rho = rho / 1000.0 * Constants::R * temperature * (1.0 + rho / Constants::rho_c * H2OWagnerPruss::calculate_phi_r_delta(temperature, rho)) - pressure;

            let sign_f_rho = old_f_rho * f_rho;

            if sign_f_rho < 0.0
            {
                let new_rho = H2OWagnerPruss::find_rho_with_estimated_density(temperature,
                                                                              pressure,
                                                                              (old_rho + rho) / 2.0);

                if rho1 == -1.0
                {
                    rho1 = new_rho
                }
                else
                {
                    rho2 = new_rho
                }
            }

            old_f_rho = f_rho;
            old_rho = rho;
        }

        // If there is only 1 root, then this is the solution
        if rho2 == -1.0
        {
            return rho1;
        }

        // Temperature above critical point; pressure above, at or below critical point 
        if temperature > Constants::T_c
        {
            return rho2;
        }

        let p_sat = H2OWagnerPruss::pressure_vapor_liquid(temperature);

        // Temperature below critical point; pressure below vapor-liquid pressure
        if temperature <= Constants::T_c && pressure >= p_sat
        {
            return rho2;
        }

        // Temperature below critical point; pressure above vapor-liquid pressure
        if temperature <= Constants::T_c && pressure < p_sat
        {
            return rho1;
        }
        
        println!("Error");
        // Error return
        -1.0
    }

    pub fn calculate_internal_energy(temperature: f32,
                                     density: f32) -> f32
    {
        let tau = H2OWagnerPruss::calculate_tau(temperature);

        let phi_0_tau = H2OWagnerPruss::calculate_phi_0_tau(temperature,
                                                            density);
                                            
        let phi_r_tau = H2OWagnerPruss::calculate_phi_r_tau(temperature,
                                                            density);

        Constants::R * temperature * tau * (phi_0_tau + phi_r_tau) // kJ/kg
    }

    pub fn calculate_enthalpy(temperature: f32,
                              density: f32) -> f32
    {
        let tau = H2OWagnerPruss::calculate_tau(temperature);
        let delta = H2OWagnerPruss::calculate_delta(density);

        let phi_r_delta = H2OWagnerPruss::calculate_phi_r_delta(temperature,
                                                                density);

        let phi_0_tau = H2OWagnerPruss::calculate_phi_0_tau(temperature,
                                                            density);
                                            
        let phi_r_tau = H2OWagnerPruss::calculate_phi_r_tau(temperature,
                                                            density);

        let extra = Constants::R * temperature * delta * phi_r_delta;

        let internal_energy = Constants::R * temperature * tau * (phi_0_tau + phi_r_tau);

        Constants::R * temperature + internal_energy + extra // kJ/kg
    }
    
    pub fn calculate_enthalpy_with_u(temperature: f32,
                                     density: f32,
                                     internal_energy: f32) -> f32
    {
        let delta = H2OWagnerPruss::calculate_delta(density);

        let phi_r_delta = H2OWagnerPruss::calculate_phi_r_delta(temperature,
                                                                density);

        let extra = Constants::R * temperature * delta * phi_r_delta;

        Constants::R * temperature + internal_energy + extra // kJ/kg
    }

    fn find_rho_with_estimated_density(temperature: f32,
                                       pressure: f32,
                                       density_estimate:f32) -> f32
    {
        let mut delta = 0.0;

        let mut rho = density_estimate;

        let iteration_goal = 1.0e-12;
        let max_iteration = 20; // old value: 1000

        let mut i: u16 = 1;

        loop
        {
            let f_rho = Constants::rho_c / 1000.0 * Constants::R * temperature * (delta + f32::powf(delta, 2.0) * H2OWagnerPruss::calculate_phi_r_delta(temperature, rho)) - pressure;
            let d_f_rho = Constants::rho_c / 1000.0 * Constants::R * temperature * (1.0 + 2.0 * delta * H2OWagnerPruss::calculate_phi_r_delta(temperature, rho)
                            + f32::powf(delta, 2.0) * H2OWagnerPruss::calculate_phi_r_delta_delta(temperature, rho));

            let d_delta = -f_rho / d_f_rho;
            delta += d_delta;

            rho = delta * Constants::rho_c;

            i += 1;

            if i > max_iteration
            {
                // TODO: Output error
                break;
            }

            if f32::abs(d_delta) <= iteration_goal
            {
                break;
            }
        }

        rho
    }

    fn calculate_phi_r_delta(temperature: f32,
                             density: f32) -> f32
    {
        let mut phi_r_delta = 0.0;

        let tau = H2OWagnerPruss::calculate_tau(temperature);
        let delta = H2OWagnerPruss::calculate_delta(density);

        for i in 1..=7
        {
            phi_r_delta += Constants::n[i] * Constants::d[i] * f32::powf(delta, Constants::d[i] - 1.0) * f32::powf(tau, Constants::t[i]);
        }
        for i in 8..=51
        {
            phi_r_delta += Constants::n[i] * f32::exp(-f32::powf(delta, Constants::c[i])) * (f32::powf(delta, Constants::d[i] - 1.0) * f32::powf(tau, Constants::t[i]) * (Constants::d[i] - Constants::c[i] * f32::powf(delta, Constants::c[i])));
        }
        for i in 52..=54
        {
            phi_r_delta += Constants::n[i] * f32::powf(delta, Constants::d[i]) * f32::powf(tau, Constants::t[i]) * f32::exp(-Constants::alpha[i] * f32::powf(delta - Constants::epsilon[i], 2.0) - Constants::beta[i] * f32::powf(tau - Constants::gamma[i], 2.0))
            * (Constants::d[i] / delta - 2.0 * Constants::alpha[i] * (delta - Constants::epsilon[i]));
        }
        for i in 55..=56
        {
            // definition of theta, gdelta, psi
            let theta = (1.0 - tau) + Constants::A[i] * f32::powf((delta - 1.0) * (delta - 1.0), 1.0 / (2.0 * Constants::beta[i]));
            let gdelta = theta * theta + Constants::B[i] * f32::powf((delta - 1.0) * (delta - 1.0), Constants::a[i]);
            let psi = f32::exp(-Constants::C[i] * (delta - 1.0) * (delta - 1.0) - Constants::D[i] * (tau - 1.0) * (tau - 1.0));
            // definition of gdelta_delta, gdelta_bi_delta
            let gdelta_delta = (delta - 1.0) * (Constants::A[i] * theta * 2.0 / Constants::beta[i] * f32::powf((delta - 1.0) * (delta - 1.0), 1.0 / (2.0 * Constants::beta[i]) - 1.0)
                            + 2.0 * Constants::B[i] * Constants::a[i] * f32::powf((delta - 1.0) * (delta - 1.0), Constants::a[i] - 1.0));
            let gdelta_bi_delta = Constants::b[i] * f32::powf(gdelta, Constants::b[i] - 1.0) * gdelta_delta;
            // definition of psi_delta
            let psi_delta = -2.0 * Constants::C[i] * (delta - 1.0) * psi;
            phi_r_delta += Constants::n[i] * (f32::powf(gdelta, Constants::b[i]) * (psi + delta * psi_delta) + gdelta_bi_delta * delta * psi);
        }

        phi_r_delta
    }

    fn calculate_phi_r_delta_delta(temperature: f32,
                                   density: f32) -> f32
    {
        let mut phi_r_delta_delta = 0.0;

        let tau = H2OWagnerPruss::calculate_tau(temperature);
        let delta = H2OWagnerPruss::calculate_delta(density);

        for i in 1..=7
        {
            phi_r_delta_delta += Constants::n[i] * Constants::d[i] * (Constants::d[i] - 1.0) * f32::powf(delta, Constants::d[i] - 2.0) * f32::powf(tau, Constants::t[i]);
        }
        for i in 8..=51
        {
            phi_r_delta_delta += Constants::n[i] * f32::exp(-f32::powf(delta, Constants::c[i])) * (f32::powf(delta, Constants::d[i] - 2.0) * f32::powf(tau, Constants::t[i])
            * ((Constants::d[i] - Constants::c[i] * f32::powf(delta, Constants::c[i])) * (Constants::d[i] - 1.0 - Constants::c[i] * f32::powf(delta, Constants::c[i])) - Constants::c[i] * Constants::c[i] * f32::powf(delta, Constants::c[i])));
        }
        for i in 52..=54
        {
            phi_r_delta_delta += Constants::n[i] * f32::powf(tau, Constants::t[i]) * f32::exp(-Constants::alpha[i] * f32::powf(delta - Constants::epsilon[i], 2.0) - Constants::beta[i] * f32::powf(tau - Constants::gamma[i], 2.0))
            * (-2.0 * Constants::alpha[i] * f32::powf(delta, Constants::d[i])
                + 4.0 * Constants::alpha[i] * Constants::alpha[i] * f32::powf(delta, Constants::d[i]) * (delta - Constants::epsilon[i]) * (delta - Constants::epsilon[i])
                - 4.0 * Constants::d[i] * Constants::alpha[i] * f32::powf(delta, Constants::d[i] - 1.0) * (delta - Constants::epsilon[i])
                + Constants::d[i] * (Constants::d[i] - 1.0) * f32::powf(delta, Constants::d[i] - 2.0));
        }
        for i in 55..=56
        {
            // definition of theta, gdelta, psi
            let theta = (1.0 - tau) + Constants::A[i] * f32::powf((delta - 1.0) * (delta - 1.0), 1.0 / (2.0 * Constants::beta[i]));
            let gdelta = theta * theta + Constants::B[i] * f32::powf((delta - 1.0) * (delta - 1.0), Constants::a[i]);
            let psi = f32::exp(-Constants::C[i] * (delta - 1.0) * (delta - 1.0) - Constants::D[i] * (tau - 1.0) * (tau - 1.0));
            // definition of gdelta_delta, gdelta_bi_delta
            let gdelta_delta = (delta - 1.0) * (Constants::A[i] * theta * 2.0 / Constants::beta[i] * f32::powf((delta - 1.0) * (delta - 1.0), 1.0 / (2.0 * Constants::beta[i]) - 1.0)
                + 2.0 * Constants::B[i] * Constants::a[i] * f32::powf((delta - 1.0) * (delta - 1.0), Constants::a[i] - 1.0));
            let gdelta_delta_delta = 1.0 / (delta - 1.0) * gdelta_delta + (delta - 1.0) * (delta - 1.0)
                * (4.0 * Constants::B[i] * Constants::a[i] * (Constants::a[i] - 1.0) * f32::powf((delta - 1.0) * (delta - 1.0), Constants::a[i] - 2.0)
                + 2.0 * Constants::A[i] * Constants::A[i] * (1.0 / Constants::beta[i]) * (1.0 / Constants::beta[i]) * (f32::powf((delta - 1.0) * (delta - 1.0), 1.0 / (2.0 * Constants::beta[i]) - 1.0))
                      * (f32::powf((delta - 1.0) * (delta - 1.0), 1.0 / (2.0 * Constants::beta[i]) - 1.0))
                + Constants::A[i] * theta * 4.0 / Constants::beta[i] * (1.0 / (2.0 * Constants::beta[i]) - 1.0) * f32::powf((delta - 1.0) * (delta - 1.0), 1.0 / (2.0 * Constants::beta[i]) - 2.0));
            let gdelta_bi_delta = Constants::b[i] * f32::powf(gdelta, Constants::b[i] - 1.0) * gdelta_delta;
            let gdelta_bi_delta_delta = Constants::b[i] * (f32::powf(gdelta, Constants::b[i] - 1.0) * gdelta_delta_delta + (Constants::b[i] - 1.0) * f32::powf(gdelta, Constants::b[i] - 2.0) * gdelta_delta * gdelta_delta);
                // definition of psi_delta
            let psi_delta = -2.0 * Constants::C[i] * (delta - 1.0) * psi;
            let psi_delta_delta = (2.0 * Constants::C[i] * (delta - 1.0) * (delta - 1.0) - 1.0) * 2.0 * Constants::C[i] * psi;
            phi_r_delta_delta += Constants::n[i] * (f32::powf(gdelta, Constants::b[i])
                                                 * (2.0 * psi_delta + delta * psi_delta_delta) + 2.0 * gdelta_bi_delta
                                                 * (psi + delta * psi_delta) + gdelta_bi_delta_delta * delta * psi);
        }

        phi_r_delta_delta
    }

    fn calculate_tau(temperature: f32) -> f32
    {
        Constants::T_c / temperature
    }

    fn calculate_delta(density: f32) -> f32
    {
        density / Constants::rho_c
    }

    fn pressure_vapor_liquid(temperature: f32) -> f32
    {
        const ALV1: f32 =  -7.85951783;
        const ALV2: f32 =   1.84408259;
        const ALV3: f32 = -11.7866497;
        const ALV4: f32 =  22.6807411;
        const ALV5: f32 = -15.9618719;
        const ALV6: f32 =   1.80122502;

        let theta = 1.0 - temperature / Constants::T_c;

        let p_vl = Constants::P_c * f32::exp(Constants::T_c / temperature * (ALV1 * theta + ALV2 * f32::powf(theta, 1.5) + ALV3 * f32::powf(theta, 3.0)
                                  + ALV4 * f32::powf(theta, 3.5) + ALV5 * f32::powf(theta, 4.0) + ALV6 * f32::powf(theta, 7.5)));
        
        p_vl
    }

    // Calcualte the ideal part of phi_0_tau with temp and density
    fn calculate_phi_0_tau(temperature: f32,
                           density: f32) -> f32
    {
        let tau = H2OWagnerPruss::calculate_tau(temperature);

        let mut phi_0_tau = Constants::n_0[2] + Constants::n_0[3] / tau;

        for i in 4..=8
        {
            phi_0_tau += Constants::n_0[i] * Constants::gamma_0[i] * (1.0 / (1.0 - f32::exp(-Constants::gamma_0[i] * tau)) - 1.0);
        }

        phi_0_tau
    }

    // Calculate the residual part of phi_r_tau
    fn calculate_phi_r_tau(temperature: f32,
                           density: f32) -> f32
    {
        let mut phi_r_tau = 0.0;

        let tau = H2OWagnerPruss::calculate_tau(temperature);
        let delta = H2OWagnerPruss::calculate_delta(density);

        for i in 1..=7
        {
            phi_r_tau += Constants::n[i] * Constants::t[i] * f32::powf(delta, Constants::d[i]) * f32::powf(tau, Constants::t[i] - 1.0);
        }
        for i in 8..=51
        {
            phi_r_tau += Constants::n[i] * Constants::t[i] * f32::powf(delta, Constants::d[i]) * f32::powf(tau, Constants::t[i] - 1.0) * f32::exp(-f32::powf(delta, Constants::c[i]));
        }
        for i in 52..=54
        {
            phi_r_tau += Constants::n[i] * f32::powf(delta, Constants::d[i]) * f32::powf(tau, Constants::t[i]) * f32::exp(-Constants::alpha[i] * f32::powf(delta - Constants::epsilon[i], 2.0) - Constants::beta[i] * f32::powf(tau - Constants::gamma[i], 2.0))
            * (Constants::t[i] / tau - 2.0 * Constants::beta[i] * (tau - Constants::gamma[i]));
        }
        for i in 55..=56
        {
            // definition of theta, gdelta, psi
            let theta = (1.0 - tau) + Constants::A[i] * f32::powf((delta - 1.0) * (delta - 1.0), 1.0 / (2.0 * Constants::beta[i]));
            let gdelta = theta * theta + Constants::B[i] * f32::powf((delta - 1.0) * (delta - 1.0), Constants::a[i]);
            let psi = f32::exp(-Constants::C[i] * (delta - 1.0) * (delta - 1.0) - Constants::D[i] * (tau - 1.0) * (tau - 1.0));
            // definition gdelta_bi_tau, psi_tau
            let gdelta_bi_tau = -2.0 * theta * Constants::b[i] * f32::powf(gdelta, Constants::b[i] - 1.0);
            let psi_tau = -2.0 * Constants::D[i] * (tau - 1.0) * psi;
            phi_r_tau += Constants::n[i] * delta * (gdelta_bi_tau * psi + f32::powf(gdelta, Constants::b[i]) * psi_tau);
        }

        phi_r_tau
    }
}

#[cfg(test)]
mod tests;