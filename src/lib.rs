const eVsqkm_to_GeV_over4: f64 = 1e-9 / 1.97327e-7 * 1e+3 / 4.0;
const YerhoE2a: f64 = 1.52e-4;

pub struct VacuumParameters {
    s12sq: f64,
    s13sq: f64,
    s23sq: f64,
    delta: f64,
    Dmsq21: f64,
    Dmsq31: f64,
    L: f64,
    E: f64,
}

pub struct MatterParameters {
    s12sq: f64,
    s13sq: f64,
    s23sq: f64,
    delta: f64,
    Dmsq21: f64,
    Dmsq31: f64,
    L: f64,
    E: f64,
    rho: f64,
    Ye: f64,
    N_Newton: u8,
}

pub fn probability_vacuum_lbl(parameters: &VacuumParameters) -> [[f64; 3]; 3] {
    let VacuumParameters { s12sq, s13sq, s23sq, delta, Dmsq21, Dmsq31, L, E } = parameters;

    /*
    ------------------------------------------
    First calculate useful simple functions of the oscillation parameters
    ------------------------------------------
    */
    let c13sq = 1.0 - s13sq;

    // Ueisq's
    let Ue3sq = s13sq;
    let Ue2sq = c13sq * s12sq;

    // Umisq's, Utisq's and Jvac
    let Um3sq = c13sq * s23sq;
    // Um2sq and Ut2sq are used here as temporary variables, will be properly defined later
    let Ut2sq = s13sq * s12sq * s23sq;
    let Um2sq = (1.0 - s12sq) * (1.0 - s23sq);

    let Jrr = (Um2sq * Ut2sq).sqrt();
    let sind = delta.sin();
    let cosd = delta.cos();
    let Um2sq = Um2sq + Ut2sq - 2.0 * Jrr * cosd;
    let Jvac = 8.0 * Jrr * c13sq * sind;

    /*
    -----------------------
    Get all elements of Usq
    -----------------------
    */
    let Ue1sq = 1.0 - Ue3sq - Ue2sq;
    let Um1sq = 1.0 - Um3sq - Um2sq;

    let Ut3sq = 1.0 - Um3sq - Ue3sq;
    let Ut2sq = 1.0 - Um2sq - Ue2sq;
    let Ut1sq = 1.0 - Um1sq - Ue1sq;

    /*
    -----------------------
    Get the kinematic terms
    -----------------------
    */
    let Lover4E = eVsqkm_to_GeV_over4 * L / E;

    let D21 = Dmsq21 * Lover4E;
    let D31 = Dmsq31 * Lover4E;

    let sinD21 = D21.sin();
    let sinD31 = D31.sin();
    let sinD32 = (D31 - D21).sin();

    let triple_sin = sinD21 * sinD31 * sinD32;

    let sinsqD21_2 = 2.0 * sinD21 * sinD21;
    let sinsqD31_2 = 2.0 * sinD31 * sinD31;
    let sinsqD32_2 = 2.0 * sinD32 * sinD32;

    /*
    ----------------------------
    Calculate the three probabilities, separating CPC and CPV
    ----------------------------
    */
    let Pme_CPC = (Ut3sq - Um2sq * Ue1sq - Um1sq * Ue2sq) * sinsqD21_2 + (Ut2sq - Um3sq * Ue1sq - Um1sq * Ue3sq) * sinsqD31_2 + (Ut1sq - Um3sq * Ue2sq - Um2sq * Ue3sq) * sinsqD32_2;

    let Pme_CPV = -Jvac * triple_sin;

    let Pmm = 1.0 - 2.0 * (Um2sq * Um1sq * sinsqD21_2 + Um3sq * Um1sq * sinsqD31_2 + Um3sq * Um2sq * sinsqD32_2);
    
    let Pee = 1.0 - 2.0 * (Ue2sq * Ue1sq * sinsqD21_2 + Ue3sq * Ue1sq * sinsqD31_2 + Ue3sq * Ue2sq * sinsqD32_2);

    /*
    ----------------------------
    Assign all the probabilities
    ----------------------------
    */
    let mut probs = [[0.0; 3], [0.0;3], [0.0;3]];
    
    probs[0][0] = Pee; // Pee
    probs[0][1] = Pme_CPC - Pme_CPV; // Pem
    probs[0][2] = 1.0 - Pee - probs[0][1]; // Pet

    probs[1][0] = Pme_CPC + Pme_CPV; // Pme
    probs[1][1] = Pmm; // Pmm
    probs[1][2] = 1.0 - probs[1][0] - Pmm; // Pmt

    probs[2][0] = 1.0 - Pee - probs[1][0]; // Pte
    probs[2][1] = 1.0 - probs[0][1] - Pmm; // Ptm
    probs[2][2] = 1.0 - probs[0][2] - probs[1][2]; // Ptt

    probs
}

