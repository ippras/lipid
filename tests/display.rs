use lipid::fatty_acid::{
    display::{COMMON, DisplayWithOptions, ID},
    fatty_acid,
};

#[test]
fn common() {
    let fatty_acid = fatty_acid!(18).display(COMMON);
    assert_eq!(fatty_acid.to_string(), "18:0");
    assert_eq!(format!("{fatty_acid:02}"), "18:00");
    assert_eq!(format!("{fatty_acid:#}"), "18:0");
    assert_eq!(format!("{fatty_acid:#02}"), "18:00");
    let fatty_acid = &fatty_acid!(18;9).display(COMMON);
    assert_eq!(fatty_acid.to_string(), "18:1");
    assert_eq!(format!("{fatty_acid:02}"), "18:01");
    assert_eq!(format!("{fatty_acid:#}"), "18:1Δ9");
    assert_eq!(format!("{fatty_acid:#02}"), "18:01Δ09");
    let fatty_acid = fatty_acid!(18;9,12).display(COMMON);
    assert_eq!(fatty_acid.to_string(), "18:2");
    assert_eq!(format!("{fatty_acid:02}"), "18:02");
    assert_eq!(format!("{fatty_acid:#}"), "18:2Δ9,12");
    assert_eq!(format!("{fatty_acid:#02}"), "18:02Δ09,12");
    // Triple
    let fatty_acid = fatty_acid!(18;9;12).display(COMMON);
    assert_eq!(fatty_acid.to_string(), "18:2");
    assert_eq!(format!("{fatty_acid:02}"), "18:02");
    assert_eq!(format!("{fatty_acid:#}"), "18:2Δ9,12ct");
    assert_eq!(format!("{fatty_acid:#02}"), "18:02Δ09,12ct");
    // Isomerism
    let fatty_acid = fatty_acid!(18;-9,-12,-15).display(COMMON);
    assert_eq!(fatty_acid.to_string(), "18:3");
    assert_eq!(format!("{fatty_acid:02}"), "18:03");
    assert_eq!(format!("{fatty_acid:#}"), "18:3Δ9t,12t,15t");
    assert_eq!(format!("{fatty_acid:#02}"), "18:03Δ09t,12t,15t");
}

#[test]
fn id() {
    let fatty_acid = fatty_acid!(18).display(ID);
    assert_eq!(fatty_acid.to_string(), "c18u0");
    assert_eq!(format!("{fatty_acid:02}"), "c18u00");
    assert_eq!(format!("{fatty_acid:#}"), "c18u0");
    assert_eq!(format!("{fatty_acid:#02}"), "c18u00");
    let fatty_acid = fatty_acid!(18;9).display(ID);
    assert_eq!(fatty_acid.to_string(), "c18u1");
    assert_eq!(format!("{fatty_acid:02}"), "c18u01");
    assert_eq!(format!("{fatty_acid:#}"), "c18u1c9");
    assert_eq!(format!("{fatty_acid:#02}"), "c18u01c09");
    let fatty_acid = fatty_acid!(18;9,12).display(ID);
    assert_eq!(fatty_acid.to_string(), "c18u2");
    assert_eq!(format!("{fatty_acid:02}"), "c18u02");
    assert_eq!(format!("{fatty_acid:#}"), "c18u2c9c12");
    assert_eq!(format!("{fatty_acid:#02}"), "c18u02c09c12");
    // Triple
    let fatty_acid = fatty_acid!(18;9;12).display(ID);
    assert_eq!(fatty_acid.to_string(), "c18u2");
    assert_eq!(format!("{fatty_acid:02}"), "c18u02");
    assert_eq!(format!("{fatty_acid:#}"), "c18u2c9ct12");
    assert_eq!(format!("{fatty_acid:#02}"), "c18u02c09ct12");
    // Isomerism
    let fatty_acid = fatty_acid!(18;-9,-12,-15).display(ID);
    assert_eq!(fatty_acid.to_string(), "c18u3");
    assert_eq!(format!("{fatty_acid:02}"), "c18u03");
    assert_eq!(format!("{fatty_acid:#}"), "c18u3t9t12t15");
    assert_eq!(format!("{fatty_acid:#02}"), "c18u03t09t12t15");
}

