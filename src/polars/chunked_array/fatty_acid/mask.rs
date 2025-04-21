use crate::prelude::*;
use polars::prelude::*;
use std::num::{NonZeroI8, NonZeroU8};

impl FattyAcidChunked {
    fn is_affix(self, index: Option<NonZeroI8>) -> PolarsResult<Option<bool>> {
        let Some(index) = index else {
            return Ok(self.identifier.not_equal(S)?.any_kleene());
        };
        if index.is_positive() {
            self.is_delta_unsaturated(index.cast_unsigned())
        } else {
            self.is_omega_unsaturated(index.unsigned_abs())
        }
    }

    // fn is_saturated_before(self, index: Option<NonZeroI8>) -> PolarsResult<Option<bool>> {
    //     let Some(index) = index else {
    //         return self.bound.is_saturated();
    //     };
    //     if index.is_positive() {
    //         self.is_delta(index.cast_unsigned())
    //     } else {
    //         self.is_omega(index.unsigned_abs())
    //     }
    //     let index = index.get();
    //     let length = self.carbons();
    //     if index >= length {
    //         return Ok(Some(false));
    //     }
    //     let mut sized = Some(0);
    //     let mut r#unsized = Some(Unsized::default());
    //     let range = index..length;
    //     let Some(sized) = sized else {
    //         return Ok(None);
    //     };
    //     Ok(Some(sized == range.len() + 1))
    // }
}

impl Mask for &FattyAcidChunked {
    type Output = PolarsResult<Option<bool>>;

    fn is_saturated(self) -> PolarsResult<Option<bool>> {
        self.identifier.is_saturated()
    }

    fn is_unsaturated(self) -> PolarsResult<Option<bool>> {
        self.identifier.is_unsaturated()
    }

    // fn is_unsaturated_before(self, index: Option<NonZeroI8>) -> PolarsResult<Option<bool>> {
    //     // forward
    //     let carbons = self.carbons();
    //     let backward = |index: i8| {
    //         if index.is_positive() {
    //             index.saturating_sub_unsigned(carbons)
    //         } else {
    //             index.saturating_add_unsigned(carbons)
    //         }
    //     };
    //     let Some(index) = index else {
    //         return Ok(self.bound.not_equal(S)?.any_kleene());
    //     };
    //     let mut sized = (Some(0), Some(0));
    //     let mut r#unsized = Some(0);
    //     let forward = index.get();
    //     let target = (forward, backward(forward));
    //     let range = (
    //         target.0.min(0)..target.0.max(0),
    //         target.1.min(0)..target.1.max(0),
    //     );
    //     for (source, bound) in self.try_iter() {
    //         let bound = bound?;
    //         match source {
    //             Some(Some(source)) => {
    //                 let forward = source.get();
    //                 let backward = backward(forward);
    //                 if forward == target.0 || backward == target.0 {
    //                     match bound {
    //                         Some(Bound::Unsaturated(_)) => {
    //                             if let Some(sized) = &mut sized.0 {
    //                                 *sized += 1
    //                             }
    //                         }
    //                         None => r#unsized = None,
    //                         _ => {}
    //                     }
    //                 } else if range.0.contains(&forward) || range.0.contains(&backward) {
    //                     match bound {
    //                         Some(Bound::Saturated) => {
    //                             if let Some(sized) = &mut sized.0 {
    //                                 *sized += 1
    //                             }
    //                         }
    //                         None => r#unsized = None,
    //                         _ => {}
    //                     }
    //                 }
    //                 if forward == target.1 || backward == target.1 {
    //                     match bound {
    //                         // Some(Bound::Saturated) => return Ok(Some(false)),
    //                         Some(Bound::Unsaturated(_)) => {
    //                             if let Some(sized) = &mut sized.1 {
    //                                 *sized += 1
    //                             }
    //                         }
    //                         None => r#unsized = None,
    //                         _ => {}
    //                     }
    //                 } else if range.1.contains(&forward) || range.1.contains(&backward) {
    //                     // println!("contains1: {forward}/{backward} {bound:?} {range:?}");
    //                     match bound {
    //                         Some(Bound::Saturated) => {
    //                             if let Some(sized) = &mut sized.1 {
    //                                 *sized += 1
    //                             }
    //                         }
    //                         // Some(Bound::Unsaturated(_)) => return Ok(Some(false)),
    //                         None => r#unsized = None,
    //                         _ => {}
    //                     }
    //                 }
    //             }
    //             Some(None) => r#unsized = r#unsized.map(|r#unsized| r#unsized + 1),
    //             None => r#unsized = None,
    //         }
    //     }
    //     // println!("sized: {sized:?}");
    //     // println!("target: {target:?}");
    //     Ok(sized
    //         .0
    //         .zip(sized.1)
    //         .map(|sized| sized.0 == target.0.abs() || sized.1 == target.1.abs()))
    // }

    // fn is_unsaturated_before(self, index: NonZeroI8) -> PolarsResult<Option<bool>> {
    //     let mut is_unsaturated = Some(index.get());
    //     let mut r#unsized = Some(0);
    //     let target = index;
    //     for (source, bound) in self.try_iter() {
    //         let bound = bound?;
    //         match source {
    //             Some(Some(source)) if source == target => match bound {
    //                 Some(Bound::Saturated) => return Ok(Some(false)),
    //                 Some(Bound::Unsaturated(_)) => {
    //                     if let Some(is_unsaturated) = &mut is_unsaturated {
    //                         *is_unsaturated -= 1
    //                     }
    //                 }
    //                 None => r#unsized = None,
    //             },
    //             Some(Some(source)) if source.abs() < target.abs() => match bound {
    //                 Some(Bound::Saturated) => {
    //                     if let Some(is_unsaturated) = &mut is_unsaturated {
    //                         *is_unsaturated -= 1
    //                     }
    //                 }
    //                 Some(Bound::Unsaturated(_)) => return Ok(Some(false)),
    //                 None => r#unsized = None,
    //             },
    //             _ => {}
    //         }
    //     }
    //     Ok(is_unsaturated.map(|is_unsaturated| is_unsaturated == 0))
    // }

