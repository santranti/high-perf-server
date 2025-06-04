pub mod api;
pub mod websocket;
pub mod metrics;

/// A convenience function to register all “named” route groups at once.
///
/// You can call this from `main.rs` like:
///     .configure(routes::register_all)
pub fn register_all(cfg: &mut actix_web::web::ServiceConfig) {
    // Register /api/v1/*
    cfg.service(api::api_scope());

    // Register /ws
    websocket::configure(cfg);

    // Register /metrics
    metrics::configure(cfg);
}
