#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]extern crate rocket;
#[macro_use]extern crate rocket_contrib;

use rocket_contrib::json::Json;
use rocket::response::Content;
use rocket::http::{ContentType, Status};
use rocket::State;
use serde::{Deserialize, Serialize}
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct FormInput{
    name: String,
    email: String
}

fn validate_input(form_input: &FormInput)-> Result<(), String>{
    if form_input.name.is_empty(){
        return Err("Name is requeired".to_string());
    }

    if form_input.email.is_empty(){
        return Err("Email is requeired".to_string());
    }
    Ok(())
}

#[get("/form")]
fn form() -> Content<String>{
    let html = r#"
    <!DOCTYPE html>
    <html>
        <head>
            <title> Form </title>
        </head>
        
        <body>
            <h1>Form example</h1>
            <form method="post" action="/form">
                <label for="name">Name: </Label>
                <input type="text" name="name">
                <br>
                <label for="email">E-mail:</label>
                <input type="email" name="email">
                <br>
                <button type="submit">Submit!</Button>
            </form>
        </body>
    </html>
    "#;
    Content(ContentType::HTML, html.to_string())
}
#[post("/form", data= "<form_input>")]
fn submit_form(form_input: Json<FormInput>) -> Result<Content<String>, Status>{
    match validate_input(&form_input){
        Ok(_) => {
            let message = format!("Name: {}\nEmail: {}", form_input.name, form_input.email)
            let html = format!(r#"
            <!DOCTYPE html>
            <html>
                <head>
                    <title> Form </title>
                </head>   
                <body>
                    <h1>Succes!</h1>
                    <p>{}<\p>
                </body>
            </html>"#, message);
            Ok(Content(ContentType::HTML, html.to_string()))
        },
        Err(error) => {
            let html = format!(r#"
            <!DOCTYPE html>
            <html>
                <head>
                    <title> Form </title>
                </head>   
                <body>
                    <h1>Error!</h1>
                    <p>{}<\p>
                    <p><a href="/form">back</a><\p>
                </body>
            </html>"#, error);
            Err(Status::UnprocessableEntity)
        }
    }
    format!("Username: {}\nPassword: {}", form_input.username, form_input.password)
}

#[get("/")]
fn index() -> &'static str {
    format!("Welcome to the API!")
}

#[catch(404)]
fn not_found() -> &'static str {
    "Page not found."
}

fn main() {

    rocket::ignite()
        .mount("/", routes![index, form, submit_form])
        .register(catchers![not_found])
        .launch();
}