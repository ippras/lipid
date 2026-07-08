#![rustfmt::skip]

use crate::prelude::*;
use fatty_acid_proc_macro::fatty_acid;
use polars::prelude::*;
use std::sync::LazyLock;

pub static C26U0U0: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 26 U 2 { 0: U, 0: U }).unwrap());

pub static C26U0U0U0: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 26 U 3 { 0: U, 0: U, 0: U }).unwrap());

pub static C26U0U0U0U0: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 26 U 4 { 0: U, 0: U, 0: U, 0: U }).unwrap());

pub static C26U0U0U0U0U0: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 26 U 5 { 0: U, 0: U, 0: U, 0: U, 0: U }).unwrap());

pub static C26U0U0U0U0U0U0: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 26 U 6 { 0: U, 0: U, 0: U, 0: U, 0: U, 0: U }).unwrap());

pub static C28U0: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 28 U 1 { 0: U }).unwrap());

pub static C28U0U0: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 28 U 2 { 0: U, 0: U }).unwrap());

pub static C32U0: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 32 U 1 { 0: U }).unwrap());

pub static C32U0U0: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 32 U 2 { 0: U, 0: U }).unwrap());

pub static C34U0: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 34 U 1 { 0: U }).unwrap());

pub static C34U0U0: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 34 U 2 { 0: U, 0: U }).unwrap());

pub static C36U0: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 36 U 1 { 0: U }).unwrap());

pub static C36U0U0: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 36 U 2 { 0: U, 0: U }).unwrap());
