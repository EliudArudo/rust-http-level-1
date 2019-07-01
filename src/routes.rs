use rocket_contrib::json::{JsonValue};

#[get("/")]
pub fn index() -> JsonValue {
    json!({ "path": "index", "route": "/"})
}