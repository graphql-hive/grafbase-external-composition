use graphql_composition::{
    Subgraphs, compose as gql_compose, render_api_sdl, render_federated_sdl,
};
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

async fn health() -> web::HttpResponse {
    web::HttpResponse::Ok().into()
}

async fn compose(body: web::types::Json<Vec<SchemaService>>) -> web::HttpResponse {
    let services = body.into_inner();
    let mut subgraphs = Subgraphs::default();

    for service in &services {
        let as_str = service.sdl.split_whitespace().collect::<Vec<_>>().join(" ");

        if let Err(e) = subgraphs.ingest_str(&as_str, &service.name, service.url.as_deref()) {
            let result = CompositionResult::Failure {
                result: CompositionFailureResult {
                    errors: vec![CompositionError {
                        message: e.to_string(),
                        source: ErrorSource::GraphQL,
                    }],
                },
            };
            return web::HttpResponse::Ok().json(&result);
        }
    }

    match gql_compose(&mut subgraphs).into_result() {
        Ok(graph) => match render_federated_sdl(&graph) {
            Ok(/*mut */ supergraph) => {
                // if supergraph.contains("@extension__directive")
                //     && !supergraph.contains("directive @extension__directive")
                // {
                //     supergraph.push_str("\ndirective @extension__directive(graph: join__Graph!, extension: grafbase__Extension!, name: String!, arguments: DirectiveArguments) repeatable ON FIELD | SCHEMA | SCALAR | OBJECT | FIELD_DEFINITION | ARGUMENT_DEFINITION | INTERFACE | UNION | ENUM | ENUM_VALUE | INPUT_OBJECT | INPUT_FIELD_DEFINITION");
                // }

                let sdl = render_api_sdl(&graph);
                let result = CompositionResult::Success {
                    result: CompositionSuccessResult { supergraph, sdl },
                };
                web::HttpResponse::Ok().json(&result)
            }
            Err(e) => {
                let result = CompositionResult::Failure {
                    result: CompositionFailureResult {
                        errors: vec![CompositionError {
                            message: e.to_string(),
                            source: ErrorSource::GraphQL,
                        }],
                    },
                };
                web::HttpResponse::Ok().json(&result)
            }
        },
        Err(diagnostics) => {
            let errors = diagnostics
                .iter_errors()
                .map(|msg| CompositionError {
                    message: msg.to_string(),
                    source: ErrorSource::Composition,
                })
                .collect();
            let result = CompositionResult::Failure {
                result: CompositionFailureResult { errors },
            };
            web::HttpResponse::Ok().json(&result)
        }
    }
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(|| {
        let json_config = web::types::JsonConfig::default().limit(52857600); // 50MB, super high because i want to make sure it's not blocked by this

        web::App::new()
            .state(json_config)
            .route("/health", web::get().to(health))
            .route("/compose", web::post().to(compose))
    })
    .bind("0.0.0.0:4000")?
    .run()
    .await
}
