use crate::prelude::*;
use polars::prelude::*;

// impl Triacylglycerol<StringChunked> {
//     pub fn mono(&self) -> StringChunked {
//         self.into_iter()
//             .map(|triacylglycerol| Mono(triacylglycerol).to_string())
//             .collect()
//     }

//     pub fn positional(&self) -> StringChunked {
//         self.into_iter()
//             .map(|triacylglycerol| Positional(triacylglycerol).to_string())
//             .collect()
//     }

//     pub fn stereo(&self) -> StringChunked {
//         self.into_iter()
//             .map(|triacylglycerol| Stereo(triacylglycerol).to_string())
//             .collect()
//     }
// }

impl TriacylglycerolChunked {
    pub fn display(&self, stereospecificity: Stereospecificity) -> PolarsResult<StringChunked> {
        Ok(self
            .fields()?
            .try_map(|field| Ok(field.str()?.clone()))?
            .into_iter()
            .map(|triacylglycerol| match stereospecificity {
                Stereospecificity::Mono => Mono(triacylglycerol).to_string(),
                Stereospecificity::Positional => Positional(triacylglycerol).to_string(),
                Stereospecificity::Stereo => Stereo(triacylglycerol).to_string(),
            })
            .collect())
    }

    pub fn mono(&self) -> PolarsResult<StringChunked> {
        self.display(Stereospecificity::Mono)
    }

    pub fn positional(&self) -> PolarsResult<StringChunked> {
        self.display(Stereospecificity::Positional)
    }

    pub fn stereo(&self) -> PolarsResult<StringChunked> {
        self.display(Stereospecificity::Stereo)
    }
}
