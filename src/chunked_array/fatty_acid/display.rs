use crate::prelude::*;
use polars::prelude::*;

impl FattyAcidChunked {
    // pub fn formula(&self) -> PolarsResult<Utf8Chunked> {
    //     let carbon = self.carbon()?;
    //     let hydrogen = self.hydrogen()?;
    //     let oxygen = self.oxygen()?;
    //     let mut builder = Utf8ChunkedBuilder::new("Formula", self.0.len(), self.0.len() * 10);
    //     for ((c, h), o) in carbon.iter().zip(hydrogen.iter()).zip(oxygen.iter()) {
    //         match (c, h, o) {
    //             (Some(c), Some(h), Some(o)) => builder.append_value(format!("C{}H{}O{}", c, h, o)),
    //             _ => builder.append_null(),
    //         }
    //     }
    //     Ok(builder.finish())
    // }

    pub fn id(&self) -> PolarsResult<StringChunked> {
        self.fields()?
            .iter()
            .map(|fatty_acid| {
                let Some(fatty_acid) = fatty_acid? else {
                    return Ok(None);
                };
                Ok(Some(fatty_acid.id().to_string()))
            })
            .collect()
    }

    pub fn delta(&self) -> PolarsResult<StringChunked> {
        self.fields()?
            .iter()
            .map(|fatty_acid| {
                let Some(fatty_acid) = fatty_acid? else {
                    return Ok(None);
                };
                Ok(Some(fatty_acid.delta().to_string()))
            })
            .collect()
    }
}
