//! A collection of types related to the Internet Computer Protocol.
//!
//! If you need support for the serde library, you will need to use the `serde` feature
//! (available by default).

pub type Sha256Digest = [u8; 32];

/// Hash Tree types and traits.
pub mod hash_tree;
#[doc(inline)]
pub use hash_tree::HashTree;

/// Certificate types
pub mod certificate;
#[doc(inline)]
pub use certificate::Certificate;
#[doc(inline)]
pub use certificate::CertificateDelegation;

/// Blob types
pub mod blob;
#[doc(inline)]
pub use blob::Blob;

mod utils;