// #[cfg(test)]
// mod test {
//     use super::*;

//     // #[test]
//     // fn isomerism() {
//     //     // 3
//     //     assert_eq!(
//     //         fatty_acid!(18;-9,12,15)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-9t12c15c",
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;9,-12,15)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-9c12t15c",
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;9,12,-15)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-9c12c15t",
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;-9,-12,15)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-9t12t15c",
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;9,-12,-15)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-9c12t15t",
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;-9,12,-15)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-9t12c15t",
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;-9,-12,-15)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-9t12t15t",
//     //     );
//     //     // 2:1
//     //     assert_eq!(
//     //         fatty_acid!(18;12,15;-9)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-12c15c-9t",
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;9,15;-12)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-9c15c-12t",
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;9,12;-15)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-9c12c-15t",
//     //     );
//     //     // 1:2
//     // }

//     // #[test]
//     // fn order() {
//     //     // 3
//     //     assert_eq!(
//     //         fatty_acid!(18;9,12,15)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-9c12c15c",
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;9,15,12)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-9c12c15c",
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;12,9,15)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-9c12c15c",
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;12,15,9)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-9c12c15c",
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;15,9,12)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-9c12c15c",
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;15,12,9)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-9c12c15c",
//     //     );
//     //     // 2:1
//     //     assert_eq!(
//     //         fatty_acid!(18;12,15;9)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-12c15c-9c",
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;15,12;9)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-12c15c-9c",
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;9,15;12)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-9c15c-12c",
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;15,9;12)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-9c15c-12c",
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;9,12;15)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-9c12c-15c",
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;12,9;15)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-9c12c-15c",
//     //     );
//     //     // 1:2
//     //     assert_eq!(
//     //         fatty_acid!(18;9;12,15)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-9c-12c15c",
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;9;15,12)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-9c-12c15c",
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;12;9,15)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-12c-9c15c",
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;12;15,9)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-12c-9c15c",
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;15;9,12)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-15c-9c12c",
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;15;12,9)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-15c-9c12c",
//     //     );
//     // }

//     // #[test]
//     // fn macros() {
//     //     // 0
//     //     assert_eq!(fatty_acid!(18), new(vec![0; 17]));
//     //     // 1
//     //     assert_eq!(
//     //         fatty_acid!(18;9),
//     //         FattyAcid::new(vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0]),
//     //     );
//     //     // 2
//     //     assert_eq!(
//     //         fatty_acid!(18;9,12),
//     //         FattyAcid::new(vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0]),
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;9;12),
//     //         FattyAcid::new(vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 2, 0, 0, 0, 0, 0]),
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;;9,12),
//     //         FattyAcid::new(vec![0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 2, 0, 0, 0, 0, 0]),
//     //     );
//     //     // 3
//     //     assert_eq!(
//     //         fatty_acid!(18;9,12,15),
//     //         FattyAcid::new(vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0]),
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;9,12;15),
//     //         FattyAcid::new(vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 2, 0, 0]),
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;9;12,15),
//     //         FattyAcid::new(vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 2, 0, 0, 2, 0, 0]),
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;;9,12,15),
//     //         FattyAcid::new(vec![0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 2, 0, 0, 2, 0, 0]),
//     //     );
//     // }

//     mod errors {
//         use super::*;

//         #[test]
//         #[should_panic(expected = "assertion failed: 0 > 0")]
//         fn zero_carbons() {
//             fatty_acid!(0);
//         }

//         #[test]
//         #[should_panic(expected = "assertion failed: 0 != 0")]
//         fn zero_index() {
//             fatty_acid!(18;0);
//         }

//         #[test]
//         #[should_panic(expected = "assertion failed: 18 < 18")]
//         fn equal_carbons() {
//             fatty_acid!(18;18);
//         }

//         #[test]
//         #[should_panic(expected = "assertion failed: 19 < 18")]
//         fn greater_carbons() {
//             fatty_acid!(18;19);
//         }
//     }

// }
