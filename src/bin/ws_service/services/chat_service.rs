use serde_json::Value;
use sqlx::{Pool, Postgres};
use std::collections::{HashMap, HashSet};

use crate::{
  models::chat_model::*,
  schemas::chat_schema::*,
  repositories::{
    chat_repository::*,
    room_repository::find_room_data,
  }
};

use scholarmate_api::{
  helpers::parse::convert_vec_to_values,
  enums::{
    error_enum::ErrorEnum,
    response_enum::ResponseDataEnum
  },
  repositories::{
    main_repository::check_data,
    auth_repository::get_user_data
  }
};

#[doc = "Display the specified resource."]
pub async fn chat_show_service(db: Pool<Postgres>, id: i32, body: GeneralChatSchema) -> anyhow::Result<ResponseDataEnum, ErrorEnum> {
  let query_is_chat_exist = format!("select 1 from chats where room_id = '{}'", id);
  let is_chat_exist = check_data(db.clone(), query_is_chat_exist.as_str()).await;

  if is_chat_exist.is_err() {
    return Err(is_chat_exist.err().unwrap());
  } else if !is_chat_exist.unwrap() {
    return Err(ErrorEnum::CustomError(String::from("chat not exist")));
  }
  
  let rooms_data = find_room_data(db.clone(), id).await;

  if rooms_data.is_err() {
    return Err(rooms_data.err().unwrap());
  }

  let mut ids = HashSet::new();
  let mut rooms_map = HashMap::new();
  let data = vec![rooms_data.clone().unwrap()];

  if !rooms_data.clone().unwrap().members.contains(body.uid.to_string().as_str()) {
    return Err(ErrorEnum::CustomError(String::from("you are not a member of this room")));
  }

  for room in &data {
    let user_ids = room
      .members
      .split(',')
      .collect::<Vec<_>>();

    #[allow(clippy::unnecessary_to_owned)]
    for id in user_ids.to_owned() {
      ids.insert(id.to_string());
    }

    rooms_map.insert(room.id.to_string(), user_ids);
  }

  let ids = ids.into_iter().collect::<Vec<_>>();

  let users_data = match get_user_data(db.clone()).await {
    Ok(data) => convert_vec_to_values(data),
    Err(_) => vec![]
  };

  let filtered_users_data = users_data
    .into_iter()
    .filter(|item| ids.contains(&item["id"].to_string()))
    .collect::<Vec<_>>();

  let chats_data = match find_chat_data(db.clone(), id).await {
    Ok(data) => convert_vec_to_values(vec![data]),
    Err(_) => vec![]
  };

  let users_map: HashMap<String, Value> = HashMap::from_iter(
    filtered_users_data
      .into_iter()
      .map(|item| (item["id"].to_string(), item)),
  );

  #[allow(clippy::needless_return)]
  let response_chats = rooms_data.into_iter().map(move |room| {
    let users = rooms_map
      .get(&room.id.to_string())
      .unwrap()
      .iter()
      .map(|id| users_map.get(id.to_owned()).unwrap().clone())
      .collect::<Vec<_>>();
   
    let chats = chats_data.clone();
    
    return ChatResponseModel{ room, users, chats };
  }).collect::<Vec<_>>();

  let converted_data = convert_vec_to_values(response_chats);
  let body = ResponseDataEnum::ArrayValue(converted_data);

  Ok(body)
}