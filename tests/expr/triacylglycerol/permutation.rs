use super::*;

fn non_stereospecific(data_frame: DataFrame, f: impl Fn(Expr) -> Expr) -> PolarsResult<DataFrame> {
    data_frame
        .lazy()
        .select([col("Triacylglycerol")
            .triacylglycerol()
            .non_stereospecific(f)?])
        .collect()
}

fn positional(data_frame: DataFrame, f: impl Fn(Expr) -> Expr) -> PolarsResult<DataFrame> {
    data_frame
        .lazy()
        .select([col("Triacylglycerol").triacylglycerol().positional(f)])
        .collect()
}

fn ecn(expr: Expr) -> Expr {
    expr.struct_()
        .field_by_name("FattyAcid")
        .fatty_acid()
        .equivalent_carbon_number()
}

fn mass(expr: Expr) -> Expr {
    expr.struct_()
        .field_by_name("FattyAcid")
        .fatty_acid()
        .mass(None)
}

fn species(expr: Expr) -> Expr {
    expr.struct_().field_by_name("Label")
}

fn unsaturation(expr: Expr) -> Expr {
    expr.struct_()
        .field_by_name("FattyAcid")
        .fatty_acid()
        .unsaturation()
}

#[rustfmt::skip]
#[test]
fn mc() -> PolarsResult<()> {
    let non_stereospecific = |data_frame| non_stereospecific(data_frame, mass);
    assert_eq!(non_stereospecific(c14u0c16u0c18u0()?)?, c14u0c16u0c18u0()?);
    assert_eq!(non_stereospecific(c14u0c18u0c16u0()?)?, c14u0c16u0c18u0()?);
    assert_eq!(non_stereospecific(c16u0c14u0c18u0()?)?, c14u0c16u0c18u0()?);
    assert_eq!(non_stereospecific(c16u0c18u0c14u0()?)?, c14u0c16u0c18u0()?);
    assert_eq!(non_stereospecific(c18u0c14u0c16u0()?)?, c14u0c16u0c18u0()?);
    assert_eq!(non_stereospecific(c18u0c16u0c14u0()?)?, c14u0c16u0c18u0()?);
    assert_eq!(non_stereospecific(c18u0c18u1dc9c18u2dc9dc12()?)?, c18u2dc9dc12c18u1dc9c18u0()?);
    assert_eq!(non_stereospecific(c18u0c18u2dc9dc12c18u1dc9()?)?, c18u2dc9dc12c18u1dc9c18u0()?);
    assert_eq!(non_stereospecific(c18u1dc9c18u0c18u2dc9dc12()?)?, c18u2dc9dc12c18u1dc9c18u0()?);
    assert_eq!(non_stereospecific(c18u1dc9c18u2dc9dc12c18u0()?)?, c18u2dc9dc12c18u1dc9c18u0()?);
    assert_eq!(non_stereospecific(c18u2dc9dc12c18u0c18u1dc9()?)?, c18u2dc9dc12c18u1dc9c18u0()?);
    assert_eq!(non_stereospecific(c18u2dc9dc12c18u1dc9c18u0()?)?, c18u2dc9dc12c18u1dc9c18u0()?);
    Ok(())
}

#[rustfmt::skip]
#[test]
fn pmc() -> PolarsResult<()> {
    let positional = |data_frame| positional(data_frame, mass);
    assert_eq!(positional(c14u0c16u0c18u0()?)?, c14u0c16u0c18u0()?);
    assert_eq!(positional(c14u0c18u0c16u0()?)?, c14u0c18u0c16u0()?);
    assert_eq!(positional(c16u0c14u0c18u0()?)?, c16u0c14u0c18u0()?);
    assert_eq!(positional(c16u0c18u0c14u0()?)?, c14u0c18u0c16u0()?);
    assert_eq!(positional(c18u0c14u0c16u0()?)?, c16u0c14u0c18u0()?);
    assert_eq!(positional(c18u0c16u0c14u0()?)?, c14u0c16u0c18u0()?);
    assert_eq!(positional(c18u0c18u1dc9c18u2dc9dc12()?)?, c18u2dc9dc12c18u1dc9c18u0()?);
    assert_eq!(positional(c18u0c18u2dc9dc12c18u1dc9()?)?, c18u1dc9c18u2dc9dc12c18u0()?);
    assert_eq!(positional(c18u1dc9c18u0c18u2dc9dc12()?)?, c18u2dc9dc12c18u0c18u1dc9()?);
    assert_eq!(positional(c18u1dc9c18u2dc9dc12c18u0()?)?, c18u1dc9c18u2dc9dc12c18u0()?);
    assert_eq!(positional(c18u2dc9dc12c18u0c18u1dc9()?)?, c18u2dc9dc12c18u0c18u1dc9()?);
    assert_eq!(positional(c18u2dc9dc12c18u1dc9c18u0()?)?, c18u2dc9dc12c18u1dc9c18u0()?);
    Ok(())
}

