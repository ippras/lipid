#[macro_export]
macro_rules! data_type {
    (CARBON) => {
        ::polars::datatypes::DataType::Struct(vec![field!(START), field!(END)])
    };
    (RANGE) => {
        ::polars::datatypes::DataType::Struct(vec![field!(START), field!(END)])
    };
    (INDEX) => {
        ::polars::datatypes::DataType::Struct(vec![field!(INDEX), field!(TRIPLE), field!(PARITY)])
    };
    (INDICES) => {
        ::polars::datatypes::DataType::List(Box::new(data_type!(INDEX)))
    };
    (UNSATURATED) => {
        ::polars::datatypes::DataType::Struct(vec![field!(RANGE), field!(INDICES)])
    };
    (FATTY_ACID) => {
        ::polars::datatypes::DataType::Struct(vec![field!(CARBON), field!(UNSATURATED)])
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
    (START) => {
        ::polars::datatypes::Field::new(
            PlSmallStr::from_static(START),
            ::polars::datatypes::DataType::UInt8,
        )
    };
    (END) => {
        ::polars::datatypes::Field::new(
            PlSmallStr::from_static(END),
            ::polars::datatypes::DataType::UInt8,
        )
    };
    (CARBON) => {
        ::polars::datatypes::Field::new(PlSmallStr::from_static(CARBON), data_type!(CARBON))
    };
    (RANGE) => {
        ::polars::datatypes::Field::new(PlSmallStr::from_static(RANGE), data_type!(CARBON))
    };
    (INDEX) => {
        ::polars::datatypes::Field::new(
            PlSmallStr::from_static(INDEX),
            ::polars::datatypes::DataType::UInt8,
        )
    };
    (TRIPLE) => {
        ::polars::datatypes::Field::new(
            PlSmallStr::from_static(TRIPLE),
            ::polars::datatypes::DataType::Boolean,
        )
    };
    (PARITY) => {
        ::polars::datatypes::Field::new(
            PlSmallStr::from_static(PARITY),
            ::polars::datatypes::DataType::Boolean,
        )
    };
    (INDICES) => {
        ::polars::datatypes::Field::new(PlSmallStr::from_static(INDICES), data_type!(INDICES))
    };
    (UNSATURATED) => {
        ::polars::datatypes::Field::new(
            PlSmallStr::from_static(UNSATURATED),
            data_type!(UNSATURATED),
        )
    };
    (FATTY_ACID) => {
        ::polars::datatypes::Field::new(PlSmallStr::from_static(FATTY_ACID), data_type!(FATTY_ACID))
    };
    (TRIACYLGLYCEROL) => {
        ::polars::datatypes::Field::new(
            PlSmallStr::from_static(TRIACYLGLYCEROL),
            data_type!(TRIACYLGLYCEROL),
        )
    };
    ($name:ident[$data_type:expr]) => {
        ::polars::datatypes::Field::new(PlSmallStr::from_static($name), data_type!([$data_type]))
    };
}
