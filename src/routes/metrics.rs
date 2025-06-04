// src/routes/metrics.rs

use actix_web::{web, HttpResponse};
use actix_web_prom::PrometheusMetrics;
use prometheus::{Encoder, TextEncoder};

/// Handler for GET /metrics
pub async fn metrics(prom: web::Data<PrometheusMetrics>) -> HttpResponse {
    let metric_families = prom.registry.gather();
    let mut buffer = Vec::<u8>::new();
    TextEncoder::new()
        .encode(&metric_families, &mut buffer)
        .unwrap();

    HttpResponse::Ok()
        .content_type("text/plain; version=0.0.4")
        .body(buffer)
}

/// Registers the /metrics route
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("/metrics", web::get().to(metrics));
}
