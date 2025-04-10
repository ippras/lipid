use super::*;

// https://byrdwell.com/Triacylglycerols/TAGbyName2.htm
const LA_M_P: u8 = 42;
const LA_M_PO: u8 = 40;
const LA_M_S: u8 = 44;
const LA_M_O: u8 = 42;
const LA_M_L: u8 = 40;
const LA_M_LN: u8 = 38;
const LA_M_A: u8 = 46;
const LA_P_PO: u8 = 42;
const LA_P_S: u8 = 46;
const LA_P_O: u8 = 44;
const LA_P_L: u8 = 42;
const LA_P_LN: u8 = 40;
const LA_P_A: u8 = 48;
const LA_PO_S: u8 = 44;
const LA_PO_O: u8 = 42;
const LA_PO_L: u8 = 40;
const LA_PO_LN: u8 = 38;
const LA_PO_A: u8 = 46;
const LA_S_O: u8 = 46;
const LA_S_L: u8 = 44;
const LA_S_LN: u8 = 42;
const LA_S_A: u8 = 50;
const LA_O_L: u8 = 42;
const LA_O_LN: u8 = 40;
const LA_O_A: u8 = 48;
const LA_L_LN: u8 = 38;
const LA_L_A: u8 = 46;
const M_P_PO: u8 = 44;
const M_P_S: u8 = 48;
const M_P_O: u8 = 46;
const M_P_L: u8 = 44;
const M_P_LN: u8 = 42;
const M_P_A: u8 = 50;
const M_PO_S: u8 = 46;
const M_PO_O: u8 = 44;
const M_PO_L: u8 = 42;
const M_PO_LN: u8 = 40;
const M_PO_A: u8 = 48;
const M_S_O: u8 = 48;
const M_S_L: u8 = 46;
const M_S_LN: u8 = 44;
const M_S_A: u8 = 52;
const M_O_L: u8 = 44;
const M_O_LN: u8 = 42;
const M_O_A: u8 = 50;
const M_L_LN: u8 = 40;
const M_L_A: u8 = 48;
const M_LN_A: u8 = 46;
const P_PO_S: u8 = 48;
const P_PO_O: u8 = 46;
const P_PO_L: u8 = 44;
const P_PO_LN: u8 = 42;
const P_PO_A: u8 = 50;
const P_S_O: u8 = 50;
const P_S_L: u8 = 48;
const P_S_LN: u8 = 46;
const P_S_A: u8 = 54;
const P_O_L: u8 = 46;
const P_O_LN: u8 = 44;
const P_O_A: u8 = 52;
const P_L_LN: u8 = 42;
const P_L_A: u8 = 50;
const P_LN_A: u8 = 48;
const PO_S_O: u8 = 48;
const PO_S_L: u8 = 46;
const PO_S_LN: u8 = 44;
const PO_S_A: u8 = 52;
const PO_O_L: u8 = 44;
const PO_O_LN: u8 = 42;
const PO_L_LN: u8 = 40;
const PO_L_A: u8 = 48;
const PO_LN_A: u8 = 46;
const S_O_L: u8 = 48;
const S_O_LN: u8 = 46;
const S_O_A: u8 = 54;
const S_L_LN: u8 = 44;
const S_L_A: u8 = 52;
const S_LN_A: u8 = 50;
const O_L_LN: u8 = 42;
const O_L_A: u8 = 50;
const O_LN_A: u8 = 48;
const L_LN_A: u8 = 46;

fn ecn(data_frame: DataFrame) -> PolarsResult<u8> {
    let data_frame = data_frame
        .lazy()
        .select([col("Triacylglycerol")
            .triacylglycerol()
            .map_expr(|expr| expr.struct_().field_by_name("FattyAcid"))
            .triacylglycerol()
            .equivalent_carbon_number()])
        .collect()?;
    Ok(data_frame["EquivalentCarbonNumber"].u8()?.get(0).unwrap())
}

