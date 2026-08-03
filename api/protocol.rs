#[cfg(unix)]
#[allow(dead_code)]
#[path = "../src/main.rs"]
mod runner;

#[cfg(unix)]
use std::sync::{Mutex, OnceLock};

#[cfg(unix)]
use http_body_util::BodyExt;
#[cfg(unix)]
use serde_json::{json, Value};
#[cfg(unix)]
use vercel_runtime::{run, service_fn, Error, Request, Response, ResponseBody};

#[cfg(unix)]
static COLLECTIONS: OnceLock<Mutex<runner::ProtocolState>> = OnceLock::new();

#[cfg(unix)]
#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(handler)).await
}

#[cfg(unix)]
fn json_response(status: u16, value: Value) -> Result<Response<ResponseBody>, Error> {
    Ok(Response::builder()
        .status(status)
        .header("content-type", "application/json; charset=utf-8")
        .header("cache-control", "no-store")
        .body(ResponseBody::from(value.to_string()))?)
}

#[cfg(unix)]
async fn handler(request: Request) -> Result<Response<ResponseBody>, Error> {
    if request.method().as_str() != "POST" {
        return json_response(405, json!({"ok": false, "error": "POST required"}));
    }

    let body = request.into_body().collect().await?.to_bytes();
    if let Ok(batch) = serde_json::from_slice::<runner::BatchRequest>(&body) {
        return json_response(200, runner::execute_batch(batch.requests));
    }

    let request = match serde_json::from_slice::<runner::Request>(&body) {
        Ok(request) => request,
        Err(error) => return json_response(400, json!({"ok": false, "error": error.to_string()})),
    };

    let state = COLLECTIONS.get_or_init(|| Mutex::new(runner::ProtocolState::new()));
    let Ok(mut state) = state.lock() else {
        return json_response(
            500,
            json!({"ok": false, "error": "Rust collection state is unavailable"}),
        );
    };

    json_response(200, state.execute(request))
}

#[cfg(not(unix))]
fn main() {
    eprintln!("Vercel's Rust runtime is built for the Linux deployment target.");
}
