#[macro_use]
extern crate rocket;

use rocket::fs::NamedFile;
use rocket::response::status::NoContent;
use rocket::serde::{json::Json, Serialize};
use rocket::State;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

struct ContactsStore {
    contacts: HashMap<i32, Contact>,
}

impl ContactsStore {
    fn new() -> ContactsStore {
        ContactsStore {
            contacts: HashMap::new(),
        }
    }
}

#[derive(Debug, Serialize, Clone)]
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

async fn get_contact(
    contacts: &State<Arc<Mutex<ContactsStore>>>,
    id: i32,
) -> Result<Json<Contact>, rocket::response::status::NoContent> {
    if let Ok(contact) = contact_from_id(contacts, id).await {
        Ok(Json(contact))
    } else {
        Err(NoContent)
    }
}

async fn contact_from_id(
    contacts: &State<Arc<Mutex<ContactsStore>>>,
    id: i32,
) -> Result<Contact, rocket::response::status::NoContent> {
    let local_store = contacts.lock().unwrap();
    let query = local_store.contacts.get(&id);
    match query {
        Some(&ref contact) => Ok(contact.clone()),
        None => Err(rocket::response::status::NoContent),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(Arc::new(Mutex::new(ContactsStore::new())))
        .mount("/", routes![index])
        .mount("/api", routes![get_contact])
}
