//! Error types
//!
use thiserror::Error;

// Errors that may be returned by an Ownable program
#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum SecurityError {
    /// The account cannot be initialized because it is already being used.
    #[error("Account is not the program owner")]
    InvalidOwner,
}
