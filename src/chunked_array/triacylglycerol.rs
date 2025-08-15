use crate::prelude::*;
use polars::prelude::*;

/// Triacylglycerol chunked
#[repr(transparent)]
pub struct TriacylglycerolChunked(pub(crate) StructChunked);

impl TriacylglycerolChunked {
    pub fn new(r#struct: StructChunked) -> Self {
        Self(r#struct)
    }

    pub fn try_new(r#struct: StructChunked) -> PolarsResult<Self> {
        check_data_type(&r#struct)?;
        Ok(Self(r#struct))
    }

    #[inline]
    pub fn get(&self, idx: usize) -> PolarsResult<Triacylglycerol<Option<FattyAcid>>> {
        let stereospecific_number1 = self.stereospecific_number1()?.try_fatty_acid()?.get(idx)?;
        let stereospecific_number2 = self.stereospecific_number2()?.try_fatty_acid()?.get(idx)?;
        let stereospecific_number3 = self.stereospecific_number3()?.try_fatty_acid()?.get(idx)?;
        Ok(Triacylglycerol([
            stereospecific_number1,
            stereospecific_number2,
            stereospecific_number3,
        ]))
    }

    #[inline]
    pub fn get_any_value(&self, idx: usize) -> PolarsResult<Triacylglycerol<AnyValue<'static>>> {
        let stereospecific_number1 = self.stereospecific_number1()?.get(idx)?.into_static();
        let stereospecific_number2 = self.stereospecific_number2()?.get(idx)?.into_static();
        let stereospecific_number3 = self.stereospecific_number3()?.get(idx)?.into_static();
        Ok(Triacylglycerol([
            stereospecific_number1,
            stereospecific_number2,
            stereospecific_number3,
        ]))
    }

    // #[inline]
    // pub fn map(&self) -> PolarsResult<Triacylglycerol<FattyAcidChunkedFields>> {
    //     Ok(Triacylglycerol([
    //         self.stereospecific_number1()?.try_fatty_acid()?.fields()?,
    //         self.stereospecific_number2()?.try_fatty_acid()?.fields()?,
    //         self.stereospecific_number3()?.try_fatty_acid()?.fields()?,
    //     ]))
    // }

    #[inline]
    pub fn fields<T>(
        &self,
        f: impl Fn(Series) -> PolarsResult<T>,
    ) -> PolarsResult<Triacylglycerol<T>> {
        Ok(Triacylglycerol([
            f(self.stereospecific_number1()?)?,
            f(self.stereospecific_number2()?)?,
            f(self.stereospecific_number3()?)?,
        ]))
    }
}

impl TriacylglycerolChunked {
    #[inline]
    pub fn stereospecific_number1(&self) -> PolarsResult<Series> {
        self.0.field_by_name(STEREOSPECIFIC_NUMBER1)
    }

    #[inline]
    pub fn stereospecific_number2(&self) -> PolarsResult<Series> {
        self.0.field_by_name(STEREOSPECIFIC_NUMBER2)
    }

    #[inline]
    pub fn stereospecific_number3(&self) -> PolarsResult<Series> {
        self.0.field_by_name(STEREOSPECIFIC_NUMBER3)
    }
}

impl<'a> TryFrom<&'a Series> for &'a TriacylglycerolChunked {
    type Error = PolarsError;

    fn try_from(value: &'a Series) -> Result<Self, Self::Error> {
        value.struct_()?.try_into()
    }
}

impl TryFrom<StructChunked> for TriacylglycerolChunked {
    type Error = PolarsError;

    fn try_from(value: StructChunked) -> Result<Self, Self::Error> {
        check_data_type(&value)?;
        Ok(Self(value))
    }
}

impl<'a> TryFrom<&'a StructChunked> for &'a TriacylglycerolChunked {
    type Error = PolarsError;

    fn try_from(value: &'a StructChunked) -> Result<Self, Self::Error> {
        check_data_type(value)?;
        // [safe](https://doc.rust-lang.org/reference/type-layout.html?highlight=transparent#the-transparent-representation)
        Ok(unsafe { &*(value as *const StructChunked as *const TriacylglycerolChunked) })
    }
}

// impl<T> Triacylglycerol<T> {
//     pub fn iter(&self) -> impl Iterator<Item = Triacylglycerol<<&T as IntoIterator>::Item>>
//     where
//         for<'a> &'a T: IntoIterator,
//     {
//         (&self.0[0])
//             .into_iter()
//             .zip(self.0[1].into_iter())
//             .zip(self.0[2].into_iter())
//             .map(
//                 |((stereospecific_number1, stereospecific_number2), stereospecific_number3)| {
//                     Triacylglycerol([
//                         stereospecific_number1,
//                         stereospecific_number2,
//                         stereospecific_number3,
//                     ])
//                 },
//             )
//     }
// }
impl<T: IntoIterator> Triacylglycerol<T> {
    pub fn iter(self) -> impl Iterator<Item = Triacylglycerol<T::Item>> {
        let [
            stereospecific_number1,
            stereospecific_number2,
            stereospecific_number3,
        ] = self.0;
        stereospecific_number1
            .into_iter()
            .zip(stereospecific_number2.into_iter())
            .zip(stereospecific_number3.into_iter())
            .map(
                |((stereospecific_number1, stereospecific_number2), stereospecific_number3)| {
                    Triacylglycerol([
                        stereospecific_number1,
                        stereospecific_number2,
                        stereospecific_number3,
                    ])
                },
            )
    }
}

impl<T: IntoIterator> IntoIterator for Triacylglycerol<T> {
    type Item = Triacylglycerol<T::Item>;

    type IntoIter = impl Iterator<Item = Triacylglycerol<T::Item>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

fn check_data_type(r#struct: &StructChunked) -> PolarsResult<()> {
    let data_type = DataType::Struct(vec![
        Field::new(
            PlSmallStr::from_static(STEREOSPECIFIC_NUMBER1),
            DataType::Null,
        ),
        Field::new(
            PlSmallStr::from_static(STEREOSPECIFIC_NUMBER2),
            DataType::Null,
        ),
        Field::new(
            PlSmallStr::from_static(STEREOSPECIFIC_NUMBER3),
            DataType::Null,
        ),
    ]);
    data_type.matches_schema_type(r#struct.dtype())?;
    Ok(())

    // if let DataType::Struct(fields) = data_type
    //     && let [fields1, fields2, fields3] = &**fields
    //     && fields1.name == STEREOSPECIFIC_NUMBER1
    //     && fields2.name == STEREOSPECIFIC_NUMBER2
    //     && fields3.name == STEREOSPECIFIC_NUMBER3
    // {
    //     return Ok(());
    // }
    // polars_bail!(SchemaMismatch: "invalid triacylglycerol data type: expected `Struct {{ {STEREOSPECIFIC_NUMBER1}, {STEREOSPECIFIC_NUMBER2}, {STEREOSPECIFIC_NUMBER3} }}`, got = `{data_type}`");

    // polars_ensure!(
    //     *r#struct.dtype() == data_type!(TRIACYLGLYCEROL),
    //     SchemaMismatch: "invalid triacylglycerol data type: expected `TRIACYLGLYCEROL`, got = `{}`",
    //     r#struct.dtype(),
    // );
    // Ok(())
}
