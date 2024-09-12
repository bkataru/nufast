
use nufast::{VacuumParameters, MatterParameters, probability_vacuum_lbl, probability_matter_lbl};

fn main() {
    let parameters = MatterParameters {
        L: 1300.0,
        E: 2.5,
        rho: 3.0,
        Ye: 0.5,
        N_Newton: 0,
        s12sq: 0.31,
        s13sq: 0.02,
        s23sq: 0.55,
        delta: 0.7 * std::f64::consts::PI,
        Dmsq21: 7.5e-5,
        Dmsq31: 2.5e-3,
    };

    let probs = probability_matter_lbl(&parameters);

    println!("L = {}, E = {}, rho = {}", parameters.L, parameters.E, parameters.rho);
    println!("Probabilities:
    n");
    println!("alpha beta P(nu_alpha -> nu_beta)");
    for alpha in 0..3 {
        for beta in 0..3 {
            println!("{} {} {}", alpha, beta, probs[alpha][beta]);
        }
    }
}
