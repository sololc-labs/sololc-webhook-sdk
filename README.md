# Sololc Webhook SDK Workspace

[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![WASI-HTTP Target](https://img.shields.io/badge/wasi--http-v0.2.0-orange.svg)](https://github.com/WebAssembly/wasi-http)

Provides a multi-crate mono-repository workspace containing a zero-dependency, ultra-high-performance WASI-HTTP Webhook SDK optimized for the Vortex and Sololc WebAssembly runtimes.

This workspace decouples standard WebAssembly Component Model (WASIp2) protocol layers from higher-level user application logic. It enables microservice developers to build sandboxed, edge-native webhooks using declarative macros with near-zero runtime and compile-time overhead.

---

## 🏗️ Workspace Architecture

The repository orchestrates multiple interlocking sub-packages alongside the official Component Model Interface (`.wit`) specifications to establish an ergonomic translation layer across guest-host boundaries:
```
Host Runtime Boundary (Vortex / Sololc)
                                 │
                                 ▼ [wasi:http/incoming-handler]
┌─────────────────────────────────────────────────────────────────────────┐
│  Sololc Webhook SDK Workspace (Guest Component)                         │
│                                                                         │
│  ┌───────────────────────────────────────────────────────────────────┐  │
│  │ 1. wit/ (Protocol Boundary)                                       │  │
│  │    Defines fields, incoming-requests, and response-outparams.     │  │
│  └─────────────────────────────────┬─────────────────────────────────┘  │
│                                    ▼                                    │
│  ┌───────────────────────────────────────────────────────────────────┐  │
│  │ 2. sololc-webhook-macros (Compile-time Synthesizer)               │  │
│  │    Parses code natively; generates high-speed token routing glue. │  │
│  └─────────────────────────────────┬─────────────────────────────────┘  │
│                                    ▼                                    │
│  ┌───────────────────────────────────────────────────────────────────┐  │
│  │ 3. sololc-webhook-sdk (Ergonomic Core Engine)                      │  │
│  │    Wraps resource-managed streams into clean Request / Response.  │  │
│  └───────────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## 📦 Repository Layout

This workspace utilizes Cargo Monorepo patterns to separate code-generation utilities from raw implementation details:

* **`wit/`**: Contains the official `wasi:http@0.2.0` component definitions (`webhook.wit`).
* **`sololc-webhook-sdk/`**: Contains the primary runtime development kit, providing higher-level, safe wrappers (`Request`, `Response`) around raw WASI handles.
* **`sololc-webhook-macros/`**: Contains a zero-dependency procedural macro suite running entirely on native compiler TokenStreams for sub-second hot-reloads.

---

## 🛠️ Prerequisites & Installation

To actively compile, test, or implement plugins utilizing this workspace, ensure the following WebAssembly toolchains are initialized on your local environment:

```bash
# 1. Update your Rust compiler toolchain to the latest stable channel
rustup update stable

# 2. Add the specialized WebAssembly WebSystem target layer
rustup target add wasm32-wasip2

# 3. Install the unified WebAssembly Component builder tooling
cargo install cargo-component --locked
```
## 🚀 Step-by-Step Local Validation
Follow these diagnostic steps within the root directory to verify structural integrity and build capabilities:

1. Execute Code Integrity Assurances
Validates compilation correctness across the entire workspace structure without emitting full binary objects:
```bash
cargo check --workspace
```

2. Compile Optimization Units
Compiles the target infrastructure and its code-generators into concrete release-ready dependencies:
```bash
cargo build --release
```

## 📝 Usage Example
Downstream plugin applications can seamlessly build on top of this workspace. Below is an example matching the routing mechanisms synthesized by this SDK:
```Rust
use sololc_webhook_sdk::{post, Request, Response};

/// Processes secure payload synchronizations dispatched from remote edge nodes.
#[post("/api/v1/trigger")]
async fn handle_webhook_event(req: Request) -> Response {
    println!("Intercepted verification request targeted to: {}", req.path());
    Response::json(201, r#"{"status": "synchronized"}"#)
}
```

## ⚖️ License
Licensed under either of:

- Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)