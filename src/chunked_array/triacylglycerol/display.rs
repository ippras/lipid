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
            .map(|triacylglycerol| triacylglycerol.display(stereospecificity))
            .collect())
    }
}
