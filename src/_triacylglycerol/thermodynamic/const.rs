//! <https://lipidlibrary.shinyapps.io/Triglyceride_Property_Calculator>
//!
//! * DOI: 10.1201/b12883-10 (Wesdorp-2012)
//!
//! * H0 - enthalpy contributions due to the glycerol head group
//! * H - enthalpy contributions of the total carbon number in the TAG
//! * S0 - entropy contributions due to the glycerol head group
//! * S - entropy contributions of the total carbon number in the TAG
//! * H_XY enthalpy contributions due to difference in chain length between fatty acid chains
//! * S_XY entropy contributions due to difference in chain length between fatty acid chains
//! * H_ODD - enthalpy contributions due to an odd number of total carbons in the fatty acids
//! * S_ODD - entropy contributions due to an odd number of total carbons in the fatty acids

pub const R: f64 = 8.314_462_618_153_24;

pub(super) mod alpha {
    const H0: Option<f64> = Some(-31.95);
    const H: Option<f64> = Some(2.70);
    const S0: Option<f64> = Some(-19.09);
    const S: Option<f64> = Some(6.79);
    const H_XY: Option<f64> = Some(-13.28);
    const S_XY: Option<f64> = Some(-36.70);
    const K: Option<f64> = Some(4.39);
    const X0: Option<f64> = Some(1.25);
    const T_INF: Option<f64> = Some(397.0);
    const H_ODD: Option<f64> = None;
    const S_ODD: Option<f64> = None;

    const A_0: Option<f64> = Some(-9.0581);
    const A_X: Option<f64> = Some(0.00290);
    const A_XX: Option<f64> = Some(-0.0619116);
    const A_XY: Option<f64> = Some(0.115128);
    const A_Y: Option<f64> = Some(-0.453461);
    const A_YY: Option<f64> = Some(-0.005827);
    const B_0: Option<f64> = Some(-4.4841);
    const B_X: Option<f64> = Some(-0.00111);
    const B_XX: Option<f64> = Some(0.148938);
    const B_XY: Option<f64> = Some(-0.365917);
    const B_Y: Option<f64> = Some(1.41154);
    const B_YY: Option<f64> = Some(-0.001766);
    // const T_INF: Option<f64> = Some(401.15);

    const A_ODD: Option<f64> = Some(-0.196);

    const H_O: Option<f64> = Some(-31.7);
    const H_E: Option<f64> = Some(-11.7);
    const H_L: Option<f64> = Some(-37.7);
}

pub(super) mod beta {}

pub(super) mod beta_prime {}

// Triglyceride Property Calculator
// <>
pub mod polymorphism {
    pub mod alpha {
        pub const H: f64 = 2.7;
        pub const H0: f64 = -31.95;
        pub const H_XY: f64 = -13.28;

        pub const S: f64 = 6.79;
        pub const S0: f64 = -19.09;
        pub const S_XY: f64 = -36.7;

        pub const K: f64 = 4.39;
        pub const K_X: f64 = K;
        pub const K_Y: f64 = K;

        pub const X0: f64 = 1.25;
    }

    pub mod beta_prime {
        pub const H: f64 = 3.86;
        pub const H0: f64 = -35.86;
        pub const H_XY: f64 = -19.35;

        pub const S: f64 = 10.13;
        pub const S0: f64 = -39.59;
        pub const S_XY: f64 = -52.51;

        pub const K: f64 = 1.99;
        pub const K_X: f64 = K;
        pub const K_Y: f64 = K;

        pub const X0: f64 = 2.46;
    }

    pub mod beta {
        pub const H: f64 = 3.89;
        pub const H0: f64 = -17.16;
        pub const H_XY: f64 = -22.29;
        pub const H_ODD: f64 = 2.29;

        pub const S: f64 = 9.83;
        pub const S0: f64 = 31.04;
        pub const S_XY: f64 = -64.58;

        pub const K: f64 = 2.88;
        pub const K_X: f64 = K;
        pub const K_Y: f64 = K;

        pub const X0: f64 = 0.77;
    }
}
