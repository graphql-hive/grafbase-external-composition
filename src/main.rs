use ntex::web;

async fn hello() -> &'static str {
    "hello-world"
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(|| {
        web::App::new().route("/", web::get().to(hello))
    })
    .bind("0.0.0.0:4000")?
    .run()
    .await
}
