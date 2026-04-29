use crate::prelude::*;
use polars::prelude::*;

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
                None => triacylglycerol.mono().to_string(),
                Some(Stereospecificity::Positional) => triacylglycerol.positional().to_string(),
                Some(Stereospecificity::Stereo) => triacylglycerol.stereo().to_string(),
            })
            .collect())
    }
}
