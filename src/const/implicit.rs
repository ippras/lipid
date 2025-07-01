#![rustfmt::skip]

use crate::prelude::*;
use fatty_acid_macro::fatty_acid;
use polars::prelude::*;
use std::sync::LazyLock;

pub static C26U0U0: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C26 { 0 => U, 0 => U }).unwrap());

pub static C26U0U0U0: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C26 { 0 => U, 0 => U, 0 => U }).unwrap());

pub static C26U0U0U0U0: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C26 { 0 => U, 0 => U, 0 => U, 0 => U }).unwrap());

pub static C26U0U0U0U0U0: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C26 { 0 => U, 0 => U, 0 => U, 0 => U, 0 => U }).unwrap());

pub static C26U0U0U0U0U0U0: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C26 { 0 => U, 0 => U, 0 => U, 0 => U, 0 => U, 0 => U }).unwrap());

pub static C28U0: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C28 { 0 => U }).unwrap());

pub static C28U0U0: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C28 { 0 => U, 0 => U }).unwrap());

pub static C32U0: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C32 { 0 => U }).unwrap());

pub static C32U0U0: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C32 { 0 => U, 0 => U }).unwrap());

pub static C34U0: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C34 { 0 => U }).unwrap());

pub static C34U0U0: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C34 { 0 => U, 0 => U }).unwrap());

pub static C36U0: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C36 { 0 => U }).unwrap());

pub static C36U0U0: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C36 { 0 => U, 0 => U }).unwrap());
