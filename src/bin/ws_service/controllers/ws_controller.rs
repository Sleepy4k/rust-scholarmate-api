use std::time::Instant;
use actix_web_actors::ws;
use actix_web::{web, Responder, HttpRequest};

use crate::structs::{
  main_struct::WSAppState,
  session_struct::WsChatSession,
};

#[doc = "Chat server client websocket handler"]
pub async fn chat_server(state: web::Data<WSAppState>, req: HttpRequest, stream: web::Payload) -> impl Responder {
  ws::start(
    WsChatSession {
      id: 0,
      hb: Instant::now(),
      room: "main".to_string(),
      name: None,
      addr: state.srv.clone(),
      db_pool: state.db.clone(),
    },
    &req,
    stream
  )
}