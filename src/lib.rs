#![deny(warnings)]

#[allow(unknown_lints, unused_imports)]

use thiserror::Error;
pub use flann_sys as raw;

mod enums;
mod index;
mod indexable;
mod indices;
mod parameters;
mod slice_index;
mod vec_index;

pub use enums::{Algorithm, CentersInit, Checks, DistanceType, LogLevel};
pub use generic_array::typenum;
pub use index::Index;
pub use indexable::Indexable;
pub use parameters::Parameters;
pub use slice_index::SliceIndex;
pub use vec_index::VecIndex;

#[derive(Copy, Clone, Debug, Error)]
pub enum FlannError {
    #[error("expected {} dimensions in point, but got {} dimensions", expected, got)]
    InvalidPointDimensionality { expected: usize, got: usize },
    #[error("expected number divisible by {}, but got {}, which is not", expected, got)]
    InvalidFlatPointsLen { expected: usize, got: usize },
    #[error("FLANN failed to build index")]
    FailedToBuildIndex,
    #[error("input must have at least one point")]
    ZeroInputPoints,
}

#[derive(Copy, Clone, Debug)]
pub struct Neighbor<D> {
    pub index: usize,
    pub distance_squared: D,
}
