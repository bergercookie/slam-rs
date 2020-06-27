#![warn(missing_debug_implementations)]
#![warn(rust_2018_idioms)]
#![allow(missing_docs)] // TODO
#![allow(clippy::missing_errors_doc)] // TODO
// #![warn(clippy::all)]
// #![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![allow(clippy::module_name_repetitions)]

pub mod drivers;
pub mod utils;

pub use self::drivers::{
    DatasetDriver, DatasetDriverState, EurocDriver, EurocStreamGray, FiniteStream, Stream,
};
pub use self::utils::errors;
