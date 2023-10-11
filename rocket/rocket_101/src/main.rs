#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, Rocket!"
}

#[catch(404)]
fn not_found() -> &'static str {
    "Page not found."
}

#[get("/users/<id>")]
fn get_user(id: u32) -> String {
    format!("Returning the user with ID: {}", id)
}

#[post("/users/<name>")]
fn create_user(name: String) {
    println!("Creating user with name: {}", name);
}

#[delete("/users/<id>")]
fn delete_user(id: u32) {
    println!("Deleting user with ID: {}", id);
}

#[put("/users?<id>&<name>")]
fn update_user(id: u32, name: String) {
    println!("Updating the user with ID: {} to Name: {}", id, name);
}

#[get("/users?<query>&<page>")]
fn search_users(query: String, page: Option<u32>) -> String {
    match page {
        Some(p) => format!("Searching for users with the query '{}' on page {}", query, p),
        None => format!("Searching for users with the query '{}' (no page specified)", query),
    }
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, get_user, create_user, delete_user, update_user, search_users])
        .register(catchers![not_found])
        .launch();
}
