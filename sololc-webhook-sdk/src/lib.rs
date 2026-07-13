//! # Sololc Webhook SDK
//!
//! Provides a high-performance, modern, and ergonomic WASI-HTTP Webhook SDK 
//! optimized for the Sololc Runtime.
//!
//! This crate simplifies the development of Webhook microservices by abstracting
//! raw WebAssembly System Interface (WASI) HTTP components into a routing style
//! reminiscent of mainstream Rust web frameworks (such as Axum or Actix-web).

// 1. 自动定位本地相对于项目根目录的 wit 协议文件，并自动焊接底层契约
wit_bindgen::generate!({
    path: "../wit/webhook.wit", 
    world: "webhook-proxy",
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
/// This module contains critical raw assets such as [`IncomingRequest`], [`OutgoingResponse`],
/// [`ResponseOutparam`], and [`Fields`]. For a more ergonomic experience, use the SDK's
/// high-level wrapper types [`Request`] and [`Response`] instead.
pub use wasi::http::types::{
    IncomingRequest, 
    ResponseOutparam, 
    OutgoingResponse, 
    Fields
};

/// Exposes the ergonomic high-level wrapper types.
///
/// Re-exports [`Request`] and [`Response`] which abstract raw incoming streams and
/// field structures into standard, intuitive web structures.
pub use http_types::{Request, Response};

/// Routes an incoming HTTP `GET` request to the annotated handler function.
///
/// Represents the entrypoint for retrieving read-only resources within your plugin.
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