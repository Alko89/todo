extern crate rocket;
extern crate rocket_contrib;
// extern crate tera;

use rocket::request::{Form, FlashMessage};
use rocket::response::{Flash, Redirect};
use rocket_contrib::Template;

// use self::tera::Context;

use db;
use post::Post;


#[derive(Debug, Serialize)]
struct Context<'a, 'b> 

{
    msg: Option<(&'a str, &'b str)>,
    posts: Vec<Post>
}

impl<'a, 'b> Context<'a, 'b> {
    pub fn err(id: i32, conn: &db::Conn, msg: &'a str) -> Context<'static, 'a> {
        Context{
            msg: Some(("error", msg)),
            posts: Post::post(id, conn)
        }
    }

    pub fn raw(id: i32, conn: &db::Conn, msg: Option<(&'a str, &'b str)>) -> Context<'a, 'b> {
        Context{
            msg: msg,
            posts: Post::post(id, conn)
        }
    }
}



#[get("/")]
fn post(msg: Option<FlashMessage>, conn: db::Conn) -> Template {
    Template::render("index",
        &match msg {
        Some(ref msg) => Context::raw(1, &conn, Some((msg.name(), msg.msg()))),
        None => Context::raw(1, &conn, None),
        })
}

#[get("/post/<id>")]
fn view_post(id: i32  ,msg: Option<FlashMessage>, conn: db::Conn) -> Template {
    Template::render("index",
        &match msg {
        Some(ref msg) => Context::raw(id, &conn, Some((msg.name(), msg.msg()))),
        None => Context::raw(id, &conn, None),
        })
}

#[get("/new")]
fn new_post(msg: Option<FlashMessage>, conn: db::Conn) -> Template {
     Template::render("edit_post", &match msg {
        Some(ref msg) => Context::raw(1, &conn, Some((msg.name(), msg.msg()))),
        None => Context::raw(1, &conn, None),
        })
}

#[post("/add", data = "<post_form>")]
fn add_post(post_form: Form<Post>, conn: db::Conn) -> Flash<Redirect> {
    let post = post_form.into_inner();
    if post.title.is_empty()
    {
        Flash::error(Redirect::to("/new"), "Title cannot be empty.")
    }
    else if post.body.is_empty()
    {
        Flash::error(Redirect::to("/new"), "Body cannot be empty.")
    } 
    else if post.insert(&conn)
    {
        Flash::success(Redirect::to("/post/1"), "Post successfully added.")
    } else
    {
        Flash::error(Redirect::to("/new"), "Whoops! The server failed.")
    }
}
