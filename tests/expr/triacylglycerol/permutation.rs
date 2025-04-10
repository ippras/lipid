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

#[test]
fn mc() -> PolarsResult<()> {
    let non_stereospecific = |data_frame| non_stereospecific(data_frame, mass);
    assert_eq!(non_stereospecific(tag(M, P, S)?)?, tag(M, P, S)?);
    assert_eq!(non_stereospecific(tag(M, S, P)?)?, tag(M, P, S)?);
    assert_eq!(non_stereospecific(tag(P, M, S)?)?, tag(M, P, S)?);
    assert_eq!(non_stereospecific(tag(P, S, M)?)?, tag(M, P, S)?);
    assert_eq!(non_stereospecific(tag(S, M, P)?)?, tag(M, P, S)?);
    assert_eq!(non_stereospecific(tag(S, P, M)?)?, tag(M, P, S)?);
    assert_eq!(non_stereospecific(tag(S, O, L)?)?, tag(L, O, S)?);
    assert_eq!(non_stereospecific(tag(S, L, O)?)?, tag(L, O, S)?);
    assert_eq!(non_stereospecific(tag(O, S, L)?)?, tag(L, O, S)?);
    assert_eq!(non_stereospecific(tag(O, L, S)?)?, tag(L, O, S)?);
    assert_eq!(non_stereospecific(tag(L, S, O)?)?, tag(L, O, S)?);
    assert_eq!(non_stereospecific(tag(L, O, S)?)?, tag(L, O, S)?);
    Ok(())
}

#[test]
fn pmc() -> PolarsResult<()> {
    let positional = |data_frame| positional(data_frame, mass);
    assert_eq!(positional(tag(M, P, S)?)?, tag(M, P, S)?);
    assert_eq!(positional(tag(M, S, P)?)?, tag(M, S, P)?);
    assert_eq!(positional(tag(P, M, S)?)?, tag(P, M, S)?);
    assert_eq!(positional(tag(P, S, M)?)?, tag(M, S, P)?);
    assert_eq!(positional(tag(S, M, P)?)?, tag(P, M, S)?);
    assert_eq!(positional(tag(S, P, M)?)?, tag(M, P, S)?);
    assert_eq!(positional(tag(S, O, L)?)?, tag(L, O, S)?);
    assert_eq!(positional(tag(S, L, O)?)?, tag(O, L, S)?);
    assert_eq!(positional(tag(O, S, L)?)?, tag(L, S, O)?);
    assert_eq!(positional(tag(O, L, S)?)?, tag(O, L, S)?);
    assert_eq!(positional(tag(L, S, O)?)?, tag(L, S, O)?);
    assert_eq!(positional(tag(L, O, S)?)?, tag(L, O, S)?);
    Ok(())
}

#[test]
fn nc() -> PolarsResult<()> {
    let non_stereospecific = |data_frame| non_stereospecific(data_frame, ecn);
    assert_eq!(non_stereospecific(tag(M, P, S)?)?, tag(M, P, S)?);
    assert_eq!(non_stereospecific(tag(M, S, P)?)?, tag(M, P, S)?);
    assert_eq!(non_stereospecific(tag(P, M, S)?)?, tag(M, P, S)?);
    assert_eq!(non_stereospecific(tag(P, S, M)?)?, tag(M, P, S)?);
    assert_eq!(non_stereospecific(tag(S, M, P)?)?, tag(M, P, S)?);
    assert_eq!(non_stereospecific(tag(S, P, M)?)?, tag(M, P, S)?);
    assert_eq!(non_stereospecific(tag(S, O, L)?)?, tag(L, O, S)?);
    assert_eq!(non_stereospecific(tag(S, L, O)?)?, tag(L, O, S)?);
    assert_eq!(non_stereospecific(tag(O, S, L)?)?, tag(L, O, S)?);
    assert_eq!(non_stereospecific(tag(O, L, S)?)?, tag(L, O, S)?);
    assert_eq!(non_stereospecific(tag(L, S, O)?)?, tag(L, O, S)?);
    assert_eq!(non_stereospecific(tag(L, O, S)?)?, tag(L, O, S)?);
    Ok(())
}

