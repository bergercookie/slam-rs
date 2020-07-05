#![allow(clippy::missing_errors_doc)] // TODO
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::multiple_crate_versions)]
#![allow(missing_docs)] // TODO
#![allow(unused_imports)] // TODO
#![allow(unused_variables)] // TODO
#![warn(clippy::cargo)]
#![warn(clippy::nursery)]
#![warn(missing_debug_implementations)]
#![warn(rust_2018_idioms)]
// #![warn(clippy::all)]
// #![warn(clippy::pedantic)]
#![cfg_attr(test, feature(proc_macro_hygiene))]

pub mod drivers;
pub mod utils;

pub use self::drivers::{
    DatasetDriver, DatasetDriverState, EurocDriver, EurocStreamGray, FiniteStream, Stream,
};
pub use self::utils::errors;
