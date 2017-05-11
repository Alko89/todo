extern crate rocket;
// extern crate rocket_contrib;
extern crate serde_json;

use rocket_contrib::{Template, JSON};

use db;
use magnet::Magnet;

#[derive(Serialize, Deserialize)]
struct Message {
    search_string: String,
    magnets_total: i64,
    magnets_filtered: i32,
    magnets: Vec<Magnet>
}

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

#[get("/search/<query>/<from>/<to>")]
fn search(query: String, from: i32, to: i32, conn: db::Conn) -> JSON<Message> {
    JSON(Message{
        search_string: query,
        magnets_total: Magnet::count(&conn),
        magnets_filtered: from + to,
        magnets: Magnet::all(&conn)
    })
}
