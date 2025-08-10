use crate::{display::FattyAcid, prelude::*};
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
    pub fn indices(&self) -> PolarsResult<ListChunked> {
        Ok(self.0.field_by_name(INDICES)?.list()?.clone())
    }

    #[inline]
    pub fn get(&self, idx: usize) -> PolarsResult<Option<FattyAcid>> {
        let Some(carbon) = self.0.field_by_name(CARBON)?.u8()?.get(idx) else {
            return Ok(None);
        };
        let Some(indices) = self.0.field_by_name(INDICES)?.list()?.get_as_series(idx) else {
            return Ok(None);
        };
        let indices = IndicesChunked::new(indices.struct_()?);
        let index = indices.index()?;
        let triple = indices.triple()?;
        let parity = indices.parity()?;
        let unsaturated = index
            .iter()
            .zip(&triple)
            .zip(&parity)
            .map(|((index, triple), parity)| Unsaturated {
                index,
                triple,
                parity,
            })
            .collect();
        Ok(Some(FattyAcid::new(carbon, unsaturated)))
    }

    #[inline]
    pub fn get_any_value(&self, idx: usize) -> PolarsResult<AnyValue<'_>> {
        self.0.get_any_value(idx)
    }
}

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
        self.carbon()?
            .into_iter()
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
                let triple = indices.triple()?;
                let parity = indices.parity()?;
                let unsaturated = index
                    .iter()
                    .zip(&triple)
                    .zip(&parity)
                    .map(|((index, triple), parity)| Unsaturated {
                        index,
                        triple,
                        parity,
                    })
                    .collect();
                let fatty_acid = FattyAcid::new(carbon, unsaturated);
                Ok(Some(fatty_acid.id().to_string()))
            })
            .collect()
    }

    pub fn delta(&self) -> PolarsResult<StringChunked> {
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
                let triple = indices.triple()?;
                let parity = indices.parity()?;
                let unsaturated = index
                    .iter()
                    .zip(&triple)
                    .zip(&parity)
                    .map(|((index, triple), parity)| Unsaturated {
                        index,
                        triple,
                        parity,
                    })
                    .collect();
                let fatty_acid = FattyAcid::new(carbon, unsaturated);
                Ok(Some(fatty_acid.delta().to_string()))
            })
            .collect()
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
        Ok(self.indices()?.lst_lengths().equal(1))
    }

    pub fn is_polyunsaturated(&self) -> PolarsResult<BooleanChunked> {
        Ok(self.indices()?.lst_lengths().gt(1))
    }

    pub fn is_saturated(&self) -> PolarsResult<BooleanChunked> {
        Ok(self.indices()?.lst_lengths().equal(0))
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
                                if index.is_empty() {
                                    return Ok(Some(false));
                                }
                                let Some(last) = index.last() else {
                                    return Ok(Some(false));
                                };
                                last == carbon - omega.unsigned_abs()
                            }
                            delta @ 0.. => {
                                if index.is_empty() {
                                    return Ok(Some(false));
                                }
                                let Some(first) = index.first() else {
                                    return Ok(Some(false));
                                };
                                first == delta as u8
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
            .map(|indices| {
                let Some(indices) = indices else {
                    return Ok(None);
                };
                let indices = IndicesChunked::new(indices.as_ref().struct_()?);
                let triple = indices.triple()?;
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

fn check_data_type(r#struct: &StructChunked) -> PolarsResult<()> {
    polars_ensure!(
        *r#struct.dtype() == data_type!(FATTY_ACID),
        SchemaMismatch: "invalid fatty acid data type: expected `FATTY_ACID`, got = `{}`",
        r#struct.dtype(),
    );
    Ok(())
}
