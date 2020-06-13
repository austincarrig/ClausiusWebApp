// ***************************************
//                SOLVER
// ***************************************

/*
 * DESCRIPTION:
 * This class is used to calculate temperature given a 
 * specific volume and pressure.
 * Converted from Obj-C code 11/9/2019
 * Converted from C# code 6/2/2020
 * 
 */
/*
class Solver {
    static M_Water = 0.01801528; // Molar mass of water expressed in kg/mol
    static T_CR    = 647.0;      // Critical temperature in K
    static P_CR    = 22100.0;    // Critical pressure in kPA
    static R_S     = 0.48;       // Gas constant in kJ/(kg.K)

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

    static CalculatePressure(specVol,
                             temp)
    {
        // This function doesn't work right now...
        let Vm = Solver.M_Water / specVol; // Molar volume in m³/mol

        let a = 0.4278 * Math.pow(Solver.R_S, 2) * Math.pow(Solver.T_CR, 2.5) / Solver.P_CR;

        let b = 0.0867 * Solver.R_S * Solver.T_CR / Solver.P_CR;

        let pressure = (Solver.R_S * temp / (Vm - b)) - (a / (Vm * (Vm + b) * Math.sqrt(temp)));

        return pressure;
    }

    // Solved using the Relich-Kwong equation of state
    static CalculateTemperature(specVol,
                                pressure)
    {
        let Vm = Solver.M_Water * specVol; // Molar volume in m³/mol

        let a = 0.4275 * Math.pow(Solver.R_S, 2) * Math.pow(Solver.T_CR, 2.5) / Solver.P_CR;

        let b = 0.08664 * Solver.R_S * Solver.T_CR / Solver.P_CR;

        let g = Solver.R_S / (Vm - b);

        let h = a / (Vm * (Vm + b));

        let c = Math.pow(27.0 * Math.pow(g, 4.0) * Math.pow(h, 2.0) - 2.0 * Math.pow(g, 3.0) * Math.pow(pressure, 3.0) + 3.0 * Math.sqrt(3.0) * Math.sqrt(27.0 * Math.pow(g, 8.0) * Math.pow(h, 4.0) - 4.0 * Math.pow(g, 7.0) * Math.pow(h, 2.0) * Math.pow(pressure, 3.0)), 1.0 / 3.0);

        let temp = (1.0 / 3.0) * (Math.pow(2.0, 1.0 / 3.0) * Math.pow(pressure, 2.0) / c + c / (Math.pow(2.0, 1.0 / 3.0) * Math.pow(g, 2.0)) + 2.0 * pressure / g);

        return temp;
    }
}
*/
// ***************************************
//            H2O Wagner Pruss
// ***************************************

