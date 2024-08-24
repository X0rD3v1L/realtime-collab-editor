mod actor;
mod errors;
mod session_manager;

use crate::actor::WebSocket;
use crate::errors::ConnectionError;
use crate::session_manager::WsSessionManager;
use actix::prelude::*;
use actix_web::web::Data;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;

async fn ws_index(
    req: HttpRequest,
    stream: web::Payload,
    server_instance: web::Data<Addr<WsSessionManager>>,
) -> Result<HttpResponse, Error> {
    let ws_actor = WebSocket {
        manager: server_instance.get_ref().clone(),
    };

    ws::start(ws_actor, &req, stream)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let manager = WsSessionManager::new().start();
    let address = "0.0.0.0:8080";

    let server = HttpServer::new(move || {
        App::new()
            .route("/ws/", web::get().to(ws_index))
            .service(actix_files::Files::new("/", "public").index_file("index.html"))
            .app_data(Data::new(manager.clone()))
    })
    .workers(1)
    .bind(address);

    match server {
        Ok(srv) => {
            println!("Server listening on: {}", address);
            srv.run().await
        }
        Err(err) => {
            panic!("{}", ConnectionError::CreateServerError(err.to_string()))
        }
    }
}
