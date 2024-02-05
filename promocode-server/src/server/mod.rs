use actix_web::{App, HttpServer};

mod routes;

#[allow(unused_variables)]
#[actix_web::main]
pub async fn serve(host: String, port: u16, debug: bool) -> std::io::Result<()> {
    let server = HttpServer::new(move || App::new().configure(routes::services));

    server.bind((host, port))?.run().await
}
