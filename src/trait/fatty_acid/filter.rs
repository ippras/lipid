use super::FattyAcidTrait;

/// Fatty acid filter
pub trait FattyAcidFilter: FattyAcidTrait {
    fn dienoics(self, expr: Self::Output) -> Self::Output;
    fn hexaenoics(self, expr: Self::Output) -> Self::Output;
    fn monoenoics(self, expr: Self::Output) -> Self::Output;
    fn pentaenoics(self, expr: Self::Output) -> Self::Output;
    fn tetraenoics(self, expr: Self::Output) -> Self::Output;
    fn trienoic(self, expr: Self::Output) -> Self::Output;
}
