#[cfg(feature = "http-client")]
pub mod client {
    use crate::wasi::http::outgoing_handler;
    use crate::wasi::http::types::{Fields, Method, OutgoingRequest, RequestOptions, Scheme};

    /// Represents a high-performance, asynchronous WASI-HTTP client.
    ///
    /// This client serves as an ergonomic wrapper around the WASIp2 standard outbound HTTP interfaces,
    /// providing both a low-level request dispatcher and high-level convenience methods for common HTTP verbs.
    pub struct HttpClient;

    impl HttpClient {
        /// Dispatches an outbound asynchronous HTTP request to the specified URL.
        ///
        /// This is the core low-level request engine. It handles URL parsing, header population,
        /// request body streaming, and timeout configurations using native WASI-HTTP system bindings.
        ///
        /// # Arguments
        ///
        /// * `method` - The HTTP method (e.g., [`Method::Get`], [`Method::Post`]) to use for the request.
        /// * `url` - A string slice containing the target URL. Must start with `http://` or `https://`.
        /// * `headers` - Optional custom headers represented as [`Fields`].
        /// * `body` - An optional byte slice containing the payload to stream to the remote server.
        /// * `timeout_ms` - An optional timeout value in milliseconds, applied to both connect and first-byte timeouts.
        ///
        /// # Errors
        ///
        /// Returns an `Err(String)` if:
        /// - The URL scheme is unsupported or malformed.
        /// - System streams fail to initialize or write the request body.
        /// - The host environment fails to dispatch the request or returns an [`ErrorCode`].
        ///
        /// [`ErrorCode`]: crate::wasi::http::types::ErrorCode
        pub async fn request(
            method: Method,
            url: &str,
            headers: Option<Fields>,
            body: Option<&[u8]>,
            timeout_ms: Option<u32>,
        ) -> Result<Vec<u8>, String> {
            let (scheme, authority, path_with_query) = Self::parse_url(url)?;

            let request_headers = headers.unwrap_or_else(Fields::new);

            let request = OutgoingRequest::new(request_headers);
            request
                .set_method(&method)
                .map_err(|_| "Failed to set HTTP Method".to_string())?;
            request
                .set_path_with_query(Some(&path_with_query))
                .map_err(|_| "Failed to set Path/Query".to_string())?;
            request
                .set_scheme(Some(&scheme))
                .map_err(|_| "Failed to set Scheme".to_string())?;
            request
                .set_authority(Some(&authority))
                .map_err(|_| "Failed to set Authority".to_string())?;

            if let Some(data) = body {
                if !data.is_empty() {
                    let request_body = request
                        .body()
                        .map_err(|_| "Failed to get request body resource".to_string())?;

                    let output_stream = request_body
                        .write()
                        .map_err(|_| "Failed to open request output stream".to_string())?;

                    output_stream
                        .write(data)
                        .map_err(|e| format!("Failed to write request body: {:?}", e))?;
                    output_stream
                        .flush()
                        .map_err(|e| format!("Failed to flush request body stream: {:?}", e))?;

                    drop(output_stream);

                    let _ = crate::wasi::http::types::OutgoingBody::finish(request_body, None);
                }
            }

            let options = RequestOptions::new();
            if timeout_ms.is_some() {
                let timeout_ns = timeout_ms.map(|ms| (ms as u64) * 1_000_000);

                options
                    .set_connect_timeout(timeout_ns)
                    .map_err(|_| "Failed to set connect timeout".to_string())?;
                options
                    .set_first_byte_timeout(timeout_ns)
                    .map_err(|_| "Failed to set first byte timeout".to_string())?;
            }

            let future_response = outgoing_handler::handle(request, Some(options))
                .map_err(|e| format!("Outbound request dispatch failed: {:?}", e))?;

            let response_result = future_response
                .get()
                .ok_or_else(|| "No response returned from host (timeout or dropped)".to_string())?
                .map_err(|e| format!("HTTP routing/network error: {:?}", e))?;

            let response =
                response_result.map_err(|e| format!("HTTP response error code: {:?}", e))?;

            let incoming_body = response
                .consume()
                .map_err(|_| "Failed to consume response body".to_string())?;

            let stream = incoming_body
                .stream()
                .map_err(|_| "Failed to open response stream".to_string())?;

            let mut payload = Vec::new();
            let chunk_size = 16384;
            loop {
                match stream.read(chunk_size) {
                    Ok(chunk) => {
                        if chunk.is_empty() {
                            break;
                        }
                        payload.extend_from_slice(&chunk);
                    }
                    Err(_) => break,
                }
            }

            Ok(payload)
        }

