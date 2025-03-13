use std::time::Duration;

use axum::{Json, Router, response::IntoResponse, routing::post};
use axum_body_split::SplitBody;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", post(root));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    // sends a req to / to show that it is working
    tokio::spawn(async {
        tokio::time::sleep(Duration::from_secs(1)).await;
        let reqw = reqwest::Client::new();
        let _ = reqw
            .post("http://localhost:3000/")
            .body(r#"{"_foo": "bar"}"#)
            .header("Content-Type", "application/json")
            .send()
            .await;
    });

    axum::serve(listener, app).await.unwrap();
}

#[derive(serde::Deserialize, Debug)]
struct RequestJson {
    _foo: String,
}

async fn root(
    SplitBody(Json(json), text, _): SplitBody<Json<RequestJson>, String, ()>,
) -> impl IntoResponse {
    dbg!(json, text);
    "Ok"
}
