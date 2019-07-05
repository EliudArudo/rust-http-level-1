use rocket_contrib::json::{Json,JsonValue};
use serde_derive::{Serialize, Deserialize};

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

#[put("/db",format = "application/json", data="<item>")]
pub fn add_new_item(item: Json<Item>) -> JsonValue {
  
  print!("{:?}", item);
  
  let _status = super::store::Item::add_item((item).id.clone(), (item).item.clone());

  print!("Status is: {:?}", _status);

  json!({"status" : "200 OK"})
}
