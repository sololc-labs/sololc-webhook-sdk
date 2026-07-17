//! # Sololc Webhook SDK
//!
//! Provides a high-performance, modern, and ergonomic WASI-HTTP Webhook SDK
//! optimized for the Sololc Runtime.
//!
//! This crate simplifies the development of Webhook microservices by abstracting
//! raw WebAssembly System Interface (WASI) HTTP components into a routing style
//! reminiscent of mainstream Rust web frameworks (such as Axum or Actix-web).

wit_bindgen::generate!({
    path: "wit",
    world: "webhook-proxy",
    generate_all,
});

mod http_types;

/// Represents the primary entrypoint contract exported to the host gateway.
///
/// Under the hood, this handles the low-level WASIp2 `wasi:http/incoming-handler`
/// interface. It is driven asynchronously by the host executor.
///
/// See also [`wasi::http::types`] for the underlying raw assets.
pub use exports::wasi::http::incoming_handler::Guest;


/// Exposes the official WASI-HTTP types used for lower-level requests and responses.
///
/// Contains critical raw assets such as [`IncomingRequest`], [`OutgoingResponse`],
/// [`ResponseOutparam`], and [`Fields`]. For a more ergonomic experience, use the SDK's
/// high-level wrapper types [`Request`] and [`Response`] instead.
pub use self::wasi::http::types::{Fields, IncomingRequest, OutgoingResponse, ResponseOutparam};

/// Exposes the ergonomic high-level wrapper types.
///
/// Re-exports [`Request`] and [`Response`] which abstract raw incoming streams and
/// field structures into standard, intuitive web structures.
pub use self::http_types::{Request, Response};

/// Routes an incoming HTTP `GET` request to the annotated handler function.
///
/// Acts as the entrypoint for retrieving read-only resources within your plugin.
///
/// # Examples
///
/// ```rust
/// use sololc_webhook_sdk::{get, Request, Response};
///
/// #[get("/api/v1/auth")]
/// async fn check_auth(req: Request) -> Response {
///     Response::json(200, r#"{"status": "approved"}"#)
/// }
/// ```
pub use sololc_webhook_macros::get;

/// Routes an incoming HTTP `POST` request to the annotated handler function.
///
/// Handles data creation, event synchronizations, or heavy payloads sent by Webhook triggers.
///
/// # Examples
///
/// ```rust
/// use sololc_webhook_sdk::{post, Request, Response};
///
/// #[post("/api/v1/telemetry")]
/// async fn record_metrics(req: Request) -> Response {
///     Response::json(201, r#"{"status": "recorded"}"#)
/// }
/// ```
pub use sololc_webhook_macros::post;

/// Routes an incoming HTTP `PUT` request to the annotated handler function.
///
/// Replaces or updates an existing entity completely with the provided payload.
pub use sololc_webhook_macros::put;

/// Routes an incoming HTTP `DELETE` request to the annotated handler function.
///
/// Removes or revokes resources associated with the matching routing path.
pub use sololc_webhook_macros::delete;

/// Routes an incoming HTTP `PATCH` request to the annotated handler function.
///
/// Applies partial modifications or fractional updates to a specific resource.
pub use sololc_webhook_macros::patch;

/// Routes an incoming HTTP `HEAD` request to the annotated handler function.
///
/// Retrieves only the headers of a resource, identical to a `GET` request but without the response body.
pub use sololc_webhook_macros::head;

/// Routes an incoming HTTP `OPTIONS` request to the annotated handler function.
///
/// Describes the communication options and allowed methods for the target resource.
pub use sololc_webhook_macros::options;

// --- 1. Logging client module ---
#[cfg(feature = "logging")]
pub mod logging;

/// Initializes the global WASIp2 system logger and binds it to the `log` ecosystem.
///
/// Returns a [`Result<(), log::SetLoggerError>`][log::SetLoggerError] indicating
/// whether the active global logger was bound successfully.
#[cfg(feature = "logging")]
pub use logging::init as init_logger;

// --- 2. Key-value storage module ---
#[cfg(feature = "kv")]
pub mod kv;

/// Represents a safe, high-level abstraction over the WASIp2 `wasi:keyvalue` system.
///
/// Leverages the state-bound APIs provided by the Sololc host executor to persist
/// and fetch arbitrary byte-buffers synchronously within standard asynchronous routines.
#[cfg(feature = "kv")]
pub use kv::KvStore;

// --- 3. Asynchronous HTTP client module ---
#[cfg(feature = "http-client")]
pub mod http_client;

/// Dispatches outgoing HTTP requests utilizing the WASIp2 `wasi:http/outgoing-handler`.
///
/// Coordinates connection pools and underlying network transports securely via the sandbox gateway.
#[cfg(feature = "http-client")]
pub use crate::http_client::client::HttpClient;
