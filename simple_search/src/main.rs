use axum::{extract::Form, response::Html, routing::get, Router};
use serde::Deserialize;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(get_form).post(post_form));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_form() -> Html<&'static str> {
    Html(
        r#"
        <!doctype html>
        <html>
            <head></head>
            <body>
                <form action="/" method="post">
                    <label for="search">
                        Search:
                        <input type="text" name="search">
                    </label>
                    <input type="submit" value="Search">
                </form>
            </body>
        </html>
        "#,
    )
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct Input {
    search: String,
}

async fn post_form(Form(input): Form<Input>) -> Html<&'static str> {
    let out = format!(
        r#"
    <!doctype html>
    <html>
        <head></head>
        <body>
            <h1>You searched {}</h1>
        </body>
    </html>
    "#,
        input.search,
    );
    Html(Box::leak(out.into_boxed_str()))
}
