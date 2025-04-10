use super::*;

// https://byrdwell.com/Triacylglycerols/TAGbyName2.htm
const LA_M_P: f64 = 722.64;
const LA_M_PO: f64 = 720.63;
const LA_M_S: f64 = 750.67;
const LA_M_O: f64 = 748.66;
const LA_M_L: f64 = 746.64;
const LA_M_LN: f64 = 744.63;
const LA_M_A: f64 = 778.71;
const LA_P_PO: f64 = 748.66;
const LA_P_S: f64 = 778.71;
const LA_P_O: f64 = 776.69;
const LA_P_L: f64 = 774.67;
const LA_P_LN: f64 = 772.66;
const LA_P_A: f64 = 806.74;
const LA_PO_S: f64 = 776.69;
const LA_PO_O: f64 = 774.67;
const LA_PO_L: f64 = 772.66;
const LA_PO_LN: f64 = 770.64;
const LA_PO_A: f64 = 804.72;
const LA_S_O: f64 = 804.72;
const LA_S_L: f64 = 802.71;
const LA_S_LN: f64 = 800.69;
const LA_S_A: f64 = 834.77;
const LA_O_L: f64 = 800.69;
const LA_O_LN: f64 = 798.67;
const LA_O_A: f64 = 832.75;
const LA_L_LN: f64 = 796.66;
const LA_L_A: f64 = 830.74;
const M_P_PO: f64 = 776.69;
const M_P_S: f64 = 806.74;
const M_P_O: f64 = 804.72;
const M_P_L: f64 = 802.71;
const M_P_LN: f64 = 800.69;
const M_P_A: f64 = 834.77;
const M_PO_S: f64 = 804.72;
const M_PO_O: f64 = 802.71;
const M_PO_L: f64 = 800.69;
const M_PO_LN: f64 = 798.67;
const M_PO_A: f64 = 832.75;
const M_S_O: f64 = 832.75;
const M_S_L: f64 = 830.74;
const M_S_LN: f64 = 828.72;
const M_S_A: f64 = 862.80;
const M_O_L: f64 = 828.72;
const M_O_LN: f64 = 826.71;
const M_O_A: f64 = 860.78;
const M_L_LN: f64 = 824.69;
const M_L_A: f64 = 858.77;
const M_LN_A: f64 = 856.75;
const P_PO_S: f64 = 832.75;
const P_PO_O: f64 = 830.74;
const P_PO_L: f64 = 828.72;
const P_PO_LN: f64 = 826.71;
const P_PO_A: f64 = 860.78;
const P_S_O: f64 = 860.78;
const P_S_L: f64 = 858.77;
const P_S_LN: f64 = 856.75;
const P_S_A: f64 = 890.83;
const P_O_L: f64 = 856.75;
const P_O_LN: f64 = 854.74;
const P_O_A: f64 = 888.81;
const P_L_LN: f64 = 852.72;
const P_L_A: f64 = 886.80;
const P_LN_A: f64 = 884.78;
const PO_S_O: f64 = 858.77;
const PO_S_L: f64 = 856.75;
const PO_S_LN: f64 = 854.74;
const PO_S_A: f64 = 888.81;
const PO_O_L: f64 = 854.74;
const PO_O_LN: f64 = 852.72;
const PO_L_LN: f64 = 850.71;
const PO_L_A: f64 = 884.78;
const PO_LN_A: f64 = 882.77;
const S_O_L: f64 = 884.78;
const S_O_LN: f64 = 882.77;
const S_O_A: f64 = 916.85;
const S_L_LN: f64 = 880.75;
const S_L_A: f64 = 914.83;
const S_LN_A: f64 = 912.81;
const O_L_LN: f64 = 878.74;
const O_L_A: f64 = 912.81;
const O_LN_A: f64 = 910.80;
const L_LN_A: f64 = 908.78;

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
fn general() -> PolarsResult<()> {
    assert_eq!(mass(tag(LA, M, P)?)?, LA_M_P);
    assert_eq!(mass(tag(LA, M, PO)?)?, LA_M_PO);
    assert_eq!(mass(tag(LA, M, S)?)?, LA_M_S);
    assert_eq!(mass(tag(LA, M, O)?)?, LA_M_O);
    assert_eq!(mass(tag(LA, M, L)?)?, LA_M_L);
    assert_eq!(mass(tag(LA, M, LN)?)?, LA_M_LN);
    assert_eq!(mass(tag(LA, M, A)?)?, LA_M_A);
    assert_eq!(mass(tag(LA, P, PO)?)?, LA_P_PO);
    assert_eq!(mass(tag(LA, P, S)?)?, LA_P_S);
    assert_eq!(mass(tag(LA, P, O)?)?, LA_P_O);
    assert_eq!(mass(tag(LA, P, L)?)?, LA_P_L);
    assert_eq!(mass(tag(LA, P, LN)?)?, LA_P_LN);
    assert_eq!(mass(tag(LA, P, A)?)?, LA_P_A);
    assert_eq!(mass(tag(LA, PO, S)?)?, LA_PO_S);
    assert_eq!(mass(tag(LA, PO, O)?)?, LA_PO_O);
    assert_eq!(mass(tag(LA, PO, L)?)?, LA_PO_L);
    assert_eq!(mass(tag(LA, PO, LN)?)?, LA_PO_LN);
    assert_eq!(mass(tag(LA, PO, A)?)?, LA_PO_A);
    assert_eq!(mass(tag(LA, S, O)?)?, LA_S_O);
    assert_eq!(mass(tag(LA, S, L)?)?, LA_S_L);
    assert_eq!(mass(tag(LA, S, LN)?)?, LA_S_LN);
    assert_eq!(mass(tag(LA, S, A)?)?, LA_S_A);
    assert_eq!(mass(tag(LA, O, L)?)?, LA_O_L);
    assert_eq!(mass(tag(LA, O, LN)?)?, LA_O_LN);
    assert_eq!(mass(tag(LA, O, A)?)?, LA_O_A);
    assert_eq!(mass(tag(LA, L, LN)?)?, LA_L_LN);
    assert_eq!(mass(tag(LA, L, A)?)?, LA_L_A);
    assert_eq!(mass(tag(M, P, PO)?)?, M_P_PO);
    assert_eq!(mass(tag(M, P, S)?)?, M_P_S);
    assert_eq!(mass(tag(M, P, O)?)?, M_P_O);
    assert_eq!(mass(tag(M, P, L)?)?, M_P_L);
    assert_eq!(mass(tag(M, P, LN)?)?, M_P_LN);
    assert_eq!(mass(tag(M, P, A)?)?, M_P_A);
    assert_eq!(mass(tag(M, PO, S)?)?, M_PO_S);
    assert_eq!(mass(tag(M, PO, O)?)?, M_PO_O);
    assert_eq!(mass(tag(M, PO, L)?)?, M_PO_L);
    assert_eq!(mass(tag(M, PO, LN)?)?, M_PO_LN);
    assert_eq!(mass(tag(M, PO, A)?)?, M_PO_A);
    assert_eq!(mass(tag(M, S, O)?)?, M_S_O);
    assert_eq!(mass(tag(M, S, L)?)?, M_S_L);
    assert_eq!(mass(tag(M, S, LN)?)?, M_S_LN);
    assert_eq!(mass(tag(M, S, A)?)?, M_S_A);
    assert_eq!(mass(tag(M, O, L)?)?, M_O_L);
    assert_eq!(mass(tag(M, O, LN)?)?, M_O_LN);
    assert_eq!(mass(tag(M, O, A)?)?, M_O_A);
    assert_eq!(mass(tag(M, L, LN)?)?, M_L_LN);
    assert_eq!(mass(tag(M, L, A)?)?, M_L_A);
    assert_eq!(mass(tag(M, LN, A)?)?, M_LN_A);
    assert_eq!(mass(tag(P, PO, S)?)?, P_PO_S);
    assert_eq!(mass(tag(P, PO, O)?)?, P_PO_O);
    assert_eq!(mass(tag(P, PO, L)?)?, P_PO_L);
    assert_eq!(mass(tag(P, PO, LN)?)?, P_PO_LN);
    assert_eq!(mass(tag(P, PO, A)?)?, P_PO_A);
    assert_eq!(mass(tag(P, S, O)?)?, P_S_O);
    assert_eq!(mass(tag(P, S, L)?)?, P_S_L);
    assert_eq!(mass(tag(P, S, LN)?)?, P_S_LN);
    assert_eq!(mass(tag(P, S, A)?)?, P_S_A);
    assert_eq!(mass(tag(P, O, L)?)?, P_O_L);
    assert_eq!(mass(tag(P, O, LN)?)?, P_O_LN);
    assert_eq!(mass(tag(P, O, A)?)?, P_O_A);
    assert_eq!(mass(tag(P, L, LN)?)?, P_L_LN);
    assert_eq!(mass(tag(P, L, A)?)?, P_L_A);
    assert_eq!(mass(tag(P, LN, A)?)?, P_LN_A);
    assert_eq!(mass(tag(PO, S, O)?)?, PO_S_O);
    assert_eq!(mass(tag(PO, S, L)?)?, PO_S_L);
    assert_eq!(mass(tag(PO, S, LN)?)?, PO_S_LN);
    assert_eq!(mass(tag(PO, S, A)?)?, PO_S_A);
    assert_eq!(mass(tag(PO, O, L)?)?, PO_O_L);
    assert_eq!(mass(tag(PO, O, LN)?)?, PO_O_LN);
    assert_eq!(mass(tag(PO, L, LN)?)?, PO_L_LN);
    assert_eq!(mass(tag(PO, L, A)?)?, PO_L_A);
    assert_eq!(mass(tag(PO, LN, A)?)?, PO_LN_A);
    assert_eq!(mass(tag(S, O, L)?)?, S_O_L);
    assert_eq!(mass(tag(S, O, LN)?)?, S_O_LN);
    assert_eq!(mass(tag(S, O, A)?)?, S_O_A);
    assert_eq!(mass(tag(S, L, LN)?)?, S_L_LN);
    assert_eq!(mass(tag(S, L, A)?)?, S_L_A);
    assert_eq!(mass(tag(S, LN, A)?)?, S_LN_A);
    assert_eq!(mass(tag(O, L, LN)?)?, O_L_LN);
    assert_eq!(mass(tag(O, L, A)?)?, O_L_A);
    assert_eq!(mass(tag(O, LN, A)?)?, O_LN_A);
    assert_eq!(mass(tag(L, LN, A)?)?, L_LN_A);
    Ok(())
}

#[test]
fn permutation() -> PolarsResult<()> {
    assert_eq!(mass(tag(M, P, S)?)?, M_P_S);
    assert_eq!(mass(tag(M, S, P)?)?, M_P_S);
    assert_eq!(mass(tag(P, M, S)?)?, M_P_S);
    assert_eq!(mass(tag(P, S, M)?)?, M_P_S);
    assert_eq!(mass(tag(S, M, P)?)?, M_P_S);
    assert_eq!(mass(tag(S, P, M)?)?, M_P_S);
    Ok(())
}
