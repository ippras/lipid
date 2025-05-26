use crate::prelude::*;
use std::num::NonZeroI8;

/// Sized
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Sized;

/// Unized
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Unized;

/// Extension methods for [`Iterator`]
pub trait IndexIteratorExt<T> {
    fn sized(self) -> impl Iterator<Item = (Option<NonZeroI8>, T)>;

    fn r#unsized(self) -> impl Iterator<Item = (Unized, T)>;

    fn explicit(self) -> impl Iterator<Item = (NonZeroI8, T)>;

    fn implicit(self) -> impl Iterator<Item = (Option<Sized>, T)>;
}

impl<I: Iterator<Item = (Option<Option<NonZeroI8>>, T)>, T> IndexIteratorExt<T> for I {
    fn sized(self) -> impl Iterator<Item = (Option<NonZeroI8>, T)> {
        self.filter_map(|(index, bound)| Some((index?, bound)))
    }

    fn r#unsized(self) -> impl Iterator<Item = (Unized, T)> {
        self.filter_map(|(index, bound)| {
            if index.is_none() {
                Some((Unized, bound))
            } else {
                None
            }
        })
    }

    fn explicit(self) -> impl Iterator<Item = (NonZeroI8, T)> {
        self.filter_map(|(index, bound)| Some((index??, bound)))
    }

    fn implicit(self) -> impl Iterator<Item = (Option<Sized>, T)> {
        self.filter_map(|(index, bound)| match index {
            None => Some((None, bound)),
            Some(None) => Some((Some(Sized), bound)),
            _ => None,
        })
    }
}

/// Extension methods for [`Iterator`]
pub trait IdentifierIteratorExt<T> {
    fn is_saturated(&mut self) -> bool;

    fn is_unsaturated(&mut self) -> bool;

    fn saturated(self) -> impl Iterator<Item = (T, Saturated)>;

    fn unsaturated(self) -> impl Iterator<Item = (T, Unsaturated)>;

    fn not_saturated(self) -> impl Iterator<Item = (T, Option<Unsaturated>)>;

    fn not_unsaturated(self) -> impl Iterator<Item = (T, Option<Saturated>)>;
}

impl<I: Iterator<Item = (T, Bound)>, T> IdentifierIteratorExt<T> for I {
    fn is_saturated(&mut self) -> bool {
        self.all(|(_, bound)| bound.0.is_some_and(Explicit::is_saturated))
    }

    fn is_unsaturated(&mut self) -> bool {
        self.any(|(_, bound)| bound.0.is_some_and(Explicit::is_unsaturated))
    }

    fn saturated(self) -> impl Iterator<Item = (T, Saturated)> {
        self.filter_map(|(index, bound)| Some((index, bound.0?.as_saturated()?)))
    }

    fn unsaturated(self) -> impl Iterator<Item = (T, Unsaturated)> {
        self.filter_map(|(index, bound)| Some((index, bound.0?.as_unsaturated()?)))
    }

    fn not_saturated(self) -> impl Iterator<Item = (T, Option<Unsaturated>)> {
        self.filter_map(|(index, bound)| match bound.0 {
            None => Some((index, None)),
            Some(Explicit::Saturated) => None,
            Some(Explicit::Unsaturated(unsaturated)) => Some((index, Some(unsaturated))),
        })
    }

    fn not_unsaturated(self) -> impl Iterator<Item = (T, Option<Saturated>)> {
        self.filter_map(|(index, bound)| match bound.0 {
            None => Some((index, None)),
            Some(Explicit::Saturated) => Some((index, Some(Saturated))),
            Some(Explicit::Unsaturated(_)) => None,
        })
    }
}
