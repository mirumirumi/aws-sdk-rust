#![allow(deprecated)]
#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::blacklisted_name)]
#![allow(clippy::vec_init_then_push)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_return)]
#![allow(rustdoc::bare_urls)]
#![warn(missing_docs)]
//! <fullname>Directory Service</fullname>
//! <p>Directory Service is a web service that makes it easy for you to setup and run directories in the
//! Amazon Web Services cloud, or connect your Amazon Web Services resources with an existing self-managed Microsoft Active
//! Directory. This guide provides detailed information about Directory Service operations, data types,
//! parameters, and errors. For information about Directory Services features, see <a href="https://aws.amazon.com/directoryservice/">Directory Service</a> and the <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/what_is.html">Directory Service
//! Administration Guide</a>.</p>
//! <note>
//! <p>Amazon Web Services provides SDKs that consist of libraries and sample code for various
//! programming languages and platforms (Java, Ruby, .Net, iOS, Android, etc.). The SDKs
//! provide a convenient way to create programmatic access to Directory Service and other Amazon Web Services
//! services. For more information about the Amazon Web Services SDKs, including how to download and
//! install them, see <a href="http://aws.amazon.com/tools/">Tools for Amazon Web
//! Services</a>.</p>
//! </note>
//!
//! # Crate Organization
//!
//! The entry point for most customers will be [`Client`]. [`Client`] exposes one method for each API offered
//! by the service.
//!
//! Some APIs require complex or nested arguments. These exist in [`model`](crate::model).
//!
//! Lastly, errors that can be returned by the service are contained within [`error`]. [`Error`] defines a meta
//! error encompassing all possible errors that can be returned by the service.
//!
//! The other modules within this crate are not required for normal usage.

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use error_meta::Error;

#[doc(inline)]
pub use config::Config;

/// Client and fluent builders for calling the service.
pub mod client;

/// Configuration for the service.
pub mod config;

/// All error types that operations can return. Documentation on these types is copied from the model.
pub mod error;

mod error_meta;

/// Input structures for operations. Documentation on these types is copied from the model.
pub mod input;

/// Data structures used by operation inputs/outputs. Documentation on these types is copied from the model.
pub mod model;

/// All operations that this crate can perform.
pub mod operation;

/// Output structures for operations. Documentation on these types is copied from the model.
pub mod output;

/// Data primitives referenced by other data types.
pub mod types;

mod aws_endpoint;

pub mod middleware;

mod no_credentials;

mod operation_deser;

mod operation_ser;

/// Paginators for the service
pub mod paginator;

mod json_deser;

mod json_ser;

/// Generated accessors for nested fields
mod lens;

mod json_errors;

/// Crate version number.
pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
static API_METADATA: aws_http::user_agent::ApiMetadata =
    aws_http::user_agent::ApiMetadata::new("directoryservice", PKG_VERSION);
pub use aws_smithy_http::endpoint::Endpoint;
pub use aws_types::app_name::AppName;
pub use aws_types::region::Region;
pub use aws_types::Credentials;
#[doc(inline)]
pub use client::Client;
