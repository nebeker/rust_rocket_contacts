#[macro_use]
extern crate rocket;

use rocket::fs::NamedFile;
use rocket::serde::{json::Json, Serialize};

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
struct Contact {
    id: i32,
    name: String,
    email: String,
}

#[get("/")]
async fn index() -> Result<NamedFile, std::io::Error> {
    NamedFile::open("wwwroot/index.html").await
}

#[get("/<id>")]
async fn get_contact(id: i32) -> Json<Contact> {
    Json(contact_from_id(id).await)
}

async fn contact_from_id(id: i32) -> Contact {
    Contact {
        id: 1,
        name: "Alice".into(),
        email: "alice@example.com".into(),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/api", routes![get_contact])
}
