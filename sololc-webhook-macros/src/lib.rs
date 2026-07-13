//! # Sololc Webhook Macros
//!
//! Provides high-performance, zero-dependency procedural macro attributes for 
//! compile-time static routing configuration within the Sololc Webhook ecosystem.
//!
//! This crate works closely with `sololc-webhook-sdk` to expand declarative attributes
//! into standard WASI-HTTP component model `wasi:http/incoming-handler` exports.

extern crate proc_macro;
use proc_macro::TokenStream;

/// Routes an incoming HTTP `GET` request to the annotated handler function.
///
/// Generates the static routing glue code at compile-time to match the URI path and HTTP method.
///
/// # Examples
///
/// ```ignore
/// use sololc_webhook_sdk::{get, Request, Response};
///
/// #[get("/api/v1/webhook")]
/// async fn my_handler(req: Request) -> Response {
///     Response::json(200, r#"{"status": "ok"}"#)
/// }
/// ```
#[proc_macro_attribute]
pub fn get(attr: TokenStream, item: TokenStream) -> TokenStream {
    generate_webhook_route("GET", attr, item)
}

/// Routes an incoming HTTP `POST` request to the annotated handler function.
///
/// Generates the static routing glue code at compile-time to match the URI path and HTTP method.
///
/// # Examples
///
/// ```ignore
/// use sololc_webhook_sdk::{post, Request, Response};
///
/// #[post("/api/v1/events")]
/// async fn process_event(req: Request) -> Response {
///     Response::json(201, r#"{"event": "accepted"}"#)
/// }
/// ```
#[proc_macro_attribute]
pub fn post(attr: TokenStream, item: TokenStream) -> TokenStream {
    generate_webhook_route("POST", attr, item)
}

/// Routes an incoming HTTP `PUT` request to the annotated handler function.
///
/// Generates the static routing glue code at compile-time to match the URI path and HTTP method.
#[proc_macro_attribute]
pub fn put(attr: TokenStream, item: TokenStream) -> TokenStream {
    generate_webhook_route("PUT", attr, item)
}

/// Routes an incoming HTTP `DELETE` request to the annotated handler function.
///
/// Generates the static routing glue code at compile-time to match the URI path and HTTP method.
#[proc_macro_attribute]
pub fn delete(attr: TokenStream, item: TokenStream) -> TokenStream {
    generate_webhook_route("DELETE", attr, item)
}

/// Routes an incoming HTTP `PATCH` request to the annotated handler function.
///
/// Generates the static routing glue code at compile-time to match the URI path and HTTP method.
#[proc_macro_attribute]
pub fn patch(attr: TokenStream, item: TokenStream) -> TokenStream {
    generate_webhook_route("PATCH", attr, item)
}

/// Routes an incoming HTTP `HEAD` request to the annotated handler function.
///
/// Generates the static routing glue code at compile-time to match the URI path and HTTP method.
#[proc_macro_attribute]
pub fn head(attr: TokenStream, item: TokenStream) -> TokenStream {
    generate_webhook_route("HEAD", attr, item)
}

/// Routes an incoming HTTP `OPTIONS` request to the annotated handler function.
///
/// Generates the static routing glue code at compile-time to match the URI path and HTTP method.
#[proc_macro_attribute]
pub fn options(attr: TokenStream, item: TokenStream) -> TokenStream {
    generate_webhook_route("OPTIONS", attr, item)
}

/// Generates the comprehensive WASI-HTTP proxy boilerplate from raw attributes and tokens.
///
/// Extracts the target handler identifier dynamically from the `item` stream, wraps it 
/// inside a standard `futures::executor::block_on` runtime sequence, and welds it directly
/// to the underlying guest capabilities export layer.
fn generate_webhook_route(method: &str, attr: TokenStream, item: TokenStream) -> TokenStream {
    let path_str = attr.to_string();
    let func_str = item.to_string();
    let expected_method = method.to_uppercase();

    // 🌟 动态命名提取算法：在无需 syn 库的极轻量模式下，精准捕获开发者的原始异步函数标识符
    let func_name = func_str
        .split_whitespace()
        .skip_while(|&token| token != "fn")
        .nth(1)
        .and_then(|name| name.split('(').next())
        .unwrap_or("handle_webhook")
        .trim();

    let code_template = format!(
        "
        // Emits the original developer-defined asynchronous block seamlessly.
        {}

        struct WitProxyImpl;

        impl sololc_webhook_sdk::Guest for WitProxyImpl {{
            fn handle(
                wasi_req: sololc_webhook_sdk::IncomingRequest,
                wasi_outparam: sololc_webhook_sdk::ResponseOutparam,
            ) {{
                let current_path = wasi_req.path_with_query().unwrap_or_default();
                let current_method_raw = wasi_req.method();
                let current_method = format!(\"{{:?}}\", current_method_raw).to_uppercase();

                if current_path.starts_with({}) && current_method == \"{}\" {{
                    let req_wrapper = sololc_webhook_sdk::Request::from_wasi(wasi_req);
                    
                    // Invokes the dynamically resolved handler identifier function.
                    let future = {}(req_wrapper); 
                    let app_resp = futures::executor::block_on(future);
                    
                    sololc_webhook_sdk::ResponseOutparam::set(wasi_outparam, Ok(app_resp.into_wasi()));
                }} else if current_path.starts_with({}) && current_method != \"{}\" {{
                    let fields = sololc_webhook_sdk::Fields::new();
                    let resp = sololc_webhook_sdk::OutgoingResponse::new(fields);
                    resp.set_status_code(405).unwrap();
                    sololc_webhook_sdk::ResponseOutparam::set(wasi_outparam, Ok(resp));
                }} else {{
                    let fields = sololc_webhook_sdk::Fields::new();
                    let resp = sololc_webhook_sdk::OutgoingResponse::new(fields);
                    resp.set_status_code(404).unwrap();
                    sololc_webhook_sdk::ResponseOutparam::set(wasi_outparam, Ok(resp));
                }}
            }}
        }}

        sololc_webhook_sdk::export!(WitProxyImpl);
        ",
        func_str, path_str, expected_method, func_name, path_str, expected_method
    );

    code_template.parse().expect("Failed to parse generated webhook glue code")
}