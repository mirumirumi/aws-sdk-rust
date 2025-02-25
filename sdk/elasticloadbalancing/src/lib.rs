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
//! <fullname>Elastic Load Balancing</fullname>
//!
//! <p>A load balancer can distribute incoming traffic across your EC2 instances.
//! This enables you to increase the availability of your application. The load balancer
//! also monitors the health of its registered instances and ensures that it routes traffic
//! only to healthy instances. You configure your load balancer to accept incoming traffic
//! by specifying one or more listeners, which are configured with a protocol and port
//! number for connections from clients to the load balancer and a protocol and port number
//! for connections from the load balancer to the instances.</p>
//! <p>Elastic Load Balancing supports three types of load balancers: Application Load Balancers, Network Load Balancers,
//! and Classic Load Balancers. You can select a load balancer based on your application needs. For more
//! information, see the <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/userguide/">Elastic Load Balancing User Guide</a>.</p>
//! <p>This reference covers the 2012-06-01 API, which supports Classic Load Balancers.
//! The 2015-12-01 API supports Application Load Balancers and Network Load Balancers.</p>
//!
//! <p>To get started, create a load balancer with one or more listeners using <a>CreateLoadBalancer</a>.
//! Register your instances with the load balancer using <a>RegisterInstancesWithLoadBalancer</a>.</p>
//!
//! <p>All Elastic Load Balancing operations are <i>idempotent</i>, which means
//! that they complete at most one time. If you repeat an operation, it succeeds with a 200 OK
//! response code.</p>
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

/// Generated accessors for nested fields
mod lens;

mod query_ser;

mod xml_deser;

mod rest_xml_wrapped_errors;

/// Crate version number.
pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
static API_METADATA: aws_http::user_agent::ApiMetadata =
    aws_http::user_agent::ApiMetadata::new("elasticloadbalancing", PKG_VERSION);
pub use aws_smithy_http::endpoint::Endpoint;
pub use aws_types::app_name::AppName;
pub use aws_types::region::Region;
pub use aws_types::Credentials;
#[doc(inline)]
pub use client::Client;
