use actix_web::{App, HttpServer};

mod routes;

/// Starts the HTTP server at the specified host and port.
///
/// # Arguments
///
/// - `host` - The IP address or hostname to bind the server to.
/// - `port` - The port number to bind the server to.
///
/// # Returns
///
/// A [Result] indicating the success or failure of the server startup.
#[allow(unused_variables)]
#[actix_web::main]
pub async fn serve(host: String, port: u16, debug: bool) -> std::io::Result<()> {
    let server = HttpServer::new(move || App::new().configure(routes::services));

    server.bind((host, port))?.run().await
}
