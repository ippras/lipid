use super::*;

macro_rules! check {
    ($sn1:ident, $sn2:ident, $sn3:ident, $expected:expr) => {{
        let mut data_frame = df! {
            TRIACYLGLYCEROL => Series::from_any_values_and_dtype(TRIACYLGLYCEROL.into(), &[
                triacylglycerol(&$sn1, &$sn2, &$sn3)
            ], &data_type!(TRIACYLGLYCEROL), true)?,
        }?;
        data_frame = data_frame
            .lazy()
            .select([col(TRIACYLGLYCEROL)
                .triacylglycerol()
                .equivalent_carbon_number()])
            .collect()?;
        let ecn = data_frame["EquivalentCarbonNumber"].u8()?.get(0).unwrap();
        assert_eq!(ecn, $expected);
    }};
}

#[test]
fn la_m_p() -> PolarsResult<()> {
    check!(LA, M, P, 42);
    Ok(())
}

#[test]
fn la_m_po() -> PolarsResult<()> {
    check!(LA, M, PO, 40);
    Ok(())
}

#[test]
fn la_m_s() -> PolarsResult<()> {
    check!(LA, M, S, 44);
    Ok(())
}

#[test]
fn la_m_o() -> PolarsResult<()> {
    check!(LA, M, O, 42);
    Ok(())
}

#[test]
fn la_m_l() -> PolarsResult<()> {
    check!(LA, M, L, 40);
    Ok(())
}

#[test]
fn la_m_ln() -> PolarsResult<()> {
    check!(LA, M, LN, 38);
    Ok(())
}

#[test]
fn la_m_a() -> PolarsResult<()> {
    check!(LA, M, A, 46);
    Ok(())
}

#[test]
fn la_p_po() -> PolarsResult<()> {
    check!(LA, P, PO, 42);
    Ok(())
}

#[test]
fn la_p_s() -> PolarsResult<()> {
    check!(LA, P, S, 46);
    Ok(())
}

#[test]
fn la_p_o() -> PolarsResult<()> {
    check!(LA, P, O, 44);
    Ok(())
}

#[test]
fn la_p_l() -> PolarsResult<()> {
    check!(LA, P, L, 42);
    Ok(())
}

#[test]
fn la_p_ln() -> PolarsResult<()> {
    check!(LA, P, LN, 40);
    Ok(())
}

#[test]
fn la_p_a() -> PolarsResult<()> {
    check!(LA, P, A, 48);
    Ok(())
}

#[test]
fn la_po_s() -> PolarsResult<()> {
    check!(LA, PO, S, 44);
    Ok(())
}

#[test]
fn la_po_o() -> PolarsResult<()> {
    check!(LA, PO, O, 42);
    Ok(())
}

#[test]
fn la_po_l() -> PolarsResult<()> {
    check!(LA, PO, L, 40);
    Ok(())
}

#[test]
fn la_po_ln() -> PolarsResult<()> {
    check!(LA, PO, LN, 38);
    Ok(())
}

#[test]
fn la_po_a() -> PolarsResult<()> {
    check!(LA, PO, A, 46);
    Ok(())
}

#[test]
fn la_s_o() -> PolarsResult<()> {
    check!(LA, S, O, 46);
    Ok(())
}

#[test]
fn la_s_l() -> PolarsResult<()> {
    check!(LA, S, L, 44);
    Ok(())
}

#[test]
fn la_s_ln() -> PolarsResult<()> {
    check!(LA, S, LN, 42);
    Ok(())
}

#[test]
fn la_s_a() -> PolarsResult<()> {
    check!(LA, S, A, 50);
    Ok(())
}

#[test]
fn la_o_l() -> PolarsResult<()> {
    check!(LA, O, L, 42);
    Ok(())
}

#[test]
fn la_o_ln() -> PolarsResult<()> {
    check!(LA, O, LN, 40);
    Ok(())
}

#[test]
fn la_o_a() -> PolarsResult<()> {
    check!(LA, O, A, 48);
    Ok(())
}

#[test]
fn la_l_ln() -> PolarsResult<()> {
    check!(LA, L, LN, 38);
    Ok(())
}

#[test]
fn la_l_a() -> PolarsResult<()> {
    check!(LA, L, A, 46);
    Ok(())
}

#[test]
fn m_p_po() -> PolarsResult<()> {
    check!(M, P, PO, 44);
    Ok(())
}

#[test]
fn m_p_s() -> PolarsResult<()> {
    check!(M, P, S, 48);
    Ok(())
}

#[test]
fn m_p_o() -> PolarsResult<()> {
    check!(M, P, O, 46);
    Ok(())
}

#[test]
fn m_p_l() -> PolarsResult<()> {
    check!(M, P, L, 44);
    Ok(())
}

#[test]
fn m_p_ln() -> PolarsResult<()> {
    check!(M, P, LN, 42);
    Ok(())
}

#[test]
fn m_p_a() -> PolarsResult<()> {
    check!(M, P, A, 50);
    Ok(())
}

#[test]
fn m_po_s() -> PolarsResult<()> {
    check!(M, PO, S, 46);
    Ok(())
}

#[test]
fn m_po_o() -> PolarsResult<()> {
    check!(M, PO, O, 44);
    Ok(())
}

#[test]
fn m_po_l() -> PolarsResult<()> {
    check!(M, PO, L, 42);
    Ok(())
}

#[test]
fn m_po_ln() -> PolarsResult<()> {
    check!(M, PO, LN, 40);
    Ok(())
}

#[test]
fn m_po_a() -> PolarsResult<()> {
    check!(M, PO, A, 48);
    Ok(())
}

