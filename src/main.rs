use ntex::web;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct SchemaService {
    pub sdl: String,
    pub name: String,
    pub url: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum CompositionResult {
    Success { result: CompositionSuccessResult },
    Failure { result: CompositionFailureResult },
}

#[derive(Debug, Serialize)]
pub struct CompositionSuccessResult {
    pub supergraph: String,
    pub sdl: String,
}

#[derive(Debug, Serialize)]
pub struct CompositionFailureResult {
    pub errors: Vec<CompositionError>,
}

#[derive(Debug, Serialize)]
pub struct CompositionError {
    pub message: String,
    pub source: ErrorSource,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ErrorSource {
    GraphQL,
    Composition,
}

async fn hello() -> &'static str {
    "hello-world"
}

async fn compose(
    body: web::types::Json<Vec<SchemaService>>,
) -> web::HttpResponse {
    let services = body.into_inner();
    let combined_sdl = services.iter().map(|s| s.sdl.as_str()).collect::<Vec<_>>().join("\n");

    let result = CompositionResult::Success {
        result: CompositionSuccessResult {
            supergraph: combined_sdl.clone(),
            sdl: combined_sdl,
        },
    };

    web::HttpResponse::Ok().json(&result)
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(|| {
        web::App::new()
            .route("/", web::get().to(hello))
            .route("/compose", web::post().to(compose))
    })
        .bind("0.0.0.0:4000")?
        .run()
        .await
}
