#[cfg(unix)]
use serde_json::json;
#[cfg(unix)]
use vercel_runtime::{run, service_fn, Error, Request};

#[cfg(unix)]
#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(handler)).await
}

#[cfg(unix)]
async fn handler(_request: Request) -> Result<serde_json::Value, Error> {
    Ok(json!({
        "ok": true,
        "engine": "Rust Vercel Function",
        "protocol": 1,
        "unsafe_blocks": 0,
    }))
}

#[cfg(not(unix))]
fn main() {
    eprintln!("Vercel's Rust runtime is built for the Linux deployment target.");
}
