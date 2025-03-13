use std::{
    sync::{
        Arc,
        atomic::{self, AtomicUsize},
    },
    time::Duration,
};

use axum::{Json, Router, extract::State, response::IntoResponse, routing::post};
use axum_body_split::SplitBody;

#[derive(Clone)]
struct AppState {
    counter: Arc<AtomicUsize>,
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", post(root)).with_state(AppState {
        counter: Default::default(),
    });

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    // sends a req to / to show that it is working
    tokio::spawn(async {
        for _ in 0..10 {
            tokio::time::sleep(Duration::from_secs(1)).await;
            let reqw = reqwest::Client::new();
            let _ = reqw
                .post("http://localhost:3000/")
                .body(r#"{"_foo": "bar"}"#)
                .header("Content-Type", "application/json")
                .send()
                .await;
        }
    });

    axum::serve(listener, app).await.unwrap();
}

#[derive(serde::Deserialize, Debug)]
struct RequestJson {
    _foo: String,
}

async fn root(
    State(state): State<AppState>,
    SplitBody(Json(json), text, _): SplitBody<Json<RequestJson>, String, AppState>,
) -> impl IntoResponse {
    dbg!(
        json,
        text,
        state.counter.fetch_add(1, atomic::Ordering::Relaxed)
    );
    "Ok"
}
