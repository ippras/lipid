#![rustfmt::skip]

use crate::prelude::*;
use fatty_acid_proc_macro::fatty_acid;
use polars::prelude::*;
use std::sync::LazyLock;

pub static C26U2: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 26 U 2 {}).unwrap());

pub static C26U3: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 26 U 3 {}).unwrap());

pub static C26U4: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 26 U 4 {}).unwrap());

pub static C26U5: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 26 U 5 {}).unwrap());

pub static C26U6: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 26 U 6 {}).unwrap());

pub static C28U1: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 28 U 1 {}).unwrap());

pub static C28U2: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 28 U 2 {}).unwrap());

pub static C32U1: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 32 U 1 {}).unwrap());

pub static C32U2: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 32 U 2 {}).unwrap());

pub static C34U1: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 34 U 1 {}).unwrap());

pub static C34U2: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 34 U 2 {}).unwrap());

pub static C36U1: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 36 U 1 {}).unwrap());

pub static C36U2: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 36 U 2 {}).unwrap());
