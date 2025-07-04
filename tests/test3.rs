use polars::{
    lazy::dsl::{col, lit},
    prelude::*,
};

// Константы для имен полей
const FORMULA: &str = "FORMULA";
const DOUBLE_BOUNDS: &str = "DOUBLE_BOUNDS";
const TRIPLE_BOUNDS: &str = "TRIPLE_BOUNDS";

// Обертка для выражения, которое вернет колонку типа Struct<Formula>
#[derive(Clone)]
pub struct FormulaExpr(pub Expr);

// Обертка для выражения, которое вернет колонку типа Struct<FattyAcid>
#[derive(Clone)]
pub struct FattyAcidExpr(pub Expr);

impl FattyAcidExpr {
    /// Создает выражение для доступа к полю 'FORMULA'
    pub fn formula(&self) -> FormulaExpr {
        let formula_expr = self.0.clone().struct_().field_by_name(FORMULA); // В реальном коде лучше обработать ошибку
        FormulaExpr(formula_expr)
    }

    /// Создает выражение для доступа к полю 'DOUBLE_BOUNDS'
    pub fn double_bounds(&self) -> Expr {
        self.0.clone().struct_().field_by_name(DOUBLE_BOUNDS)
    }

    // ... и так далее для других полей
}

impl FormulaExpr {
    /// Создает выражение для доступа к полю 'C'
    pub fn c(&self) -> Expr {
        self.0.clone().struct_().field_by_name("C")
    }

    /// Создает выражение для доступа к полю 'H'
    pub fn h(&self) -> Expr {
        self.0.clone().struct_().field_by_name("H")
    }

    /// Создает выражение для доступа к полю 'O'
    pub fn o(&self) -> Expr {
        self.0.clone().struct_().field_by_name("O")
    }

    /// Создает выражение, которое форматирует формулу в строку "C..H..O.."
    pub fn to_string_expr(&self) -> PolarsResult<Expr> {
        // Используем format_str! для векторизованного форматирования строк
        format_str("C{}H{}O{}", &[self.c(), self.h(), self.o()])
    }
}

#[test]
fn test() -> PolarsResult<()> {
    // ... (код для создания data_frame из вашего вопроса)
    let mut data_frame = df! {
        "FattyAcid" => df! {
            FORMULA => df! {
                "C" => UInt8Chunked::new(PlSmallStr::EMPTY, &[16u8, 18, 18]),
                "H" => UInt8Chunked::new(PlSmallStr::EMPTY, &[32, 30, 32]),
                "O" => UInt8Chunked::full(PlSmallStr::EMPTY, 2, 3),
            }?.into_struct(PlSmallStr::EMPTY),
            DOUBLE_BOUNDS => ListChunked::from_series_iter([
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
            TRIPLE_BOUNDS => ListChunked::from_series_iter([
                Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8),
                Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8),
                Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8),
            ]),
        }?.into_struct(PlSmallStr::EMPTY),
    }?;

    // 1. Начинаем ленивый запрос
    let lazy_df = data_frame.lazy();

    // 2. Создаем корневую оболочку для колонки "FattyAcid"
    let fa_expr = FattyAcidExpr(col("FattyAcid"));

    // 3. Используем наш новый, чистый API для построения выражения
    let result_df = lazy_df
        .with_column(
            // fa_expr.formula() -> FormulaExpr
            // .to_string_expr() -> Expr
            fa_expr.formula().to_string_expr().alias("FormulaString"),
        )
        .collect()?;

    println!("{}", result_df);

    Ok(())
}
