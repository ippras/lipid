#[macro_export]
macro_rules! data_type {
    (INDEX) => {
        ::polars::datatypes::DataType::Struct(vec![field!(INDEX), field!(TRIPLE), field!(PARITY)])
    };
    (INDICES) => {
        ::polars::datatypes::DataType::List(Box::new(data_type!(INDEX)))
    };
    (FATTY_ACID) => {
        ::polars::datatypes::DataType::Struct(vec![field!(CARBON), field!(INDICES)])
    };
    (TRIACYLGLYCEROL) => {
        ::polars::datatypes::DataType::Struct(vec![
            ::polars::datatypes::Field::new(
                PlSmallStr::from_static(STEREOSPECIFIC_NUMBERS1),
                data_type!(FATTY_ACID),
            ),
            ::polars::datatypes::Field::new(
                PlSmallStr::from_static(STEREOSPECIFIC_NUMBERS2),
                data_type!(FATTY_ACID),
            ),
            ::polars::datatypes::Field::new(
                PlSmallStr::from_static(STEREOSPECIFIC_NUMBERS3),
                data_type!(FATTY_ACID),
            ),
        ])
    };
    ([$data_type:expr]) => {
        ::polars::datatypes::DataType::Struct(vec![
            ::polars::datatypes::Field::new(
                PlSmallStr::from_static(STEREOSPECIFIC_NUMBERS1),
                $data_type,
            ),
            ::polars::datatypes::Field::new(
                PlSmallStr::from_static(STEREOSPECIFIC_NUMBERS2),
                $data_type,
            ),
            ::polars::datatypes::Field::new(
                PlSmallStr::from_static(STEREOSPECIFIC_NUMBERS3),
                $data_type,
            ),
        ])
    };
}

#[macro_export]
macro_rules! field {
    (CARBON) => {
        ::polars::datatypes::Field::new(
            PlSmallStr::from_static(CARBON),
            ::polars::datatypes::DataType::UInt8,
        )
    };
    (FATTY_ACID) => {
        ::polars::datatypes::Field::new(PlSmallStr::from_static(FATTY_ACID), data_type!(FATTY_ACID))
    };
    (INDEX) => {
        ::polars::datatypes::Field::new(
            PlSmallStr::from_static(INDEX),
            ::polars::datatypes::DataType::UInt8,
        )
    };
    (INDICES) => {
        ::polars::datatypes::Field::new(PlSmallStr::from_static(INDICES), data_type!(INDICES))
    };
    (PARITY) => {
        ::polars::datatypes::Field::new(
            PlSmallStr::from_static(PARITY),
            ::polars::datatypes::DataType::Boolean,
        )
    };
    (TRIACYLGLYCEROL) => {
        ::polars::datatypes::Field::new(
            PlSmallStr::from_static(TRIACYLGLYCEROL),
            data_type!(TRIACYLGLYCEROL),
        )
    };
    (TRIPLE) => {
        ::polars::datatypes::Field::new(
            PlSmallStr::from_static(TRIPLE),
            ::polars::datatypes::DataType::Boolean,
        )
    };
    ($name:ident[$data_type:expr]) => {
        ::polars::datatypes::Field::new(PlSmallStr::from_static($name), data_type!([$data_type]))
    };
}
