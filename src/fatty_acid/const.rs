pub use self::{saturated::*, unsaturated::*};

use crate::FattyAcid;
use fatty_acid_macro::fatty_acid;

mod saturated {
    use super::*;

    fatty_acid! {
        ///
        pub C1U0;
    }

    fatty_acid! {
        ///
        pub C2U0;
    }

    fatty_acid! {
        ///
        pub C3U0;
    }

    fatty_acid! {
        /// [Butyric acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
        pub C4U0;
    }

    fatty_acid! {
        /// [Valeric acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
        pub C5U0;
    }

    fatty_acid! {
        /// [Caproic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
        pub C6U0;
    }

    fatty_acid! {
        /// [Enanthic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
        pub C7U0;
    }

    fatty_acid! {
        /// [Caprylic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
        pub C8U0;
    }

    fatty_acid! {
        /// [Pelargonic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
        pub C9U0;
    }

    fatty_acid! {
        /// [Capric acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
        pub C10U0;
    }

    fatty_acid! {
        /// [Undecylic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
        pub C11U0;
    }

    fatty_acid! {
        /// [Lauric acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
        pub C12U0;
    }

    fatty_acid! {
        /// [Tridecylic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
        pub C13U0;
    }

    fatty_acid! {
        /// [Myristic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
        pub C14U0;
    }

    fatty_acid! {
        /// [Pentadecylic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
        pub C15U0;
    }

    fatty_acid! {
        /// [Palmitic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
        pub C16U0;
    }

    fatty_acid! {
        /// [Margaric acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
        pub C17U0;
    }

    fatty_acid! {
        /// [Stearic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
        pub C18U0;
    }

    fatty_acid! {
        /// [Nonadecylic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
        pub C19U0;
    }

    fatty_acid! {
        /// [Arachidic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
        pub C20U0;
    }

    fatty_acid! {
        /// [Heneicosylic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
        pub C21U0;
    }

    fatty_acid! {
        /// [Behenic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
        pub C22U0;
    }

    fatty_acid! {
        /// [Tricosylic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
        pub C23U0;
    }

    fatty_acid! {
        /// [Lignoceric acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
        pub C24U0;
    }

    fatty_acid! {
        /// [Hyenic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
        pub C25U0;
    }

    fatty_acid! {
        /// [Cerotic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
        pub C26U0;
    }

    fatty_acid! {
        /// [Carboceric acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
        pub C27U0;
    }

    fatty_acid! {
        /// [Montanic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
        pub C28U0;
    }

    fatty_acid! {
        /// [Nonacosylic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
        pub C29U0;
    }

    fatty_acid! {
        /// [Melissic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
        pub C30U0;
    }

    fatty_acid! {
        /// [Henatriacontylic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
        pub C31U0;
    }

    fatty_acid! {
        /// [Lacceroic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
        pub C32U0;
    }

    fatty_acid! {
        /// [Psyllic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
        pub C33U0;
    }

    fatty_acid! {
        /// [Gheddic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
        pub C34U0;
    }

    fatty_acid! {
        /// [Ceroplastic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
        pub C35U0;
    }

    fatty_acid! {
        /// [Hexatriacontylic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
        pub C36U0;
    }
}

mod unsaturated {
    use super::*;

    fatty_acid! {
        /// [Palmitoleic acid (n-7)](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
        pub C16U1DC9;
    }

    fatty_acid! {
        /// [Palmitelaidic acid (n-7)](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
        pub C16U1DT9;
    }

    fatty_acid! {
        /// [Oleic acid (n-9)](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
        pub C18U1DC9;
    }

    fatty_acid! {
        /// [Elaidic acid (n-9)](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
        pub C18U1DT9;
    }

    fatty_acid! {
        /// [Linoleic acid (ω-6)](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
        pub C18U2DC9DC12;
    }

    fatty_acid! {
        /// [α-Linolenic acid (ω-3)](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
        pub C18U3DC9DC12DC15;
    }

    fatty_acid! {
        /// [γ-Linolenic acid (ω-6), GLA](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
        pub C18U3DC6DC9DC12;
    }

    fatty_acid! {
        /// [Jacaric acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
        pub C18U3DC8DT10DC12;
    }

    fatty_acid! {
        /// [α-Eleostearic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
        pub C18U3DC9DT11DT13;
    }

    fatty_acid! {
        /// [β-Eleostearic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
        pub C18U3DT9DT11DT13;
    }

    fatty_acid! {
        /// [Catalpic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
        pub C18U3DT9DT11DC13;
    }

    fatty_acid! {
        /// [Stearidonic acid (ω-3)](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
        pub C18U4DC6DC9DC12DC15;
    }

    fatty_acid! {
        /// [Gadoleic acid (n-11)](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
        pub C20U1DC9;
    }

    fatty_acid! {
        /// [Gondoic acid (n-9)](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
        pub C20U1DC11;
    }

    fatty_acid! {
        /// [DihomoLinoleic acid (ω-6)](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
        pub C20U2DC11DC14;
    }

    fatty_acid! {
        /// [Bis-homo-α-Linolenic acid (ω-3), DTTA](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
        pub C20U3DC11DC14DC17;
    }

    fatty_acid! {
        /// [Bis-homo-γ-Linolenic acid (ω-6)](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
        pub C20U3DC8DC11DC14;
    }

    fatty_acid! {
        /// [Mead Acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
        pub C20U3DC5DC8DC11;
    }

    fatty_acid! {
        /// [Arachidonic acid (ω-6)](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
        pub C20U4DC5DC8DC11DC14;
    }

    fatty_acid! {
        /// [Eicosatetraenoic acid (ω-3)](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
        pub C20U4DC8DC11DC14DC17;
    }

    fatty_acid! {
        /// [Eicosapentaenoic EPA (ω-3)](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
        pub C20U5DC5DC8DC11DC14DC17;
    }

    fatty_acid! {
        /// [Erucic acid (n-9)](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
        pub C22U1DC13;
    }

    fatty_acid! {
        /// [Docosadienoic acid (ω-6)](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
        pub C22U2DC13DC16;
    }

    fatty_acid! {
        /// [Eranthic acid (ω-6)](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
        pub C22U3DC5DC13DC16;
    }

    fatty_acid! {
        /// [Adrenic acid (ω-6)](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
        pub C22U4DC7DC10DC13DC16;
    }

    fatty_acid! {
        /// [DPA (ω-3)](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
        pub C22U5DC7DC10DC13DC16DC19;
    }

    fatty_acid! {
        /// [DHA (ω-3)](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
        pub C22U6DC4DC7DC10DC13DC16DC19;
    }

    fatty_acid! {
        /// [Nervonic acid (n-9)](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
        pub C24U1DC15;
    }

    fatty_acid! {
        /// [Tetracosadienoic acid (ω-6)](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
        pub C24U2DC15DC18;
    }

    fatty_acid! {
        /// [Tetracosatrienylic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
        pub C24U3DC12DC15DC18;
    }

    fatty_acid! {
        /// [Tetracosatetraenylic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
        pub C24U4DC9DC12DC15DC18;
    }

    fatty_acid! {
        /// [Tetracosapentaenylic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
        pub C24U5DC6DC9DC12DC15DC18;
    }

    fatty_acid! {
        /// [Tetracosahexaenylic acid (ω-3)](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
        pub C24U6DC6DC9DC12DC15DC18DC21;
    }

    fatty_acid! {
        /// [Ximenic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
        pub C26U1DC17;
    }

    fatty_acid! {
        /// [Lumequeic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
        pub C30U1DC21;
    }
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
