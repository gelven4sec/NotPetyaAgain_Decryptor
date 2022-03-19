use axum::{
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use axum::response::Html;
use x25519_dalek::{PublicKey, StaticSecret};
use regex::Regex;

const PRIVATE_KEY: &str = include_str!("private_key.hex");

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/get_key", post(get_key));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root() -> Html<&'static str> {
    Html(include_str!("index.html"))
}

async fn get_key(Json(payload): Json<GetKey>, ) -> impl IntoResponse {
    let id = payload.id;
    let re = Regex::new(r"[0-9a-f]{64}").unwrap();

     return match re.is_match(&id) {
        true => {
            let mut private_buf = [0u8; 32];
            hex::decode_to_slice(PRIVATE_KEY, &mut private_buf).expect("hex to bytes");
            let private = StaticSecret::from(private_buf);

            let mut buf = [0u8; 32];
            hex::decode_to_slice(id, &mut buf).unwrap();

            let victim = PublicKey::from(buf);
            let key = private.diffie_hellman(&victim);

            let mut buf = [0u8; 64];
            hex::encode_to_slice(key.as_bytes(), &mut buf).expect("key to hex");

            let key_str = String::from_utf8(buf.to_vec()).unwrap();

            let res = Response{ status: true, data: key_str };

            (StatusCode::ACCEPTED, Json(res))
        }
        false => {
            let res = Response{ status: false, data: "invalid format".to_string() };
            (StatusCode::BAD_REQUEST, Json(res))
        }
    };
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct GetKey {
    id: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct Response {
    status: bool,
    data: String,
}
