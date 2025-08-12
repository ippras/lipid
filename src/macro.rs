#[macro_export]
macro_rules! data_type {
    (INDEX) => {
        DataType::Struct(vec![field!(INDEX), field!(TRIPLE), field!(PARITY)])
    };
    (INDICES) => {
        DataType::List(Box::new(data_type!(INDEX)))
    };
    (FATTY_ACID) => {
        DataType::Struct(vec![field!(CARBON), field!(INDICES)])
    };
    (TRIACYLGLYCEROL) => {
        DataType::Struct(vec![
            Field::new(
                PlSmallStr::from_static(STEREOSPECIFIC_NUMBER1),
                data_type!(FATTY_ACID),
            ),
            Field::new(
                PlSmallStr::from_static(STEREOSPECIFIC_NUMBER2),
                data_type!(FATTY_ACID),
            ),
            Field::new(
                PlSmallStr::from_static(STEREOSPECIFIC_NUMBER3),
                data_type!(FATTY_ACID),
            ),
        ])
    };
}

#[macro_export]
macro_rules! field {
    (CARBON) => {
        Field::new(PlSmallStr::from_static(CARBON), DataType::UInt8)
    };
    (FATTY_ACID) => {
        Field::new(PlSmallStr::from_static(FATTY_ACID), data_type!(FATTY_ACID))
    };
    (INDEX) => {
        Field::new(PlSmallStr::from_static(INDEX), DataType::UInt8)
    };
    (INDICES) => {
        Field::new(PlSmallStr::from_static(INDICES), data_type!(INDICES))
    };
    (PARITY) => {
        Field::new(PlSmallStr::from_static(PARITY), DataType::Boolean)
    };
    (TRIACYLGLYCEROL) => {
        Field::new(
            PlSmallStr::from_static(TRIACYLGLYCEROL),
            data_type!(TRIACYLGLYCEROL),
        )
    };
    (TRIPLE) => {
        Field::new(PlSmallStr::from_static(TRIPLE), DataType::Boolean)
    };
}
