use lazy_static;

use serde_derive::{ Deserialize, Serialize };
use std::collections::{HashMap};
use std::sync::Mutex;

lazy_static! {
    static ref HASHMAP: Mutex<HashMap<&'static str, &'static str>> = {
        let mut m = HashMap::new();
        m.insert("10", "foo");
        Mutex::new(m)
    };
}

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ItemObject {
    id: String,
    item: String
}

pub struct Item {}

impl Item {

    pub fn add_item(id: String, item: String) -> Option<bool> {
        let mut map = HASHMAP.lock().unwrap();

        let _id = string_to_static_str(id);
        let _item = string_to_static_str(item);

        map.insert(_id, _item);
        
        Some(true)
    }

    pub fn read_item(id: String) -> Option<ItemObject> {
        let map = HASHMAP.lock().unwrap();

        let _id = string_to_static_str(id);

        match map.get(_id) {
            Some(&value) => Some(ItemObject { id: _id.to_string(), item:value.to_string() }),
            _ => None,
        }
    }

    // fn readItem(&self, id: u8) -> Option<Item>;
    // fn readItems(&self) -> Option<Item>;
    // fn modifyItem(&self, id: u8, item: String) -> Option<bool>;
    // fn removeItem(&self, id: u8) -> Option<bool>;
    // fn removeEverything(&self);

}