#[rustfmt::skip]
#[test]
fn nc() -> PolarsResult<()> {
    let non_stereospecific = |data_frame| non_stereospecific(data_frame, ecn);
    assert_eq!(non_stereospecific(c14u0c16u0c18u0()?)?, c14u0c16u0c18u0()?);
    assert_eq!(non_stereospecific(c14u0c18u0c16u0()?)?, c14u0c16u0c18u0()?);
    assert_eq!(non_stereospecific(c16u0c14u0c18u0()?)?, c14u0c16u0c18u0()?);
    assert_eq!(non_stereospecific(c16u0c18u0c14u0()?)?, c14u0c16u0c18u0()?);
    assert_eq!(non_stereospecific(c18u0c14u0c16u0()?)?, c14u0c16u0c18u0()?);
    assert_eq!(non_stereospecific(c18u0c16u0c14u0()?)?, c14u0c16u0c18u0()?);
    assert_eq!(non_stereospecific(c18u0c18u1dc9c18u2dc9dc12()?)?, c18u2dc9dc12c18u1dc9c18u0()?);
    assert_eq!(non_stereospecific(c18u0c18u2dc9dc12c18u1dc9()?)?, c18u2dc9dc12c18u1dc9c18u0()?);
    assert_eq!(non_stereospecific(c18u1dc9c18u0c18u2dc9dc12()?)?, c18u2dc9dc12c18u1dc9c18u0()?);
    assert_eq!(non_stereospecific(c18u1dc9c18u2dc9dc12c18u0()?)?, c18u2dc9dc12c18u1dc9c18u0()?);
    assert_eq!(non_stereospecific(c18u2dc9dc12c18u0c18u1dc9()?)?, c18u2dc9dc12c18u1dc9c18u0()?);
    assert_eq!(non_stereospecific(c18u2dc9dc12c18u1dc9c18u0()?)?, c18u2dc9dc12c18u1dc9c18u0()?);
    Ok(())
}

#[test]
fn sc() -> PolarsResult<()> {
    let non_stereospecific = |data_frame| non_stereospecific(data_frame, species);
    assert_eq!(non_stereospecific(c14u0c16u0c18u0()?)?, c14u0c16u0c18u0()?);
    assert_eq!(non_stereospecific(c14u0c18u0c16u0()?)?, c14u0c16u0c18u0()?);
    assert_eq!(non_stereospecific(c16u0c14u0c18u0()?)?, c14u0c16u0c18u0()?);
    assert_eq!(non_stereospecific(c16u0c18u0c14u0()?)?, c14u0c16u0c18u0()?);
    assert_eq!(non_stereospecific(c18u0c14u0c16u0()?)?, c14u0c16u0c18u0()?);
    assert_eq!(non_stereospecific(c18u0c16u0c14u0()?)?, c14u0c16u0c18u0()?);
    Ok(())
}

#[test]
fn psc() -> PolarsResult<()> {
    let positional = |data_frame| positional(data_frame, species);
    assert_eq!(positional(c14u0c16u0c18u0()?)?, c14u0c16u0c18u0()?);
    assert_eq!(positional(c14u0c18u0c16u0()?)?, c14u0c18u0c16u0()?);
    assert_eq!(positional(c16u0c14u0c18u0()?)?, c16u0c14u0c18u0()?);
    assert_eq!(positional(c16u0c18u0c14u0()?)?, c14u0c18u0c16u0()?);
    assert_eq!(positional(c18u0c14u0c16u0()?)?, c16u0c14u0c18u0()?);
    assert_eq!(positional(c18u0c16u0c14u0()?)?, c14u0c16u0c18u0()?);
    Ok(())
}

