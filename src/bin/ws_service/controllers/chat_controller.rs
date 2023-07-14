use actix_web::{web, Responder};

use crate::{
  repositories::chat_repository::*,
  repositories::room_repository::*,
  schemas::chat_schema::*,
  structs::main_struct::WSAppState,
};

use scholarmate_api::helpers::response::response_json;

pub async fn get_chat(state: web::Data<WSAppState>, body: web::Query<GeneralChatSchema>) -> impl Responder {
  let chats = fetch_room_data_by_user_id(state.db.clone(), body.uid.to_owned()).await;

  response_json(
    String::from("Success"),
    String::from("Successfully fetch chat data"),
    chats
  )
}

pub async fn add_chat(state: web::Data<WSAppState>, body: web::Json<DetailChatSchema>) -> impl Responder {
  let members = format!("{},{}", body.sender, body.reciver);
  let chat = insert_room_data(state.db.clone(), body.name.to_owned(), members).await;

  response_json(
    String::from("Success"),
    String::from("Successfully add chat data"),
    chat
  )
}

pub async fn find_chat(state: web::Data<WSAppState>, body: web::Query<GeneralChatSchema>, path: web::Path<i32>) -> impl Responder {
  let room_id = path.into_inner();

  match fetch_room_data_by_room_id(state.db.clone(), room_id).await {
    Some(room_data) => {
      let filtered = room_data.members.contains(&body.uid.to_string().as_str());

      if filtered {
        let chats = fetch_chat_data_by_room_id(state.db.clone(), room_id).await;

        response_json(
          String::from("Success"),
          String::from("Successfully find chat data"),
          chats
        )
      } else {
        response_json(
          String::from("Failed"),
          String::from("You are not member of this room"),
          vec![]
        )
      }
    },
    None => {
      return response_json(
        String::from("Failed"),
        String::from("Room not found"),
        vec![]
      )
    }
  }
}

pub async fn delete_chat(state: web::Data<WSAppState>, body: web::Json<DetailChatSchema>, path: web::Path<i32>) -> impl Responder {
  let room_id = path.into_inner();

  match fetch_room_data_by_room_id(state.db.clone(), room_id).await {
    Some(room_data) => {
      let members = format!("{},{}", body.sender, body.reciver);
      let filtered = room_data.members.contains(&members) && room_data.name == body.name;

      if filtered {
        let room = delete_room_data(state.db.clone(), room_id).await;

        response_json(
          String::from("Success"),
          String::from("Successfully find chat data"),
          room
        )
      } else {
        response_json(
          String::from("Failed"),
          String::from("You are not member of this room"),
          vec![]
        )
      }
    },
    None => {
      return response_json(
        String::from("Failed"),
        String::from("Room not found"),
        vec![]
      )
    }
  }
}