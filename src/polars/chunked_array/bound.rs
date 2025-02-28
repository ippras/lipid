use crate::prelude::*;
use polars::prelude::*;

/// Bound chunked array
#[derive(Clone)]
pub struct BoundChunked(CategoricalChunked);

impl BoundChunked {
    pub fn new(series: &Series) -> PolarsResult<Self> {
        // unsafe {
        //     CategoricalChunked::from_cats_and_dtype_unchecked(
        //         series.u32()?.clone(),
        //         *Bound::DATA_TYPE,
        //     )
        // };
        let bounds = series.categorical()?.to_local();
        Ok(Self(bounds))
    }
}

impl BoundChunked {
    pub fn carbons(&self) -> u8 {
        assert!(self.0.len() < u8::MAX as _);
        self.0.len() as u8 + 1
    }

    pub fn unsaturation(&self) -> PolarsResult<u8> {
        Ok(self
            .0
            .cast(&Bound::DATA_TYPE)?
            .categorical()?
            .iter_str()
            .filter_map(|id| match Bound::new(id?) {
                Bound::Saturated => None,
                Bound::Unaturated(unsaturated) => Some(unsaturated.unsaturation?.into()),
            })
            .sum())
    }

    // pub fn get(&self, index: usize) -> Option<BoundSeries> {
    //     Some(BoundSeries(self.0.get_as_series(index)?))
    // }
}
