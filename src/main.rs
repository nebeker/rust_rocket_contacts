#[macro_use]
extern crate rocket;

use rocket::fs::NamedFile;

#[get("/")]
async fn index() -> Result<NamedFile, std::io::Error> {
    NamedFile::open("wwwroot/index.html").await
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
