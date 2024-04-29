#[macro_use]
extern crate rocket;

use rocket::fs::NamedFile;
use rocket::http::Status;
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
) -> Result<Json<Contact>, Status> {
    if let Ok(contact) = contact_from_id(contacts, id).await {
        Ok(Json(contact))
    } else {
        Err(Status::NoContent)
    }
}

async fn contact_from_id(
    contacts: &State<Arc<Mutex<ContactsStore>>>,
    id: i32,
) -> Result<Contact, Status> {
    let local_store = contacts.lock().unwrap();
    let query = local_store.contacts.get(&id);
    match query {
        Some(&ref contact) => Ok(contact.clone()),
        None => Err(Status::NoContent),
    }
}

#[post("/", format = "json", data = "<new_contact>")]
async fn create_contact(
    contacts: &State<Arc<Mutex<ContactsStore>>>,
    new_contact: Json<Contact>,
) -> Result<Json<Contact>, Status> {
    if let Ok(contact) = add_contact(contacts, new_contact).await {
        Ok(Json(contact))
    } else {
        Err(Status::NoContent)
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

#[delete("/<id>")]
async fn delete_contact(
    contacts: &State<Arc<Mutex<ContactsStore>>>,
    id: i32,
) -> rocket::http::Status {
    let result = remove_contact(contacts, id).await;

    match result {
        Ok(_) => Status::Ok,
        Err(_) => Status::NotFound,
    }
}

async fn remove_contact(contacts: &State<Arc<Mutex<ContactsStore>>>, id: i32) -> Result<(), ()> {
    let mut local_store = contacts.lock().unwrap();

    let result = local_store.contacts.remove(&id);
    match result {
        Some(_) => Ok(()),
        None => Err(()),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(Arc::new(Mutex::new(ContactsStore::new())))
        .mount("/", routes![index])
        .mount("/api", routes![get_contact, create_contact, delete_contact])
}
