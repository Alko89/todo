extern crate rocket;
extern crate rocket_contrib;

use rocket_contrib::Template;

use db;
use magnet::Magnet;


#[derive(Debug, Serialize)]
struct Context {
    magnets: Vec<Magnet>
}

impl Context {
    pub fn all(conn: &db::Conn) -> Context {
        Context{
            magnets: Magnet::all(conn)
        }
    }
}


#[get("/")]
fn list(conn: db::Conn) -> Template {
    Template::render("list", &Context::all(&conn))
}