#[test]
fn m_s_o() -> PolarsResult<()> {
    check!(M, S, O, 48);
    Ok(())
}

#[test]
fn m_s_l() -> PolarsResult<()> {
    check!(M, S, L, 46);
    Ok(())
}

#[test]
fn m_s_ln() -> PolarsResult<()> {
    check!(M, S, LN, 44);
    Ok(())
}

#[test]
fn m_s_a() -> PolarsResult<()> {
    check!(M, S, A, 52);
    Ok(())
}

#[test]
fn m_o_l() -> PolarsResult<()> {
    check!(M, O, L, 44);
    Ok(())
}

#[test]
fn m_o_ln() -> PolarsResult<()> {
    check!(M, O, LN, 42);
    Ok(())
}

#[test]
fn m_o_a() -> PolarsResult<()> {
    check!(M, O, A, 50);
    Ok(())
}

#[test]
fn m_l_ln() -> PolarsResult<()> {
    check!(M, L, LN, 40);
    Ok(())
}

#[test]
fn m_l_a() -> PolarsResult<()> {
    check!(M, L, A, 48);
    Ok(())
}

#[test]
fn m_ln_a() -> PolarsResult<()> {
    check!(M, LN, A, 46);
    Ok(())
}

#[test]
fn p_po_s() -> PolarsResult<()> {
    check!(P, PO, S, 48);
    Ok(())
}

#[test]
fn p_po_o() -> PolarsResult<()> {
    check!(P, PO, O, 46);
    Ok(())
}

#[test]
fn p_po_l() -> PolarsResult<()> {
    check!(P, PO, L, 44);
    Ok(())
}

#[test]
fn p_po_ln() -> PolarsResult<()> {
    check!(P, PO, LN, 42);
    Ok(())
}

#[test]
fn p_po_a() -> PolarsResult<()> {
    check!(P, PO, A, 50);
    Ok(())
}

#[test]
fn p_s_o() -> PolarsResult<()> {
    check!(P, S, O, 50);
    Ok(())
}

#[test]
fn p_s_l() -> PolarsResult<()> {
    check!(P, S, L, 48);
    Ok(())
}

#[test]
fn p_s_ln() -> PolarsResult<()> {
    check!(P, S, LN, 46);
    Ok(())
}

#[test]
fn p_s_a() -> PolarsResult<()> {
    check!(P, S, A, 54);
    Ok(())
}

#[test]
fn p_o_l() -> PolarsResult<()> {
    check!(P, O, L, 46);
    Ok(())
}

#[test]
fn p_o_ln() -> PolarsResult<()> {
    check!(P, O, LN, 44);
    Ok(())
}

#[test]
fn p_o_a() -> PolarsResult<()> {
    check!(P, O, A, 52);
    Ok(())
}

#[test]
fn p_l_ln() -> PolarsResult<()> {
    check!(P, L, LN, 42);
    Ok(())
}

#[test]
fn p_l_a() -> PolarsResult<()> {
    check!(P, L, A, 50);
    Ok(())
}

#[test]
fn p_ln_a() -> PolarsResult<()> {
    check!(P, LN, A, 48);
    Ok(())
}

#[test]
fn po_s_o() -> PolarsResult<()> {
    check!(PO, S, O, 48);
    Ok(())
}

#[test]
fn po_s_l() -> PolarsResult<()> {
    check!(PO, S, L, 46);
    Ok(())
}

#[test]
fn po_s_ln() -> PolarsResult<()> {
    check!(PO, S, LN, 44);
    Ok(())
}

#[test]
fn po_s_a() -> PolarsResult<()> {
    check!(PO, S, A, 52);
    Ok(())
}

#[test]
fn po_o_l() -> PolarsResult<()> {
    check!(PO, O, L, 44);
    Ok(())
}

#[test]
fn po_o_ln() -> PolarsResult<()> {
    check!(PO, O, LN, 42);
    Ok(())
}

#[test]
fn po_l_a() -> PolarsResult<()> {
    check!(PO, L, A, 48);
    Ok(())
}

#[test]
fn po_l_ln() -> PolarsResult<()> {
    check!(PO, L, LN, 40);
    Ok(())
}

// #[test]
// fn po_l_a() -> PolarsResult<()> {
//     check!(PO, L, A, 48);
//     Ok(())
// }

#[test]
fn po_ln_a() -> PolarsResult<()> {
    check!(PO, LN, A, 46);
    Ok(())
}

#[test]
fn s_o_l() -> PolarsResult<()> {
    check!(S, O, L, 48);
    Ok(())
}

#[test]
fn s_o_ln() -> PolarsResult<()> {
    check!(S, O, LN, 46);
    Ok(())
}

#[test]
fn s_o_a() -> PolarsResult<()> {
    check!(S, O, A, 54);
    Ok(())
}

#[test]
fn s_l_ln() -> PolarsResult<()> {
    check!(S, L, LN, 44);
    Ok(())
}

#[test]
fn s_l_a() -> PolarsResult<()> {
    check!(S, L, A, 52);
    Ok(())
}

#[test]
fn s_ln_a() -> PolarsResult<()> {
    check!(S, LN, A, 50);
    Ok(())
}

#[test]
fn o_l_ln() -> PolarsResult<()> {
    check!(O, L, LN, 42);
    Ok(())
}

#[test]
fn o_l_a() -> PolarsResult<()> {
    check!(O, L, A, 50);
    Ok(())
}

#[test]
fn o_ln_a() -> PolarsResult<()> {
    check!(O, LN, A, 48);
    Ok(())
}

#[test]
fn l_ln_a() -> PolarsResult<()> {
    check!(L, LN, A, 46);
    Ok(())
}
