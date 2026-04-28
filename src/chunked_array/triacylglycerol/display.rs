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
    pub fn display(
        &self,
        stereospecificity: Option<Stereospecificity>,
    ) -> PolarsResult<StringChunked> {
        Ok(self
            .fields()?
            .try_map(|field| Ok(field.str()?.clone()))?
            .into_iter()
            .map(|triacylglycerol| match stereospecificity {
                None => Mono(triacylglycerol).to_string(),
                Some(Stereospecificity::Positional) => Positional(triacylglycerol).to_string(),
                Some(Stereospecificity::Stereo) => Stereo(triacylglycerol).to_string(),
            })
            .collect())
    }

    pub fn mono(&self) -> PolarsResult<StringChunked> {
        self.display(None)
    }

    pub fn positional(&self) -> PolarsResult<StringChunked> {
        self.display(Some(Stereospecificity::Positional))
    }

    pub fn stereo(&self) -> PolarsResult<StringChunked> {
        self.display(Some(Stereospecificity::Stereo))
    }
}