    // fn is_unsaturated_before(self, index: NonZeroI8) -> PolarsResult<Option<bool>> {
    //     let mut is_unsaturated = Some(index.get());
    //     let mut map: BTreeMap<_, _> = self.try_iter().collect();
    //     let start = index.get();
    //     for index in start..0 {
    //         let Some(bound) = map.remove(&Some(NonZeroI8::new(index))) else {
    //             return Ok(Some(false));
    //         };
    //         let bound = bound?;
    //         if index == start && bound == Some(Bound::Unsaturated(unsaturated)) {
    //             return Ok(Some(false));
    //         } else {
    //         }
    //         // if !map.contains_key(&Some(NonZeroI8::new(index))) {
    //         //     return Ok(Some(false));
    //         // }
    //     }
    //     Ok(is_unsaturated.map(|is_unsaturated| is_unsaturated == 0))
    // }

    fn is_monounsaturated(self) -> PolarsResult<Option<bool>> {
        self.identifier.is_monounsaturated()
    }

    fn is_polyunsaturated(self) -> PolarsResult<Option<bool>> {
        self.identifier.is_polyunsaturated()
    }

    fn is_cis(self) -> PolarsResult<Option<bool>> {
        self.identifier.is_cis()
    }

    fn is_trans(self) -> PolarsResult<Option<bool>> {
        self.identifier.is_trans()
    }
}

enum Index {
    Delta(NonZeroU8),
    Omega(NonZeroU8),
}

impl MaskExt for &FattyAcidChunked {
    fn try_unsaturated(self, index: Option<NonZeroI8>) -> PolarsResult<Option<bool>> {
        let Some(index) = index else {
            return self.identifier.is_unsaturated();
        };
        if index.is_positive() {
            self.is_delta_unsaturated(index.cast_unsigned())
        } else {
            self.is_omega_unsaturated(index.unsigned_abs())
        }
    }

    fn is_delta_unsaturated(self, index: NonZeroU8) -> PolarsResult<Option<bool>> {
        println!("is_delta: {index}");
        let index = index.get();
        let length = self.carbons();
        if index >= length {
            return Ok(Some(false));
        }
        let mut sized = Some(0);
        let mut r#unsized = Some(Unsized::default());
        let unsaturated = index;
        let saturated = 1..index;
        for (index, bound) in self.try_iter() {
            let bound = bound?;
            match index {
                Some(Some(index)) => {
                    // Convert to delta
                    let index = if index.is_positive() {
                        index.get() as _
                    } else {
                        length.saturating_add_signed(index.get())
                    };
                    if index == unsaturated {
                        match bound {
                            Some(Bound::Saturated) => return Ok(Some(false)),
                            Some(Bound::Unsaturated(_)) => {
                                if let Some(sized) = &mut sized {
                                    *sized += 1
                                }
                            }
                            None => {
                                sized = None;
                                if let Some(r#unsized) = &mut r#unsized {
                                    r#unsized.any += 1;
                                }
                            }
                        }
                    } else if saturated.contains(&index) {
                        match bound {
                            Some(Bound::Saturated) => {
                                if let Some(sized) = &mut sized {
                                    *sized += 1
                                }
                            }
                            Some(Bound::Unsaturated(_)) => return Ok(Some(false)),
                            None => {
                                sized = None;
                                if let Some(r#unsized) = &mut r#unsized {
                                    r#unsized.any += 1;
                                }
                            }
                        }
                    }
                }
                Some(None) => {
                    if let Some(r#unsized) = &mut r#unsized {
                        match bound {
                            Some(Bound::Saturated) => r#unsized.saturated += 1,
                            Some(Bound::Unsaturated(_)) => r#unsized.unsaturated += 1,
                            None => r#unsized.any += 1,
                        }
                    }
                }
                None => r#unsized = None,
            }
        }
        let Some(sized) = sized else {
            return Ok(None);
        };
        Ok(Some(sized == unsaturated))
    }

    fn is_omega_unsaturated(self, index: NonZeroU8) -> PolarsResult<Option<bool>> {
        println!("is_omega: {index}");
        let index = index.get();
        let length = self.carbons();
        if index >= length {
            return Ok(Some(false));
        }
        let mut sized = Some(0);
        let mut r#unsized = Some(Unsized::default());
        let unsaturated = index;
        let saturated = index + 1..length;
        for (index, bound) in self.try_iter() {
            let bound = bound?;
        }
        let Some(sized) = sized else {
            return Ok(None);
        };
        Ok(Some(sized == saturated.len() + 1))
    }

    // fn is_omega(self, index: NonZeroU8) -> PolarsResult<Option<bool>> {
    //     let index = index.get();
    //     let length = self.carbons();
    //     if index >= length {
    //         return Ok(Some(false));
    //     }
    //     let delta = length - index;
    //     let Some(delta) = NonZeroU8::new(delta) else {
    //         return Ok(Some(false));
    //     };
    //     self.is_delta(delta)
    // }
}

#[derive(Clone, Copy, Debug, Default)]
struct Unsized {
    saturated: usize,
    unsaturated: usize,
    any: usize,
}
