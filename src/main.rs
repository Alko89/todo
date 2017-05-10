#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate serde_json;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
#[macro_use] extern crate serde_derive;
extern crate rocket_contrib;
extern crate r2d2;
extern crate r2d2_diesel;

mod static_files;
mod task;
mod db;
mod post;
// mod magnet;

use rocket::request::{Form, FlashMessage};
use rocket::response::{Flash, Redirect};
use rocket_contrib::Template;

use task::Task;
use post::{controller as post_controller};
// use magnet::{controller as magnet_controller};

#[derive(Debug, Serialize)]
struct Context<'a, 'b>{ msg: Option<(&'a str, &'b str)>, tasks: Vec<Task> }

impl<'a, 'b> Context<'a, 'b> {
    pub fn err(conn: &db::Conn, msg: &'a str) -> Context<'static, 'a> {
        Context{msg: Some(("error", msg)), tasks: Task::all(conn)}
    }

    pub fn raw(conn: &db::Conn, msg: Option<(&'a str, &'b str)>) -> Context<'a, 'b> {
        Context{msg: msg, tasks: Task::all(conn)}
    }
}


#[post("/", data = "<todo_form>")]
fn new(todo_form: Form<Task>, conn: db::Conn) -> Flash<Redirect> {
    let todo = todo_form.into_inner();
    if todo.description.is_empty() {
        Flash::error(Redirect::to("/todo"), "Description cannot be empty.")
    } else if todo.insert(&conn) {
        Flash::success(Redirect::to("/todo"), "Todo successfully added.")
    } else {
        Flash::error(Redirect::to("/todo"), "Whoops! The server failed.")
    }
}

#[put("/<id>")]
fn toggle(id: i32, conn: db::Conn) -> Result<Redirect, Template> {
    if Task::toggle_with_id(id, &conn) {
        Ok(Redirect::to("/todo"))
    } else {
        Err(Template::render("todo", &Context::err(&conn, "Couldn't toggle task.")))
    }
}

#[delete("/<id>")]
fn delete(id: i32, conn: db::Conn) -> Result<Flash<Redirect>, Template> {
    if Task::delete_with_id(id, &conn) {
        Ok(Flash::success(Redirect::to("/todo"), "Todo was deleted."))
    } else {
        Err(Template::render("todo", &Context::err(&conn, "Couldn't delete task.")))
    }
}

#[get("/")]
fn todo(msg: Option<FlashMessage>, conn: db::Conn) -> Template {
    Template::render("todo", &match msg {
        Some(ref msg) => Context::raw(&conn, Some((msg.name(), msg.msg()))),
        None => Context::raw(&conn, None),
    })
}


fn main() {
    rocket::ignite()
        .manage(db::init_pool())
        .mount("/", routes![post_controller::post, static_files::all])
//         .mount("/magnet", routes![magnet_controller::list])
        .mount("/todo", routes![todo])
        .mount("/todo/", routes![new, toggle, delete])
        .launch();
}