#[rustfmt::skip]
#[test]
fn tc() -> PolarsResult<()> {
    let non_stereospecific = |data_frame| non_stereospecific(data_frame, unsaturation);
    assert_eq!(non_stereospecific(c18u0c18u1dc9c18u2dc9dc12()?)?, c18u0c18u1dc9c18u2dc9dc12()?);
    assert_eq!(non_stereospecific(c18u0c18u2dc9dc12c18u1dc9()?)?, c18u0c18u1dc9c18u2dc9dc12()?);
    assert_eq!(non_stereospecific(c18u1dc9c18u0c18u2dc9dc12()?)?, c18u0c18u1dc9c18u2dc9dc12()?);
    assert_eq!(non_stereospecific(c18u1dc9c18u2dc9dc12c18u0()?)?, c18u0c18u1dc9c18u2dc9dc12()?);
    assert_eq!(non_stereospecific(c18u2dc9dc12c18u0c18u1dc9()?)?, c18u0c18u1dc9c18u2dc9dc12()?);
    assert_eq!(non_stereospecific(c18u2dc9dc12c18u1dc9c18u0()?)?, c18u0c18u1dc9c18u2dc9dc12()?);
    Ok(())
}

#[rustfmt::skip]
#[test]
fn ptc() -> PolarsResult<()> {
    let positional = |data_frame| positional(data_frame, unsaturation);
    assert_eq!(positional(c18u0c18u1dc9c18u2dc9dc12()?)?, c18u0c18u1dc9c18u2dc9dc12()?);
    assert_eq!(positional(c18u0c18u2dc9dc12c18u1dc9()?)?, c18u0c18u2dc9dc12c18u1dc9()?);
    assert_eq!(positional(c18u1dc9c18u0c18u2dc9dc12()?)?, c18u1dc9c18u0c18u2dc9dc12()?);
    assert_eq!(positional(c18u1dc9c18u2dc9dc12c18u0()?)?, c18u0c18u2dc9dc12c18u1dc9()?);
    assert_eq!(positional(c18u2dc9dc12c18u0c18u1dc9()?)?, c18u1dc9c18u0c18u2dc9dc12()?);
    assert_eq!(positional(c18u2dc9dc12c18u1dc9c18u0()?)?, c18u0c18u1dc9c18u2dc9dc12()?);
    Ok(())
}

#[rustfmt::skip]
#[test]
fn uc() -> PolarsResult<()> {
    let non_stereospecific = |data_frame| non_stereospecific(data_frame, unsaturation);
    assert_eq!(non_stereospecific(c18u0c18u1dc9c18u2dc9dc12()?)?, c18u0c18u1dc9c18u2dc9dc12()?);
    assert_eq!(non_stereospecific(c18u0c18u2dc9dc12c18u1dc9()?)?, c18u0c18u1dc9c18u2dc9dc12()?);
    assert_eq!(non_stereospecific(c18u1dc9c18u0c18u2dc9dc12()?)?, c18u0c18u1dc9c18u2dc9dc12()?);
    assert_eq!(non_stereospecific(c18u1dc9c18u2dc9dc12c18u0()?)?, c18u0c18u1dc9c18u2dc9dc12()?);
    assert_eq!(non_stereospecific(c18u2dc9dc12c18u0c18u1dc9()?)?, c18u0c18u1dc9c18u2dc9dc12()?);
    assert_eq!(non_stereospecific(c18u2dc9dc12c18u1dc9c18u0()?)?, c18u0c18u1dc9c18u2dc9dc12()?);
    Ok(())
}

#[rustfmt::skip]
#[test]
fn puc() -> PolarsResult<()> {
    let positional = |data_frame| positional(data_frame, unsaturation);
    assert_eq!(positional(c18u0c18u1dc9c18u2dc9dc12()?)?, c18u0c18u1dc9c18u2dc9dc12()?);
    assert_eq!(positional(c18u0c18u2dc9dc12c18u1dc9()?)?, c18u0c18u2dc9dc12c18u1dc9()?);
    assert_eq!(positional(c18u1dc9c18u0c18u2dc9dc12()?)?, c18u1dc9c18u0c18u2dc9dc12()?);
    assert_eq!(positional(c18u1dc9c18u2dc9dc12c18u0()?)?, c18u0c18u2dc9dc12c18u1dc9()?);
    assert_eq!(positional(c18u2dc9dc12c18u0c18u1dc9()?)?, c18u1dc9c18u0c18u2dc9dc12()?);
    assert_eq!(positional(c18u2dc9dc12c18u1dc9c18u0()?)?, c18u0c18u1dc9c18u2dc9dc12()?);
    Ok(())
}
