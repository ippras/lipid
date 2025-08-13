use crate::prelude::*;
use polars::prelude::*;

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
}

impl TriacylglycerolChunked {
    pub fn stereospecific_number1(&self) -> PolarsResult<Series> {
        self.0.field_by_name(STEREOSPECIFIC_NUMBER1)
    }

    pub fn stereospecific_number2(&self) -> PolarsResult<Series> {
        self.0.field_by_name(STEREOSPECIFIC_NUMBER2)
    }

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

fn check_data_type(r#struct: &StructChunked) -> PolarsResult<()> {
    let data_type = r#struct.dtype();
    if let DataType::Struct(fields) = data_type
        && let [fields1, fields2, fields3] = &**fields
        && fields1.name == STEREOSPECIFIC_NUMBER1
        && fields2.name == STEREOSPECIFIC_NUMBER2
        && fields3.name == STEREOSPECIFIC_NUMBER3
    {
        return Ok(());
    }
    polars_bail!(SchemaMismatch: "invalid triacylglycerol data type: expected `Struct {{ {STEREOSPECIFIC_NUMBER1}, {STEREOSPECIFIC_NUMBER2}, {STEREOSPECIFIC_NUMBER3} }}`, got = `{data_type}`");
    // polars_ensure!(
    //     *r#struct.dtype() == data_type!(TRIACYLGLYCEROL),
    //     SchemaMismatch: "invalid triacylglycerol data type: expected `TRIACYLGLYCEROL`, got = `{}`",
    //     r#struct.dtype(),
    // );
    // Ok(())
}
