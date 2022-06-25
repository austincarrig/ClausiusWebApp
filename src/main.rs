#![allow(dead_code)]
#![allow(unused_variables)]

use std::env;

mod plot_point;
mod graph;
mod calculator;

use calculator::h2o_wagner_pruss::H2OWagnerPruss;

fn main() {

    let args: Vec<String> = env::args().collect();

    let t = &args[1].parse::<f32>().unwrap();
    let p = &args[2].parse::<f32>().unwrap();

    let density = H2OWagnerPruss::calculate_density(*t, *p);

    let enthalpy = H2OWagnerPruss::calculate_enthalpy(*t, density);

    print!("{}", enthalpy)

}