#[test]
fn general() -> PolarsResult<()> {
    assert_eq!(ecn(tag(LA, M, P)?)?, LA_M_P);
    assert_eq!(ecn(tag(LA, M, PO)?)?, LA_M_PO);
    assert_eq!(ecn(tag(LA, M, S)?)?, LA_M_S);
    assert_eq!(ecn(tag(LA, M, O)?)?, LA_M_O);
    assert_eq!(ecn(tag(LA, M, L)?)?, LA_M_L);
    assert_eq!(ecn(tag(LA, M, LN)?)?, LA_M_LN);
    assert_eq!(ecn(tag(LA, M, A)?)?, LA_M_A);
    assert_eq!(ecn(tag(LA, P, PO)?)?, LA_P_PO);
    assert_eq!(ecn(tag(LA, P, S)?)?, LA_P_S);
    assert_eq!(ecn(tag(LA, P, O)?)?, LA_P_O);
    assert_eq!(ecn(tag(LA, P, L)?)?, LA_P_L);
    assert_eq!(ecn(tag(LA, P, LN)?)?, LA_P_LN);
    assert_eq!(ecn(tag(LA, P, A)?)?, LA_P_A);
    assert_eq!(ecn(tag(LA, PO, S)?)?, LA_PO_S);
    assert_eq!(ecn(tag(LA, PO, O)?)?, LA_PO_O);
    assert_eq!(ecn(tag(LA, PO, L)?)?, LA_PO_L);
    assert_eq!(ecn(tag(LA, PO, LN)?)?, LA_PO_LN);
    assert_eq!(ecn(tag(LA, PO, A)?)?, LA_PO_A);
    assert_eq!(ecn(tag(LA, S, O)?)?, LA_S_O);
    assert_eq!(ecn(tag(LA, S, L)?)?, LA_S_L);
    assert_eq!(ecn(tag(LA, S, LN)?)?, LA_S_LN);
    assert_eq!(ecn(tag(LA, S, A)?)?, LA_S_A);
    assert_eq!(ecn(tag(LA, O, L)?)?, LA_O_L);
    assert_eq!(ecn(tag(LA, O, LN)?)?, LA_O_LN);
    assert_eq!(ecn(tag(LA, O, A)?)?, LA_O_A);
    assert_eq!(ecn(tag(LA, L, LN)?)?, LA_L_LN);
    assert_eq!(ecn(tag(LA, L, A)?)?, LA_L_A);
    assert_eq!(ecn(tag(M, P, PO)?)?, M_P_PO);
    assert_eq!(ecn(tag(M, P, S)?)?, M_P_S);
    assert_eq!(ecn(tag(M, P, O)?)?, M_P_O);
    assert_eq!(ecn(tag(M, P, L)?)?, M_P_L);
    assert_eq!(ecn(tag(M, P, LN)?)?, M_P_LN);
    assert_eq!(ecn(tag(M, P, A)?)?, M_P_A);
    assert_eq!(ecn(tag(M, PO, S)?)?, M_PO_S);
    assert_eq!(ecn(tag(M, PO, O)?)?, M_PO_O);
    assert_eq!(ecn(tag(M, PO, L)?)?, M_PO_L);
    assert_eq!(ecn(tag(M, PO, LN)?)?, M_PO_LN);
    assert_eq!(ecn(tag(M, PO, A)?)?, M_PO_A);
    assert_eq!(ecn(tag(M, S, O)?)?, M_S_O);
    assert_eq!(ecn(tag(M, S, L)?)?, M_S_L);
    assert_eq!(ecn(tag(M, S, LN)?)?, M_S_LN);
    assert_eq!(ecn(tag(M, S, A)?)?, M_S_A);
    assert_eq!(ecn(tag(M, O, L)?)?, M_O_L);
    assert_eq!(ecn(tag(M, O, LN)?)?, M_O_LN);
    assert_eq!(ecn(tag(M, O, A)?)?, M_O_A);
    assert_eq!(ecn(tag(M, L, LN)?)?, M_L_LN);
    assert_eq!(ecn(tag(M, L, A)?)?, M_L_A);
    assert_eq!(ecn(tag(M, LN, A)?)?, M_LN_A);
    assert_eq!(ecn(tag(P, PO, S)?)?, P_PO_S);
    assert_eq!(ecn(tag(P, PO, O)?)?, P_PO_O);
    assert_eq!(ecn(tag(P, PO, L)?)?, P_PO_L);
    assert_eq!(ecn(tag(P, PO, LN)?)?, P_PO_LN);
    assert_eq!(ecn(tag(P, PO, A)?)?, P_PO_A);
    assert_eq!(ecn(tag(P, S, O)?)?, P_S_O);
    assert_eq!(ecn(tag(P, S, L)?)?, P_S_L);
    assert_eq!(ecn(tag(P, S, LN)?)?, P_S_LN);
    assert_eq!(ecn(tag(P, S, A)?)?, P_S_A);
    assert_eq!(ecn(tag(P, O, L)?)?, P_O_L);
    assert_eq!(ecn(tag(P, O, LN)?)?, P_O_LN);
    assert_eq!(ecn(tag(P, O, A)?)?, P_O_A);
    assert_eq!(ecn(tag(P, L, LN)?)?, P_L_LN);
    assert_eq!(ecn(tag(P, L, A)?)?, P_L_A);
    assert_eq!(ecn(tag(P, LN, A)?)?, P_LN_A);
    assert_eq!(ecn(tag(PO, S, O)?)?, PO_S_O);
    assert_eq!(ecn(tag(PO, S, L)?)?, PO_S_L);
    assert_eq!(ecn(tag(PO, S, LN)?)?, PO_S_LN);
    assert_eq!(ecn(tag(PO, S, A)?)?, PO_S_A);
    assert_eq!(ecn(tag(PO, O, L)?)?, PO_O_L);
    assert_eq!(ecn(tag(PO, O, LN)?)?, PO_O_LN);
    assert_eq!(ecn(tag(PO, L, LN)?)?, PO_L_LN);
    assert_eq!(ecn(tag(PO, L, A)?)?, PO_L_A);
    assert_eq!(ecn(tag(PO, LN, A)?)?, PO_LN_A);
    assert_eq!(ecn(tag(S, O, L)?)?, S_O_L);
    assert_eq!(ecn(tag(S, O, LN)?)?, S_O_LN);
    assert_eq!(ecn(tag(S, O, A)?)?, S_O_A);
    assert_eq!(ecn(tag(S, L, LN)?)?, S_L_LN);
    assert_eq!(ecn(tag(S, L, A)?)?, S_L_A);
    assert_eq!(ecn(tag(S, LN, A)?)?, S_LN_A);
    assert_eq!(ecn(tag(O, L, LN)?)?, O_L_LN);
    assert_eq!(ecn(tag(O, L, A)?)?, O_L_A);
    assert_eq!(ecn(tag(O, LN, A)?)?, O_LN_A);
    assert_eq!(ecn(tag(L, LN, A)?)?, L_LN_A);
    Ok(())
}

#[test]
fn permutation() -> PolarsResult<()> {
    assert_eq!(ecn(tag(M, P, S)?)?, M_P_S);
    assert_eq!(ecn(tag(M, S, P)?)?, M_P_S);
    assert_eq!(ecn(tag(P, M, S)?)?, M_P_S);
    assert_eq!(ecn(tag(P, S, M)?)?, M_P_S);
    assert_eq!(ecn(tag(S, M, P)?)?, M_P_S);
    assert_eq!(ecn(tag(S, P, M)?)?, M_P_S);
    Ok(())
}
