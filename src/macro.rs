#[macro_export]
macro_rules! data_type {
    (BOUND) => {
        DataType::Struct(vec![field!(INDEX), field!(PARITY), field!(TRIPLE)])
    };
    (BOUNDS) => {
        DataType::List(Box::new(data_type!(BOUND)))
    };
    (FATTY_ACID) => {
        DataType::Struct(vec![field!(CARBON), field!(BOUNDS)])
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
// (STEREOSPECIFIC_NUMBER) => {
//     DataType::Struct(vec![field!(LABEL), field!(FATTY_ACID)])
// };

#[macro_export]
macro_rules! field {
    (BOUNDS) => {
        Field::new(PlSmallStr::from_static(BOUNDS), data_type!(BOUNDS))
    };
    (CARBON) => {
        Field::new(PlSmallStr::from_static(CARBON), DataType::UInt8)
    };
    (FATTY_ACID) => {
        Field::new(PlSmallStr::from_static(FATTY_ACID), data_type!(FATTY_ACID))
    };
    (INDEX) => {
        Field::new(PlSmallStr::from_static(INDEX), DataType::UInt8)
    };
    (PARITY) => {
        Field::new(PlSmallStr::from_static(PARITY), DataType::Boolean)
    };
    (TRIPLE) => {
        Field::new(PlSmallStr::from_static(TRIPLE), DataType::Boolean)
    };
}
// (LABEL) => {
//     Field::new(PlSmallStr::from_static(LABEL), DataType::String)
// };

// #[macro_export]
// macro_rules! any_value {
//     (BOUND, $index:literal, $parity:literal, $triple:literal) => {
//         AnyValue::StructOwned(Box::new((
//             vec![
//                 AnyValue::UInt8($index),
//                 AnyValue::Boolean($parity),
//                 AnyValue::Boolean($triple),
//             ],
//             vec![field!(INDEX), field!(PARITY), field!(TRIPLE)],
//         )))
//     };
// }
