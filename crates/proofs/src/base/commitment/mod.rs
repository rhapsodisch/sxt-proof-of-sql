//! Types for creation and utilization of cryptographic commitments to proof-of-sql data.
use crate::base::scalar::Scalar;
pub use blitzar::compute::{init_backend, init_backend_with_config, BackendConfig};
use curve25519_dalek::ristretto::RistrettoPoint;

mod committable_column;
pub use committable_column::CommittableColumn;

mod vec_commitment_ext;
pub use vec_commitment_ext::VecCommitmentExt;

mod column_bounds;
use super::scalar::ArkScalar;
pub use column_bounds::{ColumnBounds, ColumnBoundsMismatch};

/// A trait for using commitment schemes generically.
pub trait Commitment {
    /// The associated scalar that the commitment is for.
    /// There are multiple possible commitment schemes for a scalar, but only one scalar for any commitment.
    type Scalar: Scalar;
}
impl Commitment for RistrettoPoint {
    type Scalar = ArkScalar;
}