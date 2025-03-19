use super::*;

fn mass(data_frame: DataFrame) -> PolarsResult<f64> {
    let data_frame = data_frame
        .lazy()
        .select([col("Triacylglycerol")
            .triacylglycerol()
            .map_expr(|expr| expr.struct_().field_by_name("FattyAcid"))
            .triacylglycerol()
            .mass(None)
            .round(2)])
        .collect()?;
    Ok(data_frame["Mass"].f64()?.get(0).unwrap())
}

#[test]
fn fatty_acid() -> PolarsResult<()> {
    // MPS
    assert_eq!(mass(c14u0c16u0c18u0()?)?, 806.74);
    assert_eq!(mass(c14u0c18u0c16u0()?)?, 806.74);
    assert_eq!(mass(c16u0c14u0c18u0()?)?, 806.74);
    assert_eq!(mass(c16u0c18u0c14u0()?)?, 806.74);
    assert_eq!(mass(c18u0c14u0c16u0()?)?, 806.74);
    assert_eq!(mass(c18u0c16u0c14u0()?)?, 806.74);
    // SOL
    assert_eq!(mass(c18u0c18u1dc9c18u2dc9dc12()?)?, 884.78);
    assert_eq!(mass(c18u0c18u2dc9dc12c18u1dc9()?)?, 884.78);
    assert_eq!(mass(c18u1dc9c18u0c18u2dc9dc12()?)?, 884.78);
    assert_eq!(mass(c18u1dc9c18u2dc9dc12c18u0()?)?, 884.78);
    assert_eq!(mass(c18u2dc9dc12c18u0c18u1dc9()?)?, 884.78);
    assert_eq!(mass(c18u2dc9dc12c18u1dc9c18u0()?)?, 884.78);
    Ok(())
}
