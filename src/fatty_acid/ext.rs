/// Extension methods for [`FattyAcid`]
pub trait FattyAcidExt {
    type Output;

    /// Carbons
    fn carbons(&self) -> Self::Output;

    /// Hydrogens
    ///
    /// `H = 2C - 2U`
    fn hydrogens(&self) -> Self::Output;

    /// Bounds
    ///
    /// The number of bonds.
    fn bounds(&self) -> Self::Output;

    /// Saturated
    ///
    /// The number of saturated bonds.
    fn saturated(&self) -> Self::Output;

    /// Unsaturated
    ///
    /// The number of unsaturated bonds.
    fn unsaturated(&self) -> Self::Output;
}

/// Extension methods for [`FattyAcid`]
pub trait Mass: FattyAcidExt {
    const H: Self::Output;
    const C: Self::Output;
    const O: Self::Output;

    /// Mass
    fn mass(&self) -> f64;
    //  {
    //     self.c() as f64 * C + self.h() as f64 * H + 2. * O
    // }
}

// pub mod short {
//     /// Fatty acid short
//     pub trait FattyAcidShortExt {
//         /// Carbon
//         fn c(&self) -> u8 {
//             self.b() + 1
//         }

//         /// Hydrogen
//         ///
//         /// `H = 2C - 2U`
//         fn h(&self) -> u8 {
//             2 * self.c() - 2 * self.u()
//         }

//         /// Fatty acid ECN (Equivalent carbon number)
//         ///
//         /// `ECN = C - 2U`
//         fn ecn(&self) -> u8 {
//             self.c() - 2 * self.u()
//         }

//         /// Mass
//         fn mass(&self) -> f64 {
//             self.c() as f64 * C + self.h() as f64 * H + 2. * O
//         }

//         /// Saturated
//         fn s(&self) -> bool {
//             self.u() == 0
//         }

//         fn b(&self) -> u8;

//         /// Unsaturated bounds
//         fn u(&self) -> u8;
//     }
// }
