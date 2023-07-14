use actix::prelude::*;
use actix_web_actors::ws;
use sqlx::{Pool, Postgres};
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};

use futures::FutureExt;

use crate::{
  repositories::chat_repository::insert_chat_data,
  structs::{
    main_struct::Message,
    message_struct::{ChatServer, ClientMessage},
    ws_struct::{Connect, Disconnect},
  }
};

const HEARBEET: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Debug)]
pub struct WsChatSession {
  pub id: i32,
  pub hb: Instant,
  pub room: String,
  pub name: Option<String>,
  pub addr: Addr<ChatServer>,
  pub db_pool: Pool<Postgres>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
  pub id: i32,
  pub room_id: i32,
  pub user_id: i32,
  pub message: String,
}

impl Actor for WsChatSession {
  type Context = ws::WebsocketContext<Self>;

  fn started(&mut self, ctx: &mut Self::Context) {
    self.hb(ctx);

    let addr = ctx.address();

    self.addr
      .send(Connect {
        addr: addr.recipient(),
      })
      .into_actor(self)
      .then(|res, act, ctx| {
        match res {
          Ok(res) => act.id = res,
          _ => ctx.stop(),
        }
        fut::ready(())
      })
      .wait(ctx);
  }

  fn stopping(&mut self, _: &mut Self::Context) -> Running {
    self.addr.do_send(Disconnect { id: self.id });
    Running::Stop
  }
}

impl Handler<Message> for WsChatSession {
  type Result = ();

  fn handle(&mut self, msg: Message, ctx: &mut Self::Context) -> Self::Result {
    ctx.text(msg.0);
  }
}

async fn query_data(db_pool: Pool<Postgres>, chat_msg: ChatMessage) -> () {
  let _ = insert_chat_data(db_pool, chat_msg).await;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsChatSession {
  fn handle(&mut self, item: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
    let msg = match item {
      Err(_) => {
        ctx.stop();
        return;
      }

      Ok(msg) => msg,
    };

    match msg {
      ws::Message::Ping(msg) => {
        self.hb = Instant::now();
        ctx.pong(&msg);
      }

      ws::Message::Pong(_) => {
        self.hb = Instant::now();
      }

      ws::Message::Text(text) => {
        let data_json = serde_json::from_str::<ChatMessage>(&text.to_string());

        if let Err(err) = data_json {
          println!("Failed to parse message: {err}");
          return;
        }

        let input = data_json.as_ref().unwrap();
        let conn = self.db_pool.clone();
        let chat_msg = ChatMessage {
          id: self.id,
          room_id: input.room_id,
          user_id: input.user_id,
          message: input.message.to_owned(),
        };

        actix_web::rt::spawn(query_data(conn, chat_msg.clone()).map(|_| ()));

        let msg = serde_json::to_string(&chat_msg).unwrap();

        self.addr.do_send(ClientMessage {
          id: self.id,
          msg,
          room: self.room.clone(),
        })
      }

      ws::Message::Binary(_) => println!("Unsupported binary"),

      ws::Message::Close(reason) => {
        ctx.close(reason);
        ctx.stop();
      }

      ws::Message::Continuation(_) => {
        ctx.stop();
      }

      ws::Message::Nop => (),
    }
  }
}

impl WsChatSession {
  fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
    ctx.run_interval(HEARBEET, |act, ctx| {
      if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
        act.addr.do_send(Disconnect { id: act.id });
        ctx.stop();
        return;
      }

      ctx.ping(b"");
    });
  }
}