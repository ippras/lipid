pub use self::{saturated::*, unsaturated::*};

use fatty_acid_macro::fatty_acid;

mod saturated {
    use super::*;

    fatty_acid!(C1U0);
    fatty_acid!(C2U0);
    fatty_acid!(C3U0);
    fatty_acid!(C4U0);
    fatty_acid!(C5U0);
    fatty_acid!(C6U0);
    fatty_acid!(C7U0);
    fatty_acid!(C8U0);
    fatty_acid!(C9U0);
    fatty_acid!(C10U0);
    fatty_acid!(C11U0);
    fatty_acid!(C12U0);
    fatty_acid!(C13U0);
    fatty_acid!(C14U0);
    fatty_acid!(C15U0);
    fatty_acid!(C16U0);
    fatty_acid!(C17U0);
    fatty_acid!(C18U0);
    fatty_acid!(C19U0);
    fatty_acid!(C20U0);
    fatty_acid!(C21U0);
    fatty_acid!(C22U0);
    fatty_acid!(C23U0);
    fatty_acid!(C24U0);
    fatty_acid!(C25U0);
    fatty_acid!(C26U0);
    fatty_acid!(C27U0);
    fatty_acid!(C28U0);
    fatty_acid!(C29U0);
    fatty_acid!(C30U0);
    fatty_acid!(C31U0);
    fatty_acid!(C32U0);
    fatty_acid!(C33U0);
    fatty_acid!(C34U0);
    fatty_acid!(C35U0);
    fatty_acid!(C36U0);
    fatty_acid!(C37U0);
    fatty_acid!(C38U0);
    fatty_acid!(C39U0);
    fatty_acid!(C40U0);
    fatty_acid!(C41U0);
    fatty_acid!(C42U0);
    fatty_acid!(C43U0);
    fatty_acid!(C44U0);
    fatty_acid!(C45U0);
    fatty_acid!(C46U0);
    fatty_acid!(C47U0);
    fatty_acid!(C48U0);
    fatty_acid!(C49U0);
    fatty_acid!(C50U0);
    fatty_acid!(C51U0);
    fatty_acid!(C52U0);
    fatty_acid!(C53U0);
    fatty_acid!(C54U0);
    fatty_acid!(C55U0);
    fatty_acid!(C56U0);
    fatty_acid!(C57U0);
    fatty_acid!(C58U0);
    fatty_acid!(C59U0);
    fatty_acid!(C60U0);
    fatty_acid!(C61U0);
    fatty_acid!(C62U0);
    fatty_acid!(C63U0);
    fatty_acid!(C64U0);
}

mod unsaturated {
    use super::*;

    fatty_acid!(C16U1DC9);
    fatty_acid!(C16U1DT9);
    fatty_acid!(C18U1DC9);
    fatty_acid!(C18U1DT9);
    fatty_acid!(C18U2DC9DC12);
    fatty_acid!(C18U3DC6DC9DC12);
    fatty_acid!(C18U3DC8DT10DC12);
    fatty_acid!(C18U3DC9DC12DC15);
    fatty_acid!(C18U3DC9DT11DT13);
    fatty_acid!(C18U3DT9DT11DC13);
    fatty_acid!(C18U3DT9DT11DT13);
    fatty_acid!(C18U4DC6DC9DC12DC15);
    fatty_acid!(C20U1DC9);
    fatty_acid!(C20U1DC11);
    fatty_acid!(C20U2DC11DC14);
    fatty_acid!(C20U3DC5DC8DC11);
    fatty_acid!(C20U3DC8DC11DC14);
    fatty_acid!(C20U3DC11DC14DC17);
    fatty_acid!(C20U4DC5DC8DC11DC14);
    fatty_acid!(C20U4DC8DC11DC14DC17);
    fatty_acid!(C20U5DC5DC8DC11DC14DC17);
    fatty_acid!(C22U1DC13);
    fatty_acid!(C22U2DC13DC16);
    fatty_acid!(C22U3DC5DC13DC16);
    fatty_acid!(C22U4DC7DC10DC13DC16);
    fatty_acid!(C22U5DC7DC10DC13DC16DC19);
    fatty_acid!(C22U6DC4DC7DC10DC13DC16DC19);
    fatty_acid!(C24U1DC15);
    fatty_acid!(C24U2DC15DC18);
    fatty_acid!(C24U3DC12DC15DC18);
    fatty_acid!(C24U4DC9DC12DC15DC18);
    fatty_acid!(C24U5DC6DC9DC12DC15DC18);
    fatty_acid!(C24U6DC6DC9DC12DC15DC18DC21);
    fatty_acid!(C26U1DC17);
    fatty_acid!(C30U1DC21);
}

mod wildcard {
    // fatty_acid!(C26U2X);
    // fatty_acid!(C26U3X);
    // fatty_acid!(C26U4X);
    // fatty_acid!(C26U5X);
    // fatty_acid!(C26U6X);
    // fatty_acid!(C28U1X);
    // fatty_acid!(C28U2X);
    // fatty_acid!(C32U1X);
    // fatty_acid!(C32U2X);
    // fatty_acid!(C34U1X);
    // fatty_acid!(C34U2X);
    // fatty_acid!(C36U1X);
    // fatty_acid!(C36U2X);
}

#[test]
fn test() {
    println!("C2U0: {C1U0:?}");
    println!("C18U2: {C18U2DC9DC12:?}");
}