pub fn probability_matter_lbl(parameters: &MatterParameters) -> [[f64; 3]; 3] {
    
    let MatterParameters { s12sq, s13sq, s23sq, delta, Dmsq21, Dmsq31, L, E, rho, Ye, N_Newton } = parameters;

    /*
    ---------------------------------------------------------------------
    First calculate useful simple functions of the oscillation parameters
    ---------------------------------------------------------------------
    */
    let c13sq = 1.0 - s13sq;

    // Ueisq's
    let Ue2sq = c13sq * s12sq;
    let Ue3sq = s13sq;

    // Umisq's, Utisq's and Jmatter
    let Um3sq = c13sq * s23sq;
    // Um2sq and Ut2sq are used here as temporary variables, will be properly defined later
    let Ut2sq = s13sq * s12sq * s23sq;
    let Um2sq = (1.0 - s12sq) * (1.0 - s23sq);

    let Jrr = (Um2sq * Ut2sq).sqrt();
    let sind = delta.sin();
    let cosd = delta.cos();

    let Um2sq = Um2sq + Ut2sq - 2.0 * Jrr * cosd;
    let Jmatter = 8.0 * Jrr * c13sq * sind;
    let Amatter = Ye * rho * E * YerhoE2a;
    let Dmsqee = Dmsq31 - s12sq * Dmsq21;

    // calculate A, B, C, See, Tee, and part of Tmm
    let A = Dmsq21 + Dmsq31; // temporary variable
    let See = A - Dmsq21 * Ue2sq - Dmsq31 * Ue3sq;
    let Tmm = Dmsq21 * Dmsq31;
    let Tee = Tmm * (1.0 - Ue3sq - Ue2sq);
    let C = Amatter * Tee;
    let A = A + Amatter;

    /*
    ----------------------------------
    Get lambda3 from lambda+ of MP/DMP
    ----------------------------------
    */
    let xmat = Amatter / Dmsqee;
    let tmp = 1.0 - xmat;
    let mut lambda3 = Dmsq31 + 0.5 * Dmsqee * (xmat - 1.0 + (tmp * tmp + 4.0 * s13sq * xmat).sqrt());

    /*
    --------------------------------------
    Newton iterations to improve lambda3 arbitrarily, if needed, (B needed here)
    --------------------------------------
    */
    let B = Tmm + Amatter + See; // B is only needed for N_Newton >= 1
    for i in 0..*N_Newton {
        lambda3 = (lambda3 * lambda3 * (lambda3 * lambda3 - A) + C) / (lambda3 * (2.0 * (lambda3 - A) + lambda3) + B); // this strange form prefers additions to multiplications
    }

    /*
    -----------------
    Get Delta lambdas
    -----------------
    */
    let tmp = A - lambda3;
    let Dlambda21 = (tmp * tmp - 4.0 * C / lambda3).sqrt();
    let lambda2 = 0.5 * (A - lambda3 + Dlambda21);
    let Dlambda32 = lambda3 - lambda2;
    let Dlambda31 = Dlambda32 + Dlambda21;

    /*
    -----------------------
    Use Rosetta for Veisq's
    -----------------------
    */

    // denominators
    let PiDlambdaInv = 1.0 / (Dlambda31 * Dlambda32 * Dlambda21);
    let Xp3 = PiDlambdaInv * Dlambda21;
    let Xp2 = -PiDlambdaInv * Dlambda31;

    // numerators
    let Ue3sq = (lambda3 * (lambda3 - See) + Tee) * Xp3;
    let Ue2sq = (lambda2 * (lambda2 - See) + Tee) * Xp2;

    let Smm = A - Dmsq21 * Um2sq - Dmsq31 * Um3sq;
    let Tmm = Tmm * (1.0 - Um3sq - Um2sq) + Amatter * (See + Smm - A);

    let Um3sq = (lambda3 * (lambda3 - Smm) + Tmm) * Xp3;
    let Um2sq = (lambda2 * (lambda2 - Smm) + Tmm) * Xp2;

    /*
    -------------
    Use NHS for J
    -------------
    */
    let Jmatter = Jmatter * Dmsq21 * Dmsq31 * (Dmsq31 - Dmsq21) * PiDlambdaInv;

    /*
    -----------------------
    Get all elements of Usq
    -----------------------
    */
    let Ue1sq = 1.0 - Ue3sq - Ue2sq;
    let Um1sq = 1.0 - Um3sq - Um2sq;

    let Ut3sq = 1.0 - Um3sq - Ue3sq;
    let Ut2sq = 1.0 - Um2sq - Ue2sq;
    let Ut1sq = 1.0 - Um1sq - Ue1sq;

    /*
    -----------------------
    Get the kinematic terms
    -----------------------
    */
    let Lover4E = eVsqkm_to_GeV_over4 * L / E;

    let D21 = Dlambda21 * Lover4E;
    let D32 = Dlambda32 * Lover4E;

    let sinD21 = D21.sin();
    let sinD31 = (D32 + D21).sin();
    let sinD32 = D32.sin();

    let triple_sin = sinD21 * sinD31 * sinD32;

    let sinsqD21_2 = 2.0 * sinD21 * sinD21;
    let sinsqD31_2 = 2.0 * sinD31 * sinD31;
    let sinsqD32_2 = 2.0 * sinD32 * sinD32;

    /*
    ----------------------------------------------------------------
    Calculate the three necessary probabilities, separating CPC and CPV
    ----------------------------------------------------------------
    */
    let Pme_CPC = (Ut3sq - Um2sq * Ue1sq - Um1sq * Ue2sq) * sinsqD21_2 + (Ut2sq - Um3sq * Ue1sq - Um1sq * Ue3sq) * sinsqD31_2 + (Ut1sq - Um3sq * Ue2sq - Um2sq * Ue3sq) * sinsqD32_2;
    let Pme_CPV = -Jmatter * triple_sin;

    let Pmm = 1.0 - 2.0 * (Um2sq * Um1sq * sinsqD21_2 + Um3sq * Um1sq * sinsqD31_2 + Um3sq * Um2sq * sinsqD32_2);

    let Pee = 1.0 - 2.0 * (Ue2sq * Ue1sq * sinsqD21_2 + Ue3sq * Ue1sq * sinsqD31_2 + Ue3sq * Ue2sq * sinsqD32_2);

    /*
    ----------------------------
    Assign all the probabilities
    ----------------------------
    */
    let mut probs = [[0.0; 3], [0.0;3], [0.0;3]];

    probs[0][0] = Pee; // Pee
    probs[0][1] = Pme_CPC - Pme_CPV; // Pem
    probs[0][2] = 1.0 - Pee - probs[0][1]; // Pet

    probs[1][0] = Pme_CPC + Pme_CPV; // Pme
    probs[1][1] = Pmm; // Pmm
    probs[1][2] = 1.0 - probs[1][0] - Pmm; // Pmt

    probs[2][0] = 1.0 - Pee - probs[1][0]; // Pte
    probs[2][1] = 1.0 - probs[0][1] - Pmm; // Ptm
    probs[2][2] = 1.0 - probs[0][2] - probs[1][2]; // Ptt

    probs
}