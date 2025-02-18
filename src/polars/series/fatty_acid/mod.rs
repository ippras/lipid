use self::unsaturated::UnsaturatedSeries;
use crate::fatty_acid::FattyAcid;
use polars::prelude::*;
use std::iter::from_fn;

/// Fatty acid series
#[derive(Clone, Debug)]
pub struct FattyAcidSeries {
    pub carbons: Series,
    pub unsaturated: Series,
}

impl FattyAcidSeries {
    pub fn new(series: &Series) -> PolarsResult<Self> {
        let r#struct = series.struct_()?;
        let carbons = r#struct.field_by_name("Carbons")?;
        let unsaturated = r#struct.field_by_name("Unsaturated")?;
        Ok(Self {
            carbons,
            unsaturated,
        })
    }

    pub fn len(&self) -> usize {
        self.carbons.len()
    }

    pub fn get(&self, index: usize) -> PolarsResult<Option<FattyAcid>> {
        let Some(carbons) = self.carbons.u8()?.get(index) else {
            return Ok(None);
        };
        let mut unsaturated = Vec::new();
        if let Some(series) = self.unsaturated.list()?.get_as_series(index) {
            let unsaturated_series = UnsaturatedSeries::new(&series)?;
            for index in 0..unsaturated_series.len() {
                unsaturated.push(unsaturated_series.get(index)?);
            }
        };
        Ok(Some(FattyAcid {
            carbons,
            unsaturated,
        }))
    }

    pub fn saturated_filter(&self) -> PolarsResult<BooleanChunked> {
        Ok(self
            .unsaturated
            .list()?
            .iter()
            .map(|list| Some(list?.len() == 0))
            .collect())
    }

    pub fn unsaturated_filter(&self) -> PolarsResult<BooleanChunked> {
        Ok(self
            .unsaturated
            .list()?
            .iter()
            .map(|list| Some(list?.len() != 0))
            .collect())
    }

    pub fn unsaturated(&self, index: usize) -> PolarsResult<Option<UnsaturatedSeries>> {
        let Some(unsaturated) = self.unsaturated.list()?.get_as_series(index) else {
            return Ok(None);
        };
        Ok(Some(UnsaturatedSeries::new(&unsaturated)?))
    }
}

impl IntoIterator for FattyAcidSeries {
    type Item = Option<FattyAcid>;

    type IntoIter = impl Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut index = 0;
        from_fn(move || {
            if index >= self.len() {
                return None;
            }
            let fatty_acid = self.get(index).expect("get next fatty acid");
            index += 1;
            Some(fatty_acid)
        })
    }
}

mod unsaturated;
