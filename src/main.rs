mod config;
mod tls;
mod routes;

use actix_cors::Cors;
use actix_files::Files;
use actix_web::{
    http::header,
    middleware::{Compress, DefaultHeaders, Logger},
    web, App, HttpServer,
};
use actix_web_prom::PrometheusMetricsBuilder;
use config::CONFIG;
use std::time::Duration;
use tls::load_tls;
use tracing::info;
use tracing_subscriber::{fmt, EnvFilter};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 1) Initialize tracing/logging at INFO level unconditionally
    fmt().with_env_filter(EnvFilter::new("info")).init();

    // 2) Load TLS (cert + key) so we can bind HTTPS
    info!(
        "Loading certificate from '{}' and key from '{}'",
        CONFIG.cert_path, CONFIG.key_path
    );
    let tls_cfg = load_tls(&CONFIG.cert_path, &CONFIG.key_path)?;

    // 3) Build Prometheus middleware once
    let prometheus = PrometheusMetricsBuilder::new("api").build().unwrap();

    // 4) Compute bind address & base URL for logging
    let bind_address = CONFIG.bind_address();
    let base_url = format!("https://{}", &bind_address);

    // 5) Print startup info
    info!("Server running at {}", &base_url);
    info!("Available endpoints:");
    info!("  GET  {}/api/v1/items", &base_url);
    info!("  GET  {}/ws            (WebSocket)", &base_url);
    info!("  GET  {}/metrics       (Prometheus metrics)", &base_url);
    info!("  GET  {}/              (Serves ./static/index.html)", &base_url);

    // 6) Build and run the Actix server
    HttpServer::new(move || {
        App::new()
            // Share CONFIG and Prometheus middleware as application data
            .app_data(web::Data::new(CONFIG.clone()))
            .app_data(web::Data::new(prometheus.clone()))

            // 7) Global middleware: logging, compression, security headers
            .wrap(Logger::default())
            .wrap(Compress::default())
            .wrap(
                DefaultHeaders::new()
                    .add((header::STRICT_TRANSPORT_SECURITY, "max-age=63072000; includeSubDomains"))
                    .add((header::X_CONTENT_TYPE_OPTIONS, "nosniff"))
                    .add((header::X_FRAME_OPTIONS, "DENY"))
                    .add((header::REFERRER_POLICY, "no-referrer")),
            )
            // 8) CORS: allow any origin (wildcard)
            .wrap(
                Cors::default()
                    .send_wildcard()
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT, header::CONTENT_TYPE])
                    .supports_credentials()
                    .max_age(3600),
            )
            // 9) Expose Prometheus middleware at every request
            .wrap(prometheus.clone())

            // 10) Register all “named” route groups (API, WebSocket, metrics)
            .configure(routes::register_all)

            // 11) Serve static files from "./static"; index_file = index.html
            .service(
                Files::new("/", "./static")
                    .index_file("index.html")
                    .prefer_utf8(true),
            )
    })
        // 12) Bind HTTPS using the version‐specific method for rustls 0.21
        .bind_rustls_021(bind_address, tls_cfg)?
        .workers(num_cpus::get())
        .backlog(2048)
        .client_request_timeout(Duration::from_secs(10))
        .client_disconnect_timeout(Duration::from_secs(5))
        .keep_alive(Duration::from_secs(75))
        .max_connections(10_000)
        .run()
        .await
}
