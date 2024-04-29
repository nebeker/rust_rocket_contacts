#[macro_use]
extern crate rocket;

use rocket::fs::NamedFile;
use rocket::response::status::NoContent;
use rocket::serde::{json::Json, Deserialize, Serialize};
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

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
struct Contact {
    id: Option<i32>,
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

#[post("/", format = "json", data = "<new_contact>")]
async fn create_contact(
    contacts: &State<Arc<Mutex<ContactsStore>>>,
    new_contact: Json<Contact>,
) -> Result<Json<Contact>, rocket::response::status::NoContent> {
    if let Ok(contact) = add_contact(contacts, new_contact).await {
        Ok(Json(contact))
    } else {
        Err(NoContent)
    }
}

async fn add_contact(
    contacts: &State<Arc<Mutex<ContactsStore>>>,
    new_contact: Json<Contact>,
) -> Result<Contact, rocket::response::status::NoContent> {
    let mut local_store = contacts.lock().unwrap();
    let mut contact = new_contact.into_inner();

    let new_id: i32;

    let max_id = local_store.contacts.keys().into_iter().max();
    match max_id {
        Some(value) => new_id = value + 1,
        None => new_id = 1,
    }

    contact.id = Some(new_id);

    local_store.contacts.insert(new_id, contact.clone());

    Ok(contact)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(Arc::new(Mutex::new(ContactsStore::new())))
        .mount("/", routes![index])
        .mount("/api", routes![get_contact, create_contact])
}
