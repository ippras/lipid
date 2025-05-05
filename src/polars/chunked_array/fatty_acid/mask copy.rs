use crate::prelude::*;
use polars::prelude::*;
use std::num::{NonZeroI8, NonZeroU8};

impl FattyAcidChunked {
    fn is_index(self, index: Option<NonZeroI8>) -> PolarsResult<Option<bool>> {
        let Some(index) = index else {
            return Ok(self.bound.not_equal(S)?.any_kleene());
        };
        if index.is_positive() {
            self.is_delta(index.cast_unsigned())
        } else {
            self.is_omega(index)
        }
    }

    fn is_delta(self, index: NonZeroU8) -> PolarsResult<Option<bool>> {
        todo!()
    }

    pub fn is_omega(&self, index: NonZeroI8) -> PolarsResult<Option<bool>> {
        assert!(index.is_negative());
        let mut sized = Some(0);
        let mut r#unsized = Some(Unsized::default());
        let target = index.get();
        let range = target..0;
        let carbons = self.carbons();
        for (source, bound) in self.try_iter() {
            let bound = bound?;
            match source {
                Some(Some(source)) => {
                    let forward = source.get();
                    let backward = if forward.is_positive() {
                        forward.saturating_sub_unsigned(carbons)
                    } else {
                        forward.saturating_add_unsigned(carbons)
                    };
                    if forward == target || backward == target {
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
                    } else if range.contains(&forward) || range.contains(&backward) {
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
        // println!("sized: {sized:?}");
        // println!("target: {target:?}");
        Ok(sized.map(|sized| sized == target.abs()))
    }
    // pub fn is_omega(&self, index: NonZeroI8) -> PolarsResult<Option<bool>> {
    //     assert!(index.is_negative());
    //     // forward
    //     let carbons = self.carbons();
    //     let backward = |index: i8| {
    //         if index.is_positive() {
    //             index.saturating_sub_unsigned(carbons)
    //         } else {
    //             index.saturating_add_unsigned(carbons)
    //         }
    //     };
    //     let mut sized = Some(0);
    //     let mut r#unsized = Some(Unsized::default());
    //     let target = index.get();
    //     let range = target..0;
    //     for (source, bound) in self.try_iter() {
    //         let bound = bound?;
    //         match source {
    //             Some(Some(source)) => {
    //                 let forward = source.get();
    //                 let backward = backward(forward);
    //                 if forward == target || backward == target {
    //                     match bound {
    //                         Some(Bound::Saturated) => return Ok(Some(false)),
    //                         Some(Bound::Unsaturated(_)) => {
    //                             if let Some(sized) = &mut sized {
    //                                 *sized += 1
    //                             }
    //                         }
    //                         None => {
    //                             sized = None;
    //                             if let Some(r#unsized) = &mut r#unsized {
    //                                 r#unsized.any += 1;
    //                             }
    //                         }
    //                     }
    //                 } else if range.contains(&forward) || range.contains(&backward) {
    //                     match bound {
    //                         Some(Bound::Saturated) => {
    //                             if let Some(sized) = &mut sized {
    //                                 *sized += 1
    //                             }
    //                         }
    //                         Some(Bound::Unsaturated(_)) => return Ok(Some(false)),
    //                         None => {
    //                             sized = None;
    //                             if let Some(r#unsized) = &mut r#unsized {
    //                                 r#unsized.any += 1;
    //                             }
    //                         }
    //                     }
    //                 }
    //             }
    //             Some(None) => {
    //                 if let Some(r#unsized) = &mut r#unsized {
    //                     match bound {
    //                         Some(Bound::Saturated) => r#unsized.saturated += 1,
    //                         Some(Bound::Unsaturated(_)) => r#unsized.unsaturated += 1,
    //                         None => r#unsized.any += 1,
    //                     }
    //                 }
    //             }
    //             None => r#unsized = None,
    //         }
    //     }
    //     // println!("sized: {sized:?}");
    //     // println!("target: {target:?}");
    //     Ok(sized.map(|sized| sized == target.abs()))
    // }
}

impl Mask for &FattyAcidChunked {
    type Output = PolarsResult<Option<bool>>;

    /// # Returns
    ///
    /// The output is unknown (`None`) if the array contains any null values and
    /// no `false` values.
    fn is_saturated(self) -> PolarsResult<Option<bool>> {
        let mask = self.bound.equal(S)?;
        Ok(mask.all_kleene())
    }

    /// Checks if the bound chunked array contains any unsaturated bonds.
    ///
    /// # Returns
    ///
    /// The output is unknown (`None`) if the array contains any null values and
    /// no `true` values.
    fn is_unsaturated(self) -> PolarsResult<Option<bool>> {
        let mask = self.bound.not_equal(S)?;
        Ok(mask.any_kleene())
    }

    //     *
    // 1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19;
    fn is_unsaturated_before(self, index: Option<NonZeroI8>) -> PolarsResult<Option<bool>> {
        // forward
        let carbons = self.carbons();
        let backward = |index: i8| {
            if index.is_positive() {
                index.saturating_sub_unsigned(carbons)
            } else {
                index.saturating_add_unsigned(carbons)
            }
        };

        let Some(index) = index else {
            return Ok(self.bound.not_equal(S)?.any_kleene());
        };
        let mut sized = (Some(0), Some(0));
        let mut r#unsized = Some(0);
        let forward = index.get();
        let target = (forward, backward(forward));
        let range = (
            target.0.min(0)..target.0.max(0),
            target.1.min(0)..target.1.max(0),
        );
        for (source, bound) in self.try_iter() {
            let bound = bound?;
            match source {
                Some(Some(source)) => {
                    let forward = source.get();
                    let backward = backward(forward);
                    if forward == target.0 || backward == target.0 {
                        match bound {
                            Some(Bound::Unsaturated(_)) => {
                                if let Some(sized) = &mut sized.0 {
                                    *sized += 1
                                }
                            }
                            None => r#unsized = None,
                            _ => {}
                        }
                    } else if range.0.contains(&forward) || range.0.contains(&backward) {
                        match bound {
                            Some(Bound::Saturated) => {
                                if let Some(sized) = &mut sized.0 {
                                    *sized += 1
                                }
                            }
                            None => r#unsized = None,
                            _ => {}
                        }
                    }
                    if forward == target.1 || backward == target.1 {
                        match bound {
                            // Some(Bound::Saturated) => return Ok(Some(false)),
                            Some(Bound::Unsaturated(_)) => {
                                if let Some(sized) = &mut sized.1 {
                                    *sized += 1
                                }
                            }
                            None => r#unsized = None,
                            _ => {}
                        }
                    } else if range.1.contains(&forward) || range.1.contains(&backward) {
                        // println!("contains1: {forward}/{backward} {bound:?} {range:?}");
                        match bound {
                            Some(Bound::Saturated) => {
                                if let Some(sized) = &mut sized.1 {
                                    *sized += 1
                                }
                            }
                            // Some(Bound::Unsaturated(_)) => return Ok(Some(false)),
                            None => r#unsized = None,
                            _ => {}
                        }
                    }
                }
                Some(None) => r#unsized = r#unsized.map(|r#unsized| r#unsized + 1),
                None => r#unsized = None,
            }
        }
        // println!("sized: {sized:?}");
        // println!("target: {target:?}");
        Ok(sized
            .0
            .zip(sized.1)
            .map(|sized| sized.0 == target.0.abs() || sized.1 == target.1.abs()))
    }

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

    /// Checks if the bound chunked array contains exactly one unsaturated bond.
    ///
    /// # Returns
    ///
    /// [`true`] if there is exactly one unsaturated bond, [`false`] otherwise.
    fn is_monounsaturated(self) -> PolarsResult<Option<bool>> {
        let mask = self.bound.not_equal(S)?;
        if mask.has_nulls() {
            Ok(None)
        } else {
            Ok(Some(mask.num_trues() == 1))
        }
    }

    /// Checks if the bound chunked array contains more than one unsaturated bond.
    ///
    /// # Returns
    ///
    /// [`true`] if there are more than one unsaturated bonds, [`false`] otherwise.
    fn is_polyunsaturated(self) -> PolarsResult<Option<bool>> {
        let mask = self.bound.not_equal(S)?;
        let trues = mask.num_trues();
        if trues > 1 {
            Ok(Some(true))
        } else if trues + mask.null_count() > 1 {
            Ok(None)
        } else {
            Ok(Some(false))
        }
    }

    /// Checks if the bound chunked array contains unsaturated cis-only bonds.
    ///
    /// # Returns
    ///
    /// [`true`] if all unsaturated bounds are cis, [`false`] otherwise.
    fn is_cis(self) -> PolarsResult<Option<bool>> {
        let mut is_cis = Some(false);
        for bound in self.bound.try_iter() {
            let bound = bound?;
            is_cis = match bound {
                Some(Bound::Saturated) => is_cis,
                Some(Bound::Unsaturated(Unsaturated {
                    isomerism: Some(Isomerism::Cis),
                    ..
                })) => Some(true),
                Some(Bound::Unsaturated(Unsaturated {
                    isomerism: Some(Isomerism::Trans),
                    ..
                })) => return Ok(Some(false)),
                _ => None,
            };
        }
        Ok(is_cis)
    }

    /// Checks if the bound chunked array contains any trans bonds.
    ///
    /// # Returns
    ///
    /// [`true`] if any bound is trans, [`false`] otherwise.
    fn is_trans(self) -> PolarsResult<Option<bool>> {
        let mut is_trans = Some(false);
        for bound in self.bound.try_iter() {
            let bound = bound?;
            is_trans = match bound {
                Some(Bound::Saturated) => is_trans,
                Some(Bound::Unsaturated(Unsaturated {
                    isomerism: Some(Isomerism::Cis),
                    ..
                })) => is_trans,
                Some(Bound::Unsaturated(Unsaturated {
                    isomerism: Some(Isomerism::Trans),
                    ..
                })) => return Ok(Some(true)),
                _ => None,
            };
        }
        Ok(is_trans)
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct Unsized {
    saturated: usize,
    unsaturated: usize,
    any: usize,
}