/*
 * CLASS: H20WagnerPruss
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
/*
class H2OWagnerPruss
{
    #extrapolationMaxDensity;

    #Tc
    #rhoc
    #Pc
    #R
    #n_0
    #gamma_0

    #t
    #n

    #alpha
    #beta
    #gamma
    #epsilon

    #a
    #b
    #c
    #d

    #A
    #B
    #C
    #D

    #coVolume

    constructor()
    {
        this.#extrapolationMaxDensity = 2500.0; // kg/m^3

        this.#Tc = 647.096;    // K
        this.#rhoc = 322.0;      // kg/m^3
        this.#Pc = 22.064;     // MPa
        this.#R = 0.46151805; // kJ/kg/K Rm/M

        const M = 18.015268;  // g/mol
        this.#coVolume = 30.49 / (M * 1000.0); // m^3/kg

        this.#n_0 = [];
        this.#n_0[1] = -8.32044648201;
        this.#n_0[2] = 6.6832105268;
        this.#n_0[3] = 3.00632;
        this.#n_0[4] = 0.012436;
        this.#n_0[5] = 0.97315;
        this.#n_0[6] = 1.27950;
        this.#n_0[7] = 0.96956;
        this.#n_0[8] = 0.24873;

        this.#gamma_0 = [];
        this.#gamma_0[4] = 1.28728967;
        this.#gamma_0[5] = 3.53734222;
        this.#gamma_0[6] = 7.74073708;
        this.#gamma_0[7] = 9.24437796;
        this.#gamma_0[8] = 27.5075105;

        this.#beta = [];
        this.#beta[52] = 150.0;
        this.#beta[53] = 150.0;
        this.#beta[54] = 250.0;
        this.#beta[55] = 0.3;
        this.#beta[56] = 0.3;

        this.#gamma = [];
        this.#gamma[52] = 1.21;
        this.#gamma[53] = 1.21;
        this.#gamma[54] = 1.25;

        this.#alpha = [];
        this.#alpha[52] = 20.0;
        this.#alpha[53] = 20.0;
        this.#alpha[54] = 20.0;

        this.#epsilon = [];
        this.#epsilon[52] = 1.0;
        this.#epsilon[53] = 1.0;
        this.#epsilon[54] = 1.0;

        this.#a = [];
        this.#a[55] = 3.5;
        this.#a[56] = 3.5;

        this.#b = [];
        this.#b[55] = 0.85;
        this.#b[56] = 0.95;

        this.#A = [];
        this.#B = [];
        this.#C = [];
        this.#D = [];
        this.#A[55] = 0.32;
        this.#A[56] = 0.32;
        this.#B[55] = 0.2;
        this.#B[56] = 0.2;
        this.#C[55] = 28.0;
        this.#C[56] = 32.0;
        this.#D[55] = 700.0;
        this.#D[56] = 800.0;

        this.#c = [];
        for (let i = 8; i <= 22; ++i) this.#c[i] = 1.0;
        for (let i = 23; i <= 42; ++i) this.#c[i] = 2.0;
        for (let i = 43; i <= 46; ++i) this.#c[i] = 3.0;

        this.#c[47] = 4.0;

        for (let i = 48; i <= 51; ++i) this.#c[i] = 6.0;

        this.#d = [];
        this.#d[1] = 1.0;
        this.#d[2] = 1.0;
        this.#d[3] = 1.0;
        this.#d[4] = 2.0;
        this.#d[5] = 2.0;
        this.#d[6] = 3.0;
        this.#d[7] = 4.0;
        this.#d[8] = 1.0;
        this.#d[9] = 1.0;
        this.#d[10] = 1.0;
        this.#d[11] = 2.0;
        this.#d[12] = 2.0;
        this.#d[13] = 3.0;
        this.#d[14] = 4.0;
        this.#d[15] = 4.0;
        this.#d[16] = 5.0;
        this.#d[17] = 7.0;
        this.#d[18] = 9.0;
        this.#d[19] = 10.0;
        this.#d[20] = 11.0;
        this.#d[21] = 13.0;
        this.#d[22] = 15.0;
        this.#d[23] = 1.0;
        this.#d[24] = 2.0;
        this.#d[25] = 2.0;
        this.#d[26] = 2.0;
        this.#d[27] = 3.0;
        this.#d[28] = 4.0;
        this.#d[29] = 4.0;
        this.#d[30] = 4.0;
        this.#d[31] = 5.0;
        this.#d[32] = 6.0;
        this.#d[33] = 6.0;
        this.#d[34] = 7.0;
        this.#d[35] = 9.0;
        this.#d[36] = 9.0;
        this.#d[37] = 9.0;
        this.#d[38] = 9.0;
        this.#d[39] = 9.0;
        this.#d[40] = 10.0;
        this.#d[41] = 10.0;
        this.#d[42] = 12.0;
        this.#d[43] = 3.0;
        this.#d[44] = 4.0;
        this.#d[45] = 4.0;
        this.#d[46] = 5.0;
        this.#d[47] = 14.0;
        this.#d[48] = 3.0;
        this.#d[49] = 6.0;
        this.#d[50] = 6.0;
        this.#d[51] = 6.0;
        this.#d[52] = 3.0;
        this.#d[53] = 3.0;
        this.#d[54] = 3.0;

        this.#t = [];
        this.#t[1] = -0.5;
        this.#t[2] = 0.875;
        this.#t[3] = 1.0;
        this.#t[4] = 0.5;
        this.#t[5] = 0.75;
        this.#t[6] = 0.375;
        this.#t[7] = 1.0;
        this.#t[8] = 4.0;
        this.#t[9] = 6.0;
        this.#t[10] = 12.0;
        this.#t[11] = 1.0;
        this.#t[12] = 5.0;
        this.#t[13] = 4.0;
        this.#t[14] = 2.0;
        this.#t[15] = 13.0;
        this.#t[16] = 9.0;
        this.#t[17] = 3.0;
        this.#t[18] = 4.0;
        this.#t[19] = 11.0;
        this.#t[20] = 4.0;
        this.#t[21] = 13.0;
        this.#t[22] = 1.0;
        this.#t[23] = 7.0;
        this.#t[24] = 1.0;
        this.#t[25] = 9.0;
        this.#t[26] = 10.0;
        this.#t[27] = 10.0;
        this.#t[28] = 3.0;
        this.#t[29] = 7.0;
        this.#t[30] = 10.0;
        this.#t[31] = 10.0;
        this.#t[32] = 6.0;
        this.#t[33] = 10.0;
        this.#t[34] = 10.0;
        this.#t[35] = 1.0;
        this.#t[36] = 2.0;
        this.#t[37] = 3.0;
        this.#t[38] = 4.0;
        this.#t[39] = 8.0;
        this.#t[40] = 6.0;
        this.#t[41] = 9.0;
        this.#t[42] = 8.0;
        this.#t[43] = 16.0;
        this.#t[44] = 22.0;
        this.#t[45] = 23.0;
        this.#t[46] = 23.0;
        this.#t[47] = 10.0;
        this.#t[48] = 50.0;
        this.#t[49] = 44.0;
        this.#t[50] = 46.0;
        this.#t[51] = 50.0;
        this.#t[52] = 0.0;
        this.#t[53] = 1.0;
        this.#t[54] = 4.0;

        this.#n = [];
        this.#n[1] = 0.12533547935523e-1;
        this.#n[2] = 0.78957634722828e1;
        this.#n[3] = -0.87803203303561e1;
        this.#n[4] = 0.31802509345418;
        this.#n[5] = -0.26145533859358;
        this.#n[6] = -0.78199751687981e-2;
        this.#n[7] = 0.88089493102134e-2;
        this.#n[8] = -0.66856572307965;
        this.#n[9] = 0.20433810950965;
        this.#n[10] = -0.66212605039687e-4;
        this.#n[11] = -0.19232721156002;
        this.#n[12] = -0.25709043003438;
        this.#n[13] = 0.16074868486251;
        this.#n[14] = -0.40092828925807e-1;
        this.#n[15] = 0.39343422603254e-6;
        this.#n[16] = -0.75941377088144e-5;
        this.#n[17] = 0.56250979351888e-3;
        this.#n[18] = -0.15608652257135e-4;
        this.#n[19] = 0.11537996422951e-8;
        this.#n[20] = 0.36582165144204e-6;
        this.#n[21] = -0.13251180074668e-11;
        this.#n[22] = -0.62639586912454e-9;
        this.#n[23] = -0.10793600908932;
        this.#n[24] = 0.17611491008752e-1;
        this.#n[25] = 0.22132295167546;
        this.#n[26] = -0.40247669763528;
        this.#n[27] = 0.58083399985759;
        this.#n[28] = 0.49969146990806e-2;
        this.#n[29] = -0.31358700712549e-1;
        this.#n[30] = -0.74315929710341;
        this.#n[31] = 0.47807329915480;
        this.#n[32] = 0.20527940895948e-1;
        this.#n[33] = -0.13636435110343;
        this.#n[34] = 0.14180634400617e-1;
        this.#n[35] = 0.83326504880713e-2;
        this.#n[36] = -0.29052336009585e-1;
        this.#n[37] = 0.38615085574206e-1;
        this.#n[38] = -0.20393486513704e-1;
        this.#n[39] = -0.16554050063734e-2;
        this.#n[40] = 0.19955571979541e-2;
        this.#n[41] = 0.15870308324157e-3;
        this.#n[42] = -0.16388568342530e-4;
        this.#n[43] = 0.43613615723811e-1;
        this.#n[44] = 0.34994005463765e-1;
        this.#n[45] = -0.76788197844621e-1;
        this.#n[46] = 0.22446277332006e-1;
        this.#n[47] = -0.62689710414685e-4;
        this.#n[48] = -0.55711118565645e-9;
        this.#n[49] = -0.19905718354408;
        this.#n[50] = 0.31777497330738;
        this.#n[51] = -0.11841182425981;
        this.#n[52] = -0.31306260323435e2;
        this.#n[53] = 0.31546140237781e2;
        this.#n[54] = -0.25213154341695e4;
        this.#n[55] = -0.14874640856724;
        this.#n[56] = 0.31806110878444;
    }

    CalculateDensity(temperature,
                     pressure)
    {
        let rho, oldRho, rhoMin, logRho, rhoMax, logRhoIncrement;
        let fRho, oldfRho;
        let signfRho;
        let newRho;

        let arrayOfRho = [];

        rhoMin = pressure / (2.0 * ((coVolume * pressure) + (R / 1000.0 * temperature)));

        rhoMax = extrapolationMaxDensity;

        logRhoIncrement = (Math.log(rhoMax) - Math.log(rhoMin)) / 200.0;

        logRho = Math.log(rhoMin);

        oldRho = rhoMin;

        oldfRho = rhoMin / 1000.0 * R * temperature * (1.0 + rhoMin / rhoc * CalculatePhi_r_delta(temperature, rhoMin)) - pressure;

        while (logRho < Math.log(rhoMax))
        {
            logRho += logRhoIncrement;
            rho = Math.exp(logRho);

            fRho = rho / 1000.0 * R * temperature * (1.0 + rho / rhoc * CalculatePhi_r_delta(temperature, rho)) - pressure;

            signfRho = oldfRho * fRho;

            if (signfRho < 0.0)
            {
                newRho = FindRhoWithEstimatedDensity(temperature,
                                                     pressure,
                                                    (oldRho + rho) / 2.0);

                arrayOfRho.append(newRho);
            }

            oldfRho = fRho;
            oldRho = rho;
        }

        // If there is only 1 root, then this is the solution
        if (arrayOfRho.Count == 1)
        {
            return arrayOfRho[0];
        }

        // Temperature above critical point; pressure above, at or below critical point 
        if (temperature > Tc)
        {
            return arrayOfRho[arrayOfRho.length - 1];
        }

        const pSat = PressureVapourLiquid(temperature);

        // Temperature below critical point; pressure below vapor-liquid pressure
        if (temperature <= Tc && pressure >= pSat)
        {
            return arrayOfRho[arrayOfRho.length - 1];
        }

        // Temperature below critical point; pressure above vapor-liquid pressure
        if (temperature <= Tc && pressure < pSat)
        {
            return arrayOfRho[0];
        }

        console.log("Error");
        // Error return
        return -1.0;
    }

    CalculateInternalEnergy(temperature,
                            density)
    {
        const tau = CalculateTau(temperature);

        const phi_0_tau = CalculatePhi_0_tau(temperature,
                                             density);

        const phi_r_tau = CalculatePhi_r_tau(temperature,
                                             density);

        const internalEnergy = R * temperature * tau * (phi_0_tau + phi_r_tau);

        return internalEnergy; // kJ/kg
    }

    CalculateEnthalpy(temperature,
                      density)
    {
        const tau = CalculateTau(temperature);
        const delta = CalculateDelta(density);

        const phi_r_delta = CalculatePhi_r_delta(temperature,
                                                 density);

        const phi_0_tau = CalculatePhi_0_tau(temperature,
                                             density);

        const phi_r_tau = CalculatePhi_r_tau(temperature,
                                             density);

        const extra = R * temperature * delta * phi_r_delta;

        const intEnergy = R * temperature * tau * (phi_0_tau + phi_r_tau);

        const enthalpy = R * temperature + intEnergy + extra;

        return enthalpy; // kJ/kg
    }

    // Units: MPa
    CalculatePressure(temperature,
                      density)
    {
        const delta = CalculateDelta(density);
        const rho = delta * rhoc;
        const phi_r_delta = CalculatePhi_r_delta(temperature,
                                                 density);

        return rho * R * temperature * (1.0 + (delta * phi_r_delta)) / 1000.0; // MPa
    }

    FindRhoWithEstimatedDensity(temperature,
                                pressure,
                                densityEstimate)
    {
        let delta = 0.0, dDelta = 0.0;
        let fRho = 0.0, dfRho = 0.0;

        let rho = densityEstimate;

        let iterationGoal = 1.0e-12;
        const maxIteration = 1000;

        let i = 1;

        do
        {
            fRho = rhoc / 1000.0 * R * temperature * (delta + Math.pow(delta, 2.0) * CalculatePhi_r_delta(temperature, rho)) - pressure;
            dfRho = rhoc / 1000.0 * R * temperature * (1.0 + 2.0 * delta * CalculatePhi_r_delta(temperature, rho)
                                                + Math.pow(delta, 2.0) * CalculatePhi_r_delta_delta(temperature, rho));

            dDelta = -fRho / dfRho;
            delta += dDelta;

            rho = delta * rhoc;

            i++;

            if (i > maxIteration)
            {
                // TODO: Output error
                break;
            }

        } while (Math.Abs(dDelta) > iterationGoal);

        return rho;
    }

    CalculatePhi_0(temperature,
                   density)
    {
        const tau = CalculateTau(temperature);
        const delta = CalculateDelta(density);

        const phi_0 = Math.log(delta) + n_0[1] + n_0[2] * tau + n_0[3] * Math.log(tau);
        for (let i = 4; i <= 8; ++i)
        {
            phi_0 = phi_0 + n_0[i] * Math.log(1.0 - Math.exp(-gamma_0[i] * tau));
        }

        return phi_0;
    }

    CalculatePhi_r(temperature,
                   density)
    {
        let phi_r, gdelta, theta, psi;

        const tau = CalculateTau(temperature);
        const delta = CalculateDelta(density);

        phi_r = 0.0;
        for (let i = 1; i <= 7; ++i)
        {
            phi_r = phi_r + n[i] * Math.pow(delta, d[i]) * Math.pow(tau, t[i]);
        }
        for (let i = 8; i <= 51; ++i)
        {
            phi_r = phi_r + n[i] * Math.pow(delta, d[i]) * Math.pow(tau, t[i]) * Math.exp(-Math.pow(delta, c[i]));
        }
        for (let i = 52; i <= 54; ++i)
        {
            phi_r = phi_r + n[i] * Math.pow(delta, d[i]) * Math.pow(tau, t[i]) * Math.exp(-alpha[i] * Math.pow(delta - epsilon[i], 2.0) - beta[i] * Math.pow(tau - gamma[i], 2.0));
        }
        for (let i = 55; i <= 56; ++i)
        {
            // definition of theta, gdelta, psi
            theta = (1.0 - tau) + A[i] * Math.pow((delta - 1.0) * (delta - 1.0), 1.0 / (2.0 * beta[i]));
            gdelta = theta * theta + B[i] * Math.pow((delta - 1.0) * (delta - 1.0), a[i]);
            psi = Math.exp(-C[i] * (delta - 1.0) * (delta - 1.0) - D[i] * (tau - 1.0) * (tau - 1.0));
            phi_r = phi_r + n[i] * Math.pow(gdelta, b[i]) * delta * psi;
        }

        return phi_r;
    }

    CalculatePhi_r_delta(temperature,
                         density)
    {
        let phi_r_delta = 0.0;
        let gdelta = 0.0, theta = 0.0, psi = 0.0;
        let gdelta_bi_delta = 0.0, gdelta_delta = 0.0, psi_delta = 0.0;

        const tau = CalculateTau(temperature);
        const delta = CalculateDelta(density);

        for (let i = 1; i <= 7; ++i)
        {
            phi_r_delta += n[i] * d[i] * Math.pow(delta, d[i] - 1.0) * Math.pow(tau, t[i]);
        }
        for (let i = 8; i <= 51; ++i)
        {
            phi_r_delta += n[i] * Math.exp(-Math.pow(delta, c[i])) * (Math.pow(delta, d[i] - 1.0) * Math.pow(tau, t[i]) * (d[i] - c[i] * Math.pow(delta, c[i])));
        }
        for (let i = 52; i <= 54; ++i)
        {
            phi_r_delta += n[i] * Math.pow(delta, d[i]) * Math.pow(tau, t[i]) * Math.exp(-alpha[i] * Math.pow(delta - epsilon[i], 2.0) - beta[i] * Math.pow(tau - gamma[i], 2.0))
            * (d[i] / delta - 2.0 * alpha[i] * (delta - epsilon[i]));
        }
        for (let i = 55; i <= 56; ++i)
        {
            // definition of theta, gdelta, psi
            theta = (1.0 - tau) + A[i] * Math.pow((delta - 1.0) * (delta - 1.0), 1.0 / (2.0 * beta[i]));
            gdelta = theta * theta + B[i] * Math.pow((delta - 1.0) * (delta - 1.0), a[i]);
            psi = Math.exp(-C[i] * (delta - 1.0) * (delta - 1.0) - D[i] * (tau - 1.0) * (tau - 1.0));
            // definition of gdelta_delta, gdelta_bi_delta
            gdelta_delta = (delta - 1.0) * (A[i] * theta * 2.0 / beta[i] * Math.pow((delta - 1.0) * (delta - 1.0), 1.0 / (2.0 * beta[i]) - 1.0)
                            + 2.0 * B[i] * a[i] * Math.pow((delta - 1.0) * (delta - 1.0), a[i] - 1.0));
            gdelta_bi_delta = b[i] * Math.pow(gdelta, b[i] - 1.0) * gdelta_delta;
            // definition of psi_delta
            psi_delta = -2.0 * C[i] * (delta - 1.0) * psi;
            phi_r_delta += n[i] * (Math.pow(gdelta, b[i]) * (psi + delta * psi_delta) + gdelta_bi_delta * delta * psi);
        }

        return phi_r_delta;
    }

    CalculatePhi_r_delta_delta(temperature,
                               density)
    {
        let phi_r_delta_delta = 0.0;
        let gdelta = 0.0, theta = 0.0, psi = 0.0;
        let gdelta_delta = 0.0, gdelta_delta_delta = 0.0, gdelta_bi_delta = 0.0, gdelta_bi_delta_delta = 0.0;
        let psi_delta = 0.0, psi_delta_delta = 0.0;

        const tau = CalculateTau(temperature);
        const delta = CalculateDelta(density);

        for (let i = 1; i <= 7; ++i)
        {
            phi_r_delta_delta += n[i] * d[i] * (d[i] - 1.0) * Math.pow(delta, d[i] - 2.0) * Math.pow(tau, t[i]);
        }
        for (let i = 8; i <= 51; ++i)
        {
            phi_r_delta_delta += n[i] * Math.exp(-Math.pow(delta, c[i])) * (Math.pow(delta, d[i] - 2.0) * Math.pow(tau, t[i])
            * ((d[i] - c[i] * Math.pow(delta, c[i])) * (d[i] - 1.0 - c[i] * Math.pow(delta, c[i])) - c[i] * c[i] * Math.pow(delta, c[i])));
        }
        for (let i = 52; i <= 54; ++i)
        {
            phi_r_delta_delta += n[i] * Math.pow(tau, t[i]) * Math.exp(-alpha[i] * Math.pow(delta - epsilon[i], 2.0) - beta[i] * Math.pow(tau - gamma[i], 2.0))
            * (-2.0 * alpha[i] * Math.pow(delta, d[i])
                + 4.0 * alpha[i] * alpha[i] * Math.pow(delta, d[i]) * (delta - epsilon[i]) * (delta - epsilon[i])
                - 4.0 * d[i] * alpha[i] * Math.pow(delta, d[i] - 1.0) * (delta - epsilon[i])
                + d[i] * (d[i] - 1.0) * Math.pow(delta, d[i] - 2.0));
        }
        for (let i = 55; i <= 56; ++i)
        {
            // definition of theta, gdelta, psi
            theta = (1.0 - tau) + A[i] * Math.pow((delta - 1.0) * (delta - 1.0), 1.0 / (2.0 * beta[i]));
            gdelta = theta * theta + B[i] * Math.pow((delta - 1.0) * (delta - 1.0), a[i]);
            psi = Math.exp(-C[i] * (delta - 1.0) * (delta - 1.0) - D[i] * (tau - 1.0) * (tau - 1.0));
            // definition of gdelta_delta, gdelta_bi_delta, gdelta_delta_delta, gdelta_bi_delta_delta
            gdelta_delta = (delta - 1.0) * (A[i] * theta * 2.0 / beta[i] * Math.pow((delta - 1.0) * (delta - 1.0), 1.0 / (2.0 * beta[i]) - 1.0)
                                            + 2.0 * B[i] * a[i] * Math.pow((delta - 1.0) * (delta - 1.0), a[i] - 1.0));
            gdelta_delta_delta = 1.0 / (delta - 1.0) * gdelta_delta + (delta - 1.0) * (delta - 1.0)
                                    * (4.0 * B[i] * a[i] * (a[i] - 1.0) * Math.pow((delta - 1.0) * (delta - 1.0), a[i] - 2.0)
                                    + 2.0 * A[i] * A[i] * (1.0 / beta[i]) * (1.0 / beta[i]) * (Math.pow((delta - 1.0) * (delta - 1.0), 1.0 / (2.0 * beta[i]) - 1.0))
                                            * (Math.pow((delta - 1.0) * (delta - 1.0), 1.0 / (2.0 * beta[i]) - 1.0))
                                    + A[i] * theta * 4.0 / beta[i] * (1.0 / (2.0 * beta[i]) - 1.0) * Math.pow((delta - 1.0) * (delta - 1.0), 1.0 / (2.0 * beta[i]) - 2.0));
            gdelta_bi_delta = b[i] * Math.pow(gdelta, b[i] - 1.0) * gdelta_delta;
            gdelta_bi_delta_delta = b[i] * (Math.pow(gdelta, b[i] - 1.0) * gdelta_delta_delta + (b[i] - 1.0) * Math.pow(gdelta, b[i] - 2.0) * gdelta_delta * gdelta_delta);
            // definition of psi_delta, psi_delta_delta
            psi_delta = -2.0 * C[i] * (delta - 1.0) * psi;
            psi_delta_delta = (2.0 * C[i] * (delta - 1.0) * (delta - 1.0) - 1.0) * 2.0 * C[i] * psi;
            phi_r_delta_delta += n[i] * (Math.pow(gdelta, b[i])
                                        * (2.0 * psi_delta + delta * psi_delta_delta) + 2.0 * gdelta_bi_delta
                                        * (psi + delta * psi_delta) + gdelta_bi_delta_delta * delta * psi);
        }

        return phi_r_delta_delta;
    }

    CalculateTau(temperature)
    {
        return Tc / temperature;
    }

    CalculateDelta(density)
    {
        return density / rhoc;
    }

    // Leaving for use maybe someday. Right now, huge pain in the ass for little gain
    AccuratePressureVapourLiquid(temperature)
    {
        let pNew, rhoNew;
        let f, df, deltaRho, deltaP = 1.0e+99;

        let delta, phi_r_delta, phi_r_delta_delta;

        // Calculate preliminary values for pPreliminary, rhoLiquidPreliminary and rhoVaporPreliminary
        const pPreliminary = PressureVapourLiquid(temperature);
        const rhoLiquidPreliminary = LiquidDensityAtSaturation(temperature);
        const rhoVaporPreliminary = VaporDensityAtSaturation(temperature);

        // Define Goals
        const deltaPGoal = pPreliminary / (1.0e+9);
        const deltaRhoLiquidGoal = rhoLiquidPreliminary / (1.0e+9);
        const deltaRhoVaporGoal = rhoVaporPreliminary / (1.0e+9);

        let i = 0;

        while (deltaP > deltaPGoal)
        {
            if (i >= 250) // Used to be 1000
            {
                // Output Error
                break;
            }

            i++;

            let j = 0;
            deltaRho = 1.0e+99;

            while (deltaRho > deltaRhoLiquidGoal)
            {
                if (j >= 1000) // Used to be 10000
                {
                    // Output Error
                    break;
                }

                j++;

                f = CalculatePressure(temperature,
                                        rhoLiquidPreliminary) - pPreliminary;

                delta = CalculateDelta(rhoLiquidPreliminary);
                phi_r_delta = CalculatePhi_r_delta(temperature,
                                                            rhoLiquidPreliminary);
                phi_r_delta_delta = CalculatePhi_r_delta_delta(temperature,
                                                                rhoLiquidPreliminary);

                df = R * temperature * (1.0 + 2.0 * delta * phi_r_delta + delta * delta * phi_r_delta_delta) / 1000.0;

                rhoNew = rhoLiquidPreliminary - f / df / 2.0;

                deltaRho = Math.Abs(rhoLiquidPreliminary - rhoNew);

                rhoLiquidPreliminary = rhoNew;
            }

            j = 0;
            deltaRho = 1.0e+99;

            while (deltaRho > deltaRhoVaporGoal) // Used to be 10000
            {
                if (j >= 1000)
                {
                    // Output Error
                    break;
                }

                j++;

                f = CalculatePressure(temperature,
                                        rhoVaporPreliminary) - pPreliminary;

                delta = CalculateDelta(rhoVaporPreliminary);
                phi_r_delta = CalculatePhi_r_delta(temperature,
                                                            rhoVaporPreliminary);
                phi_r_delta_delta = CalculatePhi_r_delta_delta(temperature,
                                                                rhoVaporPreliminary);

                df = R * temperature * (1.0 + 2.0 * delta * phi_r_delta + delta * delta * phi_r_delta_delta) / 1000.0;

                rhoNew = rhoVaporPreliminary - f / df / 2.0;

                deltaRho = Math.Abs(rhoVaporPreliminary - rhoNew);

                rhoVaporPreliminary = rhoNew;
            }

            pNew = -((CalculateHelmholtzFreeEnergy(temperature, rhoLiquidPreliminary) -
                    CalculateHelmholtzFreeEnergy(temperature, rhoVaporPreliminary)) /
                    (1.0 / rhoLiquidPreliminary - 1.0 / rhoVaporPreliminary)) / 1.0e6;

            if (pNew.IsNaN())
            {
                break;
            }

            deltaP = Math.Abs(pPreliminary - pNew);

            pPreliminary = pNew;
        }

        let output = [];

        output.append(pPreliminary);
        output.append(rhoLiquidPreliminary);
        output.append(rhoVaporPreliminary);

        return output;
    }

    PressureVapourLiquid(temperature)
    {
        const aLV1 = -7.85951783;
        const aLV2 = 1.84408259;
        const aLV3 = -11.7866497;
        const aLV4 = 22.6807411;
        const aLV5 = -15.9618719;
        const aLV6 = 1.80122502;

        const theta = 1.0 - temperature / Tc;

        const pLV = Pc * Math.exp(Tc / temperature * (aLV1 * theta + aLV2 * Math.pow(theta, 1.5) + aLV3 * Math.pow(theta, 3.0)
                                                    + aLV4 * Math.pow(theta, 3.5) + aLV5 * Math.pow(theta, 4.0) + aLV6 * Math.pow(theta, 7.5)));

        return pLV;
    }

    LiquidDensityAtSaturation(temperature)
    {
        const bSL1 = 1.99274064;
        const bSL2 = 1.09965342;
        const bSL3 = -0.510839303;
        const bSL4 = -1.75493479;
        const bSL5 = -45.5170352;
        const bSL6 = -6.74694450e5;

        const theta = 1.0 - temperature / Tc;

        const rhoLiquid = rhoc * (1.0 + bSL1 * Math.pow(theta, 1.0 / 3.0)
                                      + bSL2 * Math.pow(theta, 2.0 / 3.0)
                                      + bSL3 * Math.pow(theta, 5.0 / 3.0)
                                      + bSL4 * Math.pow(theta, 16.0 / 3.0)
                                      + bSL5 * Math.pow(theta, 43.0 / 3.0)
                                      + bSL6 * Math.pow(theta, 110.0 / 3.0));

        return rhoLiquid;
    }

    VaporDensityAtSaturation(temperature)
    {
        const cSV1 = 1.99274064;
        const cSV2 = 1.09965342;
        const cSV3 = -0.510839303;
        const cSV4 = -1.75493479;
        const cSV5 = -45.5170352;
        const cSV6 = -6.74694450e5;

        const theta = 1.0 - temperature / Tc;

        const rhoVapor = rhoc * Math.exp(cSV1 * Math.pow(theta, 1.0 / 3.0)
                                       + cSV2 * Math.pow(theta, 2.0 / 3.0)
                                       + cSV3 * Math.pow(theta, 4.0 / 3.0)
                                       + cSV4 * Math.pow(theta, 3.0)
                                       + cSV5 * Math.pow(theta, 37.0 / 6.0)
                                       + cSV6 * Math.pow(theta, 71.0 / 6.0));

        return rhoVapor;
    }

    // Return Helmholtz free energy, Units: J/kg
    CalculateHelmholtzFreeEnergy(temperature,
                                 density)
    {
        const phi_0 = CalculatePhi_0(temperature,
                                     density);
        const phi_r = CalculatePhi_r(temperature,
                                     density);

        return R * temperature * (phi_0 + phi_r) * 1000.0; // J/kg
    }

    // Calcualte the ideal part of phi_0_tau with temp and density
    CalculatePhi_0_tau(temperature,
                       density)
    {
        const tau = CalculateTau(temperature);

        let phi_0_tau = n_0[2] + n_0[3] / tau;

        for (let i = 4; i <= 8; i++)
        {
            phi_0_tau += n_0[i] * gamma_0[i] * (1.0 / (1.0 - Math.exp(-gamma_0[i] * tau)) - 1.0);
        }

        return phi_0_tau;
    }

    // Calculate the residual part of phi_r_tau
    CalculatePhi_r_tau(temperature,
                       density)
    {
        let gdelta, theta, psi;
        let gdelta_bi_tau, psi_tau;

        const tau = CalculateTau(temperature);
        const delta = CalculateDelta(density);

        let phi_r_tau = 0.0;

        for (let i = 1; i <= 7; ++i)
        {
            phi_r_tau += n[i] * t[i] * Math.pow(delta, d[i]) * Math.pow(tau, t[i] - 1.0);
        }
        for (let i = 8; i <= 51; ++i)
        {
            phi_r_tau += n[i] * t[i] * Math.pow(delta, d[i]) * Math.pow(tau, t[i] - 1.0) * Math.exp(-Math.pow(delta, c[i]));
        }
        for (let i = 52; i <= 54; ++i)
        {
            phi_r_tau += n[i] * Math.pow(delta, d[i]) * Math.pow(tau, t[i]) * Math.exp(-alpha[i] * Math.pow(delta - epsilon[i], 2.0) - beta[i] * Math.pow(tau - gamma[i], 2.0))
                    * (t[i] / tau - 2.0 * beta[i] * (tau - gamma[i]));
        }
        for (let i = 55; i <= 56; ++i)
        {
            // definition of theta, gdelta, psi
            theta = (1.0 - tau) + A[i] * Math.pow((delta - 1.0) * (delta - 1.0), 1.0 / (2.0 * beta[i]));
            gdelta = theta * theta + B[i] * Math.pow((delta - 1.0) * (delta - 1.0), a[i]);
            psi = Math.exp(-C[i] * (delta - 1.0) * (delta - 1.0) - D[i] * (tau - 1.0) * (tau - 1.0));
            // definition gdelta_bi_tau, psi_tau
            gdelta_bi_tau = -2.0 * theta * b[i] * Math.pow(gdelta, b[i] - 1.0);
            psi_tau = -2.0 * D[i] * (tau - 1.0) * psi;
            phi_r_tau += n[i] * delta * (gdelta_bi_tau * psi + Math.pow(gdelta, b[i]) * psi_tau);
        }

        return phi_r_tau;
    }
}

function TestUseCase1()
{
    const temperature = 449.052546574519;
    const entropy = 6.23367919921875;

    const pressure = SuperheatedRegionCalculator.CalculatePressure(temperature,
                                                                   entropy);

    const temperatureKelvin = temperature + 273.15; // temperature (C -> K)
    const pressureMPa = pressure / 1000.0; // pressure (kPA -> MPa)

    let wagPruss = new H2OWagnerPruss();

    const density = wagPruss.CalculateDensity(temperatureKelvin,
                                              pressureMPa);

    console.log("test 1")
}

function TestH2OWagnerPruss() {
    TestUseCase1()
}

function TestRedlichKwongSolver() {
    const M_Water = Solver.M_Water; // Molar mass of water expressed in kg/mol
    const CM2M = 1.0 / 100.0;       // Conversion from centimeters to meters, 1/100
    const temp = 294.4;             // T in Kelvin
    const molVol = 50.0;            // Molar volume in cm³/mol

    const specVol = 0.022//(molVol / M_Water) * Math.pow(CM2M, 3)

    const pressure = Solver.CalculatePressure(specVol, temp);

    const expectedPressure = 26.06 * 1000.0

    console.log(pressure)
    console.log(expectedPressure)
}

function Test() {
    TestRedlichKwongSolver()
    TestH2OWagnerPruss()
}
*/