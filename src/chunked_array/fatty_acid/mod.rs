use crate::prelude::*;
use polars::prelude::*;
use std::num::NonZeroI8;

#[cfg(feature = "atomic")]
use crate::r#trait::Atomic;
#[cfg(feature = "ecn")]
use crate::r#trait::EquivalentCarbonNumber;

#[repr(transparent)]
pub struct FattyAcidChunked(pub(crate) StructChunked);

impl FattyAcidChunked {
    pub fn new(r#struct: StructChunked) -> Self {
        Self(r#struct)
    }

    pub fn try_new(r#struct: StructChunked) -> PolarsResult<Self> {
        check_data_type(&r#struct)?;
        Ok(Self(r#struct))
    }

    #[inline]
    pub fn carbon(&self) -> PolarsResult<UInt8Chunked> {
        Ok(self.0.field_by_name(CARBON)?.u8()?.clone())
    }

    #[inline]
    pub fn unsaturated(&self) -> PolarsResult<UInt8Chunked> {
        Ok(self.0.field_by_name(UNSATURATED)?.u8()?.clone())
    }

    #[inline]
    pub fn indices(&self) -> PolarsResult<ListChunked> {
        Ok(self.0.field_by_name(INDICES)?.list()?.clone())
    }

    #[inline]
    pub fn get(&self, idx: usize) -> PolarsResult<Option<FattyAcid>> {
        let Some(carbon) = self.0.field_by_name(CARBON)?.u8()?.get(idx) else {
            return Ok(None);
        };
        let Some(unsaturated) = self.0.field_by_name(UNSATURATED)?.u8()?.get(idx) else {
            return Ok(None);
        };
        let Some(indices) = self.0.field_by_name(INDICES)?.list()?.get_as_series(idx) else {
            return Ok(None);
        };
        let indices = IndicesChunked::new(indices.struct_()?)
            .fields()?
            .iter()
            .collect();
        Ok(Some(FattyAcid {
            carbon,
            unsaturated,
            indices,
        }))
    }

    #[inline]
    pub fn get_any_value(&self, idx: usize) -> PolarsResult<AnyValue<'_>> {
        self.0.get_any_value(idx)
    }

    #[inline]
    pub fn fields(&self) -> PolarsResult<FattyAcid<UInt8Chunked, UInt8Chunked, ListChunked>> {
        Ok(FattyAcid {
            carbon: self.carbon()?,
            unsaturated: self.unsaturated()?,
            indices: self.indices()?,
        })
    }
}

impl FattyAcidChunked {
    pub fn is_cis(&self) -> PolarsResult<BooleanChunked> {
        self.indices()?
            .amortized_iter()
            .map(|indices| {
                let Some(indices) = indices else {
                    return Ok(None);
                };
                if indices.as_ref().is_empty() {
                    return Ok(Some(false));
                }
                let indices = IndicesChunked::new(indices.as_ref().struct_()?);
                let parity = indices.parity()?;
                let is_cis = !parity.any();
                Ok(Some(is_cis))
            })
            .collect()
    }

    pub fn is_monounsaturated(&self) -> PolarsResult<BooleanChunked> {
        Ok(self.unsaturated()?.equal(1))
    }

    pub fn is_polyunsaturated(&self) -> PolarsResult<BooleanChunked> {
        Ok(self.unsaturated()?.gt(1))
    }

    pub fn is_saturated(&self) -> PolarsResult<BooleanChunked> {
        Ok(self.unsaturated()?.equal(0))
    }

    pub fn is_trans(&self) -> PolarsResult<BooleanChunked> {
        self.indices()?
            .amortized_iter()
            .map(|indices| {
                let Some(indices) = indices else {
                    return Ok(None);
                };
                if indices.as_ref().is_empty() {
                    return Ok(Some(false));
                }
                let indices = IndicesChunked::new(indices.as_ref().struct_()?);
                let parity = indices.parity()?;
                let is_trans = parity.any();
                Ok(Some(is_trans))
            })
            .collect()
    }

    // 1 2 3 4 5 6 7 8 9 -9 -8 -7 -6 -5 -4 -3 -2 -1
    // - first: всегда указывает на самую первую связь от карбоксильного конца (минимальная Δ).
    // - last: всегда указывает на самую последнюю связь, то есть ближайшую к метильному концу (минимальная ω).
    pub fn is_unsaturated(&self, offset: Option<NonZeroI8>) -> PolarsResult<BooleanChunked> {
        self.carbon()?
            .iter()
            .zip(self.indices()?.amortized_iter())
            .map(|(carbon, indices)| {
                let Some(carbon) = carbon else {
                    return Ok(None);
                };
                let Some(indices) = indices else {
                    return Ok(None);
                };
                let indices = IndicesChunked::new(indices.as_ref().struct_()?);
                let index = indices.index()?;
                let is_unsaturated = match offset {
                    Some(offset) => {
                        let offset = offset.get();
                        match offset {
                            omega @ ..0 => {
                                let Some(last) = index.last() else {
                                    return Ok(Some(false));
                                };
                                if last < 0 {
                                    last == omega
                                } else {
                                    // получаем delta
                                    last == carbon as i8 + omega
                                }
                            }
                            0 => unreachable!("offset NonZeroI8 cannot be 0"),
                            delta @ 1.. => {
                                if index.is_empty() {
                                    return Ok(Some(false));
                                }
                                let Some(first) = index.first() else {
                                    return Ok(Some(false));
                                };
                                if first < 0 {
                                    // получаем delta
                                    carbon as i8 + first == delta
                                } else {
                                    first == delta
                                }
                            }
                        }
                    }
                    None => !index.is_empty(),
                };
                Ok(Some(is_unsaturated))
            })
            .collect()
    }

    pub fn unsaturation(&self) -> PolarsResult<UInt8Chunked> {
        self.indices()?
            .amortized_iter()
            .map(|unsaturated| {
                let Some(unsaturated) = unsaturated else {
                    return Ok(None);
                };
                let unsaturated = IndicesChunked::new(unsaturated.as_ref().struct_()?);
                let triple = unsaturated.triple()?;
                let mut unsaturation = 2 * triple.sum().unwrap_or_default();
                unsaturation += (!triple).sum().unwrap_or_default();
                Ok(Some(unsaturation as _))
            })
            .collect()
    }
}

impl FattyAcidChunked {
    #[inline]
    pub fn filter(&self, mask: &BooleanChunked) -> PolarsResult<Self> {
        Ok(Self::new(self.0.filter(mask)?))
    }

    #[inline]
    pub fn nullify(&self, mask: &BooleanChunked) -> PolarsResult<Self> {
        Ok(Self::new(self.0.zip_with(
            mask,
            &StructChunked::full_null_like(&self.0, 1),
        )?))
    }
}

#[cfg(feature = "atomic")]
impl Atomic for &FattyAcidChunked {
    type Output = PolarsResult<UInt8Chunked>;

    fn carbon(self) -> PolarsResult<UInt8Chunked> {
        self.carbon()
    }

    fn hydrogen(self) -> PolarsResult<UInt8Chunked> {
        Ok(self.carbon()? * 2 - self.unsaturation()? * 2)
    }

    fn oxygen(self) -> PolarsResult<UInt8Chunked> {
        Ok(UInt8Chunked::full(PlSmallStr::EMPTY, 2, 1))
    }
}

#[cfg(feature = "ecn")]
impl EquivalentCarbonNumber for &FattyAcidChunked {
    type Output = PolarsResult<UInt8Chunked>;

    #[inline]
    fn equivalent_carbon_number(self) -> PolarsResult<UInt8Chunked> {
        Ok(self.carbon()? - self.unsaturation()? * 2)
    }
}

impl<'a> TryFrom<&'a Series> for &'a FattyAcidChunked {
    type Error = PolarsError;

    fn try_from(value: &'a Series) -> Result<Self, Self::Error> {
        value.struct_()?.try_into()
    }
}

impl TryFrom<StructChunked> for FattyAcidChunked {
    type Error = PolarsError;

    fn try_from(value: StructChunked) -> Result<Self, Self::Error> {
        check_data_type(&value)?;
        Ok(Self(value))
    }
}

impl<'a> TryFrom<&'a StructChunked> for &'a FattyAcidChunked {
    type Error = PolarsError;

    fn try_from(value: &'a StructChunked) -> Result<Self, Self::Error> {
        check_data_type(value)?;
        // [safe](https://doc.rust-lang.org/reference/type-layout.html?highlight=transparent#the-transparent-representation)
        Ok(unsafe { &*(value as *const StructChunked as *const FattyAcidChunked) })
    }
}

impl FattyAcid<UInt8Chunked, UInt8Chunked, ListChunked> {
    pub fn iter(&self) -> impl Iterator<Item = PolarsResult<Option<FattyAcid>>> {
        self.carbon
            .iter()
            .zip(self.unsaturated.iter())
            .zip(self.indices.amortized_iter())
            .map(
                |((carbon, unsaturated), indices)| -> PolarsResult<Option<_>> {
                    let Some(carbon) = carbon else {
                        return Ok(None);
                    };
                    let Some(unsaturated) = unsaturated else {
                        return Ok(None);
                    };
                    let Some(indices) = indices else {
                        return Ok(None);
                    };
                    let indices = IndicesChunked::new(indices.as_ref().struct_()?)
                        .fields()?
                        .iter()
                        .collect();
                    Ok(Some(FattyAcid {
                        carbon,
                        unsaturated,
                        indices,
                    }))
                },
            )
    }
}

impl IntoIterator for &FattyAcid<UInt8Chunked, UInt8Chunked, ListChunked> {
    type Item = PolarsResult<Option<FattyAcid>>;

    type IntoIter = impl Iterator<Item = PolarsResult<Option<FattyAcid>>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

fn check_data_type(r#struct: &StructChunked) -> PolarsResult<()> {
    polars_ensure!(
        *r#struct.dtype() == data_type!(FATTY_ACID),
        SchemaMismatch: "invalid fatty acid data type: expected `FATTY_ACID`, got = `{}`",
        r#struct.dtype(),
    );
    Ok(())
}

mod display;
