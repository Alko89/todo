extern crate rocket;
extern crate rocket_contrib;

use rocket::request::{FlashMessage};
use rocket_contrib::Template;

use db;
use post::Post;


#[derive(Debug, Serialize)]
struct Context<'a, 'b>{ msg: Option<(&'a str, &'b str)>, posts: Vec<Post> }

impl<'a, 'b> Context<'a, 'b> {
    pub fn err(conn: &db::Conn, msg: &'a str) -> Context<'static, 'a> {
        Context{msg: Some(("error", msg)), posts: Post::all(conn)}
    }

    pub fn raw(conn: &db::Conn, msg: Option<(&'a str, &'b str)>) -> Context<'a, 'b> {
        Context{msg: msg, posts: Post::all(conn)}
    }
}



#[get("/")]
fn index(msg: Option<FlashMessage>, conn: db::Conn) -> Template {
    Template::render("index",
        &match msg {
        Some(ref msg) => Context::raw(&conn, Some((msg.name(), msg.msg()))),
        None => Context::raw(&conn, None),
        })
}

