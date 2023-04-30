// Crates
use actix_rt::task::spawn_blocking;
use actix_web::http::StatusCode;
use actix_web::{get, web, App, Error, HttpResponse, HttpServer};
use rust_summarization::summarize_text;
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    text: String,
}

#[get("/")]
async fn index() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../templates/index.html")))
}

#[get("/summary")]
async fn get_summary(info: web::Query<Info>) -> Result<HttpResponse, Error> {
    // Get the summary
    let summary = spawn_blocking(move || summarize_text(&info.text))
        .await
        .unwrap();

    let html_string = include_str!("../templates/index.html").to_owned() + format!("<label>Summary:</label> <br> <textarea id=\"summary\" name=\"summary\" rows=\"16\" cols=\"150\" READONLY>{}</textarea> <br><br>", summary).as_str();

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(html_string))
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    //add a print message to the console that the service is running
    println!("Running the service");

    // Start the server
    HttpServer::new(|| App::new().service(index).service(get_summary))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
