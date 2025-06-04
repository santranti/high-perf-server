use actix::{Actor, ActorContext, StreamHandler};
use actix_web::{web, HttpRequest, HttpResponse};
use actix_web_actors::ws;

/// A small WebSocket actor that just echoes text/binary back to the client.
pub struct EchoWs;

impl Actor for EchoWs {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for EchoWs {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        match msg {
            Ok(ws::Message::Text(text)) => ctx.text(format!("Echo: {}", text)),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Ping(p)) => ctx.pong(&p),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => {}
        }
    }
}

/// Handler for GET /ws
pub async fn ws_index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, actix_web::Error> {
    ws::start(EchoWs {}, &req, stream)
}

/// Registers the WebSocket route into an `App`
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("/ws", web::get().to(ws_index));
}
