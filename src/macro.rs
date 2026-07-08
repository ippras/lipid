#[macro_export]
macro_rules! data_type {
    (INDEX) => {
        DataType::Struct(vec![field!(INDEX), field!(TRIPLE), field!(PARITY)])
    };
    (INDICES) => {
        DataType::List(Box::new(data_type!(INDEX)))
    };
    (FATTY_ACID) => {
        DataType::Struct(vec![field!(CARBON), field!(UNSATURATED), field!(INDICES)])
    };
    (TRIACYLGLYCEROL) => {
        DataType::Struct(vec![
            Field::new(
                PlSmallStr::from_static(STEREOSPECIFIC_NUMBERS1),
                data_type!(FATTY_ACID),
            ),
            Field::new(
                PlSmallStr::from_static(STEREOSPECIFIC_NUMBERS2),
                data_type!(FATTY_ACID),
            ),
            Field::new(
                PlSmallStr::from_static(STEREOSPECIFIC_NUMBERS3),
                data_type!(FATTY_ACID),
            ),
        ])
    };
    ([$data_type:expr]) => {
        DataType::Struct(vec![
            Field::new(PlSmallStr::from_static(STEREOSPECIFIC_NUMBERS1), $data_type),
            Field::new(PlSmallStr::from_static(STEREOSPECIFIC_NUMBERS2), $data_type),
            Field::new(PlSmallStr::from_static(STEREOSPECIFIC_NUMBERS3), $data_type),
        ])
    };
}

#[macro_export]
macro_rules! field {
    (CARBON) => {
        Field::new(PlSmallStr::from_static(CARBON), DataType::UInt8)
    };
    (UNSATURATED) => {
        Field::new(PlSmallStr::from_static(UNSATURATED), DataType::UInt8)
    };
    (INDEX) => {
        Field::new(PlSmallStr::from_static(INDEX), DataType::UInt8)
    };
    (TRIPLE) => {
        Field::new(PlSmallStr::from_static(TRIPLE), DataType::Boolean)
    };
    (PARITY) => {
        Field::new(PlSmallStr::from_static(PARITY), DataType::Boolean)
    };
    (INDICES) => {
        Field::new(PlSmallStr::from_static(INDICES), data_type!(INDICES))
    };
    (FATTY_ACID) => {
        Field::new(PlSmallStr::from_static(FATTY_ACID), data_type!(FATTY_ACID))
    };
    (TRIACYLGLYCEROL) => {
        Field::new(
            PlSmallStr::from_static(TRIACYLGLYCEROL),
            data_type!(TRIACYLGLYCEROL),
        )
    };
    ($name:ident[$data_type:expr]) => {
        Field::new(PlSmallStr::from_static($name), data_type!([$data_type]))
    };
}
