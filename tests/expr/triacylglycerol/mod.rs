use super::*;

#[test]
fn temp() -> PolarsResult<()> {
    let la_m_p = triacylglycerol(&LA, &M, &P);
    println!("la_m_p: {la_m_p}");
    let tags = triacylglycerols()?;
    println!("tags: {tags}");
    // assert_eq!(la_m_p?, LA_M_P);
    Ok(())
}

// mod chain_length;
// #[cfg(feature = "mass")]
// mod mass;
// mod permutation;
mod equivalent_carbon_number;
mod indices;
