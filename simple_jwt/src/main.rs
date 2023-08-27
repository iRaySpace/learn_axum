use axum::{
    async_trait,
    extract::{FromRequestParts, TypedHeader},
    headers::{authorization::Bearer, Authorization},
    http::{request::Parts, StatusCode},
    routing::{get, post},
    Json, RequestPartsExt, Router,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Deserialize)]
struct AuthPayload {
    username: String,
    password: String,
}

#[derive(Debug, Serialize)]
struct AuthBody {
    token: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

#[derive(Debug, Serialize)]
struct MessageBody {
    message: String,
}

static SECRET_KEY: &[u8] = "SECRET_KEY".as_bytes();

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/authenticate", post(authenticate))
        .route("/protected", get(protected));
    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn authenticate(Json(payload): Json<AuthPayload>) -> Result<Json<AuthBody>, StatusCode> {
    if payload.username == "administrator" && payload.password == "password" {
        let since_the_epoch = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let exp = (since_the_epoch.as_secs() + 30) as usize;
        let claims = Claims {
            exp,
            sub: "irayspacii@gmail.com".to_owned(),
        };
        let encoding_key = EncodingKey::from_secret(SECRET_KEY);
        let token = encode(&Header::default(), &claims, &encoding_key).unwrap();
        return Ok(Json(AuthBody { token }));
    }
    Err(StatusCode::UNAUTHORIZED)
}

async fn protected(claims: Claims) -> Result<Json<MessageBody>, StatusCode> {
    Ok(Json(MessageBody {
        message: format!("Welcome, {}!", claims.sub),
    }))
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| StatusCode::UNAUTHORIZED)?;
        let decoding_key = DecodingKey::from_secret(SECRET_KEY);
        let token_data = decode::<Claims>(bearer.token(), &decoding_key, &Validation::default())
            .map_err(|_| StatusCode::UNAUTHORIZED)?;

        let Claims { exp, .. } = token_data.claims;
        let since_the_epoch = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let now = since_the_epoch.as_secs() as usize;
        if exp < now {
            return Err(StatusCode::UNAUTHORIZED);
        }

        Ok(token_data.claims)
    }
}
