use rocket_contrib::json::{Json,JsonValue};
use serde_derive::{Serialize, Deserialize};
use rocket::response::status;

use super::store;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Item {
    id: String,
    item: String 
}


#[get("/")]
pub fn index() -> JsonValue {
    json!({ "path": "index", "route": "/"})
}

#[get("/get-json")]
pub fn get_json() -> JsonValue {
    json!({"status" : "200 OK"})
}

#[get("/db?<id>")]
pub fn get_item(id: usize) -> JsonValue {

  let object = super::store::Item::read_item(id.to_string());

  print!("id is: {:?}, object is: {:?}", id, object);
  
  match object {
    Some(_) => json!(object),
    _ => { 
      status::BadRequest(Some(json!({ "status" : "400" })));
      json!({ "message" : "Bad Request" })
    }
  }
  
}


#[get("/db/all")]
pub fn get_all_items() -> JsonValue {
  let objects = super::store::Item::read_items();

  print!("vector of items is: {:?}", objects);

  json!(objects)
}

#[put("/db",format = "application/json", data="<item>")]
pub fn add_new_item(item: Json<Item>) -> JsonValue {
  
  print!("{:?}", item);
  
  let _status = super::store::Item::add_item((item).id.clone(), (item).item.clone());

  print!("Status is: {:?}", _status);

  json!({"status" : "200 OK"})
}

#[post("/db", format = "application/json", data="<item>")]
pub fn modify_item(item: Json<Item>) -> JsonValue {

  print!("{:?}", item);

  let _status = super::store::Item::modify_item((item).id.clone(), (item).item.clone());
  
  print!("Status is: {:?}", _status);

  match _status {
    Some(_) => json!({"status" : "200 OK", "id" : item.id}),
    _ => {
      status::BadRequest(Some(json!({ "status" : "400" })));
      json!({ "message" : "Bad Request" })
    }
  }
}


#[delete("/db?<id>")]
pub fn delete_item(id: usize) -> JsonValue {
  let _status = super::store::Item::remove_item(id.to_string());

  print!("id is: {:?}, status is: {:?}", id, _status);

   match _status {
    Some(_) => json!({"status" : "200 OK", "id" : id}),
    _ => { 
      status::BadRequest(Some(json!({ "status" : "400" })));
      json!({ "message" : "Bad Request" })
    }
  }
}

#[delete("/db/all")]
pub fn delete_all_items() -> JsonValue {
  let _status = super::store::Item::remove_everything();

  print!("status is: {:?}", _status);

    match _status {
      Some(_) => json!({"status" : "200 OK"}),
      _ => { 
        status::BadRequest(Some(json!({ "status" : "400" })));
        json!({ "message" : "Bad Request" })
      }
    }
}

