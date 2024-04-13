#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/alloy-rs/core/main/assets/alloy.jpg",
    html_favicon_url = "https://raw.githubusercontent.com/alloy-rs/core/main/assets/favicon.ico"
)]
#![warn(
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    unreachable_pub,
    clippy::missing_const_for_fn,
    rustdoc::all
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![deny(unused_must_use, rust_2018_idioms)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

mod receipt;
pub use receipt::{OpReceipt, OpReceiptEnvelope, OpReceiptWithBloom, OpTxReceipt};

mod transaction;
pub use transaction::{
    eip4844_utils, Blob, BlobTransactionSidecar, Bytes48, OpTxEnvelope, OpTxType,
    OpTypedTransaction, SidecarBuilder, SidecarCoder, SimpleCoder, TxDeposit, TxEip1559, TxEip2930,
    TxEip4844, TxEip4844Variant, TxEip4844WithSidecar, TxLegacy,
};

#[cfg(feature = "kzg")]
pub use transaction::BlobTransactionValidationError;

#[cfg(feature = "kzg")]
pub use alloy_eips::eip4844::env_settings::EnvKzgSettings;