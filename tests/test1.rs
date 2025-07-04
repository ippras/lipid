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
            FORMULA => df! {
                "C" => UInt8Chunked::new(PlSmallStr::EMPTY, [16u8, 18, 18]),
                "H" => UInt8Chunked::new(PlSmallStr::EMPTY, [32, 30, 32]),
                "O" => UInt8Chunked::full(PlSmallStr::EMPTY, 2, 3),
            }?.into_struct(PlSmallStr::EMPTY),
            DOUBLE_BOUNDS => &[
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
            TRIPLE_BOUNDS => &[
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
    let lazy_frame = data_frame.clone().lazy().filter(
        col("FattyAcid")
            .struct_()
            .field_by_name(DOUBLE_BOUNDS)
            .list()
            .len()
            .gt(lit(0))
            .or(col("FattyAcid")
                .struct_()
                .field_by_name(TRIPLE_BOUNDS)
                .list()
                .len()
                .gt(lit(0))),
    );
    println!("lazy_frame: {}", lazy_frame.collect().unwrap());

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
//     use lipid::polars::chunked_array::fatty_acid_new::{FATTY_ACID_DATA_TYPE, SeriesExt as _};

//     let series = df! {
//         "Formula" => df! {
//             "C" => UInt8Chunked::new(PlSmallStr::EMPTY, [16u8, 18, 18]),
//             "H" => UInt8Chunked::new(PlSmallStr::EMPTY, [32, 30, 32]),
//             "O" => UInt8Chunked::full(PlSmallStr::EMPTY, 2, 3),
//         }?.into_struct(PlSmallStr::EMPTY),
//         "DoubleBounds" => &[
//             Series::new_empty(PlSmallStr::EMPTY, &DataType::Struct(vec![
//                 Field::new(PlSmallStr::from_static("Index"), DataType::Int8),
//                 Field::new(PlSmallStr::from_static("Parity"), DataType::Boolean),
//             ])),
//             df! {
//                 "Index" => Series::from_iter([9i8, 12, 15]),
//                 "Parity" => Series::from_iter([false, false, false]),
//             }?.into_struct(PlSmallStr::EMPTY).into_series(),
//             df! {
//                 "Index" => Series::from_iter([9i8, 12]),
//                 "Parity" => Series::from_iter([false, false]),
//             }?.into_struct(PlSmallStr::EMPTY).into_series(),
//         ],
//         "TripleBounds" => &[
//             Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8),
//             Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8),
//             Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8),
//         ],
//     }?
//     .into_struct(PlSmallStr::EMPTY)
//     .into_series();
//     println!("series: {series}");
//     let fatty_acid = series.NEW_try_fatty_acid()?;
//     println!("dtype: {}", series.dtype());
//     assert_eq!(series.dtype(), &*FATTY_ACID_DATA_TYPE);
//     println!("series: {fatty_acid:#?}");
//     // for fatty_acid in fatty_acid.iter()? {
//     //     println!("fatty_acid: {fatty_acid:?}");
//     // }
//     Ok(())
// }

#[test]
fn test2() -> Result<()> {
    let series = df! {
        "Formula" => df! {
            "C" => UInt8Chunked::new(PlSmallStr::EMPTY, [16u8, 18, 18]),
            "H" => UInt8Chunked::new(PlSmallStr::EMPTY, [32, 30, 32]),
            "O" => UInt8Chunked::full(PlSmallStr::EMPTY, 2, 3),
        }?.into_struct(PlSmallStr::EMPTY),
        "DoubleBounds" => &[
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
        "TripleBounds" => &[
            Series::new(PlSmallStr::EMPTY, [9i8, 12, 15]),
            Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8),
            Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8),
        ],
    }?
    .into_struct(PlSmallStr::EMPTY)
    .into_series();
    println!("series: {series}");
    let fatty_acid = series.try_fatty_acid()?;
    println!("fatty_acid: {fatty_acid:?}");
    println!("is_saturated: {:?}", fatty_acid.is_saturated());
    println!("unsaturation: {:?}", fatty_acid.unsaturation());
    // .filter(
    //         // Условие: длина списка в колонке "DoubleBounds" больше 0
    //         col("DoubleBounds").list().len().gt(lit(0u32))
    //     )
    // col("list_of_structs").list().eval(
    //     element().struct_().field_by_name("is_active")?.any(),
    //     true
    // )
    // series.filter(series.NEW_try_fatty_acid()?.is_saturated());
    Ok(())
}