        // =========================================================================
        // 🚀 High-Level Convenience API
        // =========================================================================

        /// Sends a `GET` request to the specified URL.
        ///
        /// This is a convenience wrapper around [`HttpClient::request`] with no custom headers,
        /// payload body, or custom timeouts.
        ///
        /// # Examples
        ///
        /// ```no_run
        /// # async fn run() -> Result<(), String> {
        /// use sololc_webhook_sdk::HttpClient;
        ///
        /// let response = HttpClient::get("[https://api.github.com/zen](https://api.github.com/zen)").await?;
        /// println!("Response: {}", String::from_utf8_lossy(&response));
        /// # Ok(())
        /// # }
        /// ```
        pub async fn get(url: &str) -> Result<Vec<u8>, String> {
            Self::request(Method::Get, url, None, None, None).await
        }

        /// Sends a `POST` request with a payload and a custom `Content-Type` header.
        ///
        /// Automatically configures the `content-type` header and streams the `body`
        /// to the remote server.
        ///
        /// # Examples
        ///
        /// ```no_run
        /// # async fn run() -> Result<(), String> {
        /// use sololc_webhook_sdk::HttpClient;
        ///
        /// let json_payload = r#"{"status": "active"}"#;
        /// let response = HttpClient::post(
        ///     "[https://httpbin.org/post](https://httpbin.org/post)",
        ///     json_payload.as_bytes(),
        ///     "application/json"
        /// ).await?;
        /// # Ok(())
        /// # }
        /// ```
        pub async fn post(url: &str, body: &[u8], content_type: &str) -> Result<Vec<u8>, String> {
            let headers = Fields::new();
            headers
                .set(
                    &"content-type".to_string(),
                    &[content_type.as_bytes().to_vec()],
                )
                .map_err(|_| "Failed to set content-type header".to_string())?;
            Self::request(Method::Post, url, Some(headers), Some(body), None).await
        }

        /// Sends a `PUT` request with a payload to the specified URL.
        ///
        /// This is a convenience wrapper around [`HttpClient::request`] that maps the
        /// provided `body` to a [`Method::Put`] request.
        pub async fn put(url: &str, body: &[u8]) -> Result<Vec<u8>, String> {
            Self::request(Method::Put, url, None, Some(body), None).await
        }

        /// Sends a `DELETE` request to the specified URL.
        ///
        /// This is a convenience wrapper around [`HttpClient::request`] utilizing
        /// [`Method::Delete`] to request resource removal on the remote server.
        pub async fn delete(url: &str) -> Result<Vec<u8>, String> {
            Self::request(Method::Delete, url, None, None, None).await
        }

        // ==========================================
        // 🔒 Private Helpers
        // ==========================================

        /// Parses a URL string slice into standard WASI-HTTP scheme, authority, and path segments.
        ///
        /// # Errors
        ///
        /// Returns an `Err` if the URL prefix is neither `http://` nor `https://`.
        fn parse_url(url: &str) -> Result<(Scheme, String, String), String> {
            let scheme = if url.starts_with("https://") {
                Scheme::Https
            } else if url.starts_with("http://") {
                Scheme::Http
            } else {
                return Err("Unsupported URL scheme (must be http:// or https://)".to_string());
            };

            let raw_addr = url
                .trim_start_matches("https://")
                .trim_start_matches("http://");
            let mut parts = raw_addr.splitn(2, '/');

            let authority = parts
                .next()
                .ok_or_else(|| "Invalid URL: missing authority".to_string())?
                .to_string();

            let path_with_query = match parts.next() {
                Some(p) => format!("/{}", p),
                None => "/".to_string(),
            };

            Ok((scheme, authority, path_with_query))
        }
    }
}
