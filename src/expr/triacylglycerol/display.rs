use crate::prelude::*;
use polars::prelude::*;

impl TriacylglycerolExpr {
    #[inline]
    pub fn display(self, stereospecificity: Stereospecificity) -> Expr {
        self.0.map(
            move |column| {
                Ok(column
                    .try_triacylglycerol()?
                    .display(stereospecificity)?
                    .into_column())
            },
            |_, field| Ok(Field::new(field.name().clone(), DataType::String)),
        )
    }

    // pub fn mono(self) -> Expr {
    //     self.display(Stereospecificity::Mono)
    // }

    // pub fn positional(self) -> Expr {
    //     self.display(Stereospecificity::Positional)
    // }

    // pub fn stereo(self) -> Expr {
    //     self.display(Stereospecificity::Stereo)
    // }

    // pub fn new_display(
    //     self,
    //     stereospecificity: Stereospecificity,
    //     alternate: bool,
    // ) -> PolarsResult<Expr> {
    //     let sn1 = self.stereospecific_number1();
    //     let sn2 = self.stereospecific_number1();
    //     let sn3 = self.stereospecific_number3();
    //     match stereospecificity {
    //         Stereospecificity::Mono if alternate => format_str(
    //             "{{1:{} & 2:{} & 3:{} | 1:{} & 2:{} & 3:{} | 1:{} & 2:{} & 3:{} | 1:{} & 2:{} & 3:{} | 1:{} & 2:{} & 3:{} | 1:{} & 2:{} & 3:{}}}",
    //             &[
    //                 sn1, sn2, sn3, sn1, sn3, sn2, sn2, sn1, sn3, sn2, sn3, sn1, sn3, sn1, sn2, sn3,
    //                 sn2, sn1,
    //             ],
    //         ),
    //         Stereospecificity::Positional if alternate => format_str(
    //             "{{1:{} & 2:{} & 3:{} | 1:{} & 2:{} & 3:{}}}",
    //             &[sn1, sn2, sn3, sn3, sn2, sn1],
    //         ),
    //         Stereospecificity::Stereo if alternate => {
    //             format_str("{{1:{} & 2:{} & 3:{}}}", &[sn1, sn2, sn3])
    //         }
    //         Stereospecificity::Mono => format_str("[{};{};{}]", &[sn1, sn2, sn3]),
    //         Stereospecificity::Positional => format_str("[{}/2;{};{}/2]", &[sn1, sn2, sn3]),
    //         Stereospecificity::Stereo => format_str("[{}/3;{}/3;{}/3]", &[sn1, sn2, sn3]),
    //     }
    // }
}
