extern crate rocket;
extern crate rocket_contrib;

use rocket::request::{FlashMessage};
use rocket_contrib::Template;

use db;
use magnet::Magnet;


#[derive(Debug, Serialize)]
struct Context<'a, 'b>{ msg: Option<(&'a str, &'b str)>, posts: Vec<Magnet> }

impl<'a, 'b> Context<'a, 'b> {
    pub fn err(conn: &db::Conn, msg: &'a str) -> Context<'static, 'a> {
        Context{msg: Some(("error", msg)), posts: Magnet::all(conn)}
    }

    pub fn raw(conn: &db::Conn, msg: Option<(&'a str, &'b str)>) -> Context<'a, 'b> {
        Context{msg: msg, posts: Magnet::all(conn)}
    }
}



#[get("/")]
fn list(msg: Option<FlashMessage>, conn: db::Conn) -> Template {
    Template::render("list",
        &match msg {
        Some(ref msg) => Context::raw(&conn, Some((msg.name(), msg.msg()))),
        None => Context::raw(&conn, None),
        })
}

