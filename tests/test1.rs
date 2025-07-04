#![feature(formatting_options)]

use anyhow::Result;
use lipid::{polars::FATTY_ACID, prelude::*};
use polars::prelude::*;
use polars_arrow::array::MutableBinaryViewArray;
use std::{collections::BTreeMap, default, fmt::Sign, fs::File};

#[test]
fn test() -> Result<()> {
    let mut data_frame = df! {
        "FattyAcid" => df! {
            "Formula" => df! {
                "C" => UInt8Chunked::new(PlSmallStr::EMPTY, [16u8, 18, 18]),
                "H" => UInt8Chunked::new(PlSmallStr::EMPTY, [32, 30, 32]),
                "O" => UInt8Chunked::full(PlSmallStr::EMPTY, 2, 3),
            }?.into_struct(PlSmallStr::EMPTY),
            "DoubleBonds" => &[
                Series::new_empty(PlSmallStr::EMPTY, &DataType::Struct(vec![
                    Field::new(PlSmallStr::from_static("Index"), DataType::Int8),
                    Field::new(PlSmallStr::from_static("Parity"), DataType::Boolean),
                ])),
                df! {
                    "Index" => Series::from_iter([9i8, 12, 15]),
                    "Parity" => Series::from_iter([false, false, false]),
                }?.into_struct(PlSmallStr::EMPTY).into_series(),
                df! {
                    "Index" => Series::from_iter([9i8, 12]),
                    "Parity" => Series::from_iter([false, false]),
                }?.into_struct(PlSmallStr::EMPTY).into_series(),
            ],
            "TripleBonds" => &[
                Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8),
                Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8),
                Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8),
            ],
        }?.into_struct(PlSmallStr::EMPTY),
    }?;
    // "Value" => [
    //     Some(1f64),
    //     Some(2f64),
    //     Some(3f64),
    //     Some(4f64),
    //     Some(5f64),
    // ],
    println!("data_frame: {data_frame}");
    let mut file = File::create("file.parquet").expect("could not create file");
    let mut writer = ParquetWriter::new(&mut file);
    // let mut writer = IpcWriter::new(&mut file);
    // let custom_metadata = [
    //     ("first_name".into(), "John".into()),
    //     ("last_name".into(), "Doe".into()),
    // ]
    // .into_iter()
    // .collect();
    // writer.set_custom_schema_metadata(Arc::new(custom_metadata));
    writer.finish(&mut data_frame).unwrap();
    Ok(())
}

// #[test]
// fn test1() -> Result<()> {
//     Series::from_iter([
//         Series::new_empty(
//             PlSmallStr::EMPTY,
//             &DataType::Struct(vec![
//                 Field::new(PlSmallStr::from_static("Index"), DataType::Int8),
//                 Field::new(PlSmallStr::from_static("Parity"), DataType::Boolean),
//             ]),
//         ),
//         df! {
//             "Index" => Series::from_iter([9i8, 12, 15]),
//             "Parity" => Series::from_iter([false, false, false]),
//         }?
//         .into_struct(PlSmallStr::EMPTY)
//         .into_series(),
//         df! {
//             "Index" => Series::from_iter([9i8, 12]),
//             "Parity" => Series::from_iter([false, false]),
//         }?
//         .into_struct(PlSmallStr::EMPTY)
//         .into_series(),
//     ]);
// }
