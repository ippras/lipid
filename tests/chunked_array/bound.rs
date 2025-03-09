use atom::isotopes::*;
use lipid::prelude::{S, U, *};
use polars::prelude::*;

const C: f64 = C::Twelve.relative_atomic_mass().value;
const H: f64 = H::One.relative_atomic_mass().value;
const O: f64 = O::Sixteen.relative_atomic_mass().value;

fn check_mass(bound_chunked: &BoundChunked, mass: f64) {
    assert!((bound_chunked.mass(None) - mass).abs() < f64::EPSILON);
    assert!((bound_chunked.rco().mass(None) - (mass - H - O)).abs() < f64::EPSILON);
    assert!((bound_chunked.rcoo().mass(None) - (mass - H)).abs() < f64::EPSILON);
    assert!((bound_chunked.rcooh().mass(None) - mass).abs() < f64::EPSILON);
    assert!((bound_chunked.rcooch3().mass(None) - (mass + 2.0 * H + C)).abs() < f64::EPSILON);
}

#[test]
fn test_saturated_series() {
    const MASS: f64 = 284.27153039942;

    let series = Series::new("Saturated".into(), C18U0)
        .cast(&BOUND_DATA_TYPE)
        .unwrap()
        .into_series();
    let bound_chunked = BoundChunked::new(&series).unwrap();
    assert_eq!(bound_chunked.len(), 17);
    assert!(bound_chunked.is_saturated());
    assert!(!bound_chunked.is_unsaturated());
    let doubles: Vec<_> = bound_chunked.doubles().collect();
    assert!(doubles.is_empty());
    let triples: Vec<_> = bound_chunked.triples().collect();
    assert!(triples.is_empty());
    let unsaturated: Vec<_> = bound_chunked.unsaturated().collect();
    assert!(unsaturated.is_empty());
    assert_eq!(bound_chunked.unsaturation(), 0);
    let display = bound_chunked.display(Default::default()).unwrap();
    assert_eq!(format!("{display:#}"), "18:0");
    assert_eq!(bound_chunked.carbons(), 18);
    assert_eq!(bound_chunked.hydrogens(), 36);
    check_mass(bound_chunked, MASS);
}

#[test]
fn test_unsaturated_series() {
    const MASS: f64 = 278.22458020604;

    let series = Series::new("Unsaturated".into(), C18U3DC9DC12DC15)
        .cast(&BOUND_DATA_TYPE)
        .unwrap()
        .into_series();
    let bound_chunked = BoundChunked::new(&series).unwrap();
    assert_eq!(bound_chunked.len(), 17);
    assert!(!bound_chunked.is_saturated());
    assert!(bound_chunked.is_unsaturated());
    let doubles: Vec<_> = bound_chunked.doubles().collect();
    assert_eq!(doubles, vec![8, 11, 14]);
    let triples: Vec<_> = bound_chunked.triples().collect();
    assert!(triples.is_empty());
    let unsaturated: Vec<_> = bound_chunked.unsaturated().collect();
    assert_eq!(unsaturated, vec![8, 11, 14]);
    assert_eq!(bound_chunked.unsaturation(), 3);
    let display = bound_chunked.display(Default::default()).unwrap();
    assert_eq!(format!("{display:#}"), "18:3Δ9,12,15");
    assert_eq!(bound_chunked.carbons(), 18);
    assert_eq!(bound_chunked.hydrogens(), 30);
    check_mass(bound_chunked, MASS);
}

#[test]
fn test_all_series() {
    const ALL: [&str; 10] = [S, D, DC, DT, T, TC, TT, U, UC, UT];

    let series = Series::new("Bounds".into(), ALL)
        .cast(&BOUND_DATA_TYPE)
        .unwrap()
        .into_series();
    let bound_chunked = BoundChunked::new(&series).unwrap();
    assert_eq!(bound_chunked.len(), 10);
    let doubles: Vec<_> = bound_chunked.doubles().collect();
    assert_eq!(doubles, vec![1, 2, 3]);
    let triples: Vec<_> = bound_chunked.triples().collect();
    assert_eq!(triples, vec![4, 5, 6]);
    assert!(!bound_chunked.is_saturated());
    assert!(bound_chunked.is_unsaturated());
    let unsaturated: Vec<_> = bound_chunked.unsaturated().collect();
    assert_eq!(unsaturated, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    assert_eq!(bound_chunked.unsaturation(), 9);
    let display = bound_chunked.display(Default::default()).unwrap();
    assert_eq!(
        format!("{:#}", display),
        "11:9Δ2?,3,4t,≡5?,≡6,≡7t,?8?,?9,?10t",
    );
    assert_eq!(bound_chunked.carbons(), 11);
    assert_eq!(bound_chunked.hydrogens(), 4);
}
