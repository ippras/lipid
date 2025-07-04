// src/main.rs (продолжение)
use polars::{
    datatypes::{AnyValue, PlSmallStr},
    prelude::*,
};
use std::{collections::HashMap, fmt};

// Оболочка для структуры Formula
#[derive(Debug, Clone)]
struct Formula {
    c: u8,
    h: u8,
    o: u8,
}

impl fmt::Display for Formula {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "C{}H{}O{}", self.c, self.h, self.o)
    }
}

// Оболочка для структуры DoubleBound
#[derive(Debug, Clone)]
struct DoubleBound {
    index: i8,
    parity: bool,
}

// Главная оболочка для всей строки FattyAcid
#[derive(Debug, Clone)]
struct FattyAcid {
    formula: Formula,
    double_bounds: Vec<DoubleBound>,
    triple_bounds: Vec<i8>,
}

// Утилита для преобразования полей структуры в HashMap
fn struct_to_map(
    payload: (Vec<AnyValue<'_>>, Vec<PlSmallStr>),
) -> HashMap<PlSmallStr, AnyValue<'_>> {
    let (values, names) = payload;
    names.into_iter().zip(values.into_iter()).collect()
}

// Парсер для Formula
impl<'a> TryFrom<AnyValue<'a>> for Formula {
    type Error = PolarsError;

    fn try_from(value: AnyValue<'a>) -> PolarsResult<Self> {
        if let AnyValue::StructOwned(payload) = value {
            let map = struct_to_map(payload);
            let c = map
                .get("C")
                .ok_or_else(|| polars_err!(ComputeError: "Missing 'C' field"))?
                .try_extract()?;
            let h = map
                .get("H")
                .ok_or_else(|| polars_err!(ComputeError: "Missing 'H' field"))?
                .try_extract()?;
            let o = map
                .get("O")
                .ok_or_else(|| polars_err!(ComputeError: "Missing 'O' field"))?
                .try_extract()?;
            Ok(Formula { c, h, o })
        } else {
            polars_bail!(ComputeError: "Expected a struct to create Formula, got {:?}", value);
        }
    }
}

// Парсер для DoubleBound
impl<'a> TryFrom<AnyValue<'a>> for DoubleBound {
    type Error = PolarsError;

    fn try_from(value: AnyValue<'a>) -> PolarsResult<Self> {
        if let AnyValue::StructOwned(payload) = value {
            let map = struct_to_map(payload);
            let index = map
                .get("Index")
                .ok_or_else(|| polars_err!(ComputeError: "Missing 'Index' field"))?
                .try_extract()?;
            let parity = map
                .get("Parity")
                .ok_or_else(|| polars_err!(ComputeError: "Missing 'Parity' field"))?
                .try_extract()?;
            Ok(DoubleBound { index, parity })
        } else {
            polars_bail!(ComputeError: "Expected a struct to create DoubleBound, got {:?}", value);
        }
    }
}

// Главный парсер для FattyAcid
impl<'a> TryFrom<AnyValue<'a>> for FattyAcid {
    type Error = PolarsError;

    fn try_from(value: AnyValue<'a>) -> PolarsResult<Self> {
        if let AnyValue::StructOwned(payload) = value {
            let map = struct_to_map(payload);

            // Парсим вложенную структуру Formula
            let formula = Formula::try_from(map.get("FORMULA").unwrap().clone())?;

            // Парсим список структур DoubleBounds
            let double_bounds = if let AnyValue::List(series) = map.get("DOUBLE_BOUNDS").unwrap() {
                series
                    .struct_()?
                    .amortized_iter()
                    .map(|row_val| DoubleBound::try_from(row_val.unwrap()))
                    .collect::<PolarsResult<Vec<_>>>()?
            } else {
                vec![]
            };

            // Парсим список i8 TripleBounds
            let triple_bounds = if let AnyValue::List(series) = map.get("TRIPLE_BOUNDS").unwrap() {
                series.i8()?.into_iter().filter_map(|opt| opt).collect()
            } else {
                vec![]
            };

            Ok(FattyAcid {
                formula,
                double_bounds,
                triple_bounds,
            })
        } else {
            polars_bail!(ComputeError: "Expected a struct to create FattyAcid, got {:?}", value);
        }
    }
}

fn test() -> PolarsResult<()> {
    // Константы для имен полей
    const FORMULA: &str = "FORMULA";
    const DOUBLE_BOUNDS: &str = "DOUBLE_BOUNDS";
    const TRIPLE_BOUNDS: &str = "TRIPLE_BOUNDS";

    // Ваш код для создания DataFrame
    let data_frame = df! {
        "FattyAcid" => df! {
            FORMULA => df! {
                "C" => UInt8Chunked::new(PlSmallStr::EMPTY, &[16u8, 18, 18]),
                "H" => UInt8Chunked::new(PlSmallStr::EMPTY, &[32, 30, 32]),
                "O" => UInt8Chunked::full(PlSmallStr::EMPTY, 2, 3),
            }?.into_struct(PlSmallStr::EMPTY),
            DOUBLE_BOUNDS => ListChunked::from_iter([
                Series::new_empty(PlSmallStr::EMPTY, &DataType::Struct(vec![
                    Field::new(PlSmallStr::from_static("Index"), DataType::Int8),
                    Field::new(PlSmallStr::from_static("Parity"), DataType::Boolean),
                ])),
                df! {
                    "Index" => &[9i8, 12, 15],
                    "Parity" => &[false, false, false],
                }?.into_struct(PlSmallStr::EMPTY).into_series(),
                df! {
                    "Index" => &[9i8, 12],
                    "Parity" => &[false, false],
                }?.into_struct(PlSmallStr::EMPTY).into_series(),
            ]),
            TRIPLE_BOUNDS => ListChunked::from_iter([
                Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8),
                Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8),
                Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8),
            ]),
        }?.into_struct(PlSmallStr::EMPTY),
    }?;

    println!("Исходный DataFrame:\n{}", data_frame);

    // 1. Получаем колонку StructChunked
    let fatty_acid_ca = data_frame.column("FattyAcid")?.struct_()?;

    // 2. Итерируем по строкам, используя быстрый amortized_iter
    println!("\nИтерация с использованием удобных оболочек:");
    for (i, row_any_value) in fatty_acid_ca.amortized_iter().enumerate() {
        if let Some(row) = row_any_value {
            // 3. Преобразуем AnyValue в нашу строго типизированную оболочку
            let acid = FattyAcid::try_from(row)?;

            // 4. Теперь работаем с удобными полями и методами!
            println!("--- Запись {} ---", i);
            println!("Формула: {}", acid.formula); // Используем наш Display трейт
            println!(
                "Двойные связи ({}): {:?}",
                acid.double_bounds.len(),
                acid.double_bounds
            );
            println!(
                "Тройные связи ({}): {:?}",
                acid.triple_bounds.len(),
                acid.triple_bounds
            );
        }
    }

    Ok(())
}