#[test]
fn sc() -> PolarsResult<()> {
    let non_stereospecific = |data_frame| non_stereospecific(data_frame, species);
    assert_eq!(non_stereospecific(tag(M, P, S)?)?, tag(M, P, S)?);
    assert_eq!(non_stereospecific(tag(M, S, P)?)?, tag(M, P, S)?);
    assert_eq!(non_stereospecific(tag(P, M, S)?)?, tag(M, P, S)?);
    assert_eq!(non_stereospecific(tag(P, S, M)?)?, tag(M, P, S)?);
    assert_eq!(non_stereospecific(tag(S, M, P)?)?, tag(M, P, S)?);
    assert_eq!(non_stereospecific(tag(S, P, M)?)?, tag(M, P, S)?);
    Ok(())
}

#[test]
fn psc() -> PolarsResult<()> {
    let positional = |data_frame| positional(data_frame, species);
    assert_eq!(positional(tag(M, P, S)?)?, tag(M, P, S)?);
    assert_eq!(positional(tag(M, S, P)?)?, tag(M, S, P)?);
    assert_eq!(positional(tag(P, M, S)?)?, tag(P, M, S)?);
    assert_eq!(positional(tag(P, S, M)?)?, tag(M, S, P)?);
    assert_eq!(positional(tag(S, M, P)?)?, tag(P, M, S)?);
    assert_eq!(positional(tag(S, P, M)?)?, tag(M, P, S)?);
    Ok(())
}

#[test]
fn tc() -> PolarsResult<()> {
    let non_stereospecific = |data_frame| non_stereospecific(data_frame, unsaturation);
    assert_eq!(non_stereospecific(tag(S, O, L)?)?, tag(S, O, L)?);
    assert_eq!(non_stereospecific(tag(S, L, O)?)?, tag(S, O, L)?);
    assert_eq!(non_stereospecific(tag(O, S, L)?)?, tag(S, O, L)?);
    assert_eq!(non_stereospecific(tag(O, L, S)?)?, tag(S, O, L)?);
    assert_eq!(non_stereospecific(tag(L, S, O)?)?, tag(S, O, L)?);
    assert_eq!(non_stereospecific(tag(L, O, S)?)?, tag(S, O, L)?);
    Ok(())
}

#[test]
fn ptc() -> PolarsResult<()> {
    let positional = |data_frame| positional(data_frame, unsaturation);
    assert_eq!(positional(tag(S, O, L)?)?, tag(S, O, L)?);
    assert_eq!(positional(tag(S, L, O)?)?, tag(S, L, O)?);
    assert_eq!(positional(tag(O, S, L)?)?, tag(O, S, L)?);
    assert_eq!(positional(tag(O, L, S)?)?, tag(S, L, O)?);
    assert_eq!(positional(tag(L, S, O)?)?, tag(O, S, L)?);
    assert_eq!(positional(tag(L, O, S)?)?, tag(S, O, L)?);
    Ok(())
}

#[test]
fn uc() -> PolarsResult<()> {
    let non_stereospecific = |data_frame| non_stereospecific(data_frame, unsaturation);
    assert_eq!(non_stereospecific(tag(S, O, L)?)?, tag(S, O, L)?);
    assert_eq!(non_stereospecific(tag(S, L, O)?)?, tag(S, O, L)?);
    assert_eq!(non_stereospecific(tag(O, S, L)?)?, tag(S, O, L)?);
    assert_eq!(non_stereospecific(tag(O, L, S)?)?, tag(S, O, L)?);
    assert_eq!(non_stereospecific(tag(L, S, O)?)?, tag(S, O, L)?);
    assert_eq!(non_stereospecific(tag(L, O, S)?)?, tag(S, O, L)?);
    Ok(())
}

#[test]
fn puc() -> PolarsResult<()> {
    let positional = |data_frame| positional(data_frame, unsaturation);
    assert_eq!(positional(tag(S, O, L)?)?, tag(S, O, L)?);
    assert_eq!(positional(tag(S, L, O)?)?, tag(S, L, O)?);
    assert_eq!(positional(tag(O, S, L)?)?, tag(O, S, L)?);
    assert_eq!(positional(tag(O, L, S)?)?, tag(S, L, O)?);
    assert_eq!(positional(tag(L, S, O)?)?, tag(O, S, L)?);
    assert_eq!(positional(tag(L, O, S)?)?, tag(S, O, L)?);
    Ok(())
}
